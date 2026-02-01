use bootloader_api::info::{FrameBufferInfo};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    draw_target::DrawTarget,
};
use core::convert::Infallible;

pub struct FramebufferAdapter<'a> {
    buffer: &'a mut [u8],
    info: FrameBufferInfo,
}

impl<'a> FramebufferAdapter<'a> {
    pub fn new(buffer: &'a mut [u8], info: FrameBufferInfo) -> Self {
        Self { buffer, info }
    }
}

impl<'a> OriginDimensions for FramebufferAdapter<'a> {
    fn size(&self) -> Size {
        // TODO ajouter une sécurité sur la conversion de typage (peux probable mais on sait jamais)
        Size::new(self.info.width as u32, self.info.height as u32)
    }
}

impl<'a> DrawTarget for FramebufferAdapter<'a> {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item=Pixel<Self::Color>>
    {
        for Pixel(point, color) in pixels {
            let x = point.x as usize;
            let y = point.y as usize;
            let stride = self.info.stride;
            let offset = (y * stride + x * 3) * self.info.bytes_per_pixel;
            self.buffer[offset] = color.r();
            self.buffer[offset + 1] = color.g();
            self.buffer[offset + 2] = color.b();
        }
        Ok(())
    }
}