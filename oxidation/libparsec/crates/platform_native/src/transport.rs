// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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
    write: SplitSink<WebSocketStream<TcpStream>, Message>,
    read: SplitStream<WebSocketStream<TcpStream>>,
}

// Only because we need user-agent
async fn build_client_from_uri(uri: Uri) -> (TcpStream, Request<()>) {
    dotenv::dotenv().ok();
    let version = std::env::var("VERSION").expect("Missing VERSION env variable");
    let user_agent = format!("parsec/{version}");
    let authority = uri.authority().expect("Missing authority segment").as_str();
    let host = authority
        .find('@')
        .map(|idx| authority.split_at(idx + 1).1)
        .unwrap_or_else(|| authority);

    (
        TcpStream::connect(host)
            .await
            .expect("Failed to open TCP connection"),
        Request::builder()
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .header("User-Agent", user_agent)
            .uri(uri)
            .body(())
            .expect("Failed to build request"),
    )
}

impl Transport {
    pub async fn init_for_client<T: AsRef<str>>(uri: T) -> Self {
        let uri = uri.as_ref().parse::<Uri>().expect("Invalid URI provided");
        let (stream, request) = build_client_from_uri(uri).await;
        let (ws_stream, _) = client_async(request, stream)
            .await
            .expect("Failed to open websocket connection");
        let (write, read) = ws_stream.split();

        Self { write, read }
    }
    pub async fn init_for_server(stream: TcpStream) -> Self {
        let ws_stream = accept_async(stream)
            .await
            .expect("Failed to open websocket connection");
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
