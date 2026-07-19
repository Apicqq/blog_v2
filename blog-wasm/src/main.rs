//! Точка входа Dioxus-приложения.

mod api;
mod app;
mod components;
mod errors;
mod models;
mod session;
mod storage;
mod validation;

use app::App;

fn main() {
    dioxus::launch(App);
}
