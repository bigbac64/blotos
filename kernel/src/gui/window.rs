use alloc::boxed::Box;
use alloc::collections::btree_map::{Iter, IterMut};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::any::{TypeId};
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::primitives::{Primitive, PrimitiveStyle, Rectangle};
use crate::framebuffer_adapter::FramebufferAdapter;
use crate::graphie::DISPLAY_;
use crate::gui::layout::inset::Inset;
use crate::gui::layout::r#box::BoxLayout;
use crate::spin_lock::SpinLock;
use crate::utils::downcaster::Downcast;

pub(crate) struct WindowRegistry{
    pub(crate) register: BTreeMap<TypeId, Vec<Box<dyn Window>>>
}

impl WindowRegistry {
    pub fn insert(&mut self, id: TypeId, window: Box<dyn Window>) {
        self.register.entry(id).or_insert_with(Vec::new).push(window);
    }

    pub fn get(&self, id: &TypeId, index: usize) -> Option<&Box<dyn Window>> {
        self.register.get(id).unwrap().get(index)
    }

    pub fn get_mut(&mut self, id: &TypeId, index: usize) -> Option<&mut Box<dyn Window>> {
        self.register.get_mut(id).unwrap().get_mut(index)
    }

    pub fn all(&mut self) -> Iter<'_, TypeId, Vec<Box<dyn Window>>>{
        self.register.iter()
    }

    pub fn all_mut(&mut self) -> IterMut<'_, TypeId, Vec<Box<dyn Window>>>{
        self.register.iter_mut()
    }
}

unsafe impl Send for WindowRegistry {}
unsafe impl Sync for WindowRegistry {}

pub(crate) static WINDOW_REGISTRY: SpinLock<WindowRegistry> = SpinLock::new(WindowRegistry {
    register: BTreeMap::new() , // pas de hashmap en no_std
});


pub(crate) trait Window: Downcast {
    fn win_layout(&self) -> BoxLayout;
    fn win_position(&self) -> Point;
    fn is_dirty(&self) -> bool;
    fn mark_clean(&mut self);

    fn background_color(&self) -> Rgb888 {
        Rgb888::BLACK
    }

    fn win_stroke(&self) -> u32 {3}

    fn draw(&self, frame: &mut FramebufferAdapter){
        Rectangle::new(self.win_position(), self.win_layout().size())
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, self.win_stroke()))
            .draw(frame)
            .unwrap()
    }

    fn render(&mut self, frame: &mut FramebufferAdapter);

    fn register(self: Box<Self>) where Self: Sized + 'static {
        WINDOW_REGISTRY.lock().insert(TypeId::of::<Self>(), self);
    }
}


pub fn render_updated_window(){
    let mut guard_display = DISPLAY_.lock();
    let display = guard_display.as_mut().unwrap();

    let mut registers = WINDOW_REGISTRY.lock();

    for (_, windows) in registers.all_mut() {
        for window in windows{
            if window.is_dirty(){
                window.draw(&mut display.0.as_framebuffer_adapter());
                window.render(&mut display.0.as_framebuffer_adapter());
                window.mark_clean()
            }
        }
    }
}