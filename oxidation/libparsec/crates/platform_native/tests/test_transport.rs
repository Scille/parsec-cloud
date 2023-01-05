// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use async_std::net::TcpListener;
use async_std::task;
use libparsec_platform_native::Transport;
use libparsec_protocol::{
    AuthenticatedClientHandshakeStalled, Handshake, HandshakeResult, ServerHandshakeStalled,
};
use rstest::rstest;
use tests_fixtures::{alice, Device};

#[rstest]
fn test_transport(alice: &Device) {
    std::env::set_var("VERSION", "v2.14.0+dev");
    smol::block_on(async {
        let addr = "127.0.0.1:0";
        let listener = TcpListener::bind(addr).await.unwrap();
        let server_addr = listener.local_addr().unwrap();
        let time_provider = alice.time_provider.clone();

        let verify_key = alice.verify_key();
        task::spawn(async move {
            let (server_stream, _) = listener.accept().await.unwrap();
            let mut server = Transport::init_for_server(server_stream).await;

            let sh = ServerHandshakeStalled::default()
                .build_challenge_req(time_provider.now())
                .unwrap();

            server.send(sh.raw.clone()).await.unwrap();

            let raw = server.recv().await.unwrap();

            let sh = sh
                .process_answer_req(&raw)
                .unwrap()
                .build_result_req(Some(verify_key))
                .unwrap();

            server.send(sh.raw).await.unwrap();
        });

        let mut client =
            Transport::init_for_client("ws://".to_string() + &server_addr.to_string()).await;

        let raw = client.recv().await.unwrap();

        let ch = AuthenticatedClientHandshakeStalled::new(
            alice.organization_id().clone(),
            alice.device_id.clone(),
            alice.signing_key.clone(),
            alice.root_verify_key().clone(),
            alice.time_provider.now(),
        )
        .process_challenge_req(&raw)
        .unwrap();

        client.send(ch.raw).await.unwrap();

        let raw = client.recv().await.unwrap();

        assert_eq!(
            raw,
            Handshake::Result {
                result: HandshakeResult::Ok,
                help: None
            }
            .dump()
            .unwrap()
        );
    })
}
