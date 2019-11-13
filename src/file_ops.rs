use std::path::PathBuf;
use std::fs::read_dir;

use crate::app;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum DirectoryItem {
    File(String),
    Directory(String)
}

pub fn get_files_for_current_directory(app: &app::App) -> Vec<DirectoryItem> {
    //Get list, unwrap, and convert results to &Path 
    let dir_items: Vec<PathBuf> = read_dir(app.current_directory.as_path()).unwrap() //Handle this error properly
        .map(|f| f.unwrap().path())
        .collect();
    
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
    
    files
}