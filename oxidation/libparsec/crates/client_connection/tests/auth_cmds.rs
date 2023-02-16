// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod utils;

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

use libparsec_client_connection::{
    generate_client, AuthenticatedCmds, CommandError, CommandResult,
};
use libparsec_crypto::SigningKey;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::TestbedScope;
use libparsec_types::{BackendOrganizationAddr, DeviceID};
use tokio::{
    sync::oneshot::{channel, Receiver, Sender},
    task::JoinHandle,
};
use utils::server::MakeSignatureVerifier;

use hyper::server::Server;

const RVK: &str = "7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss";

#[tokio::test]
async fn valid_request() {
    setup_logger();

    const PING_MESSAGE: &str = "hello from the client side!";
    const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const PORT: u16 = 14689;
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(IP, PORT));

    let device_id = DeviceID::from_str("michu@michu_org").unwrap();

    let kp = SigningKey::generate();
    let vk = kp.verify_key();
    let url = generate_backend_organization(IP, PORT, RVK);
    let auth_cmds = generate_client(kp, device_id.clone(), url);

    let mut signature_verifier = MakeSignatureVerifier::default();
    signature_verifier.register_public_key(device_id, vk);

    let (server_handle, client_handle) =
        send_ping(socket_addr, signature_verifier, auth_cmds, PING_MESSAGE).await;

    let client_response = client_handle.await.expect("client handle failed");
    let server_response = server_handle.await.expect("server handle failed");

    log::debug!("[test] server result: {server_response:?}");
    server_response.expect("server failed with error");

    log::debug!("[test] client result: {client_response:?}");
    let client_response = client_response.expect("unexpected error for client response");
    assert_eq!(
        authenticated_cmds::v3::ping::Rep::Ok {
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
    const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const PORT: u16 = 14690;
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(IP, PORT));

    let device_id = DeviceID::from_str("michu@michu_org").unwrap();

    let client_kp = SigningKey::generate();
    let other_kp = SigningKey::generate();
    let url = generate_backend_organization(IP, PORT, RVK);
    let auth_cmds = generate_client(client_kp, device_id.clone(), url);

    let mut signature_verifier = MakeSignatureVerifier::default();
    signature_verifier.register_public_key(device_id, other_kp.verify_key());

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
        r#"expected "Unexpected response status" with code 401, but got {client_response}"#
    );
}

fn generate_backend_organization(ip: Ipv4Addr, port: u16, rvk: &str) -> BackendOrganizationAddr {
    let raw_url = format!("parsec://{ip}:{port}/MyOrg?rvk={rvk}&no_ssl=true");
    let server_url = BackendOrganizationAddr::from_str(&raw_url).unwrap();
    log::debug!(
        "generated backend organization url: {}",
        server_url.to_http_url(None)
    );

    server_url
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

async fn send_ping(
    socket_addr: SocketAddr,
    signature_verifier: MakeSignatureVerifier,
    auth_cmds: AuthenticatedCmds,
    message: &str,
) -> (
    JoinHandle<anyhow::Result<()>>,
    JoinHandle<CommandResult<authenticated_cmds::v3::ping::Rep>>,
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

    dbg!(graceful.await.map_err(anyhow::Error::from))
}

async fn client(
    client: AuthenticatedCmds,
    notify_stop: Sender<()>,
    message: String,
) -> CommandResult<authenticated_cmds::v3::ping::Rep> {
    let rep = client
        .send(authenticated_cmds::v3::ping::Req { ping: message })
        .await;
    log::info!("[client] recv response: {rep:?}");
    log::debug!("[client] notify server to stop");
    notify_stop
        .send(())
        .expect("cannot send stop signal to the server");
    rep
}

#[tokio::test]
async fn with_testbed() {
    TestbedScope::run_with_server("coolorg", |env| async move {
        let client = reqwest::ClientBuilder::new()
            .build()
            .expect("cannot build client");

        let device = env.template.device(&"alice@dev1".parse().unwrap());
        let cmds = AuthenticatedCmds::new(
            client,
            env.organization_addr.clone(),
            device.device_id.to_owned(),
            device.signing_key.to_owned(),
        )
        .unwrap();
        let rep = cmds
            .send(authenticated_cmds::v3::ping::Req {
                ping: "foo".to_owned(),
            })
            .await;
        assert_eq!(
            rep.unwrap(),
            libparsec_protocol::authenticated_cmds::v3::ping::Rep::Ok {
                pong: "foo".to_owned()
            }
        );
    })
    .await
}
