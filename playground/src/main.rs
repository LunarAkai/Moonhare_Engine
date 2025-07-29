use moonhare_engine::{event, log::{self}, window::{self, winit_window::WinitWindow}};

fn main() {
    let _ = log::configere_logger();
    log::info("test");
}