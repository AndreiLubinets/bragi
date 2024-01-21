use std::{ops::Add, time::Duration};

pub struct Playtime {
    total: Duration,
    current: Duration,
}

impl Playtime {
    pub fn new(total: Duration) -> Playtime {
        Playtime {
            total,
            current: Duration::ZERO,
        }
    }

    fn start(&self) {
        todo!()
    }

    pub fn current(&self) -> Duration {
        self.current
    }
}
