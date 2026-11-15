use std::path::{self, PathBuf};
use std::{fs, vec};
use std::path::Path;
use std::collections::HashMap;

mod report;

pub struct FileInfo {
    path:PathBuf,
    size:u64,
}

fn main() {

    let directorio = "test";

    let dir = scan_directory(Path::new(&directorio));
    match dir {
    
    Ok(files) => {
        for file in files {
            println!("Path: {} | Size:{}", file.path.display(), file.size)
        }
    }
    Err(error) => {
        eprintln!("{}",error)
    }

    
};
}
fn scan_directory(path:&Path) -> std::io::Result<Vec<FileInfo>> {
    let mut files: Vec<FileInfo> = Vec::new();
    
    for file in fs::read_dir(path)? {
        let file = file?;
        if !file.metadata()?.is_file() {
            if file.metadata()?.is_dir() {
                let file = file.path();
                let nested_files = scan_directory(&file)?;
                files.extend(nested_files);
            } else {
                continue;
            }
        } else {
            let file_length = file.metadata()?.len();
            let path = file.path();
            let file1 = FileInfo {
                path: path,
                size: file_length,
            };
            files.push(file1);
        }
    }
    Ok(files)
}
fn group_files_by_size(files: &[FileInfo]) -> HashMap<u64, Vec<&FileInfo>> {
    let mut groups: HashMap<u64,Vec<&FileInfo>> = HashMap::new();

    for file in files {
        groups.entry(file.size).or_default().push(file);
    }
        groups
        }
fn hash_file(path: &Path) -> std::io::Result<blake3::Hash> {
    let content = fs::read(path)?;
    Ok(blake3::hash(&content))
}
fn compare_files(file1: &FileInfo, file2: &FileInfo) -> bool {
    if file1.size != file2.size {
        return false;
    }
    match (hash_file(&file1.path), hash_file(&file2.path)) {
        (Ok(hash1), Ok(hash2)) => hash1 == hash2,
        _ => false,
    }
}