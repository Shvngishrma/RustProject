use clap::{App, Arg};
use reqwest::Client;
use serde::Deserialize;
use tokio;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

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
    let matches = App::new("Song Recommendation App")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Fetches song recommendations based on user input")
        .arg(
            Arg::new("query")
                .about("The genre, artist, or mood to search for")
                .required(true)
                .index(1),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    fetch_recommendations(query).await;
}

async fn fetch_recommendations(query: &str) {
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/search?q={}&type=track", query);
        .header("Authorization", "Bearer YOUR_SPOTIFY_API_TOKEN")
    let response = client
        .get(&url)
        .send()
        .await
        .unwrap()
        .json::<SpotifyResponse>()
        .await
        .unwrap();

    for track in response.tracks {
        println!("Track: {}", track.name);
        if let Some(preview_url) = track.preview_url {
            println!("Preview: {}", preview_url);
            play_preview(&preview_url).await;
        }
    }
}

async fn play_preview(url: &str) {
    let response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    let cursor = Cursor::new(bytes);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = Decoder::new(cursor).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
        .send()
        .await
        .unwrap()
        .json::<SpotifyResponse>()
        .await
        .unwrap();

    for track in response.tracks {
        println!("Track: {}", track.name);
        if let Some(preview_url) = track.preview_url {
            println!("Preview: {}", preview_url);
        }
    }
}
