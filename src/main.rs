use std::fs::{File, read_dir};
use std::io::BufReader;
use std::env;
use rodio::{Decoder, OutputStream, source::Source};
use rand::Rng;

fn main() {
    // Read pathname from command or default to samples
    let args: Vec<String> = env::args().collect();
    let mut pathname = "./samples/";
    if  args.len() > 1 {
        pathname = &args[1];
    }
    // Choose random sound bite from above directory
    let paths = read_dir(pathname).unwrap();
    let choices = paths.count();

    let mut rng = rand::thread_rng();
    let num =  rng.gen_range(0..(choices+1));

    let sound_bite = format!("{}meow_{}.mp3", pathname, num);

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(sound_bite).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).expect("could not play sound");
    
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
