use std::io::Stdout;
use std::path::PathBuf;
use std::path;

use tui::Terminal;
use tui::backend::TermionBackend;

use termion::raw::RawTerminal;

use crate::file_ops;
use crate::file_ops::DirectoryItem;

pub struct App<'a> {
    pub current_directory: path::PathBuf,
    pub terminal: &'a mut Terminal<TermionBackend<RawTerminal<Stdout>>>,
    pub mode: Mode,
    pub selection_index: Option<usize>,
    pub directory_contents: Vec<DirectoryItem>,
    pub command_buffer: Vec<char>,
    pub file_error: Option<std::io::Error>,

    max_file_selection: usize
}

impl<'a> App<'a> {
    pub fn new(terminal: &'a mut Terminal<TermionBackend<RawTerminal<Stdout>>>) -> App<'a> {
        let current_dir = path::PathBuf::from("/");

        let mut app = App {
            current_directory: current_dir,
            terminal,
            mode: Mode::Browse,
            selection_index: Some(0),
            max_file_selection: 0,
            directory_contents: Vec::new(),
            command_buffer: Vec::new(),
            file_error: None
        };

        if let Err(error) = app.populate_files() {
            panic!(format!("Error opening {:?}: {:?}", app.current_directory, error.kind()));
        }

        app
    }

    pub fn increment_selection(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if selection_index < self.max_file_selection - 1 { 
                self.selection_index = Some(selection_index + 1); 
            }
        }
        
    }

    pub fn decrement_selection(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if selection_index > 0 { 
                self.selection_index = Some(selection_index - 1); 
            }
        }
    }

    pub fn populate_files(&mut self) -> Result<(), std::io::Error> {
        let mut files = file_ops::get_files_for_current_directory(&self)?;

        files.sort();

        self.directory_contents = files;
        self.max_file_selection = self.directory_contents.len();

        if self.max_file_selection == 0 {
            self.selection_index = None;
        }

        Ok(())
    }

    pub fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
        match self.mode {
            Mode::Browse => self.selection_index = Some(0),
            Mode::Command => self.selection_index = None,
            _ => {}
        };
    }

    pub fn open_folder(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if let DirectoryItem::Directory(path) = &self.directory_contents[selection_index] {
                let previous_dir = self.current_directory.clone();

                self.current_directory = PathBuf::from(path);

                if let Err(err) = self.populate_files() {
                    self.current_directory = previous_dir;
                    self.file_error = Some(err);
                } else {
                    self.selection_index = Some(0);
                }
            }
        }
    }

    pub fn move_up_directory(&mut self) -> Result<(), std::io::Error> {
        let current_dir = self.current_directory.to_str().unwrap();

        if current_dir != "/" {
            let mut prev_dir_split: Vec<&str> = current_dir.split("/").collect();
            prev_dir_split.remove(prev_dir_split.len() - 1);
            let mut new_dir_string = prev_dir_split.join("/");
            if new_dir_string == "" { 
                new_dir_string.push_str("/"); 
            }

            self.current_directory = PathBuf::from(new_dir_string);
            self.selection_index = Some(0);
            self.populate_files()?;
        }

        Ok(())
    }

    pub fn add_to_command_buffer(&mut self, character: char) {
        self.command_buffer.push(character);
    }

    pub fn execute_command(&mut self) {
        let command_string = self.get_command_buffer_as_string();
        self.command_buffer = Vec::new();
        self.change_mode(Mode::Browse);
    }

    pub fn get_command_buffer_as_string(&self) -> String {
        let mut command_string = String::new();
        for c in &self.command_buffer {
            command_string.push(*c);
        }

        command_string
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Browse,
    Command,
    Select
}