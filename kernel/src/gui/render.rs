use crate::framebuffer_adapter::FramebufferAdapter;

pub(crate) trait Renderable {
    fn render(&mut self, frame: &mut FramebufferAdapter);
}