//! A simple example that demonstrates dynamic buttons.
use std::collections::{HashSet, HashMap};

use coffee::graphics::{
    Color, Frame, Image, Point, Rectangle, Sprite, Vector, Window,
    WindowSettings,
};
use coffee::input::{self, keyboard, mouse, Input, Event};
use coffee::load::Task;
use coffee::ui::{button, Align, Column, Element, Justify, Renderer, Row, Text, UserInterface, Button};
use coffee::{Game, Result, Timer};
use coffee::ui::widget::text_input::TextInput;
use std::borrow::BorrowMut;
use std::time::Instant;

const BLINK_MS: u128 = 250;

fn main() -> Result<()> {
    <TextInputExample as UserInterface>::run(WindowSettings {
        title: String::from("DynButtons - Coffee"),
        size: (800, 600),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

#[derive(Debug, Clone, Copy)]
enum UiMessage {
    ButtonPressed,
}

struct TextInputExample {
    button_states: HashMap<i32, button::State>,
}

impl Game for TextInputExample {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<TextInputExample> {
        Task::succeed(|| TextInputExample{
            button_states: [(0, button::State::new()), (1, button::State::new())].iter().cloned().collect()
        })
    }

    fn interact(&mut self, input: &mut (), _window: &mut Window) { }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) { frame.clear(Color::BLACK); }
}

impl UserInterface for TextInputExample {
    type Message = UiMessage;
    type Renderer = Renderer;

    fn react(&mut self, msg: UiMessage, _window: &mut Window) {

    }

    fn layout(&mut self, window: &Window) -> Element<UiMessage> {
        let mut content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(Text::new("Hello world"));

        for i in 0..2 {
            content = content.push(
                Button::new(
                    &mut self.button_states.get_mut(&i).unwrap(),
                    "Button"
                ).on_press(UiMessage::ButtonPressed),
            );
        }

        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .padding(20)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .push(content)
            .into()
    }
}
