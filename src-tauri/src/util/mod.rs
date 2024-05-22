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

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! assert_vec_eq {
        ($left:expr, $right:expr) => {
            let left_set: std::collections::HashSet<_> = $left.into_iter().collect();
            let right_set: std::collections::HashSet<_> = $right.into_iter().collect();
            assert_eq!(left_set, right_set);
        };
    }
}
