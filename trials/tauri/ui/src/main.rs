//! The Leptos UI of the Tauri trial that D-043 chose (T-02): the trial screen, in Tauri or on the web.

mod bridge;
mod questions;
mod trial;

fn main() {
    std::panic::set_hook(Box::new(|info| bridge::log(format!("panic: {info}"))));
    leptos::mount::mount_to_body(trial::TrialScreen);
}
