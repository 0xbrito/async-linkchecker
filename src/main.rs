use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {e}");
        process::exit(1);
    });

    todo!();
}
