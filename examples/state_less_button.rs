//! A simple example that demonstrates dynamic buttons.
use coffee::graphics::{
    Color, Frame, Window,
    WindowSettings,
};
use coffee::load::Task;
use coffee::ui::{Align, Column, Element, Justify, Renderer, Text, UserInterface};
use coffee::{Game, Result, Timer};
use coffee::ui::widget::state_less_button::StateLessButton;


fn main() -> Result<()> {
    <TextInputExample as UserInterface>::run(WindowSettings {
        title: String::from("StateLessButtons - Coffee"),
        size: (800, 600),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

#[derive(Debug, Clone, Copy)]
enum UiMessage {
    ButtonPressed(i32),
    ButtonReleased(i32),
}

struct TextInputExample {
    button_pressed: i32,
}

impl Game for TextInputExample {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<TextInputExample> {
        Task::succeed(|| TextInputExample{
            button_pressed: -1,
        })
    }

    fn interact(&mut self, _input: &mut (), _window: &mut Window) { }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) { frame.clear(Color::BLACK); }
}

impl UserInterface for TextInputExample {
    type Message = UiMessage;
    type Renderer = Renderer;

    fn react(&mut self, msg: UiMessage, _window: &mut Window) {
        match msg {
            UiMessage::ButtonPressed(id) => {
                self.button_pressed = id;
            }
            UiMessage::ButtonReleased(id) => {
                dbg!(id);
            }
        }
    }

    fn layout(&mut self, window: &Window) -> Element<UiMessage> {
        let mut content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(Text::new("Hello world"));

        for i in 0..2 {
            content = content.push(
                StateLessButton::new(
                    i == self.button_pressed,
                    "Button",
                    UiMessage::ButtonPressed(i),
                    UiMessage::ButtonReleased(i),
                ),
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
