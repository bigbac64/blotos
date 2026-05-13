use embedded_graphics::geometry::{AnchorPoint, AnchorX, AnchorY, Size};
use embedded_graphics::prelude::Point;
use crate::gui::layout::r#box::BoxLayout;

pub(crate) struct GridBoxLayout<'a> {
    box_layout: &'a BoxLayout,
}

impl<'a> GridBoxLayout<'a> {
    pub fn new(box_layout: &'a BoxLayout) -> Self {
        Self {
            box_layout,
        }
    }

    pub fn resolve(&self, cell_index: Point, anchor: AnchorPoint) -> Point {
        let left = cell_index.x * self.box_layout.size().width as i32;
        let x = match anchor.x() {
            AnchorX::Left => { left }
            AnchorX::Center => { left + self.box_layout.size().width as i32 / 2 }
            AnchorX::Right => { left + self.box_layout.size().width as i32 }
        };

        let top = cell_index.y * self.box_layout.size().height as i32;
        let y = match anchor.y() {
            AnchorY::Top => { top }
            AnchorY::Center => { top + self.box_layout.size().height as i32 / 2 }
            AnchorY::Bottom => { top + self.box_layout.size().height as i32 }
        };

        Point::new(x, y)
    }

}