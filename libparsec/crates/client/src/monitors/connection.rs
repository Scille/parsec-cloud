// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/// Client-server communication is divided into two parts:
/// - Client commands sent to the server through an RPC-like mechanism.
/// - Server events sent to the client through a Server-Sent Events (SSE) mechanism.
///
/// In practice we have a single SSE connection handled by the connection monitor (which
/// is implemented here !), and multiple concurrent RPC commands depending on what is
/// going on in the client (e.g. data synchronization, reading a file not in cache, etc.).
///
/// On top of that, low-level details about the server connection are handled by the
/// reqwest library (see `libparsec_client_connection` implementation), hence we have no
/// guarantee on the number of physical TCP connections being used to handle RPC and SSE.
///
/// However what we know is 1) RPC and SSE connect to the same server and 2) server
/// and client both support HTTP/2. Hence we can expect all connections to be multiplexed
/// over a single physical TCP connection.
///
/// With this in mind, the choice has been to have the connection-related events only
/// fired by the connection monitor: if a RPC command fails due to server disconnection,
/// it is very likely the SSE connection will also fail right away.
///
/// In theory, we should have instead a centralized handling of connection errors (so
/// that any error triggers the corresponding event, and we couldn't have a successful
/// RPC command while the system is considered offline due to a failed SSE connection).
/// However it is cumbersome to implement a middleware wrapper in Rust (we would have
/// to wrap `AuthenticatedCmds` and `SSEStream` with their weird traits, and find them
/// a good name not to confuse with the actual `AuthenticatedCmds` and `SSEStream`).
/// Another solution would be that `AuthenticatedCmds` has an `on_error` callback, but
/// this would make the code more fragile given it's easy to forget to call it by just
/// using the ? operator in `libparsec_client_connection` implementation.
///
/// So we choose (for the moment at least !) the pragmatic approach of considering
/// SSE errors are the only important ones, so that only the connection monitor have
/// to deal with events.
use std::{collections::HashMap, sync::Arc, time::Duration};

use libparsec_client_connection::{
    AuthenticatedCmds, ConnectionError, RateLimiter, SSEEvent, SSEResponseOrMissedEvents, SSEStream,
};
use libparsec_platform_async::{
    channel, pretend_future_is_send_on_web, select2_biased, sleep, stream::StreamExt,
};
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_protocol::authenticated_cmds::latest::events_listen::{APIEvent, Rep, Req};

use crate::event_bus::*;

use super::Monitor;
use crate::event_bus::{EventBus, EventMissedServerEvents, EventRealmVlobUpdated};

const CONNECTION_MONITOR_NAME: &str = "connection";

