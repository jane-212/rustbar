use crate::bar::Module;
use crate::error;
use std::{fs, path::PathBuf};

pub struct Version {
    filename: PathBuf,
    version: String,
}

impl Version {
    pub fn new() -> Self {
        Self {
            filename: PathBuf::from("/proc/version"),
            version: String::new(),
        }
    }
}

impl Module for Version {
    fn update(&mut self) -> error::IResult<()> {
        self.version = fs::read_to_string(&self.filename)?
            .split_whitespace()
            .nth(2)
            .unwrap_or("unknown")
            .to_owned();

        Ok(())
    }

    fn render(&self) -> error::IResult<String> {
        Ok(format!("  {} ", self.version))
    }
}
