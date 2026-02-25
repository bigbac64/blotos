use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::Deref;
use crate::spin_lock::SpinLock;

pub struct OnceCell<T> {
    inner: UnsafeCell<MaybeUninit<T>>,
    initialized: SpinLock<bool>,
}

impl<T> OnceCell<T> {
    /// OnceCell avec le fonctionnement similaire au core::cell::OnceCell
    /// mais thread safe et static-able (grace au const de new mais reste quand meme limité)
    pub const fn new() -> OnceCell<T> {
        Self {
            inner: UnsafeCell::new(MaybeUninit::uninit()),
            initialized: SpinLock::new(false),
        }
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        let mut init = self.initialized.lock();
        if *init {
            return Err(value);
        }

        unsafe {
            (*self.inner.get()).write(value);
        }

        *init = true;
        Ok(())
    }

    pub fn get(&self) -> Option<&T> {
        let init = self.initialized.lock();
        if *init {
            unsafe { Some((*self.inner.get()).assume_init_ref()) }
        } else {
            None
        }
    }
}

unsafe impl<T: Send> Sync for OnceCell<T> {}

pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: F,
}

impl<T, F> Lazy<T, F> where F: Fn() -> T {
    pub const fn new(init: F) -> Lazy<T, F> {
        Lazy{
            cell: OnceCell::new(),
            init,
        }
    }

    pub fn force(&self) -> &T {
        if self.cell.get().is_none() {
            let tmp = (self.init)();
            let _ = self.cell.set(tmp);
        }

        self.cell.get().unwrap()
    }
}

impl<T, F> Deref for Lazy<T, F> where F: Fn() -> T {
    type Target = T;

    fn deref(&self) -> &T {
        self.force()
    }
}

unsafe impl<T, F> Sync for Lazy<T, F>
where
    T: Send,
    F: Sync,
{}

#[macro_export]
macro_rules! lazy_static {
    // Pattern : static ref NOM: TYPE = { code };
    (static ref $name:ident : $type:ty = $init:expr; ) => {
        static $name: $crate::lazy_static::Lazy<$type> = $crate::lazy_static::Lazy::new(|| $init);
    };
}