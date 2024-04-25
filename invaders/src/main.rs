// standard library imports
use std::{io, error::Error, time::Duration, thread};
use std::sync::mpsc;

// crossterm library imports
use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
};

// other imports
use rusty_audio::Audio;
use invaders::{frame, render};
use crate::event::Event;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let cur_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &cur_frame, false);
            last_frame = cur_frame;
        }
    });

    'gameloop: loop {
        // Pre-frame init
        let cur_frame = frame::new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw and render
        // Ignore result as the first frames send will fail until the render thread will be started
        let _ = render_tx.send(cur_frame).unwrap();

        // Wait for the slower render thread
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    // The render_rx will fail as the render_tx was dropped, so the render_handel will join
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
