use std::{fs::{self, File}, path::PathBuf};

pub fn create_folder<'a>(path: &PathBuf, folder_name: &'a String) -> Result<&'a str, Box<dyn std::error::Error>> {

    let mut folder_to_create = PathBuf::from(path);
    folder_to_create.push(folder_name);
    
    if folder_to_create.exists() {
        return Err("Folder already exists.".into());
    }

    let result = fs::create_dir(&folder_to_create);

    let content = match result {
        Ok(_) => { "App created successfully" },
        Err(error) => { return Err(error.into()); }
    };

    Ok(content)
}

pub fn create_file(path: &PathBuf, file_name: &str)-> Result<File, Box<dyn std::error::Error>> {

    let mut file_path: PathBuf = path.clone();
    file_path.push(file_name);

    if file_path.exists() {
        return Err("File already exists.".into());
    }

    let result = fs::File::create(file_path);


    match result {
        Ok(file) => { Ok(file) },
        Err(error) => { return Err(error.into()); }
    }

}

pub fn capitalize_first_letter(s: &String) -> String {
    let mut modified_string = s.clone();
    
    if let Some(c) = modified_string.get_mut(0..1) {
        c.make_ascii_uppercase();
    }

    modified_string
}

pub fn remove_last_character(s: &String) -> String {
    let mut modified_string = s.clone();
    
    modified_string.pop();

    modified_string
}