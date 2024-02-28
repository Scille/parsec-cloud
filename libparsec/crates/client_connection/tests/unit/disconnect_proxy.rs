// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::io;

use libparsec_types::prelude::*;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot},
};

pub struct DisconnectProxyHandle {
    join_handle: tokio::task::JoinHandle<io::Result<()>>,
    port: u16,
    notify_disconnect: mpsc::Sender<Event>,
}

impl DisconnectProxyHandle {
    pub fn to_backend_addr(&self) -> ParsecAddr {
        ParsecAddr::new("localhost".into(), Some(self.port), false)
    }

    pub async fn disconnect(&self) {
        self.notify_disconnect
            .send(Event::Disconnect)
            .await
            .unwrap()
    }

    pub async fn close(self) -> io::Result<()> {
        self.notify_disconnect.send(Event::Close).await.unwrap();
        self.join_handle.await.unwrap()
    }
}

enum Event {
    Close,
    Disconnect,
}

pub async fn spawn(server_addr: ParsecAddr) -> io::Result<DisconnectProxyHandle> {
    let listener = TcpListener::bind(("localhost", 0)).await?;
    let port = listener.local_addr()?.port();
    let (tx_server_ready, rx_server_ready) = oneshot::channel();
    let (tx_disconnect, rx_disconnect) = mpsc::channel(1);
    let server = DisconnectProxyServer {
        listener,
        server_addr,
        notify_disconnect: rx_disconnect,
    };
    let handle = tokio::task::spawn(server.run(tx_server_ready));

    match rx_server_ready.await {
        Ok(_) => Ok(DisconnectProxyHandle {
            join_handle: handle,
            port,
            notify_disconnect: tx_disconnect,
        }),
        Err(_) => {
            let expect_err = handle
                .await
                .expect("Failed to join the proxy server thread")
                .expect_err("The server should have failed");
            Err(expect_err)
        }
    }
}

struct DisconnectProxyServer {
    listener: TcpListener,
    server_addr: ParsecAddr,
    notify_disconnect: mpsc::Receiver<Event>,
}

impl DisconnectProxyServer {
    async fn run(mut self, ready: oneshot::Sender<()>) -> io::Result<()> {
        let url = self.server_addr.to_http_url(None);
        let server_addr = extract_url_domain_and_port(&url).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot extract domain and port on url `{url}`"),
            )
        })?;
        ready
            .send(())
            .expect("The other side is waiting for this message");

        loop {
            // We wait for incoming connection from the client or until we are notified.
            let (mut client_socket, _client_addr) = tokio::select! {
                res = self.listener.accept() => {
                    res
                }
                res = self.notify_disconnect.recv() => {
                    match res {
                        Some(Event::Disconnect) => {
                            continue
                        },
                        Some(Event::Close) | None => {
                            break
                        },
                    }
                }
            }?;

            // We connect to the actual server.
            let mut server_socket = match TcpStream::connect(server_addr).await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to connect to the server: {e}");
                    drop(client_socket);
                    continue;
                }
            };

            // Now we stream the data from the client/server to server/client or until we receive a
            // message from `notify_disconnect`.
            tokio::select! {
                _copy_res = tokio::io::copy_bidirectional(&mut client_socket, &mut server_socket) => {
                }
                res = self.notify_disconnect.recv() => {
                    drop(client_socket);
                    drop(server_socket);
                    match res {
                        Some(Event::Disconnect) => {
                            continue;
                        },
                        Some(Event::Close) | None => {
                            break
                        },
                    }
                }
            }
        }
        Ok(())
    }
}

fn extract_url_domain_and_port(u: &Url) -> Option<(&str, u16)> {
    let domain = u.host_str()?;

    let port = u
        .port_or_known_default()
        .unwrap_or_else(|| panic!("Unsupported scheme: {}", u.scheme()));

    Some((domain, port))
}
