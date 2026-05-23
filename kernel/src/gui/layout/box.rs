use embedded_graphics::geometry::{AnchorPoint, AnchorX, AnchorY, Point, Size};
use crate::gui::layout::inset::Inset;


pub struct BoxLayout {
    pub position: Point, // top left position
    pub padding: Inset,
    pub content_size: Size,
}

impl BoxLayout {
    pub fn new( position: Point, content_size: Size, padding: Inset) -> Self {
        Self { position, padding, content_size }
    }

    pub fn size(&self) -> Size {
        Size::new(
            self.content_size.width + (self.padding.left + self.padding.right) as u32,
            self.content_size.height + (self.padding.top + self.padding.bottom) as u32,
        )
    }

    pub fn inner_size(&self) -> Size {
        self.content_size
    }

    pub fn outer_offset_to(&self, anchor_point: AnchorPoint) -> Point {
        let x = match anchor_point.x() {
            AnchorX::Left => { 0 }
            AnchorX::Center => { (self.padding.left + self.content_size.width as i32 + self.padding.right) / 2 }
            AnchorX::Right => { self.padding.left + self.content_size.width as i32 + self.padding.right }
        };

        let y = match anchor_point.y() {
            AnchorY::Top => { 0 }
            AnchorY::Center => { (self.padding.top + self.content_size.height as i32 + self.padding.bottom) / 2 }
            AnchorY::Bottom => { self.padding.top + self.content_size.height as i32 + self.padding.bottom }
        };

        Point::new(x, y)
    }

    pub fn outer_position_to(&self, anchor_point: AnchorPoint) -> Point {
        self.position + self.outer_offset_to(anchor_point)
    }

    pub fn inner_offset_to(&self, anchor_point: AnchorPoint) -> Point {
        let x = match anchor_point.x() {
            AnchorX::Left => { self.padding.left }
            AnchorX::Center => { self.padding.left + self.content_size.width as i32 / 2 }
            AnchorX::Right => { self.padding.left + self.content_size.width as i32 }
        };

        let y = match anchor_point.y() {
            AnchorY::Top => { self.padding.top }
            AnchorY::Center => { self.padding.top + self.content_size.height as i32 / 2 }
            AnchorY::Bottom => { self.padding.top + self.content_size.height as i32 }
        };

        Point::new(x, y)
    }

    pub fn inner_position_to(&self, anchor_point: AnchorPoint) -> Point {
        self.position + self.inner_offset_to(anchor_point)
    }
}
