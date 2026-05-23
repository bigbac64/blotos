use embedded_graphics::geometry::{AnchorPoint, AnchorX, AnchorY, Size};
use embedded_graphics::prelude::Point;
use crate::gui::layout::r#box::BoxLayout;

pub(crate) struct GridBoxLayout<'a> {
    cell_layout: &'a BoxLayout,
}

impl<'a> GridBoxLayout<'a> {
    pub fn new(cell_layout: &'a BoxLayout) -> Self {
        Self {
            cell_layout,
        }
    }

    pub fn resolve(&self, cell_index: Point, anchor: AnchorPoint) -> Point {
        let cell_size =  self.cell_layout.size();
        let cell_offset = self.cell_layout.outer_offset_to(anchor);
        Point::new(
            cell_index.x * cell_size.width as i32 + cell_offset.x,
            cell_index.y * cell_size.height as i32 + cell_offset.y
        )
    }

}