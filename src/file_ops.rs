use std::path::PathBuf;
use std::fs::read_dir;
use std::fs;

use crate::app;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum DirectoryItem {
    File(String),
    Directory(String)
}

pub fn get_files_for_current_directory(app: &app::App) -> Result<Vec<DirectoryItem>, std::io::Error> {
    //Get list, unwrap, and convert results to &Path 
    let dir_items: Vec<PathBuf> = match read_dir(app.current_directory.as_path()) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err)
    };
    
    //Convert items to DirectoryItem
    let mut files: Vec<DirectoryItem> = Vec::new();
    for item in dir_items {
        if item.is_file() {
            let file = DirectoryItem::File(String::from(item.to_str().unwrap()));
            files.push(file);
        } else {
            let file = DirectoryItem::Directory(String::from(item.to_str().unwrap()));
            files.push(file);
        }
    };
    
    Ok(files)
}

pub fn rename_file(command: &Vec<String>, current_dir: &str, app: &app::App) -> Option<String> {
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

pub fn delete_file(app: &app::App) -> Option<String> {
    if app.selection_index != None {
        let selection_index = app.selection_index.unwrap();

        let result = match &app.directory_contents[selection_index] {
            DirectoryItem::Directory(path) => fs::remove_dir_all(path),
            DirectoryItem::File(path) => fs::remove_file(path)
        };

        match result {
            Ok(_) => None,
            Err(err) => Some(String::from(err.to_string()))
        }
    } else {
        Some(String::from("Nothing to delete"))
    }
}