use crate::{bumpalo, style, Align, Bus, Element, Length, Style, Widget};

/// An element decorating some content.
///
/// It is normally used for alignment purposes.
#[allow(missing_debug_implementations)]
pub struct Container<'a, Message> {
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    horizontal_alignment: Align,
    vertical_alignment: Align,
    content: Element<'a, Message>,
}

impl<'a, Message> Container<'a, Message> {
    /// Creates an empty [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message>>,
    {
        use std::u32;

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

impl<'a, Message> Widget<Message> for Container<'a, Message>
where
    Message: 'static,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        style_sheet: &mut style::Sheet<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let column_class = style_sheet.insert(bump, Style::Column);

        let width = style::length(self.width);
        let height = style::length(self.height);

        let align_items = style::align(self.horizontal_alignment);
        let justify_content = style::align(self.vertical_alignment);

        let node = div(bump)
            .attr(
                "class",
                bumpalo::format!(in bump, "{}", column_class).into_bump_str(),
            )
            .attr(
                "style",
                bumpalo::format!(
                    in bump,
                    "width: {}; height: {}; max-width: {}px; align-items: {}; justify-content: {}",
                    width,
                    height,
                    self.max_width,
                    align_items,
                    justify_content
                )
                .into_bump_str(),
            )
            .children(vec![self.content.node(bump, bus, style_sheet)]);

        // TODO: Complete styling

        node.finish()
    }
}

impl<'a, Message> From<Container<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(container: Container<'a, Message>) -> Element<'a, Message> {
        Element::new(container)
    }
}
