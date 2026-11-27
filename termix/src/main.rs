use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::error::Error;
use std::fs::File;
use std::{env, io::BufReader};
mod metedata;
use metedata::TrackInfo;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("Usage: cargo run -- <file_path>")?;
    let track_info = TrackInfo::from_path(Path::new(&path))?;
    let _file = File::open(&path)?;

    println!("Title {}", track_info.title);
    println!("Artist {}", track_info.artist);
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open deflaut audio stream");

    let file = BufReader::new(File::open(path).unwrap());

    let player = rodio::play(&handle.mixer(), file).unwrap();
    player.sleep_until_end();
    Ok(())
}
