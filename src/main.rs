use std::{io, thread, time};

use termion::raw::IntoRawMode;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::TermionBackend;

mod app;
mod file_ops;
mod ui;
mod commands;

use app::App;

fn main() -> Result<(), io::Error> {
    //Initialize terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    //Initialize input
    let mut stdin = termion::async_stdin().keys();

    //Initialize App state
    let mut app = App::new(&mut terminal);

    //Main application loop
    loop {
        app.update_window_height();
        //Handle input
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            if app.mode == app::Mode::Browse {
                match key {
                    termion::event::Key::Char('q') => break,
                    termion::event::Key::Char('j') => app.move_selection_down(),
                    termion::event::Key::Char('k') => app.move_selection_up(),
                    termion::event::Key::Char('h') => app.move_selection_left(),
                    termion::event::Key::Char('l') => app.move_selection_right(),
                    termion::event::Key::Char('\n') => app.open_folder(),
                    termion::event::Key::Char(':') => app.change_mode(app::Mode::Command),
                    termion::event::Key::Backspace => app.move_up_directory()?,
                    termion::event::Key::Char('c') => app.load_selected_into_file_buffer(),
                    termion::event::Key::Char('x') => {
                        app.load_selected_into_file_buffer();
                        file_ops::delete_file(&app);
                    },
                    termion::event::Key::Char('v') => app.write_buffered_file(),
                    _ => {}
                }
            }

            if app.mode == app::Mode::Command {
                if let termion::event::Key::Char(chr) = key {
                    if chr != '\n' {
                        app.add_to_command_buffer(chr);
                    } else {
                        app.execute_command();
                    }
                }
                if key == termion::event::Key::Esc {
                    app.change_mode(app::Mode::Browse);
                    app.command_buffer = Vec::new();
                }
                if key == termion::event::Key::Backspace {
                    if app.command_buffer.len() > 1 {
                        app.command_buffer.truncate(app.command_buffer.len() - 1);
                    }
                }
            }
        }

        app.populate_files()?;
        ui::draw(&mut app)?;
        thread::sleep(time::Duration::from_millis(50));
    }
    Ok(())
}