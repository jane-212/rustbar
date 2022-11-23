use crate::bar::Module;
use crate::error;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::{
    fs,
    time::{Duration, Instant},
};

pub struct Version {
    filename: PathBuf,
    version: String,
    last_tick: Instant,
    interval: Duration,
    is_fisrt: bool
}

impl Default for Version {
    fn default() -> Self {
        Self {
            filename: PathBuf::from("/proc/version"),
            version: String::new(),
            last_tick: Instant::now(),
            interval: Duration::from_secs(60 * 60 * 12),
            is_fisrt: true
        }
    }
}

#[async_trait]
impl Module for Version {
    async fn update(&mut self) -> error::IResult<()> {
        if self.is_fisrt {
            self.version = fs::read_to_string(&self.filename)
                .await?
                .split_whitespace()
                .nth(2)
                .unwrap_or("unknown")
                .to_owned();
            self.is_fisrt = false;
            self.last_tick = Instant::now();
        }

        if self.last_tick.elapsed() >= self.interval {
            self.version = fs::read_to_string(&self.filename)
                .await?
                .split_whitespace()
                .nth(2)
                .unwrap_or("unknown")
                .to_owned();
            self.last_tick = Instant::now();
        }

        Ok(())
    }

    async fn render(&self) -> error::IResult<String> {
        Ok(format!(" arch: {} ", self.version))
    }
}
