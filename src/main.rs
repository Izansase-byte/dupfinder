use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use blake3::hazmat::Mode::KeyedHash;

mod report;

pub struct FileInfo {
    path: PathBuf,
    size: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("Dupfinder - Find duplicate files");
                println!("Usage: dupfinder [directory]");
                println!("\nOptions:");
                println!("  -h, --help     Show this help message");
                return;
            }
            "--version" | "-v" => {
                println!("Dupfinder {0}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {}
        }
    }

    let directory = args.get(1).map(String::as_str).unwrap_or("test");

    println!("Scanning directory: {}", directory);

    match scan_directory(Path::new(directory)) {
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

                match calculate_wasted_space(&duplicates) {
                    Ok(bytes) => {
                        let mb = bytes as f64 / 1_048_576.0;
                        println!("\nRecoverable space: {:.2} MB", mb);
                    }
                    Err(error) => eprintln!("Error calculating space: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Err(error) => eprintln!("Scan error: {}", error),
    }
}

fn scan_directory(path: &Path) -> io::Result<Vec<FileInfo>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            files.extend(scan_directory(&entry.path())?);
        } else if metadata.is_file() {
            files.push(FileInfo {
                path: entry.path(),
                size: metadata.len(),
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

fn hash_file(path: &Path) -> io::Result<blake3::Hash> {
    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}

fn hash_partial(path: &Path) -> io::Result<blake3::Hash> {
    const LIMIT: usize = 64 * 1024;

    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 65536];
    let mut remaining = LIMIT;

    while remaining > 0 {
        let to_read = remaining.min(buffer.len());
        let bytes_read = file.read(&mut buffer[..to_read])?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
        remaining -= bytes_read;
    }

    Ok(hasher.finalize())
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

fn find_duplicates(files: &[FileInfo]) -> io::Result<Vec<Vec<PathBuf>>> {
    let groups = group_files_by_size(files);
    let mut duplicates = Vec::new();

    for group in groups.into_values() {
        if group.len() < 2 {
            continue;
        }

        let mut partial: HashMap<blake3::Hash, Vec<PathBuf>> = HashMap::new();

        for file in group {
            let hash = hash_partial(&file.path)?;
            partial.entry(hash).or_default().push(file.path.clone());
        }

        for candidates in partial.into_values() {
            if candidates.len() < 2 {
                continue;
            }

            let mut full: HashMap<blake3::Hash, Vec<PathBuf>> = HashMap::new();

            for path in candidates {
                let hash = hash_file(&path)?;
                full.entry(hash).or_default().push(path)
            }

            duplicates.extend(full.into_values().filter(|paths| paths.len() >= 2));
        }
    }
    Ok(duplicates)
}

fn calculate_wasted_space(duplicates: &[Vec<PathBuf>]) -> io::Result<u64> {
    let mut wasted_space = 0;

    for group in duplicates {
        if group.len() < 2 {
            continue;
        }
        let size = fs::metadata(&group[0])?.len();
        let extra_copies = (group.len() - 1) as u64;
        wasted_space += size * extra_copies;
    }

    Ok(wasted_space)
}

#[cfg(test)]
mod tests {
    use std::{alloc::System, time::UNIX_EPOCH};

    use super::*;

    #[test]
    fn test_same_size() {
        let files = vec![
            FileInfo {
                path: PathBuf::from("a.txt"),
                size: 100,
            },
            FileInfo {
                path: PathBuf::from("b.txt"),
                size: 100,
            },
        ];

        let groups = group_files_by_size(&files);
        assert_eq!(groups.get(&100).unwrap().len(), 2);
    }

    #[test]
    fn test_different_sizes() {
        let files = vec![
            FileInfo {
                path: PathBuf::from("a.txt"),
                size: 100,
            },
            FileInfo {
                path: PathBuf::from("b.txt"),
                size: 200,
            },
        ];

        let groups = group_files_by_size(&files);
        assert_eq!(groups.len(), 2);
    }
    #[test]
    fn test_find_duplicates() -> std::io::Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let dir =
            std::env::temp_dir().join(format!("dupfinder_test_{}_{}", std::process::id(), unique));

        fs::create_dir(&dir)?;

        let result = (|| -> std::io::Result<()> {
            fs::write(dir.join("a.txt"), "Hello Rust!")?;
            fs::write(dir.join("b.txt"), "Hello Rust!")?;
            fs::write(dir.join("c.txt"), "Different content")?;

            let files = scan_directory(&dir)?;
            let duplicates = find_duplicates(&files)?;

            assert_eq!(duplicates.len(), 1);
            assert_eq!(duplicates[0].len(), 2);

            Ok(())
        })();

        fs::remove_dir_all(&dir)?;
        result
    }
}
