#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod utilites;

use tracing::info;

use app::MyApp;

fn main() -> eframe::Result {
    tracing_subscriber::fmt::init();
    info!("App started");
    let options = eframe::NativeOptions::default();
    MyApp::run(options)
}
