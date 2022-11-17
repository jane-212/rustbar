use crate::bar::Module;
use crate::error;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

pub struct Memory {
    filename: PathBuf,
    available: u64,
    total: u64,
}

impl Default for  Memory {
    fn default() -> Self {
        Self {
            filename: PathBuf::from("/proc/meminfo"),
            available: 0,
            total: 0,
        }
    }
}

#[async_trait]
impl Module for Memory {
    async fn update(&mut self) -> error::IResult<()> {
        for (i, line) in fs::read_to_string(&self.filename)
            .await?
            .lines()
            .take(3)
            .enumerate()
        {
            if i == 0 {
                self.total = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("unknown")
                    .parse::<u64>()
                    .unwrap_or(0)
            }

            if i == 2 {
                self.available = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("unknown")
                    .parse::<u64>()
                    .unwrap_or(0)
            }
        }

        Ok(())
    }

    async fn render(&self) -> error::IResult<String> {
        Ok(format!(
            " mem: {:.1}Gb/{:.1}Gb ",
            (self.total - self.available) as f64 / (1024.0 * 1024.0),
            self.total as f64 / (1024.0 * 1024.0)
        ))
    }
}
