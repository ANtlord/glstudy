use std::time::{Duration, SystemTime};

use anyhow::Context;

pub struct FrameRate {
    pub last: SystemTime,
    pub duration: Duration,
}

impl Default for FrameRate {
    fn default() -> Self {
        Self { last: SystemTime::now(), duration: Duration::default() }
    }
}

impl FrameRate {
    pub fn update(&mut self) -> anyhow::Result<()> {
        let current = SystemTime::now();
        self.duration =
            current.duration_since(self.last).context("fail getting duration between frames")?;
        self.last = current;
        Ok(())
    }
}

