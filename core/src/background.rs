use crate::Color;

/// The background of some element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Background {
    /// A solid color
    Color(Color),
    // TODO: Add gradient and image variants
}
