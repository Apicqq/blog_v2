//! Серверное приложение блога.

pub mod application;
pub mod domain;
pub mod infrastructure;

use blog_proto as _;

fn main() {
    infrastructure::telemetry::init_logging();
}
