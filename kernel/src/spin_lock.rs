use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    /* Gestion qui existe en Rust, mais je le refais pour comprendre le fonctionnement */
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.locked.compare_exchange(
            false,
            true,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).is_err() {
            // Spin !
        }

        SpinLockGuard{ lock: &self }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::SeqCst);
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,  // Référence au spinlock
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}


impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &*self.lock.data.get()
        }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.lock.data.get()
        }
    }
}
// on définit que spinlock est safe pour l'utiliser en static (avec Sync)
unsafe impl<T: Send> Sync for SpinLock<T> {}