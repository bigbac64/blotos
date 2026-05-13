use embedded_graphics::geometry::{AnchorPoint, AnchorX, AnchorY};
use embedded_graphics::prelude::{Point, Size};

pub(crate) struct Transform {
    position: Point,
    size: Size,
}

impl Transform {
    pub fn anchor_point(&self, anchor: AnchorPoint) -> Point{

        let x: i32 = match anchor.x() {
            AnchorX::Left => 0,
            AnchorX::Center => (self.size.width / 2) as i32,
            AnchorX::Right => self.size.width as i32,
        };

        let y: i32 = match anchor.y() {
            AnchorY::Top => 0,
            AnchorY::Center => (self.size.height / 2) as i32,
            AnchorY::Bottom => self.size.height as i32,
        };

        Point::new(x, y)
    }
}