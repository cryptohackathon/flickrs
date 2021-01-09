use crate::data::State;

use druid::{widget::Scroll, Widget, WindowDesc};

pub mod image_list;
pub mod theme;

pub fn make_main_window() -> WindowDesc<State> {
    WindowDesc::new(make_root)
        .title("Flick-rs")
        .with_min_size((theme::grid(25.0), theme::grid(25.0)))
        .window_size((theme::grid(100.0), theme::grid(100.0)))
}

fn make_root() -> impl Widget<State> {
    let image_list = Scroll::new(image_list::make_list()).vertical();

    image_list
}
