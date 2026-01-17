use std::path::Path;

use clap::Parser;
use rls::get_files;

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

    /// Whether to ignore directories in print
    #[arg(short, long)]
    files_only: bool,
}

fn main() {
    let args = Args::parse();
    let files = get_files(Path::new(&args.dir), args.files_only);
    println!("#---- {}", &args.dir);
    for file in files {
        println!("{}", file);
    }
}
