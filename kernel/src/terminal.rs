use alloc::vec;
use alloc::vec::Vec;
use embedded_graphics::mono_font::ascii::FONT_9X18;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use embedded_graphics::prelude::Point;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use crate::framebuffer_adapter::FramebufferAdapter;
use crate::graphie::DISPLAY_;
use crate::utils::array::PositionalSequence;
use crate::window::Window;

enum TerminalAction{
    WriteChar(char),
    NewLine,
    Clear,
    Backspace,
}

pub(crate) struct Terminal{
    actions: Vec<TerminalAction>,
    position: Point, // position des caractère col, line
    // cursor: Point, // position du curseur
    readline: PositionalSequence<(char, Point), 256>, // limitation de la taille du buffer (à réadapter au besoin)
    offset: Point, // espace pris par caractère en pixel
    padding_line: Point, // espace x entre le bord de l'écran, espace y entre chaque ligne
    style: MonoTextStyle<'static, Rgb888>,
}

impl Window for Terminal {

    fn win_position(&self) -> Point {
        Point::new(5, 5)
    }
    fn win_size(&self) -> Size {
        Size::new(740, 540)
    }
    fn render(&mut self, frame: &mut FramebufferAdapter) {
        for action in self.actions.drain(..).collect::<Vec<_>>(){
            match action {
                TerminalAction::WriteChar(c) => {
                    let mut buf = [0u8; 4]; // tableau d'octet pour utf-8 (max 4 bytes)
                    let glyph: &str = c.encode_utf8(&mut buf);
                    if glyph == "\n" {
                        self.new_line();
                    } else {
                        Text::new(glyph, self.pixel_align(), self.style)
                            .draw(frame)
                            .expect("Write error");
                        self.new_col();
                        self.readline.insert((c, self.position));
                    }
                }
                TerminalAction::NewLine => {
                    self.new_line();
                },
                TerminalAction::Clear => todo! (),
                TerminalAction::Backspace => {
                    Rectangle::new(self.pixel_align(), Size::new((self.offset.x + self.padding_line.x) as u32, (self.offset.y + self.padding_line.y) as u32))
                        .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
                        .draw(&mut DISPLAY_.lock().as_mut().unwrap().0.as_framebuffer_adapter())
                        .unwrap();
                }
            }
        }
    }
}

impl Terminal {
    pub fn new() -> Self {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18)
            .text_color(Rgb888::new(0, 255, 0))  // Vert phosphore
            .background_color(Rgb888::BLACK)
            .build();

        Self {
            actions: Vec::new(),
            position: Point::new(0, 1),
            // cursor: Point::zero(),
            readline: PositionalSequence::new(),
            offset: Point::new(9, 18),
            padding_line: Point::new(1, 5),
            style,
        }
    }

    pub fn pixel_align(&self) -> Point{
        Point::new(
            self.position.x * (self.offset.x + self.padding_line.x) + self.win_position().x,
            self.position.y * (self.offset.y + self.padding_line.y) + self.win_position().y,
        )
    }

    pub fn new_line(&mut self) {
        self.position.y += 1;
        self.position.x = 0;
    }

    pub fn remove_char(&mut self) {
        self.readline.remove();
        self.position = self.readline.current().unwrap().1;
        self.actions.push(TerminalAction::Backspace)
    }

    pub fn new_col(&mut self) {
        self.position.x += 1;
    }
}

impl<'a> core::fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars().into_iter() {
            self.actions.push(TerminalAction::WriteChar(char));
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.actions.push(TerminalAction::WriteChar(c));
        Ok(())
    }
}