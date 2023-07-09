// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// Here we expose only a subset of the client events, as some of them are for
// internal use only.

pub enum ClientEvents {
    // Dummy event for tests only
    Ping(libparsec_client::EventPing),
    // Events related to server connection
    Offline(libparsec_client::EventOffline),
    Online(libparsec_client::EventOnline),
    MissedServerEvents(libparsec_client::EventMissedServerEvents),
    TooMuchDriftWithServerClock(libparsec_client::EventTooMuchDriftWithServerClock),
    ExpiredOrganization(libparsec_client::EventExpiredOrganization),
    RevokedUser(libparsec_client::EventRevokedUser),
    IncompatibleServer(libparsec_client::EventIncompatibleServer),
    // Events related to ops
    InvalidMessage(libparsec_client::EventInvalidMessage),
    // Events related to monitors
    CertificatesMonitorCrashed(libparsec_client::EventCertificatesMonitorCrashed),
    InvalidCertificate(libparsec_client::EventInvalidCertificate),
    // TODO
    // // Logs events
    // LogInfo,
    // LogWarning,
    // LogError,
}
