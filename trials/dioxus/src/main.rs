//! Starts variant D. Desktop: `trial-dioxus [--size panel|building] [--bench <out.json>] [--slow <ms>]`.
//! Web: the same settings come from the address, `?size=building&bench=1`.

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
    use std::time::{SystemTime, UNIX_EPOCH};
    use trial_dioxus::{App, Launch, Settings};

    let start_epoch_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64() * 1000.0);
    let mut settings = Settings::from_query("");
    settings.start_epoch_ms = Some(start_epoch_ms);
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--size" => {
                if let Some(size) = args.next().as_deref().and_then(bench_core::Size::parse) {
                    settings.size = size;
                }
            }
            "--bench" => {
                settings.bench = true;
                settings.out = args.next();
            }
            "--slow" => settings.slow_ms = args.next().and_then(|s| s.parse().ok()).unwrap_or(0),
            "--capacity" => settings.capacity = true,
            _ => {}
        }
    }

    let window = WindowBuilder::new()
        .with_title("Trial D (Dioxus)")
        .with_inner_size(LogicalSize::new(1440.0, 900.0));
    let index = r#"<!DOCTYPE html><html lang="en"><head><title>Trial D</title><meta name="viewport" content="width=device-width, initial-scale=1.0"></head><body><div id="main"></div></body></html>"#;
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(window)
                .with_custom_index(index.to_string()),
        )
        .with_context(Launch(Some(settings)))
        .launch(App);
}

#[cfg(all(feature = "web", not(feature = "desktop")))]
fn main() {
    dioxus::LaunchBuilder::web()
        .with_context(trial_dioxus::Launch(None))
        .launch(trial_dioxus::App);
}

#[cfg(not(any(feature = "desktop", feature = "web")))]
fn main() {
    eprintln!("Build with `dx`, which picks the desktop or web feature for the platform.");
}
