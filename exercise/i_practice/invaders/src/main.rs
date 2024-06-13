use std::{error::Error, io};
use crossterm::{cursor::{Show, Hide}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use rusty_audio::Audio;
use std::env;

fn main() -> Result<(), Box<dyn Error>>{

    let mut audio = Audio::new();
    let default_path = format!("{}/audio",env!("CARGO_MANIFEST_DIR"));
    // println!("{}",default_path);

    for item in &["explode", "lose", 
    "move", "pew", "startup","win"]{
        audio.add(item, &format!("{}/{}.wav",default_path,item));
        // println!("{}/{}.wav",default_path, item);
    }


    audio.play("startup");
    

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())

}
