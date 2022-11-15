use crate::bar::Module;
use crate::error;
use std::{fs, path::PathBuf};

pub struct Memory {
    filename: PathBuf,
    available: u64,
    total: u64,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            filename: PathBuf::from("/proc/meminfo"),
            available: 0,
            total: 0,
        }
    }
}

impl Module for Memory {
    fn update(&mut self) -> error::IResult<()> {
        for (i, line) in fs::read_to_string(&self.filename)?.lines().take(3).enumerate() {
            if i == 0 {
                self.total = line.split_whitespace()
                    .nth(1)
                    .unwrap_or("unknown")
                    .parse::<u64>()
                    .unwrap_or(0)
            }

            if i == 2 {
                self.available = line.split_whitespace()
                    .nth(1)
                    .unwrap_or("unknown")
                    .parse::<u64>()
                    .unwrap_or(0)
            }
        }

        Ok(())
    }

    fn render(&self) -> error::IResult<String> {
        Ok(format!("  {:.1}Gb/{:.1}Gb ", (self.total - self.available) as f64 / (1024.0 * 1024.0), self.total as f64 / (1024.0 * 1024.0)))
    }
}
