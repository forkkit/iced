use crate::{style, Align, Bus, Element, Length, Style, Widget};

use dodrio::bumpalo;
use std::u32;

/// A container that distributes its contents horizontally.
///
/// A [`Row`] will try to fill the horizontal space of its container.
///
/// [`Row`]: struct.Row.html
#[allow(missing_debug_implementations)]
pub struct Row<'a, Message> {
    spacing: u16,
    padding: u16,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    align_items: Align,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Row<'a, Message> {
    /// Creates an empty [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn new() -> Self {
        Row {
            spacing: 0,
            padding: 0,
            width: Length::Fill,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            align_items: Align::Start,
            children: Vec::new(),
        }
    }

    /// Sets the horizontal spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in Iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the padding of the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`Row`].
    ///
    /// [`Row`]: struct.Row.html
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the vertical alignment of the contents of the [`Row`] .
    ///
    /// [`Row`]: struct.Row.html
    pub fn align_items(mut self, align: Align) -> Self {
        self.align_items = align;
        self
    }

    /// Adds an [`Element`] to the [`Row`].
    ///
    /// [`Element`]: ../struct.Element.html
    /// [`Row`]: struct.Row.html
    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, Message>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message> Widget<Message> for Row<'a, Message> {
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        publish: &Bus<Message>,
        style_sheet: &mut style::Sheet<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let children: Vec<_> = self
            .children
            .iter()
            .map(|element| element.widget.node(bump, publish, style_sheet))
            .collect();

        let row_class = style_sheet.insert(bump, Style::Row);

        let spacing_class =
            style_sheet.insert(bump, Style::Spacing(self.spacing));

        let padding_class =
            style_sheet.insert(bump, Style::Padding(self.padding));

        let width = style::length(self.width);
        let height = style::length(self.height);

        // TODO: Complete styling
        div(bump)
            .attr(
                "class",
                bumpalo::format!(in bump, "{} {} {}", row_class, spacing_class, padding_class)
                    .into_bump_str(),
            )
            .attr("style", bumpalo::format!(
                    in bump,
                    "width: {}; height: {}; max-width: {}px",
                    width,
                    height,
                    self.max_width
                ).into_bump_str()
            )
            .children(children)
            .finish()
    }
}

impl<'a, Message> From<Row<'a, Message>> for Element<'a, Message>
where
    Message: 'static,
{
    fn from(column: Row<'a, Message>) -> Element<'a, Message> {
        Element::new(column)
    }
}
