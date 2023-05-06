// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_protocol::authenticated_cmds::v4::events_listen::{APIEvent, Rep, Req};

use crate::event_bus::*;

pub struct ConnectionMonitor {
    worker: tokio::task::JoinHandle<()>,
}

fn dispatch_api_event(event: APIEvent, event_bus: &EventBus) {
    match event {
        APIEvent::Pinged { .. } => (),
        APIEvent::CertificatesUpdated { certificate } => {
            let event = EventCertificatesUpdated { certificate };
            event_bus.send(&event);
        }
        APIEvent::MessageReceived { index, message } => {
            let event = EventMessageReceived { index, message };
            event_bus.send(&event);
        }
        APIEvent::InviteStatusChanged {
            invitation_status,
            token,
        } => {
            let event = EventInviteStatusChanged {
                invitation_status,
                token,
            };
            event_bus.send(&event);
        }
        APIEvent::RealmMaintenanceStarted {
            encryption_revision,
            realm_id,
        } => {
            let event = EventRealmMaintenanceStarted {
                encryption_revision,
                realm_id,
            };
            event_bus.send(&event);
        }
        APIEvent::RealmMaintenanceFinished {
            encryption_revision,
            realm_id,
        } => {
            let event = EventRealmMaintenanceFinished {
                encryption_revision,
                realm_id,
            };
            event_bus.send(&event);
        }
        APIEvent::RealmVlobsUpdated {
            checkpoint,
            realm_id,
            src_id,
            src_version,
        } => {
            let event = EventRealmVlobsUpdated {
                checkpoint,
                realm_id,
                src_id,
                src_version,
            };
            event_bus.send(&event);
        }
        APIEvent::PkiEnrollmentUpdated {} => {
            let event = EventPkiEnrollmentUpdated {};
            event_bus.send(&event);
        }
    }
}

impl ConnectionMonitor {
    pub async fn start(cmds: Arc<AuthenticatedCmds>, event_bus: EventBus) -> Self {
        let worker = tokio::spawn(async move {
            enum ConnectionState {
                Offline,
                Online,
            }
            let mut state = ConnectionState::Offline;
            let mut stream = cmds.start_sse::<Req>();

            loop {
                match stream.next().await {
                    Ok(rep) => {
                        if let ConnectionState::Offline = state {
                            event_bus.send(&EventOnline);
                            state = ConnectionState::Online;
                        }

                        match rep {
                            Rep::Ok(event) => dispatch_api_event(event, &event_bus),
                            // Unexpected error status
                            _ => continue,
                        };
                    }
                    Err(err) => {
                        if let ConnectionState::Online = state {
                            event_bus.send(&EventOffline);
                            state = ConnectionState::Offline;
                        }
                        match err {
                            // The only legic error is if we couldn't reach the server...
                            libparsec_client_connection::CommandError::NoResponse(_) => (),

                            // ...otherwise the server rejected us, hence there is no use
                            // retrying to connect and we just stop this coroutine
                            libparsec_client_connection::CommandError::ExpiredOrganization => {
                                event_bus.send(&EventExpiredOrganization);
                                return;
                            }
                            libparsec_client_connection::CommandError::RevokedUser => {
                                event_bus.send(&EventRevokedUser);
                                return;
                            }
                            libparsec_client_connection::CommandError::UnsupportedApiVersion {
                                api_version,
                                supported_api_versions,
                            } => {
                                let event = EventIncompatibleServer(
                                    IncompatibleServerReason::UnsupportedApiVersion {
                                        api_version,
                                        supported_api_versions,
                                    },
                                );
                                event_bus.send(&event);
                                return;
                            }
                            err => {
                                let event = EventIncompatibleServer(
                                    IncompatibleServerReason::Unexpected(Box::new(err)),
                                );
                                event_bus.send(&event);
                                return;
                            }
                        }
                    }
                }
            }
        });

        Self { worker }
    }
}

impl Drop for ConnectionMonitor {
    fn drop(&mut self) {
        self.worker.abort()
    }
}
