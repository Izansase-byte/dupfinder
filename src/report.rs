use std::path::PathBuf;
use crate::FileInfo;

pub fn print_files(files:&mut [FileInfo]) {
    files.sort_by_key(|file| file.path.clone());
    let mut x: Option<PathBuf> = None;

    for i in files {
        let root = i.path.as_path();
        match i.path.parent() {
            Some(folder) => {
                if let Some(name) = i.path.file_name() {
                    if x.as_deref() == Some(folder) {
                        println!("File name:{}",name.to_string_lossy())
                    } else {
                        let profundity = match folder.strip_prefix(root) {
                            Ok(relative_path) => relative_path.components().count(),
                            Err(error) => {
                                eprintln!("We can't find the relative path: {}", error);
                                0
                            }
                        };
                        println!("Folder: {}", folder.display());
                        println!("{}  File name: {}"," ".repeat(profundity),name.to_string_lossy())
                    }
                }
                x = Some(folder.to_path_buf());
            } 
            None => {
                println!("This route doesn't have a parent folder")
            }
        }
        }}
pub fn print_duplicates(duplicates: &mut[FileInfo]) {
    duplicates.sort_by_key(|file| file.path.clone());
    let mut x: Option<PathBuf> = None;

    for i in duplicates {
        let root = i.path.as_path();
        match i.path.parent() {
            Some(folder) => {
                if let Some(name) = i.path.file_name() {
                    if x.as_deref() == Some(folder) {
                        println!("File name:{}",name.to_string_lossy())
                    } else {
                        let profundity = match folder.strip_prefix(root) {
                            Ok(relative_path) => relative_path.components().count(),
                            Err(error) => {
                                eprintln!("We can't find the relative path: {}", error);
                                0
                            }
                        };
                        println!("Folder: {}", folder.display());
                        println!("{}  File name: {}"," ".repeat(profundity),name.to_string_lossy())
                    }
                }
                x = i.path()
            } 
            None => {
                println!("This route doesn't have a parent folder")
            }
        }
        }}
