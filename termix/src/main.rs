use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::error::Error;
use std::fs::File;
use std::{env, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("Usage: cargo run -- <file_path>")?;
    let _file = File::open(&path)?;

    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open deflaut audio stream");

    let file = BufReader::new(File::open(path).unwrap());

    let player = rodio::play(&handle.mixer(), file).unwrap();
    player.sleep_until_end();
    Ok(())
}
