//! Desktop only: the native menu bar, updates through Velopack (D-003), and the trial's self-test.
//!
//! Two arguments drive the trial:
//! - `--update-feed <folder>`: a folder of Velopack packages to update from.
//! - `--self-test <file>`: after the first render, append "<version> rendered in N ms" to the file,
//!   then update and restart if the feed has a newer version, or exit. CI uses it to prove an
//!   update installs and relaunches.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Instant;

use dioxus::desktop::muda::accelerator::{Accelerator, CMD_OR_CTRL, Code};
use dioxus::desktop::muda::{
    AboutMetadata, IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu,
};
use dioxus::desktop::{Config, LogicalSize, WindowBuilder, use_muda_event_handler};
use dioxus::prelude::*;
use velopack::sources::{FileSource, NoneSource};
use velopack::{UpdateCheck, UpdateManager, VelopackApp};

use crate::sheet::View;

const ZOOM_IN: &str = "zoom-in";
const ZOOM_OUT: &str = "zoom-out";
const ZOOM_TO_FIT: &str = "zoom-to-fit";
const CHECK_FOR_UPDATES: &str = "check-for-updates";

static STARTED: OnceLock<Instant> = OnceLock::new();
static ARGS: OnceLock<Args> = OnceLock::new();

#[derive(Default)]
struct Args {
    update_feed: Option<PathBuf>,
    self_test: Option<PathBuf>,
}

impl Args {
    fn read() -> Self {
        let mut args = Args::default();
        let mut given = std::env::args().skip(1);
        while let Some(arg) = given.next() {
            match arg.as_str() {
                "--update-feed" => args.update_feed = given.next().map(PathBuf::from),
                "--self-test" => args.self_test = given.next().map(PathBuf::from),
                _ => {}
            }
        }
        args
    }

    /// The same arguments again, for the restart after an update.
    fn again(&self) -> Vec<String> {
        let mut again = Vec::new();
        if let Some(feed) = &self.update_feed {
            again.extend(["--update-feed".to_string(), feed.display().to_string()]);
        }
        if let Some(marker) = &self.self_test {
            again.extend(["--self-test".to_string(), marker.display().to_string()]);
        }
        again
    }
}

fn args() -> &'static Args {
    ARGS.get_or_init(Args::read)
}

/// Runs first in `main`. Velopack handles its install and update hooks here, and may exit.
pub fn start() {
    STARTED.get_or_init(Instant::now);
    VelopackApp::build().run();
    args();
}

/// The installed version, from Velopack's manifest.
pub fn version() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION.get_or_init(|| {
        UpdateManager::new(NoneSource {}, None, None)
            .map(|manager| manager.get_current_version_as_string())
            .unwrap_or_else(|_| "(not installed)".to_string())
    })
}

pub fn config() -> Config {
    let window = WindowBuilder::new()
        .with_title(format!("Hyleon {}", version()))
        .with_inner_size(LogicalSize::new(1280.0, 800.0));
    Config::new().with_window(window).with_menu(menu_bar())
}

