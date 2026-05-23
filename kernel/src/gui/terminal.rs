use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_graphics::mono_font::ascii::FONT_9X18;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::{pixelcolor::Rgb888, text::Text, Drawable};
use embedded_graphics::geometry::{AnchorPoint, Point, Size};
use embedded_graphics::pixelcolor::{RgbColor, WebColors};
use embedded_graphics::primitives::{Primitive, PrimitiveStyle, Rectangle};
use crate::dbg_println;
use crate::framebuffer_adapter::FramebufferAdapter;
use crate::gui::layout::grid::GridBoxLayout;
use crate::gui::layout::inset::Inset;
use crate::gui::layout::r#box::BoxLayout;
use crate::gui::render::Renderable;
use crate::gui::window::Window;
use crate::utils::array::PositionalSequence;
use super::layout::glyph::{Glyph, GlyphLayout, GlyphRenderer};


pub(crate) struct Terminal{
    renders: Vec<Box<dyn Renderable>>,
    cursor: Point, // position du curseur
    stdin: PositionalSequence<(char, Point), 256>, // limitation de la taille du buffer (à réadapter au besoin)
    glyph: Glyph<'static>,
    dirty: bool,
}

impl Renderable for Terminal {
    fn render(&mut self, frame: &mut FramebufferAdapter) {
        for mut rend in self.renders.drain(..).collect::<Vec<_>>(){
            rend.render(frame);
        }
    }
}

impl Window for Terminal {

    fn win_layout(&self) -> BoxLayout {
        let mut padding = Inset::new(2, 2, 2, 2);
        padding.add_all(self.win_stroke() as i32);
        BoxLayout::new(
            Point::new(5, 5),
            Size::new(740, 540),
            padding,
        )
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            renders: Vec::new(),
            cursor: Point::new(0, 0),
            stdin: PositionalSequence::new(),
            dirty: true,
            glyph: Glyph::new(
                MonoTextStyleBuilder::new()
                    .font(&FONT_9X18)
                    .text_color(Rgb888::new(0, 255, 0))  // Vert phosphore
                    .background_color(Rgb888::BLACK)
                    .build()
            )
        }
    }

    pub fn remove_char(&mut self) {
        dbg_println!("remove char");
        self.cursor = self.stdin.current().unwrap().1;
        self.stdin.remove();
        // ici RenderGlyph Empty ?
        let point = self.cursor_alignment(AnchorPoint::Center, None);
        self.renders.push(Box::new(GlyphRenderer {text: ' ', position: point, style: self.glyph.style}));
        self.dirty = true;
    }

    pub fn add_char(&mut self, c: char){
        self.stdin.insert((c, self.cursor.clone()));
        // GlyphLayout::compile selon l'etat et defini la position
        // peut etre qu'au final une struct GlyphRenderer auras une utiliter si on render un text ou un Rectangle ?
        // plus de self action mais des push de render ? et Terminal.render() boucle for l in list_renderer l.render() ?
        let point = self.cursor_alignment(AnchorPoint::Center, None);
        self.renders.push(Box::new(GlyphRenderer {text: c, position: point, style: self.glyph.style}));
        if c == '\n' {
            self.new_line();
        } else {
            self.new_col();
        }
        self.dirty = true;
    }

    pub fn new_line(&mut self){
        self.cursor.y += 1;
        self.cursor.x = 0;
    }

    pub fn new_col(&mut self){
        let compute = self.cursor_alignment(AnchorPoint::TopRight, Point::new(1, 0));
        dbg_println!("{:?}", compute);

        if self.win_layout().content_size.width < compute.x as u32 {
            self.new_line()
        } else {
            self.cursor.x += 1;
        }
    }

    /// le impl Into<...> permet la conversion de Point en Option<Point> sans avoir a faire Some(Point)
    pub fn cursor_alignment(&self, anchor_point: AnchorPoint, offset_point: impl Into<Option<Point>>) -> Point{
        dbg_println!("{:?}",self.cursor);
        GridBoxLayout::new(
            &BoxLayout::new(self.cursor, self.glyph.style.font.character_size, Inset::new(2, 0, 2, 0))
        ).resolve(self.cursor + offset_point.into().unwrap_or(Point::zero()), anchor_point)
            + self.win_layout().inner_position_to(AnchorPoint::TopLeft)
    }
}

impl<'a> core::fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars().into_iter() {
            self.add_char(char)
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.add_char(c);
        Ok(())
    }
}