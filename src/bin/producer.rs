use std::{
    env::{self},
    error::Error,
    time::Duration,
};
use tokio::{net::UdpSocket, time::sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let producer_addr = env::var("PRODUCER_ADDR")?;
    let consumer_addr = env::var("CONSUMER_ADDR")?;
    let signaling_addr = env::var("SIGNALING_ADDR")?;
    let socket = UdpSocket::bind(producer_addr).await?;

    let mut counter = 0;
    let consumers = vec![consumer_addr, signaling_addr];
    loop {
        for c in &consumers {
            socket
                .send_to(format!("hello: {}", counter).as_bytes(), c)
                .await?;
        }

        counter += 1;
        sleep(Duration::from_secs(2)).await;
    }
}
