mod cli;
mod downloader;
mod errors;
mod extractor;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    let url = cli::get_url()?;
    let html = utils::fetch_html(&url)?;

    match extractor::extract_audio_url(&html) {
        Ok(audio_url) => {
            println!("Audio URL found: {}", audio_url);
            downloader::download_audio(&audio_url)?;
        }
        Err(e) => {
            println!("Failed to extract audio URL: {:?}", e);
            std::fs::write("debug_output.html", &html)?;
        }
    }
    Ok(())
}
