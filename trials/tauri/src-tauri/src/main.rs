//! Variants T-native and T-wasm of the T-02 comparison, and the trial screen, as a Tauri 2 app. The
//! UI is Leptos, in `trials/tauri/ui`.
//!
//! - `--size panel|building --core native|wasm [--slow <ms>] [--bench <out.json>]` shows the bench
//!   screen; with `--bench` it plays the timing script, writes the results and quits. With `--core native` the UI reaches the core through the commands below; with `wasm`
//!   the core runs inside the UI, and only project bytes cross the bridge.
//! - `--self-test <file> [--update-feed <url>]` proves a signed update installs and relaunches.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use bench_core::{Command, EDITED, Finding, Project, Row, Scene, Size};
use serde::Serialize;
use tauri::menu::{
    AboutMetadata, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder,
};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_updater::UpdaterExt;

struct Args {
    bench: Option<PathBuf>,
    /// A size was given: show the bench screen, to try by hand when `--bench` isn't given.
    screen: bool,
    size: Size,
    wasm_core: bool,
    capacity: bool,
    slow_ms: u64,
    self_test: Option<PathBuf>,
    update_feed: Option<String>,
    /// Where to write what happened, for diagnosing runs on machines without a console.
    log: Option<PathBuf>,
}

impl Args {
    fn read() -> Args {
        let mut args = Args {
            bench: None,
            screen: false,
            size: Size::Panel,
            wasm_core: false,
            capacity: false,
            slow_ms: 0,
            self_test: None,
            update_feed: None,
            log: None,
        };
        let mut given = std::env::args().skip(1);
        while let Some(arg) = given.next() {
            match arg.as_str() {
                "--bench" => args.bench = given.next().map(PathBuf::from),
                "--size" => {
                    args.screen = true;
                    args.size = given
                        .next()
                        .as_deref()
                        .and_then(Size::parse)
                        .unwrap_or(Size::Panel);
                }
                "--core" => args.wasm_core = given.next().as_deref() == Some("wasm"),
                "--capacity" => args.capacity = true,
                "--slow" => args.slow_ms = given.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                "--self-test" => args.self_test = given.next().map(PathBuf::from),
                "--update-feed" => args.update_feed = given.next(),
                "--log" => args.log = given.next().map(PathBuf::from),
                _ => {}
            }
        }
        args
    }
}

struct Launch {
    args: Args,
    started: Instant,
    start_epoch_ms: f64,
}

/// The core, when it runs here (T-native).
struct Core(Mutex<Option<Project>>);

/// What the UI needs to know at start.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    mode: &'static str,
    size: String,
    core: &'static str,
    capacity: bool,
    slow_ms: u64,
    start_epoch_ms: f64,
}

/// Writes a line to the `--log` file, if one was given.
fn log(launch: &Launch, line: &str) {
    if let Some(path) = &launch.args.log {
        note(path, line);
    }
}

/// The UI reports its errors here, since a desktop webview has no console to read.
#[tauri::command]
fn ui_log(launch: State<'_, Launch>, message: String) {
    log(&launch, &format!("ui: {message}"));
}

#[tauri::command]
fn settings(launch: State<'_, Launch>) -> Settings {
    log(&launch, "the UI asked for its settings");
    let args = &launch.args;
    Settings {
        mode: match (&args.bench, args.screen) {
            (Some(_), _) => "bench",
            (None, true) => "screen",
            (None, false) => "trial",
        },
        size: args.size.name(),
        core: if args.wasm_core { "wasm" } else { "native" },
        capacity: args.capacity,
        slow_ms: args.slow_ms,
        start_epoch_ms: launch.start_epoch_ms,
    }
}

fn with_core<T>(core: &State<'_, Core>, f: impl FnOnce(&mut Project) -> T) -> Result<T, String> {
    let mut guard = core.0.lock().map_err(|e| e.to_string())?;
    guard
        .as_mut()
        .map(f)
        .ok_or_else(|| "the project isn't open".to_string())
}

// The core's commands. They're async so they run off the window's thread, as Tauri advises for
// real work.

