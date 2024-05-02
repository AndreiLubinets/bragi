use std::sync::atomic::{AtomicUsize, Ordering};

pub trait AtomicSub<T> {
    fn saturating_sub(&self, val: T);
}

impl AtomicSub<usize> for AtomicUsize {
    #[inline]
    fn saturating_sub(&self, val: usize) {
        self.store(
            self.load(Ordering::Acquire).saturating_sub(val),
            Ordering::Release,
        );
    }
}
