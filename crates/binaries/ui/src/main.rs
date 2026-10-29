//! Starts the app. `dx` builds it with the `desktop` or `web` feature.

#[cfg(feature = "desktop")]
fn main() {
    hyleon_ui::desktop::start();
    dioxus::LaunchBuilder::desktop()
        .with_cfg(hyleon_ui::desktop::config())
        .launch(hyleon_ui::App);
}

#[cfg(all(feature = "web", not(feature = "desktop")))]
fn main() {
    dioxus::launch(hyleon_ui::App);
}

#[cfg(not(any(feature = "desktop", feature = "web")))]
fn main() {
    eprintln!("Build the app with `dx`, which picks the desktop or web feature for the platform.");
}
