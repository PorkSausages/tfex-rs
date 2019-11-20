use std::path::PathBuf;
use std::fs::{read_dir, File};
use std::fs;
use std::io::prelude::*;

use crate::app;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum DirectoryItem {
    File((String, u64)),
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
        let file = File::open(&item);

        let file_size: u64 = match file {
            Ok(file) => (file.metadata().unwrap().len() as f64 / 1000.00).ceil() as u64,
            Err(_) => 0
        };

        if item.is_file() {
            let file = DirectoryItem::File((String::from(item.to_str().unwrap()), file_size));
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

        let current_name = app.get_selected_file_path().unwrap();
                
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
            DirectoryItem::File((path, _)) => fs::remove_file(path)
        };

        match result {
            Ok(_) => None,
            Err(err) => Some(String::from(err.to_string()))
        }
    } else {
        Some(String::from("Nothing to delete"))
    }
}

pub fn read_file(app: &mut app::App) -> (Option<Vec<u8>>, Option<String>) {
    let file_path = app.get_selected_file_path();
    if let Some(path) = file_path {
        //read the file
        let mut file = File::open(&path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();

        //get old filename and store it
        let split_path: Vec<String> = path.split("/")
            .map(|s| s.to_string())
            .collect();

        let result = file.read_to_end(&mut buffer);
        match result {
            Ok(_) => (Some(buffer), Some(split_path.last().unwrap().to_string())),
            Err(err) => {
                app.error = Some(err.to_string());
                (None, None)
            }
        }
    } else {
        (None, None)
    }
}

pub fn write_file(app: &mut app::App) -> Result<(), std::io::Error> {
    let buffered_file = app.get_buffered_file();
    if buffered_file != (None, None) {
        let mut file = File::create(format!(
            "{}/{}", 
            app.current_directory
                .to_str()
                .unwrap(), 
            buffered_file.1
                .clone()
                .unwrap()
                .as_str()
            )
        )?;

        let result = file.write(&buffered_file.0.unwrap());

        if let Err(err) = result {
            app.error = Some(err.to_string());
            Err(err)
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }

}