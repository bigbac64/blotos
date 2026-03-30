use embedded_graphics::geometry::Point;
use crate::framebuffer_adapter::StaticFramebufferAdapter;

pub(crate) struct WindowContent{

}

pub(crate) struct Window{
    display: StaticFramebufferAdapter,
    size: Point
}