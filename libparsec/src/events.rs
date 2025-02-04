// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Here we expose only a subset of the client events, as some of them are for
// internal use only.

use std::sync::Arc;

use libparsec_client::EventBusConnectionLifetime;
use libparsec_types::prelude::*;

use crate::handle::Handle;

#[derive(Debug)]
pub enum ClientEvent {
    // Dummy event for tests only
    Ping {
        ping: String,
    },

    // Server connection
    Offline,
    Online,

    // Server-notified changes
    ServerConfigChanged,

    WorkspacesSelfListChanged,
    WorkspaceLocallyCreated,
    WorkspaceWatchedEntryChanged {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    WorkspaceOpsOutboundSyncStarted {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    WorkspaceOpsOutboundSyncProgress {
        realm_id: VlobID,
        entry_id: VlobID,
        blocks: IndexInt,
        block_index: IndexInt,
        blocksize: SizeInt,
    },
    WorkspaceOpsOutboundSyncAborted {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    WorkspaceOpsOutboundSyncDone {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    WorkspaceOpsInboundSyncDone {
        realm_id: VlobID,
        entry_id: VlobID,
    },

    InvitationChanged {
        token: InvitationToken,
        status: InvitationStatus,
    },

    GreetingAttemptReady {
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
    },

    GreetingAttemptCancelled {
        token: InvitationToken,
        greeting_attempt: GreetingAttemptID,
    },

    // Error from server & server-provided data
    ExpiredOrganization,
    RevokedSelfUser,
    MustAcceptTos,
    TooMuchDriftWithServerClock {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: Float,
        ballpark_client_late_offset: Float,
    },
    IncompatibleServer {
        detail: String,
    },
    // TODO
    // InvalidKeysBundle {
    //     detail: String,
    // },
    // TODO
    // InvalidCertificate {
    //     detail: String,
    // },
    // TODO
    // InvalidManifest {
    //     detail: String,
    // },
}

pub(crate) struct OnEventCallbackPlugged {
    pub event_bus: libparsec_client::EventBus,

    _ping: EventBusConnectionLifetime<libparsec_client::EventPing>,
    _offline: EventBusConnectionLifetime<libparsec_client::EventOffline>,
    _online: EventBusConnectionLifetime<libparsec_client::EventOnline>,
    _server_config_changed: EventBusConnectionLifetime<libparsec_client::EventServerConfigChanged>,
    _workspace_self_access_changed:
        EventBusConnectionLifetime<libparsec_client::EventWorkspacesSelfListChanged>,
    _workspace_locally_created:
        EventBusConnectionLifetime<libparsec_client::EventWorkspaceLocallyCreated>,
    _workspace_watched_entry_changed:
        EventBusConnectionLifetime<libparsec_client::EventWorkspaceWatchedEntryChanged>,
    _workspace_ops_outbound_sync_started:
        EventBusConnectionLifetime<libparsec_client::EventWorkspaceOpsOutboundSyncStarted>,
    _workspace_ops_outbound_sync_progress:
        EventBusConnectionLifetime<libparsec_client::EventWorkspaceOpsOutboundSyncProgress>,
    _workspace_ops_outbound_sync_aborted:
        EventBusConnectionLifetime<libparsec_client::EventWorkspaceOpsOutboundSyncAborted>,
    _workspace_ops_outbound_sync_done:
        EventBusConnectionLifetime<libparsec_client::EventWorkspaceOpsOutboundSyncDone>,
    _workspace_ops_inbound_sync_done:
        EventBusConnectionLifetime<libparsec_client::EventWorkspaceOpsInboundSyncDone>,
    _invitation_changed: EventBusConnectionLifetime<libparsec_client::EventInvitationChanged>,
    _greeting_attempt_ready:
        EventBusConnectionLifetime<libparsec_client::EventGreetingAttemptReady>,
    _greeting_attempt_cancelled:
        EventBusConnectionLifetime<libparsec_client::EventGreetingAttemptCancelled>,
    _expired_organization: EventBusConnectionLifetime<libparsec_client::EventExpiredOrganization>,
    _revoked_self_user: EventBusConnectionLifetime<libparsec_client::EventRevokedSelfUser>,
    _must_accept_tos: EventBusConnectionLifetime<libparsec_client::EventMustAcceptTos>,
    _too_much_drift_with_server_clock:
        EventBusConnectionLifetime<libparsec_client::EventTooMuchDriftWithServerClock>,
    _incompatible_server: EventBusConnectionLifetime<libparsec_client::EventIncompatibleServer>,
    // _invalid_keys_bundle: EventBusConnectionLifetime<libparsec_client::EventInvalidKeysBundle>,
    // _invalid_certificate: EventBusConnectionLifetime<libparsec_client::EventInvalidCertificate>,
    // _invalid_manifest: EventBusConnectionLifetime<libparsec_client::EventInvalidManifest>,
}

impl OnEventCallbackPlugged {
    pub fn new(
        handle: Handle,
        // Access to the event bus is done through this callback.
        // Ad-hoc code should be added to the binding system to handle this (hence
        // why this is passed as a parameter instead of as part of `ClientConfig`:
        // we can have a simple `if func_name == "client_login"` that does a special
        // cooking of it last param.
        #[cfg(not(target_arch = "wasm32"))] on_event_callback: Arc<
            dyn Fn(Handle, ClientEvent) + Send + Sync,
        >,
        // On web we run on the JS runtime which is mono-threaded, hence everything is !Send
        #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(Handle, ClientEvent)>,
    ) -> Self {
        // SAFETY: `EventBus` requires callback to be `Send`, however on web the runtime
        // is strictly single-threaded and callback must be `!Send`.
        // So here we are going "trust me bro" considering it is fine to lie about
        // send'ness of the callback given it will never leave the current thread.
        #[cfg(target_arch = "wasm32")]
        let on_event_callback = unsafe {
            std::mem::transmute::<
                Arc<dyn Fn(Handle, ClientEvent)>,
                Arc<dyn Fn(Handle, ClientEvent) + Send + Sync>,
            >(on_event_callback)
        };

        let event_bus = libparsec_client::EventBus::default();

        // Connect events

        let ping = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventPing| {
                (on_event_callback)(
                    handle,
                    ClientEvent::Ping {
                        ping: e.ping.clone(),
                    },
                );
            })
        };

        let offline = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventOffline| {
                (on_event_callback)(handle, ClientEvent::Offline);
            })
        };
        let online = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventOnline| {
                (on_event_callback)(handle, ClientEvent::Online);
            })
        };

        let server_config_changed = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventServerConfigChanged| {
                (on_event_callback)(handle, ClientEvent::ServerConfigChanged);
            })
        };
        let workspace_self_access_changed = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |_: &libparsec_client::EventWorkspacesSelfListChanged| {
                    (on_event_callback)(handle, ClientEvent::WorkspacesSelfListChanged);
                },
            )
        };
        let workspace_locally_created = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventWorkspaceLocallyCreated| {
                (on_event_callback)(handle, ClientEvent::WorkspaceLocallyCreated);
            })
        };
        let workspace_watched_entry_changed = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventWorkspaceWatchedEntryChanged| {
                    (on_event_callback)(
                        handle,
                        ClientEvent::WorkspaceWatchedEntryChanged {
                            realm_id: e.realm_id,
                            entry_id: e.entry_id,
                        },
                    );
                },
            )
        };
        let workspace_ops_outbound_sync_started = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventWorkspaceOpsOutboundSyncStarted| {
                    (on_event_callback)(
                        handle,
                        ClientEvent::WorkspaceOpsOutboundSyncStarted {
                            realm_id: e.realm_id,
                            entry_id: e.entry_id,
                        },
                    );
                },
            )
        };
        let workspace_ops_outbound_sync_progress = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventWorkspaceOpsOutboundSyncProgress| {
                    (on_event_callback)(
                        handle,
                        ClientEvent::WorkspaceOpsOutboundSyncProgress {
                            realm_id: e.realm_id,
                            entry_id: e.entry_id,
                            blocks: e.blocks,
                            block_index: e.block_index,
                            blocksize: e.blocksize,
                        },
                    );
                },
            )
        };
        let workspace_ops_outbound_sync_aborted = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventWorkspaceOpsOutboundSyncAborted| {
                    (on_event_callback)(
                        handle,
                        ClientEvent::WorkspaceOpsOutboundSyncAborted {
                            realm_id: e.realm_id,
                            entry_id: e.entry_id,
                        },
                    );
                },
            )
        };
        let workspace_ops_outbound_sync_done = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventWorkspaceOpsOutboundSyncDone| {
                    (on_event_callback)(
                        handle,
                        ClientEvent::WorkspaceOpsOutboundSyncDone {
                            realm_id: e.realm_id,
                            entry_id: e.entry_id,
                        },
                    );
                },
            )
        };
        let workspace_ops_inbound_sync_done = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventWorkspaceOpsInboundSyncDone| {
                    (on_event_callback)(
                        handle,
                        ClientEvent::WorkspaceOpsInboundSyncDone {
                            realm_id: e.realm_id,
                            entry_id: e.entry_id,
                        },
                    );
                },
            )
        };
        let invitation_changed = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventInvitationChanged| {
                (on_event_callback)(
                    handle,
                    ClientEvent::InvitationChanged {
                        token: e.token,
                        status: e.status,
                    },
                );
            })
        };
        let greeting_attempt_ready = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventGreetingAttemptReady| {
                (on_event_callback)(
                    handle,
                    ClientEvent::GreetingAttemptReady {
                        token: e.token,
                        greeting_attempt: e.greeting_attempt,
                    },
                );
            })
        };
        let greeting_attempt_cancelled = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventGreetingAttemptCancelled| {
                (on_event_callback)(
                    handle,
                    ClientEvent::GreetingAttemptCancelled {
                        token: e.token,
                        greeting_attempt: e.greeting_attempt,
                    },
                );
            })
        };
        let expired_organization = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventExpiredOrganization| {
                (on_event_callback)(handle, ClientEvent::ExpiredOrganization);
            })
        };
        let revoked_self_user = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventRevokedSelfUser| {
                (on_event_callback)(handle, ClientEvent::RevokedSelfUser);
            })
        };
        let must_accept_tos = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventMustAcceptTos| {
                (on_event_callback)(handle, ClientEvent::MustAcceptTos);
            })
        };
        let too_much_drift_with_server_clock = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventTooMuchDriftWithServerClock| {
                    (on_event_callback)(
                        handle,
                        ClientEvent::TooMuchDriftWithServerClock {
                            ballpark_client_early_offset: e.ballpark_client_early_offset,
                            ballpark_client_late_offset: e.ballpark_client_late_offset,
                            client_timestamp: e.client_timestamp,
                            server_timestamp: e.server_timestamp,
                        },
                    );
                },
            )
        };
        let incompatible_server = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventIncompatibleServer| {
                (on_event_callback)(
                    handle,
                    ClientEvent::IncompatibleServer {
                        detail: e.0.to_string(),
                    },
                );
            })
        };
        // let invalid_keys_bundle = {
        //     let on_event_callback = on_event_callback.clone();
        //     event_bus.connect(move |e: &libparsec_client::EventInvalidKeysBundle| {
        //         (on_event_callback)(ClientEvent::InvalidKeysBundle);
        //     })
        // };
        // let invalid_certificate = {
        //     let on_event_callback = on_event_callback.clone();
        //     event_bus.connect(move |e: &libparsec_client::EventInvalidCertificate| {
        //         (on_event_callback)(ClientEvent::InvalidCertificate {
        //             detail: e.0.to_string(),
        //         });
        //     })
        // };
        // let invalid_manifest = {
        //     let on_event_callback = on_event_callback.clone();
        //     event_bus.connect(move |e: &libparsec_client::EventInvalidManifest| {
        //         (on_event_callback)(ClientEvent::InvalidManifest);
        //     })
        // };

        Self {
            event_bus,
            _ping: ping,
            _offline: offline,
            _online: online,
            _server_config_changed: server_config_changed,
            _workspace_self_access_changed: workspace_self_access_changed,
            _workspace_locally_created: workspace_locally_created,
            _workspace_watched_entry_changed: workspace_watched_entry_changed,
            _workspace_ops_outbound_sync_started: workspace_ops_outbound_sync_started,
            _workspace_ops_outbound_sync_progress: workspace_ops_outbound_sync_progress,
            _workspace_ops_outbound_sync_aborted: workspace_ops_outbound_sync_aborted,
            _workspace_ops_outbound_sync_done: workspace_ops_outbound_sync_done,
            _workspace_ops_inbound_sync_done: workspace_ops_inbound_sync_done,
            _invitation_changed: invitation_changed,
            _greeting_attempt_ready: greeting_attempt_ready,
            _greeting_attempt_cancelled: greeting_attempt_cancelled,
            _too_much_drift_with_server_clock: too_much_drift_with_server_clock,
            _expired_organization: expired_organization,
            _revoked_self_user: revoked_self_user,
            _must_accept_tos: must_accept_tos,
            _incompatible_server: incompatible_server,
            // _invalid_keys_bundle: invalid_keys_bundle,
            // _invalid_certificate: invalid_certificate,
            // _invalid_manifest: invalid_manifest,
        }
    }
}
