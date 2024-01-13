use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use futures_util::StreamExt;
use reqwest::{Client, Response};
use tokio::time::{interval, Interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }
    let url: &str = &args[1]; // "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    // create client, send get-request
    let client: Client = Client::new();
    let response: Response = client
        .get(url)
        .send()
        .await?;

    // get filename, create file (url should contain extension like .png)
    let filename: &str = url.split('/').last().unwrap_or("downloaded_file");
    let mut file: File = File::create(filename)?;

    // create bytes stream (without stream it wouldn't work)
    let mut stream = response.bytes_stream();

    // timer
    let mut interval: Interval = interval(Duration::from_secs(1));
    let mut total_bytes: u64 = 0;

    // write data to file
    while let Some(result) = stream.next().await {
        let chunk = result?;
        file.write_all(&chunk)?;
        total_bytes += chunk.len() as u64;

        // bytes output
        interval.tick().await;
        println!("Downloaded {} bytes", total_bytes);
    }

    Ok(())
}
