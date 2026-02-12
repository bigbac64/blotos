use bootloader_api::info::{FrameBufferInfo};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    draw_target::DrawTarget,
};
use core::convert::Infallible;


/// La structure `StaticFramebufferAdapter` fournit une représentation d'un framebuffer statique.
/// Elle est utilisée pour interagir directement avec la mémoire du framebuffer.
///
/// Cette structure permet de manipuler le framebuffer en static dans un contexte d'OS
///
/// # Champs
///
/// * `buffer` - Un pointeur brut mutable vers une mémoire de framebuffer,
///              qui contient les données des pixels. La manipulation du pointeur servira à créer
///              des références pour sécuriser sa modification sans créer de conflit au lifetime
///
/// * `info` - Une structure `FrameBufferInfo` contenant des métadonnées importantes sur
///            le framebuffer, telles que sa résolution, son pas (stride) et son format de pixel.
///
pub struct StaticFramebufferAdapter{
    buffer: *mut u8,
    info: FrameBufferInfo,
}

/// TODO : comprendre et documenter la raison de ces implementations
unsafe impl Send for StaticFramebufferAdapter {}
unsafe impl Sync for StaticFramebufferAdapter {}

impl StaticFramebufferAdapter {
    pub unsafe fn new(buffer: *mut u8, info: FrameBufferInfo) -> Self {
        Self { buffer, info }
    }

    /// Cette méthode permet de créer un FramebufferAdapter à partir d'un StaticFramebufferAdapter
    /// en garantissant la sécurité de la manipulation du pointeur brut.
    pub fn as_framebuffer_adapter(&self) -> FramebufferAdapter<'_> {
        unsafe{
            FramebufferAdapter::new( {
                core::slice::from_raw_parts_mut(self.buffer, self.info.byte_len) }, self.info)
        }
    }
}

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
    /// Cette implementation permet a `embedded-graphics` de connaitre la dimension du framebuffer.
    fn size(&self) -> Size {
        // TODO ajouter une sécurité sur la conversion de typage (peux probable mais on sait jamais)
        Size::new(self.info.width as u32, self.info.height as u32)
    }
}

impl<'a> DrawTarget for FramebufferAdapter<'a> {
    type Color = Rgb888;
    type Error = Infallible;

    /// Cette implementation permet a `embedded-graphics` de dessiner des pixels sur le framebuffer.
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item=Pixel<Self::Color>>
    {
        for Pixel(point, color) in pixels {
            let x = point.x as usize;
            let y = point.y as usize;
            let stride = self.info.stride;
            let offset = (y * stride + x) * self.info.bytes_per_pixel;
            self.buffer[offset] = color.r();
            self.buffer[offset + 1] = color.g();
            self.buffer[offset + 2] = color.b();

            if self.info.bytes_per_pixel == 4 {
                self.buffer[offset + 3] = 255;
            }
        }
        Ok(())
    }
}