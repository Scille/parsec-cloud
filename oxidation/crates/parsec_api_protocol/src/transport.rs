// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use async_std::net::TcpStream;
use async_tungstenite::{
    accept_async, client_async, tungstenite,
    tungstenite::{handshake::client::generate_key, Message},
    WebSocketStream,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use http::{Request, Uri};

pub struct Transport {
    pub write: SplitSink<WebSocketStream<TcpStream>, Message>,
    pub read: SplitStream<WebSocketStream<TcpStream>>,
}

// Only because we need user-agent
fn build_request_from_uri<T: AsRef<str>>(uri: T) -> Request<()> {
    dotenv::dotenv().ok();
    let version = std::env::var("VERSION").unwrap();
    let user_agent = format!("parsec/{version}");

    let uri = uri.as_ref().parse::<Uri>().unwrap();
    let authority = uri.authority().unwrap().as_str();
    let host = authority
        .find('@')
        .map(|idx| authority.split_at(idx + 1).1)
        .unwrap_or_else(|| authority);

    Request::builder()
        .header("Host", host)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .header("User-Agent", user_agent)
        .uri(uri)
        .body(())
        .unwrap()
}

impl Transport {
    pub async fn init_for_client<T: AsRef<str>>(stream: TcpStream, uri: T) -> Self {
        let request = build_request_from_uri(uri);
        let (ws_stream, _) = client_async(request, stream).await.unwrap();
        let (write, read) = ws_stream.split();

        Self { write, read }
    }
    pub async fn init_for_server(stream: TcpStream) -> Self {
        let ws_stream = accept_async(stream).await.unwrap();
        let (write, read) = ws_stream.split();

        Self { write, read }
    }
    pub async fn send(&mut self, data: Vec<u8>) -> tungstenite::Result<()> {
        self.write.send(Message::Binary(data)).await
    }
    pub async fn recv(&mut self) -> Option<Vec<u8>> {
        self.read
            .next()
            .await
            .and_then(|res| res.ok())
            .map(Message::into_data)
    }
}
