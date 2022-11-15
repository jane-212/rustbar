mod bar;
mod error;
mod memory;
mod net;
mod time;
mod version;

use bar::Bar;
use memory::Memory;
use net::Net;
use time::Time;
use version::Version;

fn main() -> error::IResult<()> {
    Bar::new()
        .push(Box::new(Version::new()))
        .push(Box::new(Net::new()))
        .push(Box::new(Memory::new()))
        .push(Box::new(Time::new()))
        .run()
}
