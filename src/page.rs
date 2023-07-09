use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    WebSocketStream,
    MaybeTlsStream,
};
use tokio::net::TcpStream;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use base64::{
    Engine as _,
    engine::{self, general_purpose}
};
use serde::{Deserialize, Serialize};

pub struct Page {
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    uri: String,
}

#[derive(Debug, Deserialize)]
pub struct PageResponse {
    id: u64,
    result: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    command: String,
    parameters: serde_json::Value,
}

impl Page {
    pub fn new(uri: String) -> Self {
        Self {
            ws: None,
            uri,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(self.uri.clone()).await?;
        self.ws = Some(ws_stream);
        Ok(())
    }

    async fn send(&self, page_request: PageRequest) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(&page_request)?;
        self.ws.as_mut().unwrap().send(msg).await?;
        Ok(())
    }

    pub async fn screenshot(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let msg = Message::Text(r#"{"id": 1, "method": "Page.captureScreenshot", "params": {}}"#.to_string());
        self.ws.as_mut().unwrap().send(msg).await?;
        let data = self.ws.as_mut().unwrap().next().await.unwrap()?;
        let data = serde_json::from_str::<PageResponse>(&data.to_string())?;
        let data = general_purpose::STANDARD
            .decode(data.result["data"].as_str().unwrap().as_bytes())
            .unwrap();
        Ok(data)
    }
}
