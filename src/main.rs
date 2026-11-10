use std::path::PathBuf;
use std::fs;
use std::path::Path;

struct FileInfo {
    path:PathBuf,
    size:u64,
}

fn main() {
    let path = PathBuf::from("Cargo.toml");

    let info_test = FileInfo {
        path: "ejemplo.txt".into(),
        size: 100,
    };

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
    let mut x: Vec<FileInfo> = Vec::new();
    
    for file in fs::read_dir(path)? {
        let file = file?;
        if !file.metadata()?.is_file() {
            if file.metadata()?.is_dir() {
                let file = file.path();
                x = scan_directory(&file)?;
                files.extend(x);
            } else {
            continue;
            }
        } else {
        let file_length = file.metadata()?.len();
        let path = file.path();
        let file1 = FileInfo {
            path:path,
            size:file_length,
        };
        files.push(file1);
    }}
    Ok(files)
}
