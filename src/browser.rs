use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "Browser")]
    pub browser: String,
    #[serde(rename = "Protocol-Version")]
    pub protocol_version: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "V8-Version")]
    pub v8_version: String,
    #[serde(rename = "WebKit-Version")]
    pub webkit_version: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_debugger_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketTarget {
    pub description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    pub devtools_frontend_url: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_debugger_url: String,
}

pub struct Browser {
    pub http: Client,
    pub browser_uri: String,
}

impl Browser {
    pub fn new(browser_uri: String) -> Self {
        Self {
            http: Client::new(),
            browser_uri,
        }
    }

    fn request(&self, method: Method, path: &str) -> Result<reqwest::RequestBuilder, reqwest::Error> {
        let url = format!("{}{}", self.browser_uri, path);
        let res = self.http.request(method, &url);
        Ok(res)
    }

    pub async fn version(&self) -> Result<Version, reqwest::Error> {
        let req = self.request(Method::GET, "/json/version");
        let res = req?.send().await?;
        let version = res.json::<Version>().await?;
        Ok(version)
    }

    pub async fn get_pages(&self) -> Result<Vec<WebSocketTarget>, reqwest::Error> {
        let req = self.request(Method::GET, "/json/list");
        let res = req?.send().await?;
        let pages = res.json::<Vec<WebSocketTarget>>().await?;
        Ok(pages)
    }

    pub async fn new_page(&self, uri: &str) -> Result<WebSocketTarget, reqwest::Error> {
        let req = self.request(Method::PUT, "/json/new")?
            .query(&[("url", uri)]);
        let res = req.send().await?;
        let page = res.json::<WebSocketTarget>().await?;
        Ok(page)
    }

    pub async fn close_page(&self, id: &str) -> Result<(), reqwest::Error> {
        let req = self.request(Method::DELETE, &format!("/json/close/{}", id));
        let _ = req?.send().await?;
        Ok(())
    }
}