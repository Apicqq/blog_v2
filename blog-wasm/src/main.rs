//! Точка входа Dioxus-приложения.

mod api;
mod app;
mod errors;
mod models;
mod storage;

use app::App;

fn main() {
    dioxus::launch(App);
}
