//! Create text input.
use crate::graphics::{
    Color, HorizontalAlignment, Point, Rectangle, VerticalAlignment,
};
use crate::input::{mouse, ButtonState};
use crate::ui::core::{
    Align, Element, Event, Hasher, Layout, MouseCursor, Node, Widget,
};
use crate::ui::widget::{text, Row, Text};

use std::hash::Hash;

/// A text input.
pub struct TextInput<Id, Message> {
    id: Id,
    on_selected: Box<dyn Fn(Id) -> Message>,
    label: String,
    value: String,
    color: Color,
    hover_color: Color,
    blink_char: Option<char>,
}

impl<I, Message> std::fmt::Debug for TextInput<I, Message>
where
    I: Copy,
    Message: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextInput")
            .field("label", &self.label)
            .field("value", &self.value)
            .field("color", &self.color)
            .field("hover_color", &self.hover_color)
            .finish()
    }
}

impl<I, Message> TextInput<I, Message> {
    /// todo doc
    pub fn new<S>(id: I, label: &str, value: &str, on_selected: S, blink_char: Option<char>) -> Self
    where
        S: 'static + Fn(I) -> Message,
    {
        TextInput {
            id,
            on_selected: Box::new(on_selected),
            label: String::from(label),
            value: String::from(value),
            color: Color::WHITE,
            hover_color: Color::GREEN,
            blink_char,
        }
    }

    /// todo doc
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// todo doc
    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }
}

impl<I, Message, Renderer> Widget<Message, Renderer> for TextInput<I, Message>
where
    I: Copy,
    Renderer: self::Renderer + text::Renderer,
    Message: Copy + std::fmt::Debug,
{
    fn node(&self, renderer: &Renderer) -> Node {
        // FIXME BS NOW: Whien it is called ?!
        Row::<(), Renderer>::new()
            .spacing(15)
            .align_items(Align::Center)
            .push(Text::new(format!("{}: {}", self.label, self.value).as_str()))
            .node(renderer)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
    ) {
        match event {
            Event::Mouse(mouse::Event::Input {
                button: mouse::Button::Left,
                state: ButtonState::Pressed,
            }) => {
                if layout.bounds().contains(cursor_position) {
                    messages.push((self.on_selected)(self.id))
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> MouseCursor {
        let hover = layout.bounds().contains(cursor_position);
        let color = if hover {self.hover_color} else {self.color};
        let mut text = format!("{}: {}", self.label, self.value);
        if let Some(blink_char) = self.blink_char {
            text.push(blink_char);
        }

        text::Renderer::draw(
            renderer,
            layout.bounds(),
            text.as_str(),
            20.0,
            color,
            HorizontalAlignment::Left,
            VerticalAlignment::Top,
        );

        if hover {
            MouseCursor::Pointer
        } else {
            MouseCursor::OutOfBounds
        }
    }

    fn hash(&self, state: &mut Hasher) {
        self.label.hash(state);
    }
}


/// todo doc
pub trait Renderer {
    /// todo doc
    fn draw(
        &mut self,
        cursor_position: Point,
        bounds: Rectangle<f32>,
        label_bounds: Rectangle<f32>,
        is_selected: bool,
    ) -> MouseCursor;
}

impl<'a, I: 'a, Message, Renderer> From<TextInput<I, Message>>
    for Element<'a, Message, Renderer>
where
    I: Copy,
    Renderer: self::Renderer + text::Renderer,
    Message: 'static + Copy + std::fmt::Debug,
{
    fn from(text_input: TextInput<I, Message>) -> Element<'a, Message, Renderer> {
        Element::new(text_input)
    }
}
