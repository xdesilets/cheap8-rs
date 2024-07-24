use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use iced::mouse::Cursor;
use super::environment::*;
use iced::widget::canvas;
use iced::widget::canvas::{Frame, Geometry};
use crate::application::Message;

pub struct DisplayView<'a>{
    pub display: &'a [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl<'a> DisplayView<'a> {
    pub fn new(display: &'a [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]) -> Self {
        Self { display }
    }
}

impl<'a> canvas::Program<Message> for DisplayView<'a>{
    type State = ();

    fn draw(&self, state: &Self::State, renderer: &Renderer, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, Size::new(bounds.width, bounds.height));

        let pixel_width = bounds.width / DISPLAY_WIDTH as f32;
        let pixel_height = bounds.height / DISPLAY_HEIGHT as f32;

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let color: Color;

                if self.display[y * DISPLAY_WIDTH + x]{
                    color = Color::WHITE;
                }
                else {
                    color = Color::BLACK;
                }

                frame.fill_rectangle(
                    Point::new(x as f32 * pixel_width, y as f32 * pixel_height),
                    Size::new(pixel_width, pixel_height),
                    color,
                );
            }
        }

        vec![frame.into_geometry()]
    }
}
