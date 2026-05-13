use embedded_graphics::geometry::{Point, Size};
use crate::gui::layout::inset::Inset;


pub struct BoxLayout {
    pub padding: Inset,
    pub content_size: Size,
}

impl BoxLayout {
    pub fn new(content_size: Size, padding: Inset) -> Self {
        Self { padding, content_size }
    }

    pub fn size(&self) -> Size {
        Size::new(
            self.content_size.width + (self.padding.left + self.padding.right) as u32,
            self.content_size.height + (self.padding.top + self.padding.bottom) as u32,
        )
    }

    pub fn content_offset(&self) -> Point {
        Point::new(self.padding.left, self.padding.top)
    }
}
