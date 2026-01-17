use std::{fs, path::Path};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to list
    #[arg(default_value = ".")]
    dir: String,

    /// Number of times to greet
    #[arg(short, long)]
    recursive: bool,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    let files = get_files(Path::new(&args.dir));
    println!("{:?}", files);
}

fn get_files(dir: &Path) -> Vec<String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory {:?}: {}", dir, e);
            return Vec::new();
        }
    };
    
    entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}
