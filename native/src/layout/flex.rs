//! Distribute elements using a flex-based layout.
// This code is heavily inspired by the [`druid`] codebase.
//
// [`druid`]: https://github.com/xi-editor/druid
//
// Copyright 2018 The xi-editor Authors, Héctor Ramón
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::{
    layout::{Limits, Node},
    Align, Element, Size,
};

/// The main axis of a flex layout.
#[derive(Debug)]
pub enum Axis {
    /// The horizontal axis
    Horizontal,

    /// The vertical axis
    Vertical,
}

impl Axis {
    fn main(&self, size: Size) -> f32 {
        match self {
            Axis::Horizontal => size.width,
            Axis::Vertical => size.height,
        }
    }

    fn cross(&self, size: Size) -> f32 {
        match self {
            Axis::Horizontal => size.height,
            Axis::Vertical => size.width,
        }
    }

    fn pack(&self, main: f32, cross: f32) -> (f32, f32) {
        match self {
            Axis::Horizontal => (main, cross),
            Axis::Vertical => (cross, main),
        }
    }
}

/// Computes the flex layout with the given axis and limits, applying spacing,
/// padding and alignment to the items as needed.
///
/// It returns a new layout [`Node`].
///
/// [`Node`]: ../struct.Node.html
pub fn resolve<Message, Renderer>(
    axis: Axis,
    renderer: &Renderer,
    limits: &Limits,
    padding: f32,
    spacing: f32,
    align_items: Align,
    items: &[Element<'_, Message, Renderer>],
) -> Node
where
    Renderer: crate::Renderer,
{
    let limits = limits.pad(padding);

    let mut total_non_fill =
        spacing as f32 * (items.len() as i32 - 1).max(0) as f32;
    let mut fill_sum = 0;
    let mut cross = axis.cross(limits.min());

    let mut nodes: Vec<Node> = Vec::with_capacity(items.len());
    nodes.resize(items.len(), Node::default());

    for (i, child) in items.iter().enumerate() {
        let fill_factor = match axis {
            Axis::Horizontal => child.width(),
            Axis::Vertical => child.height(),
        }
        .fill_factor();

        if fill_factor == 0 {
            let child_limits = Limits::new(Size::ZERO, limits.max());

            let layout = child.layout(renderer, &child_limits);
            let size = layout.size();

            total_non_fill += axis.main(size);
            cross = cross.max(axis.cross(size));

            nodes[i] = layout;
        } else {
            fill_sum += fill_factor;
        }
    }

    let available = axis.main(limits.max());
    let remaining = (available - total_non_fill).max(0.0);

    for (i, child) in items.iter().enumerate() {
        let fill_factor = match axis {
            Axis::Horizontal => child.width(),
            Axis::Vertical => child.height(),
        }
        .fill_factor();

        if fill_factor != 0 {
            let max_main = remaining * fill_factor as f32 / fill_sum as f32;
            let min_main = if max_main.is_infinite() {
                0.0
            } else {
                max_main
            };

            let (min_main, min_cross) =
                axis.pack(min_main, axis.cross(limits.min()));

            let (max_main, max_cross) =
                axis.pack(max_main, axis.cross(limits.max()));

            let child_limits = Limits::new(
                Size::new(min_main, min_cross),
                Size::new(max_main, max_cross),
            );

            let layout = child.layout(renderer, &child_limits);
            cross = cross.max(axis.cross(layout.size()));

            nodes[i] = layout;
        }
    }

    let mut main = padding;

    for (i, node) in nodes.iter_mut().enumerate() {
        if i > 0 {
            main += spacing;
        }

        let (x, y) = axis.pack(main, padding);

        node.bounds.x = x;
        node.bounds.y = y;

        match axis {
            Axis::Horizontal => {
                node.align(Align::Start, align_items, Size::new(0.0, cross));
            }
            Axis::Vertical => {
                node.align(align_items, Align::Start, Size::new(cross, 0.0));
            }
        }

        let size = node.size();

        main += axis.main(size);
    }

    let (width, height) = axis.pack(main, cross);
    let size = limits.resolve(Size::new(width, height));

    let (padding_x, padding_y) = axis.pack(padding, padding * 2.0);

    Node::with_children(
        Size::new(size.width + padding_x, size.height + padding_y),
        nodes,
    )
}
