**Genius Lyrics Fetcher**
-------------------------

![crates.io](https://img.shields.io/crates/v/genius_lyrics.svg)
[![Rust](https://img.shields.io/badge/rust-1.73.0-orange.svg?logo=rust)](https://www.rust-lang.org)

## **Developer**
[Paranid5](https://github.com/dinaraparanid)

Fetches lyrics of song from genius by it's URL

# **Example**

```Rust
extern crate tokio;
use genius_lyrics::get_lyrics_from_url;

#[tokio::main]
async fn main() {
    println!(
        "{}",
        get_lyrics_from_url("https://genius.com/Michael-jackson-bad-lyrics")
            .await
            .unwrap()
    )
}
```
