use core::fmt::Display;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{AnchorPoint, AnchorX, AnchorY, Point, Size};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::text::Text;
use crate::dbg_println;
use crate::framebuffer_adapter::FramebufferAdapter;
use crate::gui::layout::r#box::BoxLayout;
use crate::gui::layout::inset::Inset;
use crate::gui::render::Renderable;
/*
    pub fn compute(&self, mode: AnchorPoint, p: Point) -> Point {
        let x = match mode.x() {
            AnchorX::Left => p.x * (self.size().width as i32),  //
            AnchorX::Center => p.x * self.size().width as i32 + self.size().width as i32 / 2 + self.margin.left, //
            AnchorX::Right => (p.x + 1) * (self.margin.left + self.offset().width as i32 + self.margin.right), // p.y * (self.offset().height as i32)
        };

        let y: i32 = match mode.y() {
            AnchorY::Top => p.y * (self.size().height as i32),
            AnchorY::Center => p.y * self.size().height as i32 + self.size().height as i32 / 2 + self.margin.top,
            AnchorY::Bottom => p.y * (self.margin.top + self.offset().height as i32 + self.margin.bottom),
        };

        Point::new(x, y)
    }
}
*/

pub (crate) struct Glyph<'a> {
    pub(crate) style: MonoTextStyle<'a, Rgb888>
}

impl<'a> Glyph<'a>{
    pub fn new(style: MonoTextStyle<'a, Rgb888>) -> Self{
        Self {
            style
        }
    }
}

pub struct GlyphLayout {
    layout: BoxLayout
}

impl<'a> GlyphLayout {
    pub fn new(glyph: &Glyph, position: Point, margin: Inset) -> Self {
        let layout = BoxLayout::new(position, glyph.style.font.character_size, margin);
        Self {layout}
    }

    pub fn glyph_text_position(&self) -> Point {
        self.layout.inner_position_to(AnchorPoint::Center)
    }
}


pub struct GlyphRenderer {
    pub(crate) text: char,
    pub(crate) position: Point,
    pub(crate) style: MonoTextStyle<'static, Rgb888>,
}

impl Renderable for GlyphRenderer {
    fn render(&mut self, frame: &mut FramebufferAdapter) {
        // tableau d'octet pour utf-8 (max 4 bytes)
        Text::new(self.text.encode_utf8(&mut [0u8; 4]), self.position, self.style).draw(frame).expect("");
    }
}