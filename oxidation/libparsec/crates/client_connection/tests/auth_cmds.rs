// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod utils;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use libparsec_client_connection::{AuthenticatedCmds, CommandError};
use parsec_api_crypto::SigningKey;
use parsec_api_protocol::authenticated_cmds;
use tokio::{
    sync::oneshot::{channel, Receiver, Sender},
    task::JoinHandle,
};
use utils::server::MakeSignatureVerifier;

use hyper::server::Server;

#[tokio::test]
async fn valid_request() {
    setup_logger();

    const PING_MESSAGE: &str = "hello from the client side!";
    const USER_ID: &str = "foobar";
    const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const PORT: u16 = 14689;
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(IP, PORT));

    let kp = SigningKey::generate();
    let vk = kp.verify_key();
    let url = format!("http://{}:{}", IP, PORT);
    let auth_cmds = generate_client(kp, USER_ID.as_bytes(), &url);

    let mut signature_verifier = MakeSignatureVerifier::default();
    signature_verifier.register_public_key(USER_ID.to_string(), vk);

    let (server_handle, client_handle) =
        send_ping(socket_addr, signature_verifier, auth_cmds, PING_MESSAGE).await;

    let client_response = client_handle.await.expect("client handle failed");
    let server_response = server_handle.await.expect("server handle failed");

    log::debug!("[test] server response: {server_response:?}");
    server_response.expect("server failed with error");

    log::debug!("[test] client response: {client_response:?}");
    let client_response = client_response.expect("unexpected error for client response");
    assert_eq!(
        authenticated_cmds::ping::Rep::Ok {
            pong: format!(
                r#"hello from the server side!, thanks for your message: "{}""#,
                PING_MESSAGE
            )
        },
        client_response
    );
}

#[tokio::test]
async fn invalid_request() {
    setup_logger();

    const PING_MESSAGE: &str = "hello from the client side!";
    const USER_ID: &str = "foobar_invalid";
    const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const PORT: u16 = 14690;
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(IP, PORT));

    let client_kp = SigningKey::generate();
    let other_kp = SigningKey::generate();
    let url = format!("http://{}:{}", IP, PORT);
    let auth_cmds = generate_client(client_kp, USER_ID.as_bytes(), &url);

    let mut signature_verifier = MakeSignatureVerifier::default();
    signature_verifier.register_public_key(USER_ID.to_string(), other_kp.verify_key());

    let (server_handle, client_handle) =
        send_ping(socket_addr, signature_verifier, auth_cmds, PING_MESSAGE).await;

    let client_response = client_handle.await.expect("client handle failed");
    let server_response = server_handle.await.expect("server handle failed");

    log::debug!("[test] server response: {server_response:?}");
    server_response.expect("server failed with error");

    log::debug!("[test] client response: {client_response:?}");
    let client_response =
        client_response.expect_err("unexpected valid result for a client response");

    assert!(
        matches!(
            client_response,
            CommandError::InvalidResponseStatus(reqwest::StatusCode::UNAUTHORIZED, _)
        ),
        r#"expected "unexpected response status" with code 401, but got {client_response}"#
    );
}

fn setup_logger() {
    if let Err(e) = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .parse_write_style("auto")
        .is_test(true)
        .try_init()
    {
        log::warn!("failed to initialize logger, reason: {e}");
    }
}

fn generate_client(signing_key: SigningKey, user_id: &[u8], root_url: &str) -> AuthenticatedCmds {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("cannot build client");
    AuthenticatedCmds::new(client, root_url, user_id, signing_key)
        .expect("failed to build auth cmds client")
}

async fn send_ping(
    socket_addr: SocketAddr,
    signature_verifier: MakeSignatureVerifier,
    auth_cmds: AuthenticatedCmds,
    message: &str,
) -> (
    JoinHandle<anyhow::Result<()>>,
    JoinHandle<libparsec_client_connection::command_error::Result<authenticated_cmds::ping::Rep>>,
) {
    let (ready_send, ready_recv) = channel();
    let (stop_send, stop_recv) = channel();

    let server_handle = tokio::task::spawn(server(
        socket_addr,
        signature_verifier,
        ready_send,
        stop_recv,
    ));

    ready_recv
        .await
        .expect("failed to recv ready signal from server");
    log::debug!("[test] server is ready, we can continue");

    let client_handle = tokio::task::spawn(client(auth_cmds, stop_send, message.to_string()));

    (server_handle, client_handle)
}

async fn server(
    socket_addr: SocketAddr,
    signature_verifier: MakeSignatureVerifier,
    notify_ready: Sender<()>,
    notify_stop: Receiver<()>,
) -> anyhow::Result<()> {
    let server = Server::bind(&socket_addr).serve(signature_verifier);
    let graceful = server.with_graceful_shutdown(async {
        notify_stop.await.expect("notify stop receiver failed");
        log::debug!("[server] recv signal to stop");
    });

    notify_ready.send(()).expect("failed to send server ready");
    log::debug!("[server] signal we're ready");

    Ok(dbg!(graceful.await)?)
}

async fn client(
    client: AuthenticatedCmds,
    notify_stop: Sender<()>,
    message: String,
) -> libparsec_client_connection::command_error::Result<authenticated_cmds::ping::Rep> {
    let rep = client.ping(message).await;
    log::info!("[client] recv response: {rep:?}");
    log::debug!("[client] notify server to stop");
    notify_stop
        .send(())
        .expect("cannot send stop signal to the server");
    rep
}
