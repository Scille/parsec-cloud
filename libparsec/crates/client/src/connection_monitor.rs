// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{ops::ControlFlow, sync::Arc};

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError, SSEResponseOrMissedEvents};
use libparsec_platform_async::{spawn, JoinHandle};
use libparsec_protocol::authenticated_cmds::v4::events_listen::{APIEvent, Rep, Req};

use crate::event_bus::*;

pub struct ConnectionMonitor {
    worker: JoinHandle<()>,
}

fn dispatch_api_event(event: APIEvent, event_bus: &EventBus) {
    match event {
        APIEvent::Pinged { .. } => (),
        APIEvent::CertificatesUpdated { index } => {
            let event = EventCertificatesUpdated { index };
            event_bus.send(&event);
        }
        APIEvent::MessageReceived { index } => {
            let event = EventMessageReceived { index };
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

#[derive(PartialEq, Eq)]
enum ConnectionState {
    Offline,
    Online,
}

impl ConnectionMonitor {
    pub async fn start(cmds: Arc<AuthenticatedCmds>, event_bus: EventBus) -> Self {
        let worker = spawn(async move {
            let mut state = ConnectionState::Offline;
            let mut stream = cmds.start_sse::<Req>();

            loop {
                match stream.next().await {
                    Ok(SSEResponseOrMissedEvents::MissedEvents) => {
                        event_bus.send(&EventMissedServerEvents);
                    }
                    Ok(SSEResponseOrMissedEvents::Response(rep)) => {
                        handle_sse_response(&mut state, &event_bus, rep);
                    }
                    Err(err) => {
                        if handle_sse_error(&mut state, &event_bus, err).is_break() {
                            return;
                        }
                    }
                }
            }
        });

        Self { worker }
    }
}

fn handle_sse_error(
    state: &mut ConnectionState,
    event_bus: &EventBus,
    err: ConnectionError,
) -> ControlFlow<()> {
    if &ConnectionState::Online == state {
        event_bus.send(&EventOffline);
        *state = ConnectionState::Offline;
    }

    match err {
        // The only legit error is if we couldn't reach the server...
        ConnectionError::NoResponse(_) => ControlFlow::Continue(()),

        // ...otherwise the server rejected us, hence there is no use
        // retrying to connect and we just stop this coroutine
        ConnectionError::ExpiredOrganization => {
            event_bus.send(&EventExpiredOrganization);
            ControlFlow::Break(())
        }
        ConnectionError::RevokedUser => {
            event_bus.send(&EventRevokedUser);
            ControlFlow::Break(())
        }
        ConnectionError::UnsupportedApiVersion {
            api_version,
            supported_api_versions,
        } => {
            let event = EventIncompatibleServer(IncompatibleServerReason::UnsupportedApiVersion {
                api_version,
                supported_api_versions,
            });
            event_bus.send(&event);
            ControlFlow::Break(())
        }
        err => {
            let event = EventIncompatibleServer(IncompatibleServerReason::Unexpected(err.into()));
            event_bus.send(&event);
            ControlFlow::Break(())
        }
    }
}

fn handle_sse_response(state: &mut ConnectionState, event_bus: &EventBus, rep: Rep) {
    if &ConnectionState::Offline == state {
        event_bus.send(&EventOnline);
        *state = ConnectionState::Online;
    }

    match rep {
        Rep::Ok(event) => dispatch_api_event(event, event_bus),
        // Unexpected error status
        rep => {
            log::warn!("`events_listen` unexpected error response: {:?}", rep);
        }
    };
}

impl Drop for ConnectionMonitor {
    fn drop(&mut self) {
        self.worker.abort()
    }
}