#[tauri::command(async)]
fn core_open(
    core: State<'_, Core>,
    launch: State<'_, Launch>,
) -> Result<(Scene, Vec<Finding>), String> {
    let project = Project::generate(launch.args.size, 1);
    let opened = (project.scene(), project.check());
    *core.0.lock().map_err(|e| e.to_string())? = Some(project);
    Ok(opened)
}

#[tauri::command(async)]
fn core_move(core: State<'_, Core>, part: u32, x: f32, y: f32) -> Result<(), String> {
    with_core(&core, |p| p.apply(Command::Move { part, x, y }))
}

#[tauri::command(async)]
fn core_set_address(
    core: State<'_, Core>,
    launch: State<'_, Launch>,
    address: u8,
) -> Result<Vec<Finding>, String> {
    let slow = launch.args.slow_ms;
    with_core(&core, |p| {
        p.apply(Command::SetAddress {
            part: EDITED,
            address,
        });
        if slow > 0 {
            let until = Instant::now() + std::time::Duration::from_millis(slow);
            while Instant::now() < until {}
        }
        p.check()
    })
}

#[tauri::command(async)]
fn core_sheet(core: State<'_, Core>, page: u16) -> Result<String, String> {
    with_core(&core, |p| p.sheet_svg(page))
}

#[tauri::command(async)]
fn core_overview(core: State<'_, Core>) -> Result<Vec<String>, String> {
    with_core(&core, |p| {
        (0..p.pages).map(|page| p.sheet_svg(page)).collect()
    })
}

#[tauri::command(async)]
fn core_rows(core: State<'_, Core>, filter: String) -> Result<(Vec<Row>, usize), String> {
    with_core(&core, |p| p.rows(&filter, 500))
}

/// An outside write: the other generated project, read back from its bytes, here.
#[tauri::command(async)]
fn core_reload(
    core: State<'_, Core>,
    launch: State<'_, Launch>,
    seed: u64,
) -> Result<(Scene, Vec<Finding>), String> {
    let project = Project::from_bytes(&Project::generate(launch.args.size, seed).to_bytes())?;
    let reloaded = (project.scene(), project.check());
    *core.0.lock().map_err(|e| e.to_string())? = Some(project);
    Ok(reloaded)
}

/// The same outside write for T-wasm: the bytes cross the bridge and the UI reads them.
#[tauri::command(async)]
fn project_bytes(launch: State<'_, Launch>, seed: u64) -> Result<tauri::ipc::Response, String> {
    Ok(tauri::ipc::Response::new(
        Project::generate(launch.args.size, seed).to_bytes(),
    ))
}

#[tauri::command]
fn bench_done(app: AppHandle, launch: State<'_, Launch>, results: String) {
    if let Some(out) = &launch.args.bench
        && let Err(e) = std::fs::write(out, results)
    {
        eprintln!("couldn't write {}: {e}", out.display());
    }
    app.exit(0);
}

/// The trial screen's first render. With `--self-test`, it notes the time, then updates or exits.
#[tauri::command]
async fn rendered(app: AppHandle, launch: State<'_, Launch>) -> Result<(), String> {
    log(&launch, "the trial screen rendered");
    let Some(marker) = launch.args.self_test.clone() else {
        return Ok(());
    };
    let millis = launch.started.elapsed().as_millis();
    note(
        &marker,
        &format!("{} rendered in {millis} ms", app.package_info().version),
    );
    let outcome = update(&app, launch.args.update_feed.as_deref()).await;
    note(&marker, &outcome);
    app.exit(0);
    Ok(())
}

#[tauri::command]
async fn check_for_updates(app: AppHandle, launch: State<'_, Launch>) -> Result<String, String> {
    Ok(update(&app, launch.args.update_feed.as_deref()).await)
}

/// Checks the feed and, when it has a newer, correctly signed version, installs it and relaunches.
/// Returns what happened, unless it relaunched.
async fn update(app: &AppHandle, feed: Option<&str>) -> String {
    let mut builder = app.updater_builder();
    if let Some(feed) = feed {
        match feed
            .parse::<tauri::Url>()
            .map_err(|e| e.to_string())
            .and_then(|url| builder.endpoints(vec![url]).map_err(|e| e.to_string()))
        {
            Ok(with_feed) => builder = with_feed,
            Err(e) => return format!("The update feed isn't valid: {e}"),
        }
    }
    let updater = match builder.build() {
        Ok(updater) => updater,
        Err(e) => return format!("Updates aren't set up: {e}"),
    };
    match updater.check().await {
        Ok(Some(update)) => match update.download_and_install(|_, _| {}, || {}).await {
            Ok(()) => app.restart(),
            Err(e) => format!("The update didn't install: {e}"),
        },
        Ok(None) => format!("Hyleon {} is up to date.", app.package_info().version),
        Err(e) => format!("The update check failed: {e}"),
    }
}

