use clap::{App, Arg};
use reqwest::Client;
use serde::Deserialize;
use tokio;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use log::{info, error};

#[derive(Deserialize)]
struct Track {
    name: String,
    preview_url: Option<String>,
}

#[derive(Deserialize)]
struct SpotifyResponse {
    tracks: Vec<Track>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("Song Recommendation App")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Fetches song recommendations based on user input")
        .arg(
            Arg::new("query")
                .help("The genre, artist, or mood to search for")
                .required(true)
                .index(1),
        )

        .get_matches();

    let query = matches.value_of("query").unwrap();
    if let Err(e) = fetch_recommendations(query).await {
        error!("Error fetching recommendations: {}", e);
    }
}

async fn fetch_recommendations(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/search?q={}&type=track", query);
    let response = client
        .get(&url)
        .header("Authorization", "Bearer YOUR_SPOTIFY_API_TOKEN")
        .send()
        .await?;

    if response.status().is_success() {
        let spotify_response = response.json::<SpotifyResponse>().await?;
        for track in spotify_response.tracks {
            info!("Track: {}", track.name);
            if let Some(preview_url) = track.preview_url {
                info!("Preview: {}", preview_url);
                tokio::spawn(play_preview(preview_url.to_string()));
            }
        }
    } else {
        error!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}

async fn play_preview(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;
    let cursor = Cursor::new(bytes);

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let source = Decoder::new(cursor)?;
    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}