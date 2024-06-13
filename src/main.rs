use clap::Parser;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(default_value = ".")]
    starting_dir: PathBuf, // Starting directory, default is current directory
    #[arg(short, long, value_parser, num_args=0.., value_delimiter=',')]
    exclude: Vec<PathBuf>, // List of paths to exclude
}

fn main() {
    let args = Args::parse();
    // Get name of current directory if one is not supplied
    let starting_dir_name = args
        .starting_dir
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            env::current_dir()
                .ok()
                .and_then(|dir| {
                    dir.file_name()
                        .map(|name| name.to_string_lossy().to_string())
                })
                .unwrap_or_else(|| ".".to_string())
        });

    println!("{}", starting_dir_name);
    parse_directory(&args.starting_dir, 1, &args.exclude).unwrap()
}

fn parse_directory(directory: &Path, level: i32, exclude: &Vec<PathBuf>) -> Result<(), String> {
    let entries = match directory.read_dir() {
        Ok(entries) => entries,
        Err(err) => return Err(err.to_string()),
    };
    for entry in entries.flatten() {
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(err) => return Err(err.to_string()),
        };
        if exclude
            .iter()
            .any(|excluded_path| &entry.file_name() == excluded_path)
        {
            continue;
        }
        for _ in 0..level {
            if level > 0 {
                print!("\t");
            }
        }
        if file_type.is_dir() {
            println!("|{}", entry.file_name().to_string_lossy());
            match parse_directory(&entry.path(), level + 1, exclude) {
                Ok(()) => {}
                Err(_) => continue,
            };
        } else if file_type.is_file() {
            print!("|____");
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}
