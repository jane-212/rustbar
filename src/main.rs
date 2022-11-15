mod bar;
mod error;
mod version;
mod net;
mod memory;
mod time;

fn main() -> error::IResult<()> {
    let mut bar = bar::Bar::new();

    let version = version::Version::new();
    bar.push(Box::new(version));

    let net = net::Net::new();
    bar.push(Box::new(net));

    let memory = memory::Memory::new();
    bar.push(Box::new(memory));

    let time = time::Time::new();
    bar.push(Box::new(time));

    bar.run()?;

    Ok(())
}
