mod homo;

use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get("https://mpyw.kb10uy.org").send().await?;

    let headers = response.headers();
    for (key, value) in headers {
        println!("{}: {:?}", key, value);
    }
    println!("remote: {:?}", response.remote_addr());
    println!("Hello, world!");
    Ok(())
}

