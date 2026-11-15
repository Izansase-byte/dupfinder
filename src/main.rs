use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

mod report;

pub struct FileInfo {
    path: PathBuf,
    size: u64,
}

fn main() {
    let directorio = "test";

    match scan_directory(Path::new(directorio)) {
        Ok(files) => match find_duplicates(&files) {
            Ok(duplicates) => {
                if duplicates.is_empty() {
                    println!("No duplicates found!");
                }

                for (i, group) in duplicates.iter().enumerate() {
                    println!("\nDuplicate group {}:", i + 1);

                    for path in group {
                        println!("  {}", path.display());
                    }
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Err(error) => eprintln!("Scan error: {}", error),
    }
}

fn scan_directory(path: &Path) -> std::io::Result<Vec<FileInfo>> {
    let mut files: Vec<FileInfo> = Vec::new();

    for file in fs::read_dir(path)? {
        let file = file?;
        if file.metadata()?.is_dir() {
            files.extend(scan_directory(&file.path())?);
        } else if file.metadata()?.is_file() {
            files.push(FileInfo {
                path: file.path(),
                size: file.metadata()?.len(),
            });
        }
    }

    Ok(files)
}

fn group_files_by_size(files: &[FileInfo]) -> HashMap<u64, Vec<&FileInfo>> {
    let mut groups: HashMap<u64, Vec<&FileInfo>> = HashMap::new();

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

fn find_duplicates(files: &[FileInfo]) -> std::io::Result<Vec<Vec<PathBuf>>> {
    let groups = group_files_by_size(files);
    let mut duplicates: Vec<Vec<PathBuf>> = Vec::new();

    for (_size, group) in groups {
        if group.len() < 2 {
            continue;
        }

        let mut hashes: HashMap<blake3::Hash, Vec<PathBuf>> = HashMap::new();

        for file in group {
            let hash = hash_file(&file.path)?;
            hashes.entry(hash).or_default().push(file.path.clone());
        }

        for (_hash, paths) in hashes {
            if paths.len() >= 2 {
                duplicates.push(paths);
            }
        }
    }

    Ok(duplicates)
}
