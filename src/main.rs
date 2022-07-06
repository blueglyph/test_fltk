use fltk::{prelude::*, *};
mod ui;

fn main() {
    let app = app::App::default();
    let mut gui = ui::UserInterface::make_window();
    let mut win = gui.my_win.clone();
    gui.btn.set_callback(move |b| {
        b.set_label("clicked");
        win.set_label("Button clicked");
        println!("Works!");
    });
    app.run().unwrap();
}
