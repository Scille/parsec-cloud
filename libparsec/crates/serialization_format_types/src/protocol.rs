// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/// There is two kinds of protocol family. On the one side, Anonymous, Invited and Authenticated
/// (with Tos being kind of like Authenticated) that are the regular families used to
/// interact between the metadata server and the client. And on the other side,
/// AuthenticatedAccount and AnonymousAccount that are used to store a device key file on
/// a server (typically used to use parsec's web version).
#[derive(Debug, Clone, Copy)]
pub enum ProtocolFamily {
    /// Family used for all requests done by a device
    Authenticated,
    /// Special case for requests done by a device before it has accepted the server's
    /// Terms Of Service (TOS)
    Tos,
    /// Family used for requests done without device (typically organization bootstrap)
    Anonymous,
    /// Family used by an invitation claimer in order to obtain a device
    Invited,
    /// Family used for non-authentication operations at server level.
    /// This is used to create a new Parsec account or get the server configuration.
    AnonymousServer,
    /// Family used for operations authenticated with Parsec account (ex: list organizations for a given account)
    AuthenticatedAccount,
    /// Used only for testing purposes
    #[doc(hidden)]
    Family,
}

pub trait ProtocolRequest<const V: u32> {
    const API_MAJOR_VERSION: u32 = V;
    const FAMILY: ProtocolFamily;
    type Response: for<'de> serde::Deserialize<'de> + std::fmt::Debug;
    type DumpError: std::fmt::Debug;

    fn api_dump(&self) -> Result<Vec<u8>, Self::DumpError>;

    fn api_load_response(buf: &[u8]) -> Result<Self::Response, rmp_serde::decode::Error>;
}
