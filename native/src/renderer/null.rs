use crate::{
    button, checkbox, column, radio, row, scrollable, text, text_input,
    Background, Color, Element, Font, HorizontalAlignment, Layout, Point,
    Rectangle, Renderer, Size, VerticalAlignment,
};

/// A renderer that does nothing.
#[derive(Debug, Clone, Copy)]
pub struct Null;

impl Renderer for Null {
    type Output = ();
}

impl column::Renderer for Null {
    fn draw<Message>(
        &mut self,
        _content: &[Element<'_, Message, Self>],
        _layout: Layout<'_>,
        _cursor_position: Point,
    ) {
    }
}

impl row::Renderer for Null {
    fn draw<Message>(
        &mut self,
        _content: &[Element<'_, Message, Self>],
        _layout: Layout<'_>,
        _cursor_position: Point,
    ) {
    }
}

impl text::Renderer for Null {
    fn default_size(&self) -> u16 {
        20
    }

    fn measure(
        &self,
        _content: &str,
        _size: u16,
        _font: Font,
        _bounds: Size,
    ) -> (f32, f32) {
        (0.0, 20.0)
    }

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _content: &str,
        _size: u16,
        _font: Font,
        _color: Option<Color>,
        _horizontal_alignment: HorizontalAlignment,
        _vertical_alignment: VerticalAlignment,
    ) {
    }
}

impl scrollable::Renderer for Null {
    fn is_mouse_over_scrollbar(
        &self,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _cursor_position: Point,
    ) -> bool {
        false
    }

    fn draw(
        &mut self,
        _scrollable: &scrollable::State,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _is_mouse_over: bool,
        _is_mouse_over_scrollbar: bool,
        _offset: u32,
        _content: Self::Output,
    ) {
    }
}

impl text_input::Renderer for Null {
    fn default_size(&self) -> u16 {
        20
    }

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _text_bounds: Rectangle,
        _cursor_position: Point,
        _size: u16,
        _placeholder: &str,
        _value: &text_input::Value,
        _state: &text_input::State,
    ) -> Self::Output {
    }
}

impl button::Renderer for Null {
    fn draw(
        &mut self,
        _bounds: Rectangle,
        _cursor_position: Point,
        _is_pressed: bool,
        _background: Option<Background>,
        _border_radius: u16,
        _content: Self::Output,
    ) -> Self::Output {
    }
}

impl radio::Renderer for Null {
    fn default_size(&self) -> u32 {
        20
    }

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _is_selected: bool,
        _is_mouse_over: bool,
        _label: Self::Output,
    ) {
    }
}

impl checkbox::Renderer for Null {
    fn default_size(&self) -> u32 {
        20
    }

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _is_checked: bool,
        _is_mouse_over: bool,
        _label: Self::Output,
    ) {
    }
}
