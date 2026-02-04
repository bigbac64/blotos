use core::fmt::Write;
use embedded_graphics::mono_font::ascii::FONT_9X18;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use embedded_graphics::prelude::Point;
use crate::framebuffer_adapter::FramebufferAdapter;

pub(crate) struct Terminal<'a>{
    display: &'a mut FramebufferAdapter<'a>,
    position: Point, // position des caractère col, line
    // cursor: Point, // position du curseur
    offset: Point, // espace pris par caractère en pixel
    padding_line: Point, // espace x entre le bord de l'écran, espace y entre chaque ligne
    style: MonoTextStyle<'a, Rgb888>,
}

impl<'a> Terminal<'a> {
    pub fn new(display: &'a mut FramebufferAdapter<'a>) -> Self {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18)
            .text_color(Rgb888::new(0, 255, 0))  // Vert phosphore
            .background_color(Rgb888::BLACK)
            .build();
        Self {
            display,
            position: Point::new(0, 1),
            // cursor: Point::zero(),
            offset: Point::new(9, 18),
            padding_line: Point::new(5, 5),
            style,
        }
    }

    pub fn pixel_align(&self) -> Point{
        Point::new(self.position.x * (self.offset.x + self.padding_line.x), self.position.y * (self.offset.y + self.padding_line.y))
    }

    pub fn new_line(&mut self) {
        self.position.y += 1;
        self.position.x = 0;
    }
}

impl<'a> Write for Terminal<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let collection = s.split('\n');
        let count = collection.clone().count();
        for (i, line) in collection.into_iter().enumerate()  {
            Text::new(line, self.pixel_align(), self.style).draw(self.display).expect("Write error");
            self.position.x += line.len() as i32;
            if i != count - 1 {
                self.new_line();
            }
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        let mut buf = [0u8; 4]; // tableau d'octet pour utf-8 (max 4 bytes)
        self.write_str(c.encode_utf8(&mut buf))
    }
}