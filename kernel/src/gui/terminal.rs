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
use crate::gui::window::Window;
use crate::utils::array::PositionalSequence;
use super::layout::glyph::{Glyph, GlyphBox, GlyphLayout};

enum TerminalAction{
    WriteChar(char),
    NewLine,
    Clear,
    Backspace,
}

pub(crate) struct Terminal{
    actions: Vec<TerminalAction>,
    cursor: Point, // position du curseur
    stdin: PositionalSequence<(char, Point), 256>, // limitation de la taille du buffer (à réadapter au besoin)
    glyph: Glyph<'static>,
    dirty: bool,
}



impl Window for Terminal {

    fn win_layout(&self) -> BoxLayout {
        let mut padding = Inset::new(2, 2, 2, 2);
        padding.add_all(self.win_stroke() as i32);
        BoxLayout::new(
            Size::new(740, 540),
            padding,
        )
    }

    fn win_position(&self) -> Point {
        Point::new(5, 5)
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn mark_clean(&mut self) {
        self.dirty = false;
    }

    fn render(&mut self, frame: &mut FramebufferAdapter) {
        for action in self.actions.drain(..).collect::<Vec<_>>(){
            match action {
                TerminalAction::WriteChar(c) => {
                    let mut buf = [0u8; 4]; // tableau d'octet pour utf-8 (max 4 bytes)
                    let character: &str = c.encode_utf8(&mut buf);
                    if character == "\n" {
                        self.new_line();
                    } else {
                        let inset = Inset::new(2, 0, 2, 0);
                        // gerer le positionnement du texte il doit aussi etre utilisable dans backspace pour rect
                        GlyphBox::new(character, self.glyph, inset).draw(self.win_layout().content_offset(), frame);
                        self.new_col();
                    }
                }
                TerminalAction::NewLine => {
                    self.new_line();
                },
                TerminalAction::Clear => todo! (),
                TerminalAction::Backspace => {
                    Rectangle::new(self.alignment(self.cursor), self.glyph.size())
                        .into_styled(PrimitiveStyle::with_fill(Rgb888::CSS_GRAY))
                        .draw(frame)
                        .unwrap();
                }
            }
        }
    }
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            cursor: Point::new(0, 0),
            stdin: PositionalSequence::new(),
            dirty: true,
            glyph: Glyph::new(
                MonoTextStyleBuilder::new()
                    .font(&FONT_9X18)
                    .text_color(Rgb888::new(0, 255, 0))  // Vert phosphore
                    .background_color(Rgb888::RED)
                    .build()
            )
        }
    }

    pub fn remove_char(&mut self) {
        dbg_println!("remove char");
        self.cursor = self.stdin.current().unwrap().1;
        self.stdin.remove();
        self.actions.push(TerminalAction::Backspace); // TODO ici on part du principe que actions est executé plus tard, si cursor change alors ca va bugger
        self.dirty = true;
    }

    pub fn add_char(&mut self, c: char){
        self.stdin.insert((c, self.cursor.clone()));
        self.actions.push(TerminalAction::WriteChar(c));
        self.dirty = true;
    }

    pub fn new_line(&mut self){
        self.cursor.y += 1;
        self.cursor.x = 0;
    }

    pub fn new_col(&mut self){
        let compute = self.alignment(AnchorPoint::TopRight, self.cursor + Point::new(1, 0));

        if self.win_size().width < compute.x as u32 {
            self.new_line()
        } else {
            self.cursor.x += 1;
        }
    }

    pub fn cursor_alignment(&self, anchor_point: AnchorPoint) -> Point{
        GridBoxLayout::new(&self.win_layout()).resolve(self.cursor, anchor_point)
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