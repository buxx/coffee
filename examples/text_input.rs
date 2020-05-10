//! A simple example that demonstrates text input.
use std::collections::{HashSet, HashMap};

use coffee::graphics::{
    Color, Frame, Image, Point, Rectangle, Sprite, Vector, Window,
    WindowSettings,
};
use coffee::input::{self, keyboard, mouse, Input, Event};
use coffee::load::Task;
use coffee::ui::{
    Align, Column, Element, Justify, Renderer, Row, Text, UserInterface,
};
use coffee::{Game, Result, Timer};
use coffee::ui::widget::text_input::TextInput;
use std::borrow::BorrowMut;
use std::time::Instant;

const BLINK_MS: u128 = 250;

fn main() -> Result<()> {
    <TextInputExample as UserInterface>::run(WindowSettings {
        title: String::from("TextInput - Coffee"),
        size: (800, 600),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

#[derive(Debug, Clone, Copy)]
enum UiMessage {
    TextInputSelected(i32),
}


struct CustomInput {
    text_buffer: String,
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            text_buffer: String::new(),
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::TextEntered { character } => {
                    self.text_buffer.push(character);
                }
                _ => {}
            }
            _ => {}
        }
    }

    fn clear(&mut self) {}
}

struct TextInputExample {
    text_input_selected: i32,
    text_input_values: HashMap<i32, String>,
    blink_time: Instant,
}

impl TextInputExample {
    fn apply_text_buffer(&mut self, text_input_id: i32, text_buffer: String) {
        if let Some(text_input_value) = self.text_input_values.get_mut(&text_input_id) {
            for c in text_buffer.chars() {
                match c {
                    // Match ASCII backspace and delete from the text buffer
                    '\u{8}' => {
                        text_input_value.pop();
                    }
                    // Tabulation | Enter | Escape
                    '\t' | '\r' | '\u{1b}' => {}
                    _ => {
                        text_input_value.push(c);
                    }
                }
            }
        }
    }
}


impl Game for TextInputExample {
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<TextInputExample> {
        Task::succeed(|| TextInputExample{
            text_input_selected: -1,
            text_input_values: [(1, "".to_string()), (2, "".to_string())].iter().cloned().collect(),
            blink_time: Instant::now(),
        })
    }

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        if !input.text_buffer.is_empty() {
            self.apply_text_buffer(self.text_input_selected, input.text_buffer.clone());
            input.text_buffer = String::new();
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
    }
}

impl UserInterface for TextInputExample {
    type Message = UiMessage;
    type Renderer = Renderer;

    fn react(&mut self, msg: UiMessage, _window: &mut Window) {
        match msg {
            UiMessage::TextInputSelected(id) => {
                self.text_input_selected = id;
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<UiMessage> {
        let elapsed = self.blink_time.elapsed().as_millis();
        let blink_char = if elapsed < BLINK_MS as u128 {
            None
        } else if elapsed <= (BLINK_MS * 2) as u128 {
            Some('_')
        } else {
            self.blink_time = Instant::now();
            None
        };

        let input1 = TextInput::new(
            1,
            "Input1",
            &self.text_input_values.get(&1).unwrap(),
            UiMessage::TextInputSelected,
            if self.text_input_selected == 1 {blink_char} else {None},
        );
        let input2 = TextInput::new(
            2,
            "Input2",
            &self.text_input_values.get(&2).unwrap(),
            UiMessage::TextInputSelected,
            if self.text_input_selected == 2 {blink_char} else {None},
        );

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(Text::new("Hello world"))
            .push(input1)
            .push(input2);

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
