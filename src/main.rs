use std::{
    error::Error,
    io::{stdout, Write},
    thread,
    time::Duration,
};

use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = "SoriPhoono", version = "1.0", about = "A pomodoro timer", long_about=None)]
struct Args {
    #[arg(short, long, default_value = "15")]
    productive_time: f32,
    #[arg(short, long, default_value = "15")]
    break_time: f32,
    #[arg(short, long, default_value = "4")]
    cycles: f32,
    #[arg(short, long, default_value = "./config/bell.wav")]
    sound_file: String,
}

fn play_bell(sound_file: &str) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let file = std::fs::File::open(sound_file)?;
    let source = Decoder::new(file)?.convert_samples::<f32>();

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!(
        "Begining timer for {} cycles of {} minutes of work and {} minutes of break",
        args.cycles, args.productive_time, args.break_time
    );

    for cycle in 1..=args.cycles {
        println!("Cycle {} of {}", cycle, args.cycles);
        thread::sleep(Duration::from_secs(args.productive_time as u64 * 60));
        play_bell(&args.sound_file)?;
        println!("Break time!");

        thread::sleep(Duration::from_secs(args.break_time as u64 * 60));
        play_bell(&args.sound_file)?;
        println!("Back to work!");
    }

    Ok(())
}
