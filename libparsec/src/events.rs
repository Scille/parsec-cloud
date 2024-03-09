// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Here we expose only a subset of the client events, as some of them are for
// internal use only.

use std::sync::Arc;

use libparsec_client::EventBusConnectionLifetime;
use libparsec_types::prelude::*;

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
    // TODO
    // WorkspaceSelfRoleChanged {
    //     workspace_id: VlobID,
    // },
    // TODO
    // WorkspaceEntryChanged {
    //     workspace_id: VlobID,
    //     entry_id: VlobID,
    // },
    InvitationChanged {
        token: InvitationToken,
        status: InvitationStatus,
    },
    // Error from server & server-provided data
    // TODO
    // ExpiredOrganization,
    // TODO
    // RevokedSelfUser,
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
    // _workspace_self_role_changed: EventBusConnectionLifetime<libparsec_client::EventWorkspaceSelfRoleChanged>,
    // _workspace_entry_changed: EventBusConnectionLifetime<libparsec_client::EventWorkspaceEntryChanged>,
    _invitation_changed: EventBusConnectionLifetime<libparsec_client::EventInvitationChanged>,
    // _expired_organization: EventBusConnectionLifetime<libparsec_client::EventExpiredOrganization>,
    // _revoked_user: EventBusConnectionLifetime<libparsec_client::EventRevokedSelfUser>,
    _too_much_drift_with_server_clock:
        EventBusConnectionLifetime<libparsec_client::EventTooMuchDriftWithServerClock>,
    _incompatible_server: EventBusConnectionLifetime<libparsec_client::EventIncompatibleServer>,
    // _invalid_keys_bundle: EventBusConnectionLifetime<libparsec_client::EventInvalidKeysBundle>,
    // _invalid_certificate: EventBusConnectionLifetime<libparsec_client::EventInvalidCertificate>,
    // _invalid_manifest: EventBusConnectionLifetime<libparsec_client::EventInvalidManifest>,
}

impl OnEventCallbackPlugged {
    pub fn new(
        // Access to the event bus is done through this callback.
        // Ad-hoc code should be added to the binding system to handle this (hence
        // why this is passed as a parameter instead of as part of `ClientConfig`:
        // we can have a simple `if func_name == "client_login"` that does a special
        // cooking of it last param.
        #[cfg(not(target_arch = "wasm32"))] on_event_callback: Arc<
            dyn Fn(ClientEvent) + Send + Sync,
        >,
        // On web we run on the JS runtime which is mono-threaded, hence everything is !Send
        #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(ClientEvent)>,
    ) -> Self {
        // SAFETY: `EventBus` requires callback to be `Send`, however on web the runtime
        // is strictly single-threaded and callback must be `!Send`.
        // So here we are going "trust me bro" considering it is fine to lie about
        // send'ness of the callback given it will never leave the current thread.
        #[cfg(target_arch = "wasm32")]
        let on_event_callback = unsafe {
            std::mem::transmute::<Arc<dyn Fn(ClientEvent)>, Arc<dyn Fn(ClientEvent) + Send + Sync>>(
                on_event_callback,
            )
        };

        let event_bus = libparsec_client::EventBus::default();

        // Connect events

        let ping = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventPing| {
                (on_event_callback)(ClientEvent::Ping {
                    ping: e.ping.clone(),
                });
            })
        };

        let offline = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventOffline| {
                (on_event_callback)(ClientEvent::Offline);
            })
        };
        let online = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventOnline| {
                (on_event_callback)(ClientEvent::Online);
            })
        };

        let server_config_changed = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |_: &libparsec_client::EventServerConfigChanged| {
                (on_event_callback)(ClientEvent::ServerConfigChanged);
            })
        };
        // let workspace_self_role_changed = {
        //     let on_event_callback = on_event_callback.clone();
        //     event_bus.connect(move |e: &libparsec_client::EventWorkspaceSelfRoleChanged| {
        //         (on_event_callback)(ClientEvent::WorkspaceSelfRoleChanged);
        //     })
        // };
        // let workspace_entry_changed = {
        //     let on_event_callback = on_event_callback.clone();
        //     event_bus.connect(move |e: &libparsec_client::EventWorkspaceEntryChanged| {
        //         (on_event_callback)(ClientEvent::WorkspaceEntryChanged);
        //     })
        // };
        let invitation_changed = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventInvitationChanged| {
                (on_event_callback)(ClientEvent::InvitationChanged {
                    token: e.token,
                    status: e.status,
                });
            })
        };
        // let expired_organization = {
        //     let on_event_callback = on_event_callback.clone();
        //     event_bus.connect(move |_: &libparsec_client::EventExpiredOrganization| {
        //         (on_event_callback)(ClientEvent::ExpiredOrganization);
        //     })
        // };
        // let revoked_user = {
        //     let on_event_callback = on_event_callback.clone();
        //     event_bus.connect(move |_: &libparsec_client::EventRevokedSelfUser| {
        //         (on_event_callback)(ClientEvent::RevokedSelfUser);
        //     })
        // };
        let too_much_drift_with_server_clock = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(
                move |e: &libparsec_client::EventTooMuchDriftWithServerClock| {
                    (on_event_callback)(ClientEvent::TooMuchDriftWithServerClock {
                        ballpark_client_early_offset: e.ballpark_client_early_offset,
                        ballpark_client_late_offset: e.ballpark_client_late_offset,
                        client_timestamp: e.client_timestamp,
                        server_timestamp: e.server_timestamp,
                    });
                },
            )
        };
        let incompatible_server = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventIncompatibleServer| {
                (on_event_callback)(ClientEvent::IncompatibleServer {
                    detail: e.0.to_string(),
                });
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
            // _workspace_self_role_changed: workspace_self_role_changed,
            // _workspace_entry_changed: workspace_entry_changed,
            _invitation_changed: invitation_changed,
            _too_much_drift_with_server_clock: too_much_drift_with_server_clock,
            // _expired_organization: expired_organization,
            // _revoked_user: revoked_user,
            _incompatible_server: incompatible_server,
            // _invalid_keys_bundle: invalid_keys_bundle,
            // _invalid_certificate: invalid_certificate,
            // _invalid_manifest: invalid_manifest,
        }
    }
}
