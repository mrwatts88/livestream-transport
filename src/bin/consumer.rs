use std::{
    env::{self, args},
    error::Error,
};
use tokio::{
    io::{AsyncWriteExt, stdout},
    net::UdpSocket,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let args: Vec<String> = args().collect();

    let producer_addr = env::var("PRODUCER_ADDR")?;
    let consumer_addr = env::var("CONSUMER_ADDR")?;
    let signaling_addr = env::var("SIGNALING_ADDR")?;

    let is_c = args[1] == "c";

    let addr = if is_c {
        &consumer_addr
    } else {
        &signaling_addr
    };

    let socket = UdpSocket::bind(addr).await?;
    socket.connect(&producer_addr).await?;
    let mut std_out = stdout();

    loop {
        let mut buf = [0u8; 1024];
        let num_bytes = socket.recv(&mut buf).await?;

        std_out.write_all(&buf[0..num_bytes]).await?;
    }
}
