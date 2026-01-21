use fltk::{app::App, prelude::*, window::Window};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "My Window");
    wind.end();
    wind.show();
    app.run().unwrap();
}
