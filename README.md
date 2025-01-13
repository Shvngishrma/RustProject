# RustProject

## Description
This is a Song Recommendation App that fetches song recommendations based on user input using the Spotify API. It also allows you to play song previews.

## Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/RustProject.git
    cd RustProject
    ```

2. Update the dependencies:
    ```sh
    cargo update
    ```

3. Set your Spotify API token:
    Replace `YOUR_SPOTIFY_API_TOKEN` in `src/main.rs` with your actual Spotify API token.

## Usage
Run the application with a query parameter to get song recommendations:
```sh
cargo run -- "your query"
```

Example:
```sh
cargo run -- "chill"
```

## Features
- Fetches song recommendations based on genre, artist, or mood.
- Plays song previews using the `rodio` library.
- Error handling and logging for better traceability.

## Dependencies
- `clap` for command-line argument parsing.
- `reqwest` for making HTTP requests.
- `serde` for deserializing JSON responses.
- `tokio` for asynchronous runtime.
- `rodio` for playing audio.

## License
This project is licensed under the MIT License.
