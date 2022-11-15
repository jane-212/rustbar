use std::{
    time::Duration, thread,
    process::Command
};

use crate::error;

pub struct Bar
{
    interval: u64,
    modules: Vec<Box<dyn Module>>,
    bar: String
}

impl Bar
{
    pub fn new() -> Self {
        Self{
            interval: 1,
            modules: Vec::new(),
            bar: String::new()
        }
    }

    pub fn push(mut self, module: Box<dyn Module>) -> Self {
        self.modules.push(module);
        self
    }

    pub fn run(mut self) -> error::IResult<()> {
        loop {
            self.update()?;
            self.render()?;

            thread::sleep(Duration::from_secs(self.interval));
        }
    }

    pub fn update(&mut self) -> error::IResult<()> {
        for module in &mut self.modules {
            module.update()?;
        }

        Ok(())
    }

    pub fn render(&mut self) -> error::IResult<()> {
        self.bar.clear();

        for module in &self.modules {
            self.bar.push('|');
            self.bar.push_str(&module.render()?);
        }

        self.set_root()?;

        Ok(())
    }


    fn set_root(&self) -> error::IResult<()> {
        if Command::new("xsetroot")
            .arg("-name")
            .arg(&self.bar)
            .status()?
            .success() {
            Ok(())
        } else {
            Err("execute xsetroot failed".into())
        }
    }
}

pub trait Module {
    fn update(&mut self) -> error::IResult<()>;
    fn render(&self) -> error::IResult<String>;
}

