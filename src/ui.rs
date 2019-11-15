use std::path::PathBuf;
use std::io::Stdout;
use std::io;

use termion::raw::RawTerminal;

use tui::Frame;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Style, Modifier, Color};

use crate::file_ops;
use crate::app::App;

pub fn draw(app: &mut App) -> Result<(), io::Error> {
    let command_string = app.get_command_buffer_as_string();
    let mut reset_error = false;

    let App {
        current_directory,
        terminal,
        directory_contents,
        selection_index,
        file_error,
        ..
    } = app;

    terminal.hide_cursor()?;

    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3)
            ].as_ref())
            .split(f.size());
        
        draw_file_list(&mut f, chunks[0], directory_contents, selection_index, current_directory);

        if let Some(err) = file_error {
            draw_error(&mut f, chunks[1], err);
            reset_error = true;
        } else {
            draw_command_buffer(&mut f, chunks[1], command_string);
        }
    })?;

    if reset_error {
        app.file_error = None;
    }

    Ok(())
}

pub fn draw_file_list(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect, files: &Vec<file_ops::DirectoryItem>, selected_file: &Option<usize>, current_dir: &PathBuf) {
    let mut text: Vec<Text> = Vec::new();
    let inner_rect = Rect::new(area.x + 1, area.y + 1, area.width - 1, area.height - 1); //Shrinking the area by 1 in every direction for the text columns, as border is drawn separately
    
    //Draw the border
    Block::default()
        .borders(Borders::ALL)
        .title(format!("Contents─{}", current_dir.to_str().unwrap()).as_ref())
        .render(frame, area);

    if files.len() != 0 {
        //Convert DirectoryItems to Text
        for file in files {
            match file {
                file_ops::DirectoryItem::File(path) => {
                    let split: Vec<&str> = path.split('/').collect();
                    let string = String::from(format!("📄 {}\n", split[split.len() - 1 as usize]));
                    text.push(Text::raw(string));
                },
                file_ops::DirectoryItem::Directory(path) => {
                    let split: Vec<&str> = path.split('/').collect();
                    let string = String::from(format!("📁 {}\n", split[split.len() - 1 as usize]));
                    text.push(Text::raw(string));
                }
            }
        }

        //Highlight selected file
        if let Some(selection_index) = selected_file {
            //Get name of selected file
            let selected = match &mut text[*selection_index] {
                Text::Raw(value) => value,
                _ => { "" }
            }.to_string();
    
            //Replace name of selected file with bold name
            text.insert(*selection_index, Text::styled(selected, Style::default().modifier(Modifier::BOLD)));
            text.remove(selection_index + 1);
        }

        //Figure out number of clumns and their spacing
        let columns: u16 = (text.len() as f32 / area.height as f32).ceil() as u16;
        let column_size: u16 = 100 / columns;
        let mut constraints: Vec<Constraint> = Vec::new();

        //Create the constraints
        for _ in 1..=columns as u32 {
            constraints.push(Constraint::Percentage(column_size));
        }

        //Create the chunks
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner_rect);
        
        for i in 0..=columns - 1 {
            let height: usize = (area.height - 2) as usize; // -2 to account for the border
            let from: usize = (i as usize * height) as usize;
            let mut to: usize = (i as usize * height) + (height);

            if to >= text.len() {
                to = text.len();
            }

            let iter = text[from..to].iter();

            Paragraph::new(iter)
                .wrap(false)
                .render(frame, chunks[i as usize]);
        }
    }
}

pub fn draw_command_buffer(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect, command_string: String) {
    let text: Vec<Text> = vec!(Text::raw(command_string));

    Paragraph::new(text.iter())
        .block(
            Block::default()
                .title("Command")
                .borders(Borders::ALL)
        )
        .render(frame, area);
}

pub fn draw_error(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect, error: &std::io::Error) {
    let text: Vec<Text> = vec!(Text::styled(format!("ERROR: {:?}", error.kind()), Style::default().fg(Color::Red)));

    Paragraph::new(text.iter())
        .block(
            Block::default()
                .title("Error")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red))
        )
        .render(frame, area);
}