use std::path::PathBuf;
use crate::Result;

pub fn get_directory(string_path: &String) -> Result<PathBuf> {
    let path = PathBuf::from(string_path);
    if path.exists() && path.is_dir() {
        return Ok(path);
    }
    Err("directory not found!".into())
}

pub fn get_directory_musics(path_buf: &PathBuf) -> Vec<String> {
    let mut list = Vec::new();
    let read_dir = std::fs::read_dir(&path_buf).unwrap();
    for file in read_dir {
        let file_itself = file.unwrap().path();
        if file_itself.extension().unwrap() == "mp3" || file_itself.extension().unwrap() == "WAV" {
            list.push(file_itself.into_os_string().into_string().unwrap())
        }
    }
    return list;
}