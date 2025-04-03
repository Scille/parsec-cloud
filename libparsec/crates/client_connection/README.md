# Libparsec Client Connection

This crate provides support for client connection to the server, from transport
to application (HTTP/RPC) layer.

Client requests are based on [`reqwest`] HTTP Client including proxy
configuration and certificate management.

Specifics regarding RPC commands, such as handling headers and status codes,
are implemented by command family. In particular, SSE support is included for
the `authenticated_cmds` family.

[`reqwest`]: https://docs.rs/reqwest/latest/reqwest/
