Fetches lyrics of song from genius by it's URL

# Example

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