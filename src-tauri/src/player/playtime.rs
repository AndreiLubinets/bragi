use log::debug;
use std::time::{Duration, Instant};

/// Tracks the playtime. Use `play` and `pause` to start and stop the playtime. Use `time` to get the duration of the playtime.
///
/// # Examples
/// ```rust
/// use bragi::player::playtime::Playtime;
/// let mut playtime = Playtime::default();
/// playtime.play();
/// playtime.pause();
/// let time = playtime.time();
/// ```
#[derive(Clone, Default)]
pub struct Playtime {
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    pause_duration: Duration,
}

impl Playtime {
    /// Pauses the playtime. Use `play` to start it again.
    ///
    ///
    /// # Examples
    /// ```rust
    /// use bragi::player::playtime::Playtime;
    /// let mut playtime = Playtime::default();
    /// playtime.pause();
    /// playtime.play();
    /// ```
    pub fn pause(&mut self) {
        self.pause_time = Some(Instant::now());
        debug!("Paused at: {:?}", self.pause_time);
    }

    /// Starts the playtime. Use `pause` to stop it.
    ///
    /// # Examples
    /// ```rust
    /// use bragi::player::playtime::Playtime;
    /// let mut playtime = Playtime::default();
    /// playtime.play();
    /// playtime.pause();
    /// ```
    pub fn play(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
            debug!("Started at: {:?}", self.start_time);
        }

        if let Some(t) = self.pause_time.take() {
            self.pause_duration += t.elapsed();
        }
    }

    /// Returns the duration of the playtime.
    ///
    /// # Examples
    /// ```rust
    /// use bragi::player::playtime::Playtime;
    /// let mut playtime = Playtime::default();
    /// playtime.play();
    /// let time = playtime.time();
    /// ```
    pub fn time(&self) -> Duration {
        match self.start_time {
            Some(start) => match self.pause_time {
                Some(t) => start.elapsed() - t.elapsed() - self.pause_duration,
                None => start.elapsed() - self.pause_duration,
            },
            None => Duration::ZERO,
        }
    }

    pub fn change(&mut self, time: Duration) {
        self.pause_duration += time;
    }
}

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};

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
        let mut playtime = Playtime::default();
        playtime.play();
        playtime.pause();

        let first = playtime.time();
        //TODO: Find a way to remove sleep
        sleep(Duration::from_secs(1));
        let second = playtime.time();

        assert_eq!(first.as_secs(), second.as_secs());
    }

    #[test]
    fn time_not_started() {
        assert!(Playtime::default().time().is_zero());
    }
}
