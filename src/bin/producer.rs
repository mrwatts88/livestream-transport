use std::{
    env::{self},
    error::Error,
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, stdin},
    net::UdpSocket,
    time::sleep,
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
    let mut seq: u32 = 0;
    loop {
        let num_bytes = std_in.read(&mut buf).await?;

        if num_bytes == 0 {
            break;
        }

        for c in &consumers {
            let mut msg: Vec<u8> = Vec::with_capacity(num_bytes + 4);
            msg.extend_from_slice(&seq.to_be_bytes());
            msg.extend_from_slice(&buf[0..num_bytes]);
            let num_bytes_sent = socket.send_to(msg.as_slice(), c).await?;
            if num_bytes + 4 != num_bytes_sent {
                eprintln!("we didn't send everything!!!");
            }
        }

        // sleep(Duration::from_millis(30)).await;
        seq += 1;
    }

    Ok(())
}
