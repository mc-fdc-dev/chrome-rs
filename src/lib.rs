mod browser;
mod page;

use browser::Browser;
use page::Page;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_version() {
        let browser = browser::Browser::new("http://localhost:9222".to_string());
        let version = browser.version().await.unwrap();
        println!("{:?}", version);
    }

    #[tokio::test]
    async fn test_get_pages() {
        let browser = browser::Browser::new("http://localhost:9222".to_string());
        let pages = browser.get_pages().await.unwrap();
        println!("{:?}", pages);
    }

    #[tokio::test]
    async fn test_new_page() {
        let browser = browser::Browser::new("http://localhost:9222".to_string());
        let page = browser.new_page("https://www.google.com").await.unwrap();
        let mut page_client = page::Page::new(page.websocket_debugger_url);
        page_client.connect().await.unwrap();
        let data = page_client.screenshot().await.unwrap();
        std::fs::write("screenshot.png", data).unwrap();
        browser.close_page(&page.id).await.unwrap();
    }
}
