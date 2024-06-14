use std::{error::Error, io, sync::mpsc, thread};
use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use invaders::{frame::{self, new_frame}, render};
use rusty_audio::Audio;
use std::env;
use std::time::Duration;


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

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move||
    {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop{
            let curr_frame = match render_rx.recv()
            {
                Ok(x) => x,
                Err(_)=> break,
            };
            render::render(& mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });


    // Game loop
    'gameloop: loop{
        // Per frame init
        let curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())?
        {
            if let Event::Key(key_event) = event::read()?
            {
                match key_event.code{
                    KeyCode::Esc | KeyCode::Char('q') =>{
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {},
                }
            }
        }

        // Draw and render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())

}
