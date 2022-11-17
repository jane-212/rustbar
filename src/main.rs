mod bar;
mod error;
mod memory;
mod net;
mod time;
mod version;
mod weather;

use bar::Bar;
use memory::Memory;
use net::Net;
use time::Time;
use version::Version;
use weather::Weather;

#[tokio::main]
async fn main() -> error::IResult<()> {
    Bar::default()
        .push(Box::new(Weather::default()))
        .push(Box::new(Version::default()))
        .push(Box::new(Net::default()))
        .push(Box::new(Memory::default()))
        .push(Box::new(Time::default()))
        .run()
        .await
}
