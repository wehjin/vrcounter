use std::time::{Instant, Duration};

pub struct Beat {
    id: u64,
    fade_in: Instant,
    fade_out: Instant,
}

impl Beat {
    pub fn new(id: u64, duration: Duration) -> Self {
        let fade_in = Instant::now();
        let fade_out = fade_in + duration;
        Beat { id: id, fade_in: fade_in, fade_out: fade_out }
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn contains(&self, instant: &Instant) -> bool {
        instant >= &self.fade_in && instant < &self.fade_out
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn new_beat() {
        let beat = Beat::new(1, Duration::from_millis(1000));
        assert_eq!(1, beat.id());
        assert!(beat.contains(&Instant::now()));
    }
}