// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use async_std::net::TcpListener;
use async_std::task;
use libparsec_platform_native::Transport;
use rstest::rstest;

#[rstest]
fn test_transport() {
    smol::block_on(async {
        let addr = "127.0.0.1:8080";
        let listener = TcpListener::bind(addr).await.unwrap();

        task::spawn(async move {
            let (server_stream, _) = listener.accept().await.unwrap();
            let mut server = Transport::init_for_server(server_stream).await;

            let data = server.recv().await.unwrap();
            assert_eq!(data, b"foobar");

            server.send(b"foobar".to_vec()).await.unwrap();
        });

        let mut client = Transport::init_for_client("ws://".to_string() + addr).await;

        client.send(b"foobar".to_vec()).await.unwrap();

        let data = client.recv().await.unwrap();
        assert_eq!(data, b"foobar");
    })
}
