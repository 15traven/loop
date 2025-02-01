use iced::Size;

mod context;
use context::Context;

pub fn run() -> iced::Result {
    iced::application(
        "Loop - history", 
        Context::update,
        Context::view
    )
    .resizable(false)
    .window_size(Size::new(350.0, 450.0))
    .run()
}