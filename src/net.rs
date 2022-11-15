use crate::bar::Module;
use crate::error;
use std::{fs, path::PathBuf};

pub struct Net {
    filename: PathBuf,
    bytes: u64,
    last_bytes: u64
}

impl Net {
    pub fn new() -> Self {
        Self {
            filename: PathBuf::from("/proc/net/dev"),
            bytes: 0,
            last_bytes: 0
        }
    }
}

impl Module for Net {
    fn update(&mut self) -> error::IResult<()> {
        self.last_bytes = self.bytes;
        self.bytes ^= self.bytes;

        fs::read_to_string(&self.filename)?
            .lines()
            .skip(2)
            .for_each(|line| {
               self.bytes += line.split(':')
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

    fn render(&self) -> error::IResult<String> {
        let diff = (self.bytes - self.last_bytes) as f64 / 1024.0;

        if diff < 1024.0 {
            Ok(format!("  {:.1}Kb/s ", diff))
        } else {
            Ok(format!("  {:.1}Mb/s ", diff / 1024.0))
        }
    }
}
