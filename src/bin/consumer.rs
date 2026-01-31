use std::{
    env::{self, args},
    error::Error,
    time::Duration,
};
use tokio::{
    io::{AsyncWriteExt, stdout},
    net::UdpSocket,
    time::sleep,
};

struct Frame(u32, Vec<u8>);

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

    let mut jitter: Vec<Frame> = vec![];
    loop {
        // sleep(Duration::from_millis(30)).await;
        let mut buf = [0u8; 1028];
        let num_bytes = socket.recv(&mut buf).await?;
        let seq: [u8; 4] = buf[0..4].try_into()?;
        let seq = u32::from_be_bytes(seq);
        let payload = &buf[4..num_bytes];
        // std_out.write_all(payload).await?;
        // std_out.flush().await?;

        jitter.push(Frame(seq, payload.to_vec()));

        if jitter.len() > 100 {
            jitter.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            for b in &jitter {
                std_out.write_all(b.1.as_slice()).await?;
            }
            std_out.flush().await?;
            jitter.clear();
        }
    }
}
