//! The Leptos + Tauri trial that D-043 chose (T-02), kept as the reference for Hyleon's app: the
//! trial screen, native menus, and Tauri's signed updates. The UI is Leptos, in `trials/tauri/ui`.
//!
//! - `--self-test <file> [--update-feed <url>]` proves a signed update installs and relaunches.
//! - `--log <file>` records what happened, for machines without a console.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use tauri::menu::{
    AboutMetadata, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder,
};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_updater::UpdaterExt;

#[derive(Default)]
struct Args {
    self_test: Option<PathBuf>,
    update_feed: Option<String>,
    log: Option<PathBuf>,
}

impl Args {
    fn read() -> Args {
        let mut args = Args::default();
        let mut given = std::env::args().skip(1);
        while let Some(arg) = given.next() {
            match arg.as_str() {
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

/// The menu bar, laid out the way each operating system expects (DESIGN.md, Familiarity).
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
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(Launch {
            args: Args::read(),
            started,
        })
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
        .invoke_handler(tauri::generate_handler![ui_log, rendered])
        .run(tauri::generate_context!())
        .expect("the trial app runs");
}
