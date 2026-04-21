#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {// scope servant a isolé le nom des variables et éviter les conflits
            use core::fmt::Write;
            /* le crate est necessaire pour ne pas demander les imports a chaque utilisation */
            let mut registry = crate::window::WINDOW_REGISTRY.lock();
            if let Some(window) = registry.get_mut(&core::any::TypeId::of::<crate::terminal::Terminal>(), 0) {
                let _ = write!(window.as_any_mut().downcast_mut::<crate::terminal::Terminal>().unwrap(), $($arg)*);
            }

        }
    }
}

#[macro_export]
macro_rules! println {
    () => { crate::print!("\n") };
    ($($arg:tt)*) => { crate::print!("{}\n", format_args!($($arg)*)) }
}
