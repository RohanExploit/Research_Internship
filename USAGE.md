# Internet Connectivity Checker and UUID File Generator

## Overview
This Rust utility program checks for internet connectivity and creates multiple files with UUID-generated names.

## Features
- **Internet Connectivity Check**: Makes an HTTP GET request to verify internet connection
- **Random File Count**: Creates between 30 and 47 files (randomly determined)
- **UUID File Names**: Each file is named using a unique UUID v4 identifier
- **Safe Exit**: If no internet connection is detected, the program exits without creating any files

## Requirements
- Rust toolchain (cargo)
- Internet connection (required for the program to create files)

## Dependencies
- `reqwest` - HTTP client for connectivity check
- `uuid` - UUID generation for file names
- `rand` - Random number generation for file count
- `tokio` - Async runtime (required by reqwest)

## Building
```bash
cargo build --release
```

## Running
```bash
cargo run
```

Or run the compiled binary:
```bash
./target/release/internet_checker
```

## Behavior
1. The program first checks internet connectivity by making an HTTP request to `https://www.google.com`
2. If the connection fails, the program exits with an error message
3. If the connection succeeds, it generates a random number between 30 and 47
4. Creates that many files in the current working directory, each named with a unique UUID (format: `{uuid}.txt`)
5. Each file contains a line stating its UUID

## Example Output
```
Checking internet connectivity...
Internet connection verified.
Creating 34 files with UUID names...
Created file 1: a1b2c3d4-e5f6-7890-abcd-ef1234567890.txt
Created file 2: b2c3d4e5-f6a7-8901-bcde-f12345678901.txt
...
Successfully created 34 files.
```

## Error Handling
- If no internet connection is detected, the program exits with exit code 1
- File creation errors are reported but don't stop the program
- Write errors to files generate warnings but continue processing
