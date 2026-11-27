use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::channel;
use std::{thread};
use std::{path::PathBuf, sync::mpsc::Sender};
pub enum AudioCommand {
    Play(PathBuf),
    TogglePause,
    SetVolume(f32),
    Stop,
    Quit,
}

pub fn spawn_audio_thread() -> Sender<AudioCommand> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let stream_handel = DeviceSinkBuilder::open_default_sink()
            .expect("Failed to open default audio output sink");
        let player = Player::connect_new((stream_handel.mixer()));

        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCommand::Play(Path) => {
                    player.clear();

                    if let Ok(file) = File::open(&Path) {
                        if let Ok(source) = Decoder::try_from(BufReader::new(file)) {
                            player.append(source);
                            player.play();
                        } else {
                            eprintln!("Failed to decode audio file: {:?}", Path);
                        }
                    } else {
                        eprintln!("Failed to open audio file: {:?}", Path);
                    }
                }

                AudioCommand::TogglePause => {
                    if player.is_paused() {
                        player.play();
                    } else {
                        player.pause();
                    }
                }

                AudioCommand::SetVolume(vol) => {
                    let clamped = vol.clamp(0.0, 1.5);
                    player.set_volume(clamped);
                }
                AudioCommand::Stop => {
                    player.stop();
                }
                AudioCommand::Quit => {
                    player.stop();
                    break;
                }
            }
        }
    });
    tx
}
