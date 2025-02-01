mod context;
use context::Context;

pub fn run() -> iced::Result {
    iced::run(
        "Loop - history", 
        Context::update,
        Context::view
    )
}