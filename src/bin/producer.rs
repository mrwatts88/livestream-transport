use std::{
    env::{self},
    error::Error,
};
use tokio::{
    io::{AsyncReadExt, stdin},
    net::UdpSocket,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let producer_addr = env::var("PRODUCER_ADDR")?;
    let consumer_addr = env::var("CONSUMER_ADDR")?;
    // let signaling_addr = env::var("SIGNALING_ADDR")?;
    let socket = UdpSocket::bind(producer_addr).await?;

    // let consumers = vec![consumer_addr, signaling_addr];
    let consumers = vec![consumer_addr];
    let mut std_in = stdin();
    let mut buf = [0u8; 1024];
    loop {
        let num_bytes = std_in.read(&mut buf).await?;

        if num_bytes == 0 {
            break;
        }

        for c in &consumers {
            let num_bytes_sent = socket.send_to(&buf[0..num_bytes], c).await?;
            if num_bytes != num_bytes_sent {
                eprintln!("we didn't send everything!!!");
            }
        }
    }

    Ok(())
}
