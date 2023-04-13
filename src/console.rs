use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::{play_bell, Args};

pub struct Progressbar {
    max: usize,
    current: usize,
}

impl Progressbar {
    pub fn new(max: usize) -> Self {
        Self { max, current: 0 }
    }

    pub fn increment(&mut self) {
        self.current += 1;
    }
}

impl Default for Progressbar {
    fn default() -> Self {
        Self::new(100)
    }
}

impl Display for Progressbar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[0G")?;

        write!(
            f,
            "{}",
            "*".repeat((self.current as f32 / self.max as f32 * 10.).floor() as usize)
        )?;

        Ok(())
    }
}

fn perform_clock(minutes: u32, sound_file: &str) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let mut progress = Progressbar::new(minutes as usize * 60);
    while Instant::now() - now < Duration::from_secs(minutes as u64 * 60) {
        thread::sleep(Duration::from_secs(1));
        progress.increment();
        print!("{}", progress);
        stdout().flush()?;
    }

    play_bell(sound_file)?;

    Ok(())
}

pub fn console(args: Args) -> Result<(), Box<dyn Error>> {
    println!(
        "Begining timer for {} cycles of {} minutes of work and {} minutes of break",
        args.cycles, args.productive_time, args.break_time
    );

    for cycle in 1..=args.cycles {
        println!("Cycle {} of {}", cycle, args.cycles);
        println!("Work time!");
        perform_clock(args.productive_time, &args.sound_file)?;

        println!("Break time!");
        perform_clock(args.break_time, &args.sound_file)?;
    }

    Ok(())
}
