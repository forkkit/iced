//! Decorate content and apply alignment.
use std::hash::Hash;

use crate::{
    layout, Align, Element, Event, Hasher, Layout, Length, Point, Widget,
};

use std::u32;

/// An element decorating some content.
///
/// It is normally used for alignment purposes.
#[allow(missing_debug_implementations)]
pub struct Container<'a, Message, Renderer> {
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    horizontal_alignment: Align,
    vertical_alignment: Align,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Container<'a, Message, Renderer> {
    /// Creates an empty [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
    {
        Container {
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            horizontal_alignment: Align::Start,
            vertical_alignment: Align::Start,
            content: content.into(),
        }
    }

    /// Sets the width of the [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`Container`] in pixels.
    ///
    /// [`Container`]: struct.Container.html
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Centers the contents in the horizontal axis of the [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    pub fn center_x(mut self) -> Self {
        self.horizontal_alignment = Align::Center;

        self
    }

    /// Centers the contents in the vertical axis of the [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    pub fn center_y(mut self) -> Self {
        self.vertical_alignment = Align::Center;

        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Container<'a, Message, Renderer>
where
    Renderer: crate::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .loose()
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);

        let mut content = self.content.layout(renderer, &limits.loose());
        let size = limits.resolve(content.size());

        content.align(self.horizontal_alignment, self.vertical_alignment, size);

        layout::Node::with_children(size, vec![content])
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
    ) {
        self.content.widget.on_event(
            event,
            layout.children().next().unwrap(),
            cursor_position,
            messages,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self.content.draw(
            renderer,
            layout.children().next().unwrap(),
            cursor_position,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        0.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);

        self.content.hash_layout(state);
    }
}

impl<'a, Message, Renderer> From<Container<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + crate::Renderer,
    Message: 'static,
{
    fn from(
        column: Container<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(column)
    }
}
