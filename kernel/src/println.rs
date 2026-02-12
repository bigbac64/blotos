#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        { // scope servant a isolé le nom des variables et éviter les conflits
            let mut guard = DISPLAY.lock();
            if let Some(static_framebuffer) = guard.as_mut() {
                let display = &mut static_framebuffer.as_framebuffer_adapter();
                let mut terminal = Terminal::new(display);
                let _ = write!(&mut terminal, $($arg)*);
            }
        }
    }
}

#[macro_export]
macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => { print!("{}\n", format_args!($($arg)*)) }
}
