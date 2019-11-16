use std::fs;

use crate::app::App;
use crate::file_ops::DirectoryItem;

pub fn process_command(command_string: String, app: &mut App) {
    //split command buffer 
    let split_command: Vec<String> = command_string
        .trim_start_matches(":")
        .split_ascii_whitespace()
        .map(|f| f.to_string())
        .collect();

    let current_dir = &app.current_directory.to_str().unwrap();
    
    match split_command[0].to_ascii_uppercase().as_ref() {
        "RENAME" => app.error = rename_file(&split_command, current_dir, &app),
        _ => app.error = Some(String::from("Not a command")) 
    }
}

fn rename_file(command: &Vec<String>, current_dir: &str, app: &App) -> Option<String> {
    if command.len() > 1 && app.selection_index != None {
        //put new file name back together after originally splitting on whitespace
        let new_name_split = &command[1..command.len()];
        let mut concat = String::new();
        for s in new_name_split {
            concat.push_str(format!("{} ", s).as_str());
        }
        let new_name = concat.trim_end();

        let selection_index = app.selection_index.unwrap();
        
        let current_name = match &app.directory_contents[selection_index] {
            DirectoryItem::Directory(path) => path,
            DirectoryItem::File(path) => path
        };
                
        match fs::rename(current_name, format!("{}/{}", current_dir, new_name)) {
            Ok(_) => None,
            Err(err) => Some(String::from(err.to_string()))
        }
    } else {
        Some(String::from("Wrong number of arguments supplied"))
    }
}