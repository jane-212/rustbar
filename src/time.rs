use crate::bar::Module;
use crate::error;
use async_trait::async_trait;
use chrono::{DateTime, Local};

pub struct Time {
    now: DateTime<Local>,
}

impl Default for Time {
    fn default() -> Self {
        Self { now: Local::now() }
    }
}

#[async_trait]
impl Module for Time {
    async fn update(&mut self) -> error::IResult<()> {
        self.now = Local::now();

        Ok(())
    }

    async fn render(&self) -> error::IResult<String> {
        Ok(format!(
            "  {}  {} ",
            self.now.format("%Y-%m-%d"),
            self.now.format("%H:%M:%S")
        ))
    }
}
