mod utils;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use libparsec_client_connection::AuthenticatedCmds;
use parsec_api_crypto::SigningKey;
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
    let client = generate_client(kp.clone(), USER_ID.as_bytes(), &url);

    let mut signature_verifier = MakeSignatureVerifier::default();
    signature_verifier.register_public_key(USER_ID.to_string(), kp.verify_key());
    let server = Server::bind(&socket_addr).serve(signature_verifier);

    let ping_handle =
        tokio::task::spawn_local(client.ping("hello from the client side!".to_string()));
}

fn generate_client(signing_key: SigningKey, user_id: &[u8], root_url: &str) -> AuthenticatedCmds {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("cannot build client");
    AuthenticatedCmds::new(client, root_url, user_id, signing_key)
        .expect("failed to build auth cmds client")
}
