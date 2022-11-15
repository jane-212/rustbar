use crate::bar::Module;
use crate::error;
use chrono::{
    DateTime, Local
};

pub struct Time {
    now: DateTime<Local>
}

impl Time {
    pub fn new() -> Self {
        Self {
            now: Local::now()
        }
    }
}

impl Module for Time {
    fn update(&mut self) -> error::IResult<()> {
        self.now = Local::now();

        Ok(())
    }

    fn render(&self) -> error::IResult<String> {
        Ok(format!("  {}  {} ", self.now.format("%Y-%m-%d"), self.now.format("%H:%M:%S")))
    }
}
