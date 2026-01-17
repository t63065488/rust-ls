use std::{fs, path::Path};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to list
    #[arg(default_value = ".")]
    dir: String,

    /// Recursively list directories
    #[arg(short, long)]
    recursive: bool,

    /// Recursively list directories
    #[arg(short, long)]
    files_only: bool,
}

fn main() {
    let args = Args::parse();
    let files = get_files(Path::new(&args.dir), args.files_only);
    println!("---- {}", &args.dir);
    for file in files {
        println!("{}", file);
    }
}

fn get_files(dir: &Path, files_only: bool) -> Vec<String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory {:?}: {}", dir, e);
            return Vec::new();
        }
    };
    
    entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| if files_only { return entry.path().is_file() } else { return true })
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}
