use mpvipc::{Mpv, MpvCommand, Error, PlaylistAddOptions};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Error> {
    // Path to the mpv socket file
    let socket_path = "/tmp/mpvsocket"; // Corrected socket path

    // Connect to mpv instance
    let mpv = Mpv::connect(socket_path)?;
    println!("Connected to mpv instance");


    mpv.set_property("fullscreen", true)?;

    // Load the video file and play it
    mpv.run_command(MpvCommand::LoadFile {
        file: "/home/pimedia/PINAS/bazmnt/MTV/Movies/Cartoons/Up (2009).mp4".into(),
        option: PlaylistAddOptions::Replace,
    })?;

    // Wait for 30 seconds
    thread::sleep(Duration::from_secs(65));

    // Pause the video
    mpv.set_property("pause", true)?;

    // Wait for 10 seconds
    thread::sleep(Duration::from_secs(10));

    // Resume playing the video
    mpv.set_property("pause", false)?;

    // Wait for 10 seconds
    thread::sleep(Duration::from_secs(15));

    // Stop Playback
    mpv.run_command(MpvCommand::Stop)?;

    Ok(())
}