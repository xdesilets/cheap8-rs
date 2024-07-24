use crate::interpreter::core::Core;
use crate::interpreter::Interpreter;
use crate::interpreter::display_view::DisplayView;
use std::fs::File;
use iced::time::Instant;
use iced::{executor, window, Subscription, Length};
use iced::{Command, Element, Theme};
use iced::widget::Canvas;

pub struct Application {
    interpreter: Interpreter,
    current_instruction: u16
}

#[derive(Debug, Clone)]
pub enum Message{
    Tick(Instant)
}

impl iced::Application for Application{
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Application, Command<Self::Message>) {
        let file = File::open("roms/IBM Logo.ch8").unwrap();
        let mut core_instance = Core::default();

        core_instance.load_rom(file);

        let interpreter = Interpreter::new(core_instance);

        return (Application {
            interpreter,
            current_instruction: 0
        }, Command::none());
    }

    fn title(&self) -> String {
        String::from("Cheap8 Emulator")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Message::Tick(_) => {
                let instruction = self.interpreter.fetch();
                self.current_instruction = instruction;
                self.interpreter.execute(instruction);
            }
        }
        return Command::none();
    }

    fn view(&self) -> Element<Self::Message> {
        //let instruction = format!("Executing instruction: 0x{:X}", self.current_instruction);
        //let text = iced::widget::text(instruction);
        let display_view = DisplayView::new(&self.interpreter.core.display);
        Canvas::new(display_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}
