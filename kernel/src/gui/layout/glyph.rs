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
    pub(crate) font: &'a MonoFont<'a>,
    pub(crate) style: MonoTextStyle<'a, Rgb888>
}

impl<'a> Glyph<'a>{
    pub fn new(style: MonoTextStyle<'a, Rgb888>) -> Self{
        Self {font: style.font, style}
    }

    pub fn setting<'b>(&self, data:&'b str, position: Point) -> Text<'b, MonoTextStyle<'a, Rgb888>> {
        Text::new(data, position, self.style)
    }
}

pub struct GlyphBox<'a> {
    layout: BoxLayout,
    glyph: Glyph<'a>,
    text: &'a str,
}

impl<'a> GlyphBox<'a> {
    pub fn new(text: &'a str, glyph: Glyph<'a>, margin: Inset) -> Self {
        let layout = BoxLayout::new(glyph.font.character_size, margin);
        Self { text, layout, glyph }
    }

    pub fn draw(&self, position: Point, display: &mut FramebufferAdapter) {
        self.glyph.setting(self.text, position).draw(display).expect("");
    }
}
