mod audio;
mod metedata;

use audio::{AudioCommand, spawn_audio_thread};
use metedata::TrackInfo;
use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::{env, io::BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("Usage: cargo run -- <file_path>")?;
    let track_info = TrackInfo::from_path(Path::new(&path))?;
    let _file = File::open(&path)?;

    println!("Title {}", track_info.title);
    println!("Artist {}", track_info.artist);
    println!("Album {}", track_info.album);

    let tx = spawn_audio_thread();

    tx.send(AudioCommand::Play((PathBuf::from(&path))));

    let mut volume: f32 = 1.0;
    let mut input = String::new();

    println!("\nCommands: [p] Toggle Pause | [+] Volume Up | [-] Volume Down | [q] Quit");

    // 3. Temporary CLI control loop
    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "p" => {
                tx.send(AudioCommand::TogglePause)?;
            }
            "+" => {
                volume = (volume + 0.1).clamp(0.0, 1.5);
                println!("Volume: {:.0}%", volume * 100.0);
                tx.send(AudioCommand::SetVolume(volume))?;
            }
            "-" => {
                volume = (volume - 0.1).clamp(0.0, 1.5);
                println!("Volume: {:.0}%", volume * 100.0);
                tx.send(AudioCommand::SetVolume(volume))?;
            }
            "q" => {
                tx.send(AudioCommand::Quit)?;
                break;
            }
            _ => {
                println!("Unknown command. Use p, +, -, or q.");
            }
        }
    }
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open deflaut audio stream");

    let file = BufReader::new(File::open(path).unwrap());

    let player = rodio::play(&handle.mixer(), file).unwrap();
    Ok(())
}
