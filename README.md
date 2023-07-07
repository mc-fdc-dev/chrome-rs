# chrome-rs
chromiumのdevtools protocol向けの低レベルAPIラッパー

## Sample code
```rs
use chrome_rs::{Browser, Page};

#[tokio::main]
async fn main() {
    let browser = Browser::new("http://localhost:9222".to_string());
    let page = browser.new_page("https://www.google.com").await.unwrap();
    let mut page_client = Page::new(page.websocket_debugger_url);
    page_client.connect().await.unwrap();
    let data = page_client.screenshot().await.unwrap();
    std::fs::write("screenshot.png", data).unwrap();
    browser.close_page(&page.id).await.unwrap();
}
```