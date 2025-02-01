use iced::{
    widget::text,
    Element
};

#[derive(Default)]
pub struct Context {}

#[derive(Debug)]
pub enum Message {}

impl Context {
    pub fn view(&self) -> Element<Message> {
        text("History").into()
    }

    pub fn update(&mut self, _message: Message) {}
}