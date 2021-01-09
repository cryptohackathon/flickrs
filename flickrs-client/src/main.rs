use data::State;
use druid::AppLauncher;

pub mod data;
pub mod error;
pub mod ui;
pub mod widget;

pub fn main() {
    let win = ui::make_main_window();
    let launcher = AppLauncher::with_window(win);

    let mut state = State::default();
    state.load_images();
    state.load_images();
    state.load_images();
    state.load_images();

    launcher
        .use_simple_logger()
        .launch(state)
        .expect("Application launch");
}
