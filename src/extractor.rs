use anyhow::{Context, Result};
use scraper::{Html, Selector};

pub fn extract_audio_url(html: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("meta[property='og:audio']").unwrap();

    let audio_url = document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(String::from)
        .context(format!(
            "Failed to find og:audio meta tag. HTML length: {}",
            html.len()
        ))?;

    Ok(audio_url)
}
