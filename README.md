# Dupfinder

A simple command-line tool written in Rust to find duplicate files and estimate how much storage space could be recovered.

## Features

- Recursively scans directories and subdirectories
- Groups files by size before hashing to avoid unnecessary work
- Uses BLAKE3 to identify files with matching contents
- Reads files in 8 KiB chunks instead of loading them entirely into memory
- Lists duplicate groups and estimates recoverable space
- Does not delete or modify files

## Requirements

- Rust and Cargo (edition 2024 compatible)

## Getting started

```bash
git clone https://github.com/Izansase-byte/dupfinder.git
cd dupfinder
cargo run -- /path/to/folder
```

Without a directory argument, Dupfinder scans the local `test` directory:

```bash
cargo run
```

## Commands

```bash
cargo run -- --help
cargo run -- --version
cargo run -- .
cargo test
```

## How it works

1. Walk through the selected directory recursively.
2. Group files by size; skip size groups with only one file.
3. Hash remaining candidates using BLAKE3.
4. Report groups sharing a size and hash.
5. Estimate recoverable space by keeping one file per group.

The recoverable-space figure is an estimate; files are never removed automatically.

## Status

Work in progress (Cargo package version `0.1.0`). Planned improvements include more CLI options, additional tests, and better error handling.

Built for learning Rust and file-system programming.
