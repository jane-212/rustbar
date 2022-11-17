use crate::bar::Module;
use crate::error;
use async_trait::async_trait;
use serde::Deserialize;
use tokio::time::{Duration, Instant};

pub struct Weather {
    url: &'static str,
    data: Data,
    last_tick: Instant,
    interval: Duration,
    is_fisrt: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Default, Debug)]
struct Data {
    nums: i32,
    cityid: String,
    city: String,
    date: String,
    week: String,
    update_time: String,
    wea: String,
    wea_img: String,
    tem: String,
    tem_day: String,
    tem_night: String,
    win: String,
    win_speed: String,
    win_meter: String,
    air: String,
    pressure: String,
    humidity: String,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            url: "https://www.yiketianqi.com/free/day?appid=52157466&appsecret=NS3Eh7NI&unescape=1",
            data: Data::default(),
            last_tick: Instant::now(),
            interval: Duration::from_secs(60 * 60),
            is_fisrt: true,
        }
    }
}

#[async_trait]
impl Module for Weather {
    async fn update(&mut self) -> error::IResult<()> {
        if self.is_fisrt {
            self.data = reqwest::get(self.url).await?.json::<Data>().await?;
            self.is_fisrt = false;
            self.last_tick = Instant::now();
        }

        if self.last_tick.elapsed() >= self.interval {
            self.data = reqwest::get(self.url).await?.json::<Data>().await?;
            self.last_tick = Instant::now();
        }

        Ok(())
    }

    async fn render(&self) -> error::IResult<String> {
        let data = &self.data;
        let wea = {
            let wea_img = &data.wea_img;
            if wea_img.starts_with("xue") {
                "snow"
            } else if wea_img.starts_with("lei") {
                "thunder"
            } else if wea_img.starts_with("shachen") {
                "dust"
            } else if wea_img.starts_with("wu") {
                "fog"
            } else if wea_img.starts_with("bingbao") {
                "hail"
            } else if wea_img.starts_with("yun") {
                "cloud"
            } else if wea_img.starts_with("yu") {
                "rain"
            } else if wea_img.starts_with("yin") {
                "cloud"
            } else if wea_img.starts_with("qing") {
                "sun"
            } else {
                ""
            }
        };

        Ok(format!(
            " {} {} {}/{} ",
            wea, data.tem, data.tem_day, data.tem_night
        ))
    }
}
