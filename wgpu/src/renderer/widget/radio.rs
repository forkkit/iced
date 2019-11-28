use crate::{Primitive, Renderer};
use iced_native::{radio, Background, MouseCursor, Rectangle};

const SIZE: f32 = 28.0;
const DOT_SIZE: f32 = SIZE / 2.0;

impl radio::Renderer for Renderer {
    fn default_size(&self) -> u32 {
        SIZE as u32
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_selected: bool,
        is_mouse_over: bool,
        (label, _): Self::Output,
    ) -> Self::Output {
        let (radio_border, radio_box) = (
            Primitive::Quad {
                bounds,
                background: Background::Color([0.6, 0.6, 0.6].into()),
                border_radius: (SIZE / 2.0) as u16,
            },
            Primitive::Quad {
                bounds: Rectangle {
                    x: bounds.x + 1.0,
                    y: bounds.y + 1.0,
                    width: bounds.width - 2.0,
                    height: bounds.height - 2.0,
                },
                background: Background::Color(
                    if is_mouse_over {
                        [0.90, 0.90, 0.90]
                    } else {
                        [0.95, 0.95, 0.95]
                    }
                    .into(),
                ),
                border_radius: (SIZE / 2.0 - 1.0) as u16,
            },
        );

        (
            Primitive::Group {
                primitives: if is_selected {
                    let radio_circle = Primitive::Quad {
                        bounds: Rectangle {
                            x: bounds.x + DOT_SIZE / 2.0,
                            y: bounds.y + DOT_SIZE / 2.0,
                            width: bounds.width - DOT_SIZE,
                            height: bounds.height - DOT_SIZE,
                        },
                        background: Background::Color([0.3, 0.3, 0.3].into()),
                        border_radius: (DOT_SIZE / 2.0) as u16,
                    };

                    vec![radio_border, radio_box, radio_circle, label]
                } else {
                    vec![radio_border, radio_box, label]
                },
            },
            if is_mouse_over {
                MouseCursor::Pointer
            } else {
                MouseCursor::OutOfBounds
            },
        )
    }
}
