use std::fs;
use std::path::Path;

pub fn is_file(path : &str) -> bool {
    Path::new(path).is_file()
}

pub fn is_extension(path: &str, extension: &str) -> bool{
    Path::new(path).extension().and_then(|ext| ext.to_str()) == Some(extension)
}

pub fn read_file(path : String) -> String {
    let content = fs::read_to_string(path);
    match content {
        Ok(file_content) => {
            file_content
        }
        Err(_) => {
            String::from("Error while reading the file")
        }
    }
}

