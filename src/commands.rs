use crate::app::App;
use crate::file_ops;

pub fn process_command(command_string: String, app: &mut App) {
    //split command buffer 
    let split_command: Vec<String> = command_string
        .trim_start_matches(":")
        .split_ascii_whitespace()
        .map(|f| f.to_string())
        .collect();

    let current_dir = &app.current_directory.to_str().unwrap();
    
    match split_command[0].to_ascii_uppercase().as_ref() {
        "RENAME" => app.error = file_ops::rename_file(&split_command, current_dir, &app),
        "DELETE" => app.error = {
            let result = file_ops::delete_file(&app);
            app.decrement_selection();
            result
        },
        _ => app.error = Some(String::from("Not a command")) 
    };
}   



