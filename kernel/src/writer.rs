use embedded_graphics::prelude::Point;

struct BufferWrite{
    position: Point,
    offset: Point,
    padding_line: Point,
}


impl BufferWrite {
    pub fn new(offset: Point) -> Self {
        Self { position: Point::zero(), offset, padding_line: Point::zero() }
    }
}