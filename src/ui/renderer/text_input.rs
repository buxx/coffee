use crate::graphics::{Point, Rectangle, Sprite};
use crate::ui::core::MouseCursor;
use crate::ui::widget::text_input;
use crate::ui::Renderer;

impl text_input::Renderer for Renderer {
    fn draw(
        &mut self,
        cursor_position: Point,
        bounds: Rectangle<f32>,
        bounds_with_label: Rectangle<f32>,
        is_selected: bool,
    ) -> MouseCursor {
        // FIXME BS NOW: this code is used ?!
        let mouse_over = bounds_with_label.contains(cursor_position);

        if mouse_over {
            MouseCursor::Pointer
        } else {
            MouseCursor::OutOfBounds
        }
    }
}
