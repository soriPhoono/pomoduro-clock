mod console;
mod ui;

use std::{error::Error, fs, path::Path};

use rodio::source::Source;
use rodio::{Decoder, OutputStream, Sink};

use clap::Parser;

#[derive(Debug, Parser, serde::Deserialize)]
#[command(author = "SoriPhoono", version = "1.0", about = "A pomodoro timer", long_about=None)]
pub struct Args {
    #[cfg(debug_assertions)]
    #[arg(short, long, default_value = "1")]
    productive_time: u32,
    #[cfg(not(debug_assertions))]
    #[arg(short, long, default_value = "15")]
    productive_time: u32,

    #[cfg(debug_assertions)]
    #[arg(short, long, default_value = "1")]
    break_time: u32,
    #[cfg(not(debug_assertions))]
    #[arg(short, long, default_value = "15")]
    break_time: u32,

    #[cfg(debug_assertions)]
    #[arg(short, long, default_value = "1")]
    cycles: u32,
    #[cfg(not(debug_assertions))]
    #[arg(short, long, default_value = "4")]
    cycles: u32,

    #[arg(short, long, default_value = "./config/bell.wav")]
    sound_file: String,

    #[arg(short, long, default_value = "false")]
    use_gui: bool,
}

pub fn play_bell(sound_file: &str) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let file = fs::File::open(sound_file)?;
    let source = Decoder::new(file)?.convert_samples::<f32>();

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

slint::include_modules!();
fn main() -> Result<(), Box<dyn Error>> {
    let args = if Path::new("./config.json").exists() {
        serde_json::from_str(&fs::read_to_string("./config.json")?)?
    } else {
        Args::parse()
    };

    if args.use_gui {
        HelloWorld::new().unwrap().run().unwrap();

        Ok(())
    } else {
        console::console(args)
    }
}