/// Connection monitor must be the last monitor to start !
pub(crate) async fn start_connection_monitor(
    cmds: Arc<AuthenticatedCmds>,
    event_bus: EventBus,
) -> Monitor {
    let task_future = {
        let task_future = task_future_factory(cmds, event_bus.clone());
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(event_bus, CONNECTION_MONITOR_NAME, None, task_future, None).await
}

fn dispatch_api_event(event: APIEvent, event_bus: &EventBus) {
    match event {
        APIEvent::Pinged { .. } => (),
        APIEvent::OrganizationConfig {
            active_users_limit,
            user_profile_outsider_allowed,
            sse_keepalive_seconds: _,
        } => {
            let event = EventServerConfigNotified {
                active_users_limit,
                user_profile_outsider_allowed,
            };
            event_bus.send(&event);
        }
        APIEvent::Invitation {
            token,
            invitation_status,
        } => {
            let event = EventInvitationChanged {
                token,
                status: invitation_status,
            };
            event_bus.send(&event);
        }
        APIEvent::GreetingAttemptReady {
            token,
            greeting_attempt,
        } => {
            let event = EventGreetingAttemptReady {
                token,
                greeting_attempt,
            };
            event_bus.send(&event);
        }
        APIEvent::GreetingAttemptCancelled {
            token,
            greeting_attempt,
        } => {
            let event = EventGreetingAttemptCancelled {
                token,
                greeting_attempt,
            };
            event_bus.send(&event);
        }
        APIEvent::GreetingAttemptJoined {
            token,
            greeting_attempt,
        } => {
            let event = EventGreetingAttemptJoined {
                token,
                greeting_attempt,
            };
            event_bus.send(&event);
        }
        APIEvent::PkiEnrollment => {
            let event = EventPkiEnrollmentUpdated {};
            event_bus.send(&event);
        }
        APIEvent::AsyncEnrollment => {
            let event = EventAsyncEnrollmentUpdated {};
            event_bus.send(&event);
        }
        APIEvent::CommonCertificate { timestamp } => {
            let event = EventCertificatesUpdated {
                last_timestamps: PerTopicLastTimestamps {
                    common: Some(timestamp),
                    realm: HashMap::default(),
                    sequester: None,
                    shamir_recovery: None,
                },
            };
            event_bus.send(&event);
        }
        APIEvent::SequesterCertificate { timestamp } => {
            let event = EventCertificatesUpdated {
                last_timestamps: PerTopicLastTimestamps {
                    common: None,
                    realm: HashMap::default(),
                    sequester: Some(timestamp),
                    shamir_recovery: None,
                },
            };
            event_bus.send(&event);
        }
        APIEvent::ShamirRecoveryCertificate { timestamp } => {
            let event = EventCertificatesUpdated {
                last_timestamps: PerTopicLastTimestamps {
                    common: None,
                    realm: HashMap::default(),
                    sequester: None,
                    shamir_recovery: Some(timestamp),
                },
            };
            event_bus.send(&event);
        }
        APIEvent::RealmCertificate {
            realm_id,
            timestamp,
        } => {
            let event = EventCertificatesUpdated {
                last_timestamps: PerTopicLastTimestamps {
                    common: None,
                    realm: HashMap::from([(realm_id, timestamp)]),
                    sequester: None,
                    shamir_recovery: None,
                },
            };
            event_bus.send(&event);
        }
        APIEvent::Vlob {
            author,
            blob,
            last_common_certificate_timestamp,
            last_realm_certificate_timestamp,
            realm_id,
            timestamp,
            version,
            vlob_id,
        } => {
            let event = EventRealmVlobUpdated {
                author,
                blob,
                last_common_certificate_timestamp,
                last_realm_certificate_timestamp,
                realm_id,
                timestamp,
                version,
                vlob_id,
            };
            event_bus.send(&event);
        }
    }
}

enum HandleSseErrorOutcome {
    WaitForOnline,
    WaitForTosAccepted,
    StopMonitor,
}

fn handle_sse_error(
    state: &mut ConnectionState,
    event_bus: &EventBus,
    err: ConnectionError,
) -> HandleSseErrorOutcome {
    if &ConnectionState::Online == state {
        event_bus.send(&EventOffline);
        *state = ConnectionState::Offline;
    }

    match err {
        // Legit errors...

        // We couldn't reach the server
        ConnectionError::NoResponse(_) => HandleSseErrorOutcome::WaitForOnline,
        // We must accept the TOS before being able to connect
        ConnectionError::UserMustAcceptTos => {
            event_bus.send(&EventMustAcceptTos);
            HandleSseErrorOutcome::WaitForTosAccepted
        }

        // ...otherwise the server rejected us, hence there is no use
        // retrying to connect and we just stop this coroutine
        _ => {
            match err {
                ConnectionError::NoResponse(_) | ConnectionError::UserMustAcceptTos => {
                    unreachable!()
                }

                ConnectionError::OrganizationNotFound => event_bus.send(&EventOrganizationNotFound),
                ConnectionError::ExpiredOrganization => event_bus.send(&EventExpiredOrganization),
                ConnectionError::InvitationAlreadyUsedOrDeleted => {
                    event_bus.send(&EventInvitationAlreadyUsedOrDeleted)
                }
                ConnectionError::RevokedUser => event_bus.send(&EventRevokedSelfUser),
                ConnectionError::FrozenUser => event_bus.send(&EventFrozenSelfUser),
                ConnectionError::WebClientNotAllowedByOrganization => {
                    event_bus.send(&EventWebClientNotAllowedByOrganization)
                }
                ConnectionError::AuthenticationTokenExpired => {
                    event_bus.send(&EventClientErrorResponse {
                        error_type: ClientErrorResponseType::MissingAuthenticationInfo,
                    })
                }
                ConnectionError::MissingAuthenticationInfo => {
                    event_bus.send(&EventClientErrorResponse {
                        error_type: ClientErrorResponseType::MissingAuthenticationInfo,
                    })
                }
                ConnectionError::BadAuthenticationInfo => {
                    event_bus.send(&EventClientErrorResponse {
                        error_type: ClientErrorResponseType::BadAuthenticationInfo,
                    })
                }
                ConnectionError::BadAcceptType => event_bus.send(&EventClientErrorResponse {
                    error_type: ClientErrorResponseType::BadAcceptType,
                }),
                ConnectionError::BadContent => event_bus.send(&EventClientErrorResponse {
                    error_type: ClientErrorResponseType::BadContent,
                }),
                ConnectionError::UnsupportedApiVersion {
                    api_version,
                    supported_api_versions,
                } => event_bus.send(&EventIncompatibleServer {
                    api_version,
                    supported_api_version: supported_api_versions,
                }),
                ConnectionError::MissingSupportedApiVersions { api_version } => {
                    event_bus.send(&EventIncompatibleServer {
                        api_version,
                        supported_api_version: Vec::new(),
                    })
                }
                ConnectionError::InvalidResponseStatus(status_code) => {
                    event_bus.send(&EventServerInvalidResponseStatus {
                        status_code: status_code.to_string(),
                    })
                }
                ConnectionError::InvalidResponseContent(protocol_decode_error) => {
                    event_bus.send(&EventServerInvalidResponseContent {
                        protocol_decode_error: protocol_decode_error.to_string(),
                    })
                }
            };
            HandleSseErrorOutcome::StopMonitor
        }
    }
}

#[derive(PartialEq, Eq)]
enum ConnectionState {
    Offline,
    Online,
}

#[derive(Debug, Default)]
struct KeepaliveTracking {
    sse_keepalive: Option<Duration>,
}

impl KeepaliveTracking {
    // Add 20% to the keepalive duration to avoid a false positive on
    // the timeout if the server is a bit late to send the next event.
    const TIMEOUT_FACTOR: f32 = 1.2;

    async fn next_event(
        &mut self,
        stream: &mut SSEStream<Req>,
    ) -> Option<Result<SSEEvent<Req>, ConnectionError>> {
        let result = match self.sse_keepalive {
            // No keepalive duration, just wait for the next event
            None => stream.next().await,
            // We have a keepalive duration, use it as a timeout
            Some(keepalive) => {
                let timeout = keepalive.mul_f32(Self::TIMEOUT_FACTOR);
                select2_biased!(
                    res = stream.next() => res,
                    // In case of a timeout, return a `ConnectionError::NoResponse`
                    // to get the same behavior as if the server was unreachable.
                    _ = sleep(timeout) => {
                        log::info!("Keepalive timeout after {timeout:?}");
                        Some(Err(ConnectionError::NoResponse(None)))
                    }
                )
            }
        };
        // Update the keepalive duration if we received an `OrganizationConfig` event
        if let Some(Ok(SSEEvent {
            message:
                SSEResponseOrMissedEvents::Response(Rep::Ok(APIEvent::OrganizationConfig {
                    sse_keepalive_seconds,
                    ..
                })),
            ..
        })) = result
        {
            self.sse_keepalive = sse_keepalive_seconds.map(|x| Duration::from_secs(x.get()));
            match self.sse_keepalive {
                Some(keepalive) => log::info!(
                    "Set expected keepalive duration: {} seconds",
                    keepalive.as_secs()
                ),
                None => log::info!("Unset expected keepalive duration"),
            }
        }
        result
    }
}

async fn task_future_factory(cmds: Arc<AuthenticatedCmds>, event_bus: EventBus) {
    let mut state = ConnectionState::Offline;
    let mut keepalive_tracking = KeepaliveTracking::default();
    let mut last_event_id = None;
    // Backoff is used to wait longer and longer after each failed connection
    // the server.
    let mut backoff = RateLimiter::new();
    // This channel is use to reset the backoff system, typically when some outside
    // event requires to retry the connection right away.
    let (retry_now_tx, retry_now_rx) = channel::bounded::<()>(1);

    let _event_lifetime = {
        event_bus.connect(move |_: &EventShouldRetryConnectionNow| {
            // If the channel already contains something it means we have nothing
            // to do: the backoff is already planned to be reset on the next wait.
            let _ = retry_now_tx.send(());
        })
    };

    // As last monitor to start, we send this event to wake up all the other monitors
    event_bus.send(&EventMissedServerEvents);

    'connection_loop: loop {
        // Note we listen on `retry_now_rx`
        let should_reset_backoff = select2_biased!(
            _ = backoff.wait() => false,
            _ = retry_now_rx.recv_async() => true,
        );
        if should_reset_backoff {
            backoff.reset();
            // Under normal circumstances the backoff first does a wait, then do
            // the actual attempt.
            // However here we have reset the backoff, so if we do the actual attempt
            // without correction, we will have a decorrelation with the wait (e.g. we
            // will have a 0s wait on the second attempt instead of 1s).
            backoff.set_attempt(1);
        }

        let mut stream = match cmds.start_sse::<Req>(last_event_id.clone()).await {
            Ok(stream) => stream,
            Err(err) => match handle_sse_error(&mut state, &event_bus, err) {
                HandleSseErrorOutcome::WaitForOnline
                | HandleSseErrorOutcome::WaitForTosAccepted => continue 'connection_loop,
                HandleSseErrorOutcome::StopMonitor => return,
            },
        };

        backoff.reset();

        while let Some(res) = keepalive_tracking.next_event(&mut stream).await {
            match res {
                Ok(event) => {
                    if let Some(retry) = event.retry {
                        backoff.set_desired_duration(retry);
                    }
                    if let Some(event_id) = event.id {
                        last_event_id.replace(event_id);
                    }
                    match event.message {
                        SSEResponseOrMissedEvents::MissedEvents => {
                            event_bus.send(&EventMissedServerEvents);
                        }

                        SSEResponseOrMissedEvents::Response(rep) => {
                            if ConnectionState::Offline == state {
                                event_bus.send(&EventOnline);
                                state = ConnectionState::Online;
                            }

                            match rep {
                                Rep::Ok(event) => dispatch_api_event(event, &event_bus),
                                // Unexpected error status
                                rep => {
                                    log::warn!(
                                        "`events_listen` unexpected error response: {rep:?}"
                                    );
                                }
                            };
                        }

                        SSEResponseOrMissedEvents::Empty => (),
                    }
                }
                Err(err) => match handle_sse_error(&mut state, &event_bus, err) {
                    HandleSseErrorOutcome::WaitForOnline
                    | HandleSseErrorOutcome::WaitForTosAccepted => continue 'connection_loop,
                    HandleSseErrorOutcome::StopMonitor => return,
                },
            }
        }
    }
}
