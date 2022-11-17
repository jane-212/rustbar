use crate::bar::Module;
use crate::error;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

pub struct Version {
    filename: PathBuf,
    version: String,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            filename: PathBuf::from("/proc/version"),
            version: String::new(),
        }
    }
}

#[async_trait]
impl Module for Version {
    async fn update(&mut self) -> error::IResult<()> {
        self.version = fs::read_to_string(&self.filename)
            .await?
            .split_whitespace()
            .nth(2)
            .unwrap_or("unknown")
            .to_owned();

        Ok(())
    }

    async fn render(&self) -> error::IResult<String> {
        Ok(format!(" arch: {} ", self.version))
    }
}
