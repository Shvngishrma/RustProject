use clap::{App, Arg};
use futures::stream::{self, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use tokio;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use log::{info, error};
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tokio_retry::RetryIf;

#[derive(Deserialize)]
struct Track {
    name: String,
    preview_url: Option<String>,
}

#[derive(Deserialize)]
struct TracksObject {
    items: Vec<Track>,
}

#[derive(Deserialize)]
struct SpotifyResponse {
    tracks: TracksObject,
}

struct AppState {
    stream_handle: rodio::OutputStreamHandle,
}

impl AppState {
    fn new(stream_handle: rodio::OutputStreamHandle) -> Self {
        AppState { stream_handle }
    }
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
        .arg(
            Arg::new("concurrency")
                .long("concurrency")
                .takes_value(true)
                .help("Maximum number of tracks to play concurrently"),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let concurrency_limit = matches.value_of("concurrency").unwrap_or("5").parse().unwrap_or(5);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let state = AppState { stream_handle };

    if let Err(e) = fetch_recommendations(query, concurrency_limit, &state).await {
        error!("Error fetching recommendations: {}", e);
    }
}

async fn fetch_recommendations(query: &str, concurrency_limit: usize, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/search?q={}&type=track", query);

    let response = client
        .get(&url)
        .header("Authorization", "Bearer YOUR_SPOTIFY_API_TOKEN")
        .send()
        .await?;

    if response.status().is_success() {
        let spotify_response = response.json::<SpotifyResponse>().await?;
        let tracks = spotify_response.tracks.items;

        stream::iter(tracks)
            .for_each_concurrent(Some(concurrency_limit), |track| {
                let state = state;
                async move {
                    if let Some(preview_url) = track.preview_url {
                        if let Err(e) = play_preview(preview_url.to_string(), state).await {
                            error!("Failed to play preview: {}", e);
                        }
                    }
                }
            })
            .await;
    } else {
        error!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}

async fn play_preview(url: String, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let retry_strategy = ExponentialBackoff::from_millis(10)
        .map(jitter)
        .take(3);

    let response = RetryIf::spawn(retry_strategy, || async {
        info!("Attempting to fetch preview from URL: {}", url);
        let result = reqwest::get(&url).await;
        if let Err(ref e) = result {
            error!("Retryable error occurred: {}", e);
        }
        result
    }, |e: &reqwest::Error| e.is_timeout()).await?;

    let bytes = response.bytes().await?;
    let cursor = Cursor::new(bytes);

    let sink = Sink::try_new(&state.stream_handle)?;
    let source = Decoder::new(cursor)?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}