/// The menu bar, laid out the way each operating system expects (DESIGN.md, Familiarity).
fn menu_bar() -> Menu {
    let shortcut = |code| Some(Accelerator::new(Some(CMD_OR_CTRL), code));
    let zoom_in = MenuItem::with_id(ZOOM_IN, "Zoom In", true, shortcut(Code::Equal));
    let zoom_out = MenuItem::with_id(ZOOM_OUT, "Zoom Out", true, shortcut(Code::Minus));
    let fit = MenuItem::with_id(ZOOM_TO_FIT, "Zoom to Fit", true, shortcut(Code::Digit0));
    let updates = MenuItem::with_id(CHECK_FOR_UPDATES, "Check for Updates…", true, None);
    let about = PredefinedMenuItem::about(
        Some("About Hyleon"),
        Some(AboutMetadata {
            name: Some("Hyleon".to_string()),
            version: Some(version().to_string()),
            ..Default::default()
        }),
    );
    let view = submenu("View", &[&zoom_in, &zoom_out, &fit]);
    let cut = PredefinedMenuItem::cut(None);
    let copy = PredefinedMenuItem::copy(None);
    let paste = PredefinedMenuItem::paste(None);
    let select_all = PredefinedMenuItem::select_all(None);

    let menus: Vec<Submenu> = if cfg!(target_os = "macos") {
        vec![
            submenu(
                "Hyleon",
                &[
                    &about,
                    &PredefinedMenuItem::separator(),
                    &updates,
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::hide(None),
                    &PredefinedMenuItem::hide_others(None),
                    &PredefinedMenuItem::show_all(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::quit(None),
                ],
            ),
            submenu(
                "Edit",
                &[
                    &PredefinedMenuItem::undo(None),
                    &PredefinedMenuItem::redo(None),
                    &PredefinedMenuItem::separator(),
                    &cut,
                    &copy,
                    &paste,
                    &select_all,
                ],
            ),
            view,
            submenu(
                "Window",
                &[
                    &PredefinedMenuItem::minimize(None),
                    &PredefinedMenuItem::maximize(None),
                    &PredefinedMenuItem::fullscreen(None),
                ],
            ),
        ]
    } else {
        vec![
            submenu("File", &[&PredefinedMenuItem::quit(Some("Exit"))]),
            submenu("Edit", &[&cut, &copy, &paste, &select_all]),
            view,
            submenu("Help", &[&updates, &about]),
        ]
    };
    let menus: Vec<&dyn IsMenuItem> = menus.iter().map(|menu| menu as &dyn IsMenuItem).collect();
    Menu::with_items(&menus).expect("the menu bar is built from fixed items")
}

fn submenu(title: &str, items: &[&dyn IsMenuItem]) -> Submenu {
    Submenu::with_items(title, true, items).expect("menus are built from fixed items")
}

/// Wires the menu to the sheet and to updates, and runs the self-test. Returns the update status
/// to show, if any.
pub fn use_desktop(view: View) -> Option<String> {
    let mut status = use_signal_sync(|| None::<String>);
    use_muda_event_handler(move |event| match event.id().0.as_str() {
        ZOOM_IN => view.zoom_in(),
        ZOOM_OUT => view.zoom_out(),
        ZOOM_TO_FIT => view.reset(),
        CHECK_FOR_UPDATES => {
            status.set(Some("Checking for updates…".to_string()));
            std::thread::spawn(move || {
                let mut status = status;
                status.set(Some(update()));
            });
        }
        _ => {}
    });
    use_effect(self_test);
    status()
}

/// Checks the feed and, when it has a newer version, downloads it and restarts into it. Velopack
/// has no async API, so callers run this off the UI thread. Returns what happened, unless it
/// restarted.
fn update() -> String {
    let Some(feed) = &args().update_feed else {
        return "This build has no update feed. Start it with --update-feed <folder>.".to_string();
    };
    let manager = match UpdateManager::new(FileSource::new(feed), None, None) {
        Ok(manager) => manager,
        Err(e) => return format!("Only an installed build can update: {e}"),
    };
    let info = match manager.check_for_updates() {
        Ok(UpdateCheck::UpdateAvailable(info)) => info,
        Ok(UpdateCheck::NoUpdateAvailable | UpdateCheck::RemoteIsEmpty) => {
            return format!("Hyleon {} is up to date.", version());
        }
        Err(e) => return format!("The update check failed: {e}"),
    };
    if let Err(e) = manager.download_updates(&info, None) {
        return format!("The update didn't download: {e}");
    }
    if let Err(e) = manager.apply_updates_and_restart_with_args(&*info, args().again()) {
        return format!("The update didn't install: {e}");
    }
    "Restarting…".to_string()
}

fn self_test() {
    let Some(marker) = &args().self_test else {
        return;
    };
    let millis = STARTED
        .get()
        .map_or(0, |started| started.elapsed().as_millis());
    note(marker, &format!("{} rendered in {millis} ms", version()));
    let marker = marker.clone();
    std::thread::spawn(move || {
        note(&marker, &update());
        std::process::exit(0);
    });
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