fn note(marker: &Path, line: &str) {
    let written = OpenOptions::new()
        .create(true)
        .append(true)
        .open(marker)
        .and_then(|mut file| writeln!(file, "{line}"));
    if let Err(e) = written {
        eprintln!("The self-test couldn't write to {}: {e}", marker.display());
    }
}

/// The menu bar, laid out the way each operating system expects, as in the Dioxus trial.
fn menu_bar(app: &AppHandle) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    let zoom_in = MenuItemBuilder::with_id("zoom-in", "Zoom In")
        .accelerator("CmdOrCtrl+=")
        .build(app)?;
    let zoom_out = MenuItemBuilder::with_id("zoom-out", "Zoom Out")
        .accelerator("CmdOrCtrl+-")
        .build(app)?;
    let fit = MenuItemBuilder::with_id("zoom-to-fit", "Zoom to Fit")
        .accelerator("CmdOrCtrl+0")
        .build(app)?;
    let updates = MenuItemBuilder::with_id("check-for-updates", "Check for Updates…").build(app)?;
    let about = || AboutMetadata {
        name: Some("Hyleon".to_string()),
        version: Some(app.package_info().version.to_string()),
        ..Default::default()
    };
    let view = SubmenuBuilder::new(app, "View")
        .items(&[&zoom_in, &zoom_out, &fit])
        .build()?;
    if cfg!(target_os = "macos") {
        let app_menu = SubmenuBuilder::new(app, "Hyleon")
            .about(Some(about()))
            .separator()
            .item(&updates)
            .separator()
            .hide()
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;
        let edit = SubmenuBuilder::new(app, "Edit")
            .undo()
            .redo()
            .separator()
            .cut()
            .copy()
            .paste()
            .select_all()
            .build()?;
        let window = SubmenuBuilder::new(app, "Window")
            .minimize()
            .maximize()
            .fullscreen()
            .build()?;
        MenuBuilder::new(app)
            .items(&[&app_menu, &edit, &view, &window])
            .build()
    } else {
        let exit = PredefinedMenuItem::quit(app, Some("Exit"))?;
        let file = SubmenuBuilder::new(app, "File").item(&exit).build()?;
        let edit = SubmenuBuilder::new(app, "Edit")
            .cut()
            .copy()
            .paste()
            .select_all()
            .build()?;
        let help = SubmenuBuilder::new(app, "Help")
            .item(&updates)
            .about(Some(about()))
            .build()?;
        MenuBuilder::new(app)
            .items(&[&file, &edit, &view, &help])
            .build()
    }
}

fn main() {
    let started = Instant::now();
    let start_epoch_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64() * 1000.0);
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(Launch {
            args: Args::read(),
            started,
            start_epoch_ms,
        })
        .manage(Core(Mutex::new(None)))
        .setup(|app| {
            let menu = menu_bar(app.handle())?;
            app.set_menu(menu)?;
            log(&app.state::<Launch>(), "started");
            Ok(())
        })
        .on_menu_event(|app, event| {
            let id = event.id().as_ref().to_string();
            if id == "check-for-updates" {
                let app = app.clone();
                let _ = app.emit("status", "Checking for updates…");
                tauri::async_runtime::spawn(async move {
                    let feed = app.state::<Launch>().args.update_feed.clone();
                    let status = update(&app, feed.as_deref()).await;
                    let _ = app.emit("status", status);
                });
            } else {
                let _ = app.emit("menu", id);
            }
        })
        .invoke_handler(tauri::generate_handler![
            ui_log,
            settings,
            core_open,
            core_move,
            core_set_address,
            core_sheet,
            core_overview,
            core_rows,
            core_reload,
            project_bytes,
            bench_done,
            rendered,
            check_for_updates,
        ])
        .run(tauri::generate_context!())
        .expect("the trial app runs");
}
