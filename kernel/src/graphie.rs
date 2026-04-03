use crate::framebuffer_adapter::StaticFramebufferAdapter;
use crate::spin_lock::SpinLock;

#[repr(transparent)]
pub(crate) struct Display(StaticFramebufferAdapter);

/// Une fois le kernel lancer, on peut récupérer et utiliser l'affichage sur une table de pixel, 
/// avec quelques informations sur le framebuffer.
/// ## Usage
/// ```
/// let mut guard = crate::DISPLAY.lock();
/// 
/// if let Some(display) = guard.as_mut() {
///   ...
/// }
/// ```
pub(crate) static DISPLAYY: SpinLock<Option<Display>> = SpinLock::new(None);

/// Initialise l'affichage static pour le static DISPLAY
pub(crate) fn init_display(buffer: *mut u8, info: bootloader_api::info::FrameBufferInfo) {
    *DISPLAYY.lock() = Some(Display { 0: unsafe { StaticFramebufferAdapter::new(buffer, info) } });
}

