# Directory Lister

A simple command-line tool written in Rust to list files and directories.

## Features

- List all files and directories in a given path
- Filter to show files only (exclude directories)
- Recursive directory listing support (flag implemented, functionality pending)
- Clean, minimal output

## Installation

Make sure you have Rust installed. Then clone this repository and build:

```bash
cargo build --release
```

The binary will be available at `target/release/rls`.

## Usage

### Basic Usage

List contents of the current directory:
```bash
./rls
```

List contents of a specific directory:
```bash
./rls /path/to/directory
```

### Options

- `-f, --files-only` - Show only files, excluding directories
- `-r, --recursive` - Recursively list directories (coming soon)
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Examples

List only files in the current directory:
```bash
./rls -f
```

List only files in a specific directory:
```bash
./rls /home/user/documents -f
```

List contents recursively (when implemented):
```bash
./rls /home/user/documents -r
```

## Output Format

The tool outputs a header showing the directory being listed, followed by the contents:

```
#---- /path/to/directory
file1.txt
file2.rs
subdirectory
```

## Error Handling

If the specified directory doesn't exist or can't be read, an error message will be printed to stderr and an empty list will be returned.

## Requirements

- Rust 1.70 or higher (for clap 4.0 features)

## Dependencies

- `clap` - Command-line argument parsing

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

For more information, please refer to <http://unlicense.org/>

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.