# Dupfinder

Dupfinder is a lightweight command-line utility written in Rust that searches directories for duplicate files and estimates the amount of disk space occupied by redundant copies.

I built Dupfinder to learn more about Rust, recursive file traversal, efficient file reading, hash maps, and content hashing. The application is read-only: it reports duplicates but does not delete or modify your files.

## How it works

Dupfinder recursively scans a directory and groups files by size. Files with a unique size cannot be duplicates and are skipped. For groups of equal-sized files, Dupfinder calculates a BLAKE3 hash of the first 64 KiB. Only files with matching partial hashes are then hashed in full. Matching full hashes are reported as duplicate groups. Finally, Dupfinder estimates recoverable storage by counting all but one copy in each group.

## Features

- Recursive directory scanning
- File grouping by size
- Partial BLAKE3 hashing (first 64 KiB)
- Full BLAKE3 verification for candidates
- Duplicate file path reporting
- Estimated recoverable disk space
- Help and version commands
- Unit tests

## Build from source

Install Rust and Cargo, then run:

```bash
git clone https://github.com/Izansase-byte/dupfinder.git
cd dupfinder
cargo test
cargo build --release
```

## Usage

On Linux or macOS:

```bash
./target/release/dupfinder /path/to/directory
./target/release/dupfinder .
./target/release/dupfinder ~/Downloads
./target/release/dupfinder --help
./target/release/dupfinder --version
```

On Windows (PowerShell, after building on Windows):

```powershell
.\target\release\dupfinder.exe "C:\Users\YourName\Downloads"
```

If no directory is provided, Dupfinder scans the included `test` directory. Paths containing spaces should be quoted.

## Releases

Prebuilt binaries will be available on the [Releases page](https://github.com/Izansase-byte/dupfinder/releases).

## Testing

Run `cargo test` to check the existing unit tests. Run `cargo run -- test` to try the included sample directory. This directory contains empty files, so its duplicate groups may show `0.00 MB` recoverable space.

## Limitations and roadmap

The tool currently reports results in the terminal and does not provide a graphical interface or automatic deletion. Planned improvements include clearer reporting, progress indication, export formats, and more tests for large files and edge cases.
