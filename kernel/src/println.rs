#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        { // scope servant a isolé le nom des variables et éviter les conflits
            use core::fmt::Write; // necessaire pour utiliser la fonction write!
            /* le crate est necessaire pour ne pas demander les imports a chaque utilisation */
            let mut guard = crate::DISPLAY.lock();
            if let Some(static_framebuffer) = guard.as_mut() {
                let display = &mut static_framebuffer.as_framebuffer_adapter();
                let mut terminal = crate::writer::Terminal::new(display);
                let _ = write!(&mut terminal, $($arg)*);
            }
        }
    }
}

#[macro_export]
macro_rules! println {
    () => { crate::print!("\n") };
    ($($arg:tt)*) => { crate::print!("{}\n", format_args!($($arg)*)) }
}
