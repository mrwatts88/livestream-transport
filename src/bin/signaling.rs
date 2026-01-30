use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    println!("{}", env::var("SIGNALING_ADDR").unwrap());
    Ok(())
}
