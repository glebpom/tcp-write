#[macro_use]
extern crate log;

use std::net::IpAddr;
use std::str::FromStr;

use tokio::fs::File;
use tokio::io;
use tokio::net::TcpListener;
use std::path::Path;

pub const DIR: &str = "./storage";
pub const PORT: u16 = 12345;

#[tokio::main]
pub async fn main() -> io::Result<()> {
    env_logger::init();

    let mut listener = TcpListener::bind((IpAddr::from_str("0.0.0.0").unwrap(), PORT)).await?;
    info!("Listening on port {}", PORT);

    let mut counter = 0;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        counter += 1;

        info!("Accepted {}th TCP connection {:?}", counter, addr);
        let mut file = File::create(Path::join(DIR.as_ref(), format!("file-{}", counter))).await?;

        tokio::spawn(async move {
            io::copy(&mut socket, &mut file).await.unwrap();
            info!("Connection {} closed", counter);
        });
    }
}
