use clap::Parser;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;
use arboard::Clipboard;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 루트 웹사이트 링크
    #[arg(required = true)]
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // 주어진 URL을 파싱
    let root_url = match Url::parse(&args.url) {
        Ok(url) => {
            // url path가 /로 끝나지 않으면 하위 prefix 판별시 문제가 될 수 있음.
            // 하지만 사용자가 https://docs.rs/tokio/latest/tokio/ 와 같이 입력하므로 우선은 받은 그대로 사용.
            url
        }
        Err(e) => {
            eprintln!("Invalid URL: {}", e);
            std::process::exit(1);
        }
    };

    let client = reqwest::blocking::Client::new();
    
    // HTML 가져오기 (동기)
    let response = match client.get(root_url.clone()).send() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to fetch URL: {}", e);
            std::process::exit(1);
        }
    };
    
    let html_text = match response.text() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Failed to read response text: {}", e);
            std::process::exit(1);
        }
    };

    // HTML 파싱
    let document = Html::parse_document(&html_text);
    let selector = Selector::parse("a").unwrap();
    
    // 중복 제거 및 정렬을 위한 HashSet (출력은 임의 순서)
    let mut collected_links = HashSet::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            // 루트의 절대 경로를 기반으로 상대경로/절대경로 병합
            match root_url.join(href) {
                Ok(mut url) => {
                    // 링크의 해시(#) 플래그먼트 제거 (동일 페이지 내 이동은 동일 문서로 취급)
                    url.set_fragment(None);
                    
                    // URL이 루트 URL과 동일한 host를 가지고, 경로가 루트 URL 경로로 시작하는지 검사
                    if url.host_str() == root_url.host_str() && url.path().starts_with(root_url.path()) {
                        collected_links.insert(url.to_string());
                    }
                }
                Err(_) => {
                    // 유효하지 않은 URL 무시
                }
            }
        }
    }

    // 결과 출력 (오름차순 정렬)
    let mut sorted_links: Vec<String> = collected_links.into_iter().collect();
    sorted_links.sort();
    
    // 결과 문자열 생성 및 클립보드 복사
    let output = sorted_links.join("\n");
    print!("{}\n", output);

    if let Ok(mut clipboard) = Clipboard::new() {
        if let Err(e) = clipboard.set_text(output) {
            eprintln!("Failed to copy to clipboard: {}", e);
        } else {
            println!("(Copied to clipboard!)");
        }
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    Ok(())
}
