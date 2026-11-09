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

    let dir = scan_directory(Path::new("."));
    
    println!("{}",dir);

    println!("Path: {} | Size: {}",info_test.path.display(),info_test.size
)
}
fn scan_directory(path:&Path) -> std::io::Result<Vec<FileInfo>> {
    let mut files: Vec<FileInfo> = Vec::new();
    
    for file in fs::read_dir(path)? {
        let file = file?;
        if !file.metadata()?.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "La ruta debe ser una carpeta"
            ));
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
