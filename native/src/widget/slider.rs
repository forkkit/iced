//! Display an interactive selector of a single value from a range of values.
//!
//! A [`Slider`] has some local [`State`].
//!
//! [`Slider`]: struct.Slider.html
//! [`State`]: struct.State.html
use crate::{
    input::{mouse, ButtonState},
    layout, Element, Event, Hasher, Layout, Length, Point, Rectangle, Size,
    Widget,
};

use std::{hash::Hash, ops::RangeInclusive};

/// An horizontal bar and a handle that selects a single value from a range of
/// values.
///
/// A [`Slider`] will try to fill the horizontal space of its container.
///
/// [`Slider`]: struct.Slider.html
///
/// # Example
/// ```
/// # use iced_native::{slider, Slider};
/// #
/// pub enum Message {
///     SliderChanged(f32),
/// }
///
/// let state = &mut slider::State::new();
/// let value = 50.0;
///
/// Slider::new(state, 0.0..=100.0, value, Message::SliderChanged);
/// ```
///
/// ![Slider drawn by Coffee's renderer](https://github.com/hecrj/coffee/blob/bda9818f823dfcb8a7ad0ff4940b4d4b387b5208/images/ui/slider.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct Slider<'a, Message> {
    state: &'a mut State,
    range: RangeInclusive<f32>,
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message>,
    width: Length,
}

impl<'a, Message> Slider<'a, Message> {
    /// Creates a new [`Slider`].
    ///
    /// It expects:
    ///   * the local [`State`] of the [`Slider`]
    ///   * an inclusive range of possible values
    ///   * the current value of the [`Slider`]
    ///   * a function that will be called when the [`Slider`] is dragged.
    ///   It receives the new value of the [`Slider`] and must produce a
    ///   `Message`.
    ///
    /// [`Slider`]: struct.Slider.html
    /// [`State`]: struct.State.html
    pub fn new<F>(
        state: &'a mut State,
        range: RangeInclusive<f32>,
        value: f32,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(f32) -> Message,
    {
        Slider {
            state,
            value: value.max(*range.start()).min(*range.end()),
            range,
            on_change: Box::new(on_change),
            width: Length::Fill,
        }
    }

    /// Sets the width of the [`Slider`].
    ///
    /// [`Slider`]: struct.Slider.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

/// The local state of a [`Slider`].
///
/// [`Slider`]: struct.Slider.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_dragging: bool,
}

impl State {
    /// Creates a new [`State`].
    ///
    /// [`State`]: struct.State.html
    pub fn new() -> State {
        State::default()
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Slider<'a, Message>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .width(self.width)
            .height(Length::Units(renderer.height() as u16));

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
    ) {
        let mut change = || {
            let bounds = layout.bounds();

            if cursor_position.x <= bounds.x {
                messages.push((self.on_change)(*self.range.start()));
            } else if cursor_position.x >= bounds.x + bounds.width {
                messages.push((self.on_change)(*self.range.end()));
            } else {
                let percent = (cursor_position.x - bounds.x) / bounds.width;
                let value = (self.range.end() - self.range.start()) * percent
                    + self.range.start();

                messages.push((self.on_change)(value));
            }
        };

        match event {
            Event::Mouse(mouse::Event::Input {
                button: mouse::Button::Left,
                state,
            }) => match state {
                ButtonState::Pressed => {
                    if layout.bounds().contains(cursor_position) {
                        change();
                        self.state.is_dragging = true;
                    }
                }
                ButtonState::Released => {
                    self.state.is_dragging = false;
                }
            },
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if self.state.is_dragging {
                    change();
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
    ) -> Renderer::Output {
        renderer.draw(
            layout.bounds(),
            cursor_position,
            self.range.clone(),
            self.value,
            self.state.is_dragging,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.width.hash(state);
    }
}

/// The renderer of a [`Slider`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Slider`] in your user interface.
///
/// [`Slider`]: struct.Slider.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: crate::Renderer {
    /// Returns the height of the [`Slider`].
    ///
    /// [`Slider`]: struct.Slider.html
    fn height(&self) -> u32;

    /// Draws a [`Slider`].
    ///
    /// It receives:
    ///   * the current cursor position
    ///   * the bounds of the [`Slider`]
    ///   * the local state of the [`Slider`]
    ///   * the range of values of the [`Slider`]
    ///   * the current value of the [`Slider`]
    ///
    /// [`Slider`]: struct.Slider.html
    /// [`State`]: struct.State.html
    /// [`Class`]: enum.Class.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        range: RangeInclusive<f32>,
        value: f32,
        is_dragging: bool,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Slider<'a, Message>>
    for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Message: 'static,
{
    fn from(slider: Slider<'a, Message>) -> Element<'a, Message, Renderer> {
        Element::new(slider)
    }
}
