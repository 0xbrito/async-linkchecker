use reqwest::Client;
use scraper::{Html, Selector};
use std::{env, fs, process, sync::Arc};
use tokio::{spawn, sync::Semaphore};

const CONCURRENY: usize = 32;

fn parse_urls(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed.strip_prefix("- ").map(|url| url.trim().into())
        })
        .collect()
}

// fn fetch_url(client: Client, url: &str) -> Option<String> {}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {e}");
        process::exit(1);
    });
    let urls = parse_urls(&content);

    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(CONCURRENY));

    let mut jhs = Vec::new();
    for url in urls {
        let semaphore = semaphore.clone();
        let client = client.clone();

        let jh = spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();

            let result = client.get(&url).send().await;
            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        let body = response.text().await.unwrap();
                        let selector = Selector::parse("title").unwrap();
                        let page_title: String = Html::parse_document(&body)
                            .select(&selector)
                            .next()
                            .map(|el| el.inner_html())
                            .unwrap_or("No title".to_string())
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ");

                        format!("- [{page_title}]({url})")
                    } else {
                        let reason = response
                            .status()
                            .canonical_reason()
                            .unwrap_or("Unknown Error");

                        format!("- [{reason}]({url})")
                    }
                }
                Err(e) => format!("- [{}]({})", e.without_url(), url),
            }
        });

        jhs.push(jh);
    }

    for jh in jhs {
        let mapped = jh.await.unwrap();

        println!("{}", mapped);
    }
}
