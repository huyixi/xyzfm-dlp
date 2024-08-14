use anyhow::Result;
use indicatif::ProgressBar;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Write;

pub fn download_audio(url: &str) -> Result<()> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);

    let mut file = File::create("audio.m4a")?;
    let mut downloaded: u64 = 0;

    for chunk in response.bytes()?.chunks(8192) {
        file.write_all(chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}
