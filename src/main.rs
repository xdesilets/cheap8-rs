pub mod interpreter;
mod application;
use iced::Settings;
use iced::Application;

pub fn main() -> iced::Result{
    application::Application::run(Settings::default())
}
