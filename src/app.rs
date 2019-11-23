use std::io::Stdout;
use std::path;
use std::path::PathBuf;

use tui::backend::TermionBackend;
use tui::Terminal;

use termion::raw::RawTerminal;

use crate::commands;
use crate::file_ops;
use crate::file_ops::DirectoryItem;

pub struct App<'a> {
    pub current_directory: path::PathBuf,
    pub terminal: &'a mut Terminal<TermionBackend<RawTerminal<Stdout>>>,
    pub mode: Mode,
    pub selection_index: Option<usize>,
    pub directory_contents: Vec<DirectoryItem>,
    pub command_buffer: Vec<char>,
    pub error: Option<String>,
    pub buffered_file_name: Option<String>,
    pub window_height: u16,

    file_buffer: Option<Vec<u8>>,
    max_file_selection: usize,
}

impl<'a> App<'a> {
    pub fn new(terminal: &'a mut Terminal<TermionBackend<RawTerminal<Stdout>>>) -> App<'a> {
        let current_dir = path::PathBuf::from("/");
        let window_height = terminal.size().unwrap().height - 5; //borders + command window height add up to 5

        let mut app = App {
            current_directory: current_dir,
            terminal,
            mode: Mode::Browse,
            selection_index: Some(0),
            max_file_selection: 0,
            directory_contents: Vec::new(),
            command_buffer: Vec::new(),
            file_buffer: None,
            buffered_file_name: None,
            error: None,
            window_height: window_height,
        };

        if let Err(error) = app.populate_files() {
            panic!(format!(
                "Error opening {:?}: {:?}",
                app.current_directory,
                error.kind()
            ));
        }

        app
    }

    pub fn move_selection_down(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if selection_index < self.max_file_selection - 1 {
                self.selection_index = Some(selection_index + 1);
            }
        }
    }

    pub fn move_selection_up(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if selection_index > 0 {
                self.selection_index = Some(selection_index - 1);
            }
        }
    }

    pub fn move_selection_left(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if selection_index >= self.window_height as usize {
                self.selection_index = Some(selection_index - self.window_height as usize);
            } else {
                self.selection_index = Some(0);
            }
        }
    }

    pub fn move_selection_right(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if selection_index + self.window_height as usize <= self.directory_contents.len() - 1 {
                self.selection_index = Some(selection_index + self.window_height as usize);
            } else {
                self.selection_index = Some(self.directory_contents.len() - 1);
            }
        }
    }

    pub fn update_window_height(&mut self) {
        self.window_height = self.terminal.size().unwrap().height - 5; //borders + command window height add up to 5
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
    }

    pub fn open_folder(&mut self) {
        if let Some(selection_index) = self.selection_index {
            if let DirectoryItem::Directory(path) = &self.directory_contents[selection_index] {
                let previous_dir = self.current_directory.clone();

                self.current_directory = PathBuf::from(path);

                if let Err(err) = self.populate_files() {
                    self.current_directory = previous_dir;
                    self.error = Some(err.to_string());
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
        commands::process_command(command_string, self);

        self.change_mode(Mode::Browse);
    }

    pub fn get_command_buffer_as_string(&self) -> String {
        let mut command_string = String::new();
        for c in &self.command_buffer {
            command_string.push(*c);
        }

        command_string
    }

    pub fn get_selected_file_path(&self) -> Option<String> {
        if self.selection_index != None {
            let dir_item = self.directory_contents[self.selection_index.unwrap()].clone();
            match dir_item {
                DirectoryItem::Directory(path) | DirectoryItem::File((path, _)) => Some(path),
            }
        } else {
            None
        }
    }

    pub fn load_selected_into_file_buffer(&mut self) {
        let result = file_ops::read_file(self);
        self.file_buffer = result.0;
        self.buffered_file_name = result.1;
    }

    pub fn get_buffered_file(&self) -> (Option<Vec<u8>>, Option<String>) {
        (self.file_buffer.clone(), self.buffered_file_name.clone())
    }

    pub fn write_buffered_file(&mut self) {
        let result = file_ops::write_file(self);
        if let Ok(_) = result {
            self.buffered_file_name = None;
            self.file_buffer = None;
        }
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Browse,
    Command,
    _Select,
}
