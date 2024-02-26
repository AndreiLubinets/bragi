use std::time::{Duration, Instant};

#[derive(Clone, Default)]
pub struct Playtime {
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    pause_duration: Duration,
}

impl Playtime {
    pub fn pause(&mut self) {
        self.pause_time = Some(Instant::now());
    }

    pub fn play(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        if let Some(t) = self.pause_time.take() {
            self.pause_duration += t.elapsed();
        }
    }

    pub fn time(&self) -> Duration {
        match self.start_time {
            Some(start) => match self.pause_time {
                Some(t) => start.elapsed() - t.elapsed() - self.pause_duration,
                None => start.elapsed() - self.pause_duration,
            },
            None => Duration::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Playtime;

    #[test]
    fn time_test() {
        let mut playtime = Playtime::default();
        playtime.play();

        let time = playtime.time();

        assert!(!time.is_zero());
    }

    #[test]
    fn time_with_pause_test() {
        //FIXME: Current approach does not work
        let mut playtime = Playtime::default();
        playtime.play();
        playtime.pause();

        let first = playtime.time();
        let second = playtime.time();

        assert_eq!(first, second);
    }

    #[test]
    fn time_not_started() {
        assert!(Playtime::default().time().is_zero());
    }
}
