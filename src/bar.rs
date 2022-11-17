use crate::error;
use async_trait::async_trait;
use tokio::{process::Command, time::{self, Duration}};

pub struct Bar {
    interval: Duration,
    modules: Vec<Box<dyn Module>>,
    bar: String,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            modules: Vec::default(),
            bar: String::default()
        }
    }
}

impl Bar {
    pub fn push(mut self, module: Box<dyn Module>) -> Self {
        self.modules.push(module);
        self
    }

    pub async fn run(mut self) -> error::IResult<()> {
        loop {
            self.update().await?;
            self.render().await?;

            time::sleep(self.interval).await;
        }
    }

    pub async fn update(&mut self) -> error::IResult<()> {
        for module in &mut self.modules {
            module.update().await?;
        }

        Ok(())
    }

    pub async fn render(&mut self) -> error::IResult<()> {
        self.bar.clear();

        for module in &self.modules {
            self.bar.push('|');
            self.bar.push_str(&module.render().await?);
        }

        self.set_root().await?;

        Ok(())
    }

    async fn set_root(&self) -> error::IResult<()> {
        if Command::new("xsetroot")
            .arg("-name")
            .arg(&self.bar)
            .status()
            .await?
            .success()
        {
            Ok(())
        } else {
            Err("execute xsetroot failed".into())
        }
    }
}

#[async_trait]
pub trait Module {
    async fn update(&mut self) -> error::IResult<()>;
    async fn render(&self) -> error::IResult<String>;
}
