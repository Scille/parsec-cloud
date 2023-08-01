// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Here we expose only a subset of the client events, as some of them are for
// internal use only.

use std::sync::Arc;

use libparsec_client::EventBusConnectionLifetime;

pub enum ClientEvent {
    // Dummy event for tests only
    Ping { ping: String },
    // // Events related to server connection
    // Offline(libparsec_client::EventOffline),
    // Online(libparsec_client::EventOnline),
    // MissedServerEvents(libparsec_client::EventMissedServerEvents),
    // TooMuchDriftWithServerClock(libparsec_client::EventTooMuchDriftWithServerClock),
    // ExpiredOrganization(libparsec_client::EventExpiredOrganization),
    // RevokedUser(libparsec_client::EventRevokedUser),
    // IncompatibleServer(libparsec_client::EventIncompatibleServer),
    // // Events related to ops
    // InvalidMessage(libparsec_client::EventInvalidMessage),
    // // Events related to monitors
    // CertificatesMonitorCrashed(libparsec_client::EventCertificatesMonitorCrashed),
    // InvalidCertificate(libparsec_client::EventInvalidCertificate),
    // // TODO
    // // // Logs events
    // // LogInfo,
    // // LogWarning,
    // // LogError,
}

pub(crate) struct OnEventCallbackPlugged {
    pub event_bus: libparsec_client::EventBus,

    _ping: EventBusConnectionLifetime<libparsec_client::EventPing>,
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
        // On web we run on the JS runtime which is monothreaded, hence everything is !Send
        #[cfg(target_arch = "wasm32")] on_event_callback: Arc<dyn Fn(ClientEvent)>,
    ) -> Self {
        // SAFETY: `EventBus` requires callback to be `Send`, however on web the runtime
        // is strictly single-threaded and callback must be `!Send`.
        // So here we are going "trust me bro" considering it is fine to lie about
        // sendness of the callback given it will never leave the current thread.
        #[cfg(target_arch = "wasm32")]
        let on_event_callback = unsafe {
            std::mem::transmute::<Arc<dyn Fn(ClientEvent)>, Arc<dyn Fn(ClientEvent) + Send + Sync>>(
                on_event_callback,
            )
        };

        let event_bus = libparsec_client::EventBus::default();

        let ping = {
            let on_event_callback = on_event_callback.clone();
            event_bus.connect(move |e: &libparsec_client::EventPing| {
                (on_event_callback)(ClientEvent::Ping {
                    ping: e.ping.clone(),
                });
            })
        };

        Self {
            event_bus,
            _ping: ping,
        }
    }
}
