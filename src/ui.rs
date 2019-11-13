use std::path::PathBuf;
use std::io::Stdout;
use std::io;

use termion::raw::RawTerminal;

use tui::Frame;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Style, Modifier};

use crate::file_ops;
use crate::app::App;

pub fn draw(app: &mut App) -> Result<(), io::Error> {
    let selected = app.selection_index;
    let current_dir = app.current_directory.clone();
    let dir_items = app.directory_contents.clone();
    let command_string = app.get_command_buffer_as_string();

    app.terminal.hide_cursor()?;

    app.terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3)
            ].as_ref())
            .split(f.size());
        
        draw_file_list(&mut f, chunks[0], dir_items, selected, current_dir);
        draw_command_buffer(&mut f, chunks[1], command_string);
    })
}

pub fn draw_file_list(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, area: Rect, files: Vec<file_ops::DirectoryItem>, selected_file: Option<usize>, current_dir: PathBuf) {
    let mut text: Vec<Text> = Vec::new();
    
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

        if let Some(selection_index) = selected_file {
            //Get name of selected file
            let selected = match &mut text[selection_index] {
                Text::Raw(value) => value,
                _ => { "" }
            }.to_string();
    
            //Replace name of selected file with bold name
            text.insert(selection_index, Text::styled(selected, Style::default().modifier(Modifier::BOLD)));
            text.remove(selection_index + 1);
        }
    }

    //Draw the text
    Paragraph::new(text.iter())
        .block(
            Block::default()
                .title(format!("Contents─{}", current_dir.to_str().unwrap()).as_ref())
                .borders(Borders::ALL)   
        )
        .wrap(true)
        .render(frame, area);
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