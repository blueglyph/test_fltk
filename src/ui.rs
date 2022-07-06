// Automatically generated by fl2rust

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::needless_update)]

use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;



#[derive(Debug, Clone)]
pub struct UserInterface {
    pub my_win: Window,
    pub btn: Button,
}


impl UserInterface {
    pub fn make_window() -> Self {
	let mut my_win = Window::new(60, 729, 265, 190, "My Window");
	my_win.end();
	my_win.set_label_color(Color::by_index(32));
	my_win.make_resizable(true);
	my_win.show();
	let mut btn = Button::new(92, 80, 80, 30, "button");
	my_win.add(&btn);
	let mut fl2rust_widget_0 = MenuBar::new(10, 5, 240, 20, "App");
	fl2rust_widget_0.end();
	fl2rust_widget_0.set_frame(FrameType::FlatBox);
	fl2rust_widget_0.set_down_frame(FrameType::ThinUpFrame);
	my_win.add(&fl2rust_widget_0);
	{F&ile}.set_label_size(12);
	fl2rust_widget_0.add("F&ile/Open", Shortcut::None, MenuFlag::Normal, |_| {});
	fl2rust_widget_0.add("F&ile/Save", Shortcut::None, MenuFlag::Normal, |_| {});
	fl2rust_widget_0.add("F&ile/Exit", Shortcut::None, MenuFlag::Normal, |_| {});
	{&Edit}.set_label_size(12);
	fl2rust_widget_0.add("&Edit/Copy", Shortcut::None, MenuFlag::Normal, |_| {});
	fl2rust_widget_0.add("&Edit/Paste", Shortcut::None, MenuFlag::Normal, |_| {});
	Self { my_win, btn, }
    }
}


