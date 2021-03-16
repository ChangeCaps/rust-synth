mod app;

pub use app::*;

fn main() {
    eframe::run_native(Box::new(app::App::new().unwrap()));
}
