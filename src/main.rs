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

async fn fetch_title(client: &Client, url: &str) -> Result<String, String> {
    let response = client.get(url).send().await.map_err(|e| {
        if e.is_timeout() {
            "Timeout".into()
        } else if e.is_connect() {
            "Connection Error".into()
        } else {
            format!("{}", e.without_url())
        }
    })?;

    if !response.status().is_success() {
        let code = response.status().as_u16();
        let reason = response
            .status()
            .canonical_reason()
            .unwrap_or("Unknown Error");

        return Err(format!("{code} {reason}"));
    }

    let body = response
        .text()
        .await
        .map_err(|e| e.without_url().to_string())?;
    let selector = Selector::parse("title").unwrap();

    let title = Html::parse_document(&body)
        .select(&selector)
        .next()
        .map(|el| {
            el.inner_html()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or("No Title".into());

    Ok(title)
}

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
            match fetch_title(&client, &url).await {
                Ok(title) => format!("- [{title}]({url})"),
                Err(reason) => format!("- [{reason}]({url})"),
            }
        });

        jhs.push(jh);
    }

    for jh in jhs {
        let mapped = jh.await.unwrap();

        println!("{}", mapped);
    }
}
