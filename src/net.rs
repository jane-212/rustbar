use crate::bar::Module;
use crate::error;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

pub struct Net {
    filename: PathBuf,
    bytes: u64,
    last_bytes: u64,
}

impl Default for Net {
    fn default() -> Self {
        Self {
            filename: PathBuf::from("/proc/net/dev"),
            bytes: 0,
            last_bytes: 0,
        }
    }
}

#[async_trait]
impl Module for Net {
    async fn update(&mut self) -> error::IResult<()> {
        self.last_bytes = self.bytes;
        self.bytes ^= self.bytes;

        fs::read_to_string(&self.filename)
            .await?
            .lines()
            .skip(2)
            .for_each(|line| {
                self.bytes += line
                    .split(':')
                    .into_iter()
                    .last()
                    .unwrap_or("unknown")
                    .split_whitespace()
                    .next()
                    .unwrap_or("unknown")
                    .parse::<u64>()
                    .unwrap_or(0)
            });

        Ok(())
    }

    async fn render(&self) -> error::IResult<String> {
        let diff = (self.bytes - self.last_bytes) as f64 / 1024.0;

        if diff < 1024.0 {
            Ok(format!("  {:.1}Kb/s ", diff))
        } else {
            Ok(format!("  {:.1}Mb/s ", diff / 1024.0))
        }
    }
}
