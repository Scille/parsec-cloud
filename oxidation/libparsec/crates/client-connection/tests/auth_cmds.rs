mod utils;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use libparsec_client_connection::AuthenticatedCmds;
use parsec_api_crypto::SigningKey;
use parsec_api_protocol::authenticated_cmds;
use tokio::sync::oneshot::{channel, Receiver, Sender};
use utils::server::MakeSignatureVerifier;

use hyper::{server::Server, service::Service};

#[tokio::test]
async fn verify_signed_request() {
    const USER_ID: &str = "foobar";
    const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const PORT: u16 = 14689;
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(IP, PORT));

    let kp = SigningKey::generate();
    let url = format!("http://{}:{}", IP, PORT);
    let auth_cmds = generate_client(kp.clone(), USER_ID.as_bytes(), &url);

    let mut signature_verifier = MakeSignatureVerifier::default();
    signature_verifier.register_public_key(USER_ID.to_string(), kp.verify_key());
    let (ready_send, ready_recv) = channel();
    let (stop_send, stop_recv) = channel();

    let server = server(socket_addr, signature_verifier, ready_send, stop_recv);
    let server_handle = tokio::task::spawn_local(server);

    ready_recv
        .await
        .expect("failed to recv ready signal from server");

    let client_handle = tokio::task::spawn_local(client(auth_cmds, stop_send));
}

fn generate_client(signing_key: SigningKey, user_id: &[u8], root_url: &str) -> AuthenticatedCmds {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("cannot build client");
    AuthenticatedCmds::new(client, root_url, user_id, signing_key)
        .expect("failed to build auth cmds client")
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
    });

    notify_ready.send(()).expect("failed to send server ready");

    Ok(graceful.await?)
}

async fn client(
    client: AuthenticatedCmds,
    notify_stop: Sender<()>,
) -> anyhow::Result<authenticated_cmds::ping::Rep> {
    let rep = client
        .ping("hello from the client side!".to_string())
        .await
        .map_err(anyhow::Error::from);
    notify_stop
        .send(())
        .expect("cannot send stop signal to the server");
    rep
}
