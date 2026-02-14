use std::{env, fs, process};

fn parse_urls(content: &str) -> Vec<&str> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed.strip_prefix("- ").map(|url| url.trim())
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {e}");
        process::exit(1);
    });

    let _urls = parse_urls(&content);

    todo!();
}
