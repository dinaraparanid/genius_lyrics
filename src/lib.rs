extern crate html2text;
extern crate regex;
extern crate reqwest;
extern crate scraper;

use regex::Regex;
use scraper::{Html, Selector};

/// Errors when parsing lyrics

#[derive(Debug, Clone, Copy)]
pub enum GetLyricsError {
    UrlError,
    NoTextError,
}

#[inline]
fn get_lyrics_from_doc(doc: &str) -> std::result::Result<String, GetLyricsError> {
    Ok(Regex::new(r"\[\d*]|/.*|[\[\]]|https:.*")
        .unwrap()
        .replace_all(
            html2text::from_read(
                match Html::parse_document(doc)
                    .select(&Selector::parse("div[id=application]").unwrap())
                    .next()
                {
                    Some(x) => x,
                    None => return Err(GetLyricsError::NoTextError),
                }
                    .select(&Selector::parse("div[data-lyrics-container=true]").unwrap())
                    .fold(String::new(), |acc, e| format!("{}{}", acc, e.html()))
                    .as_bytes(),
                1000,
            )
                .as_str(),
            "",
        )
        .as_ref()
        .trim()
        .to_string())
}

/// Gets lyrics from Genius by url asynchronously
///
/// # Example
///
/// ```Rust
/// extern crate tokio;
/// use genius_lyrics::get_lyrics_from_url;
///
/// #[tokio::main]
/// async fn main() {
///     println!(
///         "{}",
///         get_lyrics_from_url("https://genius.com/Michael-jackson-bad-lyrics")
///             .await
///             .unwrap()
///     )
/// }
/// ```
///

#[inline]
pub async fn get_lyrics_from_url(url: &str) -> std::result::Result<String, GetLyricsError> {
    get_lyrics_from_doc(
        match reqwest::get(url).await {
            Ok(x) => x,
            Err(_) => return Err(GetLyricsError::UrlError),
        }
            .text()
            .await
            .unwrap()
            .as_str(),
    )
}

/// **Blocks thread** and gets lyrics from Genius by url
///
/// # Example
///
/// ```Rust
/// use genius_lyrics::get_lyrics_from_url_blocking;
///
/// fn main() {
///     println!("{}", get_lyrics_from_url_blocking("https://genius.com/Michael-jackson-bad-lyrics").unwrap())
/// }
/// ```
///

#[inline]
pub fn get_lyrics_from_url_blocking(url: &str) -> std::result::Result<String, GetLyricsError> {
    get_lyrics_from_doc(
        match reqwest::blocking::get(url) {
            Ok(x) => x,
            Err(_) => return Err(GetLyricsError::UrlError),
        }
            .text()
            .unwrap()
            .as_str(),
    )
}