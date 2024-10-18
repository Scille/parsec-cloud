// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use paste::paste;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

macro_rules! impl_any_spied_event_type {
    ($([$event:ident, $event_struct: ident])*) => {
        #[cfg(test)]
        #[derive(Debug, Clone)]
        pub enum AnySpiedEvent {
            $(
                $event($event_struct),
            )*
        }
    }
}

macro_rules! impl_event_bus_internal_and_event_bus_debug {
    ($([$event_struct:ident, $field_on_event_cbs:ident])*) => {
        #[derive(Default)]
        pub struct EventBusInternal {
            $(
                $field_on_event_cbs: Mutex<Vec<Box<dyn Fn(&$event_struct) + Send>>>,
            )*
        }

        impl std::fmt::Debug for EventBus {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut f = f.debug_struct("EventBus");
                $(
                let count = self.internal.$field_on_event_cbs
                    .lock()
                    .expect("Mutex is poisoned")
                    .len();
                if count > 0 {
                    f.field(stringify!($field_on_event_cbs), &count);
                }
                )*
                f.finish()
            }
        }
    };
}

macro_rules! impl_broadcastable {
    ($event: ident, $event_struct:ident, $field_on_event_cbs:ident) => {
        impl Broadcastable for $event_struct {
            fn send(&self, event_bus: &EventBusInternal) {
                let guard = event_bus
                    .$field_on_event_cbs
                    .lock()
                    .expect("Mutex is poisoned");
                for callback in guard.iter() {
                    callback(self);
                }
            }
            fn connect(
                event_bus: Arc<EventBusInternal>,
                callback: Box<dyn Fn(&Self) + Send>,
            ) -> EventBusConnectionLifetime<Self> {
                let ptr = {
                    let mut guard = event_bus
                        .$field_on_event_cbs
                        .lock()
                        .expect("Mutex is poisoned");
                    let fat_ptr: *const _ = callback.as_ref();
                    let ptr = fat_ptr as *const () as usize;
                    guard.push(callback);
                    ptr
                };
                EventBusConnectionLifetime {
                    ptr,
                    phantom: PhantomData,
                    event_bus,
                }
            }
            fn disconnect(event_bus: &EventBusInternal, ptr: usize) {
                let mut guard = event_bus
                    .$field_on_event_cbs
                    .lock()
                    .expect("Mutex is poisoned");
                guard
                    .iter()
                    .position(|e| {
                        // TODO: `dyn Trait` are not correctly compared by pointer when
                        // using regular tools (e.g. `Arc::ptr_eq`), this is because
                        // th comparison is done on the fat pointer (i.e. both the pointer
                        // on the data and the pointer on the vtable).
                        // see: https://github.com/rust-lang/rust/issues/106447
                        // So the solution is to manually transform the fat pointer into
                        // a thin one (i.e. the pointer on the data)
                        let e_fat_ptr: *const _ = e.as_ref();
                        let e_thin_ptr = e_fat_ptr as *const () as usize;
                        e_thin_ptr == ptr
                    })
                    .map(|index| guard.swap_remove(index));
            }
            #[cfg(test)]
            fn to_any_spied_event(&self) -> AnySpiedEvent {
                let cloned = (*self).clone();
                AnySpiedEvent::$event(cloned)
            }
            #[cfg(test)]
            fn try_from_any_spied_event(event: &AnySpiedEvent) -> Option<&Self> {
                match event {
                    AnySpiedEvent::$event(e) => Some(&e),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! impl_events {
    // Final step (output contains the list of all events at this point)
    (@munch () -> ($([$event: ident, $event_struct:ident, $field_on_event_cbs:ident])*)) => {
        impl_any_spied_event_type!($([$event, $event_struct])*);
        impl_event_bus_internal_and_event_bus_debug!($([$event_struct, $field_on_event_cbs])*);
    };

    // e.g. Foo
    (@munch ( $(#[$event_attr:meta])* $event:ident, $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            #[derive(Debug, Clone)]
            $(#[$event_attr])* pub struct [< Event $event >];
            impl_broadcastable!($event, [< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [$event, [< Event $event >], [< on_ $event:lower _cbs >]]));
        }
    };

    // e.g. Foo(u64)
    (@munch ( $(#[$event_attr:meta])* $event:ident ( $($ty:ty),* $(,)? ), $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            #[derive(Debug, Clone)]
            $(#[$event_attr])* pub struct [< Event $event>]( $(pub $ty),* );
            impl_broadcastable!($event, [< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [$event, [< Event $event >], [< on_ $event:lower _cbs >]]));
        }
    };

    // e.g. Foo{ bar: u64 }
    (@munch ( $(#[$event_attr:meta])* $event:ident { $($id:ident: $ty:ty),* $(,)? }, $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            #[derive(Debug, Clone)]
            $(#[$event_attr])* pub struct [< Event $event>] {
                $(pub $id:$ty),*
            }
            impl_broadcastable!($event, [< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [$event, [< Event $event >], [< on_ $event:lower _cbs >]]));
        }
    };

    // entry point (this is where a macro call starts)
    // The idea here is to parse the input recursively to spit the implementation of a
    // single event at each step and store the identifier of this event in the output.
    // Once we have processed all the inputs, we spit the implementation of `EventBusInternal`
    // (and some `EventBus`'s methods) by using output which contains the list of all events.
    ($($input:tt)*) => {
        impl_events!(@munch ($($input)*) -> ());
        //                  ^^^^^^^^^^^^    ^^
        //                     input       output
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum IncompatibleServerReason {
    #[error("Server is incompatible given it doesn't support API version: {api_version} (supported versions: {supported_api_versions:?})")]
    UnsupportedApiVersion {
        api_version: ApiVersion,
        supported_api_versions: Vec<ApiVersion>,
    },
    #[error("Server is incompatible due to an unexpected error: {0}")]
    Unexpected(Arc<anyhow::Error>),
}

// All those items will be named with a `Event` prefix (e.g. `Foo` => `EventFoo`)
// Note all errors in the events are wrapped with an `Arc`, this due to`anyhow::Error`
// not being `Clone`. If you wonder, performance impact should be minimal because:
// 1) Error-containing events are not often fired
// 2) `anyhow::Error` itself is just a pointer, so converting it to an Arc is cheap
impl_events!(
    // ***********************************************************************
    // Server connection related events.
    // ***********************************************************************

    // Client-server communication is divided into two parts:
    // - Client commands sent to the server through an RPC-like mechanism.
    // - Server events sent to the client through a Server-Sent Events (SSE) mechanism.
    //
    // In practice we have a single SSE connection handled by the connection monitor, and
    // multiple concurrent RPC commands depending on what is going on in the client (e.g.
    // data synchronization, reading a file not in cache, etc.).
    //
    // On top of that, low-level details about the server connection are handled by the
    // reqwest library (see `libparsec_client_connection` implementation), hence we have no
    // guarantee on the number of physical TCP connections being used to handle RPC and SSE.
    //
    // However what we know is 1) RPC and SSE connect to the same server and 2) server
    // and client both support HTTP/2. Hence we can expect all connections to be multiplexed
    // over a single physical TCP connection.
    //
    // With this in mind, the choice has been to have the connection-related events only
    // fired by the connection monitor: if a RPC command fails due to server disconnection,
    // it is very likely the SSE connection will also fail right away.

    /// This event is fired by the connection monitor.
    ///
    /// The SSE connection with the server has been established.
    Online,
    /// This event is fired by the connection monitor.
    ///
    /// The SSE connection with the server has been lost.
    Offline,
    /// This event is fired by `Client::accept_tos` when the TOS have been accepted.
    /// It's most likely the server will now accept the next connection attempt.
    ///
    /// This event is used to signify to the connection monitor that it should try
    /// to connect to the server without delay. This is needed since, when offline,
    /// the connection monitor has a backoff mechanism to avoid hammering the server
    /// with connection attempts.
    ShouldRetryConnectionNow,
    /// This event is fired by the connection monitor.
    ///
    /// The SSE connection is able to catch up with the event it has missed when
    /// re-connecting to the server.
    /// However this falls short on two occasions:
    /// - The connection monitor has just started.
    /// - The server informed us that it has skipped some event is the SSE connection.
    ///
    /// In both cases, the issue is there is too much to catch up with and the monitors that
    /// expect to be in sync with the server should do a polling operation to catch up instead.
    MissedServerEvents,
    /// This event is fired by the connection monitor.
    ///
    /// The server has informed us that the organization we are part of has expired.
    ExpiredOrganization,
    /// This event is fired by the connection monitor.
    ///
    /// The server has informed us that the user we are authenticated with has been revoked.
    /// Note this can also occur if the user has been frozen (i.e. the user is temporary not
    /// allowed to connect to the server).
    RevokedSelfUser,
    /// This event is fired by the connection monitor.
    ///
    /// The server has informed us that the authenticated user has not accepted the
    /// Terms of Service (TOS) yet.
    MustAcceptTos,
    /// This event is fired by the connection monitor.
    ///
    /// A connection has been made between the client and the server, but they cannot
    /// settle on a common API to communicate.
    IncompatibleServer(IncompatibleServerReason),
    /// The server and client clocks are too much out of sync.
    ///
    /// This event is a special snowflake: it is fired anywhere the client sends a RPC
    /// command to the server that may return a `timestamp_out_of_ballpark` error status.
    ///
    /// Note the SSE connection never returns a `timestamp_out_of_ballpark` so only the
    /// RPC commands can fire this event (which is exactly the opposite of other
    /// connection-related errors that are only fired by the connection monitor !).
    ///
    /// The reason for this weird behavior is the fact the error status is specific to
    /// each RPC command.
    // TODO: If we wrap `AuthenticatedCmds::send` to make it handle event sending, we
    //       could also have `ProtocolRequest::api_load_response` informing us in a
    //       generic way that the response should lead to a `TooMuchDriftWithServerClock`
    //       event being fired.
    TooMuchDriftWithServerClock {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: Float,
        ballpark_client_late_offset: Float,
    },

    // ***********************************************************************
    // Server configuration related events
    // ***********************************************************************

    /// This event is fired by the connection monitor.
    ///
    /// `ServerConfigNotified` correspond to the SSE event that is always send by the
    /// server when connecting to it. `ServerConfigChanged` is the higher level event
    /// that detects an actual change in the server configuration (e.g. in case of
    /// disconnection we can have multiple `ServerConfigNotified`, but no
    /// `ServerConfigChanged`).
    ServerConfigNotified {
        active_users_limit: ActiveUsersLimit,
        user_profile_outsider_allowed: bool,
    },
    /// This event is fired by the server config monitor.
    ///
    /// `ServerConfigChanged` doesn't provide the actual config, use
    /// `Client::get_server_config` to get it instead.
    ServerConfigChanged,

    // ***********************************************************************
    // Certificates related events
    // ***********************************************************************

    /// This event is fired by the connection monitor.
    ///
    /// There is new certificates available on the server.
    ///
    /// This event is used by the certificates poll monitor to trigger a polling
    /// operation.
    /// `CertificatesUpdated` correspond to the SSE event that is send by the server.
    /// `NewCertificates` is the higher level event that occurs once the new
    /// certificates has been integrated in the local storage and, hence, are visible
    /// for all practical purpose.
    ///
    /// Note this event will be fired (i.e. the server pushes it to us)  even if the
    /// new certificates comes from our client.
    /// This is because the certificates can only be integrated in the local storage in
    /// strict chronological order, which is only possible by first polling the server for
    /// new certificates to make sure we are not missing a concurrently added certificate.
    CertificatesUpdated { last_timestamps: PerTopicLastTimestamps },
    /// This event is fired by the certificate ops when new certificates has been
    /// integrated into the local storage.
    ///
    /// This event is used by the workspaces refresh list monitor to trigger a refresh
    /// of the local workspaces list (i.e. the list of workspaces the client considers
    /// the user has access to)
    // TODO: two types of new certificates events:
    //       - poll from scratch
    //       - per-type event: rename, key rotation, share/unshare with self, shamir for self, etc.
    NewCertificates {
        // The local certificates storage was empty, hence the server sent all certificates.
        storage_initially_empty: bool,
        // If a `*_new_since` field is `None`, that means either:
        // - The local certificates storage was initially empty.
        // - No new certificate have been obtained for this topic.
        // If it is `Some`, the field contains the last timestamp of the topic prior
        // to the integration of the new certificates.
        common_new_since: Option<DateTime>,
        sequester_new_since: Option<DateTime>,
        shamir_recovery_new_since: Option<DateTime>,
        realm_new_since: Vec<(VlobID, DateTime)>,
    },
    /// This event is fired by the certificate ops when unsuccessfully trying to
    /// integrate new certificates.
    InvalidCertificate(Box<crate::certif::InvalidCertificateError>),

    // ***********************************************************************
    // Invitation related events
    // ***********************************************************************

    /// This event is fired by the connection monitor.
    ///
    /// The invitation status has changed on server side (e.g. a claimer is online,
    /// an invitation has been deleted etc.).
    ///
    /// Note this event will be fired (i.e. the server pushes it to us) even if
    /// we are at the origin of the change (e.g. we cancelled the invitation).
    InvitationChanged {
        status: InvitationStatus,
        token: InvitationToken,
    },

    /// This event is fired by the connection monitor.
    ///
    /// The PKI enrollments have changed on server side (e.g. a new enrollment
    /// is available).
    ///
    /// Notes:
    /// - This event will be fired (i.e. the server pushes it to us) even if
    ///   we are at the origin of the change (e.g. we accepted/rejected the enrollment).
    /// - This event is received by all users with ADMIN profile.
    PkiEnrollmentUpdated,

    // ***********************************************************************
    // Vlob related events
    // ***********************************************************************

    /// This event is fired by the connection monitor.
    ///
    /// A vlob has been created/updated on the server.
    ///
    /// This event is used by the workspaces inbound sync monitor (and the user
    /// sync monitor) to be notified an inbound sync operation is needed.
    ///
    /// Note this event WILL NOT by fired if the vlob create/update comes from our
    /// client. This is because a vlob (unlike certificates) can be directly
    /// integrated in the local storage without the need for other knowledge from
    /// the server (i.e. vlobs can be considered isolated from each others).
    RealmVlobUpdated {
        author: DeviceID,
        blob: Option<Bytes>,
        last_common_certificate_timestamp: DateTime,
        last_realm_certificate_timestamp: DateTime,
        realm_id: VlobID,
        timestamp: DateTime,
        version: VersionInt,
        vlob_id: VlobID,
    },

    /// This event is fired by the user ops when a sync of the user manifest is needed.
    ///
    /// This event is hence used by the user sync monitor to be notified an outbound
    /// sync operation is needed.
    UserOpsOutboundSyncNeeded,
    /// This event is fired by the user ops after a successful outbound sync of
    /// the user manifest (i.e. the local user manifest is no longer need sync).
    UserOpsOutboundSyncDone,

    /// This event is fired by the workspace ops when a sync of one of its manifest is needed.
    ///
    /// This event is hence used by the workspace sync monitor to be notified an outbound
    /// sync operation is needed.
    /// This event is also used to implement `WorkspaceOps::workspace_watch_entry_oneshot`.
    WorkspaceOpsOutboundSyncNeeded {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    /// This event is fired by the workspace ops when starting an outbound sync
    WorkspaceOpsOutboundSyncStarted {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    /// This event is fired by the workspace ops when doing an outbound sync for a file.
    ///
    /// This event aims at providing feedback about the file's blocks upload (hence it
    /// never occurs for folders or when a file is empty or just gets truncated, as in
    /// those cases only the manifest is uploaded which should be fast and is a single
    /// operation anyway so started & done events are enough).
    ///
    /// Also note that the progress may become misleading if the synchronization only
    /// uploads a subset of the file (e.g. a small modification in the file leads to
    /// a single block being modified), given in this case only the block index to
    /// upload will fire a corresponding the progress event.
    WorkspaceOpsOutboundSyncProgress {
        realm_id: VlobID,
        entry_id: VlobID,
        blocks: IndexInt,
        block_index: IndexInt,
        blocksize: SizeInt,
    },
    /// This event is fired by the workspace ops when an outbound sync cannot be
    /// completed. This is typically the case if an inbound sync is needed first.
    WorkspaceOpsOutboundSyncAborted {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    /// This event is fired by the workspace ops after a successful outbound sync
    /// (i.e. the corresponding local manifest is no longer need sync).
    WorkspaceOpsOutboundSyncDone {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    /// This event is fired by the workspace ops after a successful inbound sync.
    ///
    /// Note, unlike for outbound sync, we don't have started & progress events for
    /// this kind of sync. This is because inbound sync is very fast once the remote
    /// manifest has been fetched (and it's useless to communicate progress about
    /// this fetch step).
    ///
    /// This event is used to implement `WorkspaceOps::workspace_watch_entry_oneshot`.
    WorkspaceOpsInboundSyncDone {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    /// This event is fired by the workspace ops when a manifest that is watched gets
    /// modified or local or remote (see `WorkspaceOps::workspace_watch_entry_oneshot`).
    WorkspaceWatchedEntryChanged {
        realm_id: VlobID,
        entry_id: VlobID,
    },

    // ***********************************************************************
    // Misc events
    // ***********************************************************************

    /// This event is fired in `Client::refresh_workspaces_list` when a refresh of the
    /// local workspaces list (i.e. the list of workspaces the client considers the
    /// user has access to) leads to an actual change in the list.
    ///
    /// When this event is fired, you can expect:
    /// - A new workspace has been added to the list.
    /// - A workspace has been removed from the list.
    /// - A workspace had its name changed.
    WorkspacesSelfListChanged,

    /// This event is fired in `Client::create_workspace`.
    ///
    /// This event is used by the workspaces bootstrap monitor to trigger a bootstrap
    /// of the newly created workspace (i.e. the workspaces gets created on the server
    /// and its initial key rotation and realm name certificates get uploaded).
    WorkspaceLocallyCreated {
        name: EntryName,
        realm_id: VlobID,
    },

    /// A monitor has crashed :'(
    ///
    /// This is not supposed to occur, however if that's the case the monitor is
    /// not restarted (to avoid crash loop).
    MonitorCrashed{
        monitor: &'static str,
        workspace_id: Option<VlobID>,
        error: Arc<anyhow::Error>,
    },

    /// Dummy event for tests only (only test code should fire it).
    Ping { ping: String },
);

pub trait Broadcastable: std::fmt::Debug + Send
where
    Self: Sized,
{
    fn send(&self, event_bus: &EventBusInternal);
    fn connect(
        event_bus: Arc<EventBusInternal>,
        callback: Box<dyn Fn(&Self) + Send>,
    ) -> EventBusConnectionLifetime<Self>;
    fn disconnect(event_bus: &EventBusInternal, ptr: usize);
    #[cfg(test)]
    fn to_any_spied_event(&self) -> AnySpiedEvent;
    #[cfg(test)]
    fn try_from_any_spied_event(event: &AnySpiedEvent) -> Option<&Self>;
}

enum ServerState {
    Online,
    Offline,
}

struct ServerStateListener {
    watch: libparsec_platform_async::watch::Receiver<ServerState>,
    _lifetimes: (
        EventBusConnectionLifetime<EventOnline>,
        EventBusConnectionLifetime<EventOffline>,
    ),
}

impl ServerStateListener {
    fn new(event_bus: &Arc<EventBusInternal>) -> Self {
        let (tx, rx) = libparsec_platform_async::watch::channel(ServerState::Offline);
        let tx_online = Arc::new(Mutex::new(tx));
        let tx_offline = tx_online.clone();
        let lifetimes = (
            EventOnline::connect(
                event_bus.clone(),
                Box::new(move |_| {
                    let _ = tx_online
                        .lock()
                        .expect("Mutex is poisoned")
                        .send(ServerState::Online);
                }),
            ),
            EventOffline::connect(
                event_bus.clone(),
                Box::new(move |_| {
                    let _ = tx_offline
                        .lock()
                        .expect("Mutex is poisoned")
                        .send(ServerState::Offline);
                }),
            ),
        );
        Self {
            watch: rx,
            _lifetimes: lifetimes,
        }
    }
}

#[cfg(test)]
mod spy {
    use libparsec_platform_async::event::Event;

    use super::*;

    #[derive(Default)]
    struct EventBusSpyInternal {
        events: Vec<AnySpiedEvent>,
        on_new_event: Option<Event>,
    }

    #[derive(Default, Clone)]
    pub struct EventBusSpy {
        internal: Arc<Mutex<EventBusSpyInternal>>,
    }

    impl EventBusSpy {
        pub(super) fn add(&self, event: AnySpiedEvent) {
            let mut guard = self.internal.lock().expect("Mutex is poisoned");
            guard.events.push(event);
            if let Some(on_new_event) = guard.on_new_event.take() {
                on_new_event.notify(usize::MAX);
            }
        }

        /// Return all events that occured so far, useful for dump as last ditch attempt
        /// during debugging.
        ///
        /// In tests, use `EventBusSpy::start_expecting` instead.
        pub fn events(&self) -> Vec<AnySpiedEvent> {
            let guard = self.internal.lock().expect("Mutex is poisoned");
            guard.events.clone()
        }

        /// Convenient way to ensure some events occured in at a given place.
        /// Note any non-acknowledged event will cause a panic on drop.
        pub fn start_expecting(&self) -> EventBusSpyExpectContext {
            let current_offset = {
                let guard = self.internal.lock().expect("Mutex is poisoned");
                guard.events.len()
            };
            EventBusSpyExpectContext {
                acknowledged_offset: current_offset,
                internal: self.internal.clone(),
            }
        }
    }

    pub struct EventBusSpyExpectContext {
        acknowledged_offset: usize,
        internal: Arc<Mutex<EventBusSpyInternal>>,
    }

    impl EventBusSpyExpectContext {
        pub fn list_not_acknowledged(&self) -> Vec<AnySpiedEvent> {
            let guard = self.internal.lock().expect("Mutex is poisoned");
            let not_acknowledged = &guard.events[self.acknowledged_offset..];
            not_acknowledged.to_owned()
        }

        /// Check and mark acknowledged the next event, panic if no
        /// new event are available.
        pub fn assert_next<E: Broadcastable>(&mut self, check: impl FnOnce(&E)) {
            let guard = self.internal.lock().expect("Mutex is poisoned");
            let not_acknowledged = &guard.events[self.acknowledged_offset..];
            match not_acknowledged.first() {
                None => panic!("No new events !"),
                // Event available, but is it the expected one ?
                Some(any_event) => {
                    match E::try_from_any_spied_event(any_event) {
                        None => panic!("Unexpected event: {:?}", any_event),
                        // callback is expected to panic if unhappy with the event
                        Some(event) => check(event),
                    }
                }
            }
            self.acknowledged_offset += 1;
        }

        /// Check and mark acknowledged the next event available, block if no
        /// event are currently available.
        pub async fn wait_and_assert_next<E: Broadcastable>(&mut self, check: impl FnOnce(&E)) {
            // 1) Ensure a new event is available, or else wait for one !
            let maybe_wait_for_event = {
                let mut guard = self.internal.lock().expect("Mutex is poisoned");
                if guard.events[self.acknowledged_offset..].is_empty() {
                    let listener = match &guard.on_new_event {
                        Some(on_new_event) => on_new_event.listen(),
                        None => {
                            let on_new_event = Event::new();
                            let listener = on_new_event.listen();
                            guard.on_new_event = Some(on_new_event);
                            listener
                        }
                    };
                    Some(listener)
                } else {
                    None
                }
            };
            if let Some(listener) = maybe_wait_for_event {
                listener.await;
            }

            // 2) Assert the new event is what we expect
            self.assert_next(check);
        }

        pub fn assert_no_events(&self) {
            // If the events mutex is poisoned, that means a panic is already bubbling up
            let guard = self.internal.lock().expect("Mutex is poisoned");
            // Ensure all events have been acknowledged
            let not_acknowledged = &guard.events[self.acknowledged_offset..];
            if !not_acknowledged.is_empty() {
                panic!(
                    "Event spy expect context hasn't acknowledge all events: {:#?}",
                    not_acknowledged
                )
            }
        }
    }

    impl Drop for EventBusSpyExpectContext {
        fn drop(&mut self) {
            // If the events mutex is poisoned, that means a panic is already bubbling up
            if let Ok(guard) = self.internal.lock() {
                // Ensure all events have been acknowledged
                let not_acknowledged = &guard.events[self.acknowledged_offset..];
                if !not_acknowledged.is_empty() {
                    panic!(
                        "Event spy expect context hasn't acknowledge all events: {:#?}",
                        not_acknowledged
                    )
                }
            }
        }
    }
}
#[cfg(test)]
pub use spy::{EventBusSpy, EventBusSpyExpectContext};

#[derive(Clone)]
pub struct EventBus {
    internal: Arc<EventBusInternal>,
    server_state: Arc<AsyncMutex<ServerStateListener>>,
    #[cfg(test)]
    pub spy: EventBusSpy,
}

impl Default for EventBus {
    fn default() -> Self {
        let internal = Arc::new(EventBusInternal::default());
        let server_state = Arc::new(AsyncMutex::new(ServerStateListener::new(&internal)));
        Self {
            internal,
            server_state,
            #[cfg(test)]
            spy: EventBusSpy::default(),
        }
    }
}

impl EventBus {
    pub fn send(&self, event: &impl Broadcastable) {
        #[cfg(test)]
        {
            self.spy.add(event.to_any_spied_event());
        }
        log::debug!("Sending event: {:?}", event);
        event.send(&self.internal);
    }

    #[must_use]
    pub fn connect<F, B>(&self, callback: F) -> EventBusConnectionLifetime<B>
    where
        F: Fn(&B) + Send + 'static,
        B: Broadcastable + 'static,
    {
        B::connect(self.internal.clone(), Box::new(callback))
    }

    pub async fn wait_server_online(&self) {
        let mut guard = self.server_state.lock().await;
        if let ServerState::Online = *guard.watch.borrow_and_update() {
            return;
        }
        // The server is currently offline, must wait !
        guard
            .watch
            .wait_for(|state| matches!(*state, ServerState::Online))
            .await
            .expect("Same lifetime for senders&receiver");
    }
}

pub struct EventBusConnectionLifetime<B>
where
    B: Broadcastable,
{
    // Pointer on the callback's box, we use it as a unique identifier when disconnecting.
    // Of course pointer are not *really* unique given an address can be reused, but this
    // is not an issue here given we keep the pointer only as long as the box exist, so
    // there is no risk of re-using an old pointer which would remove an unrelated box
    // that happens to have reused the address.
    ptr: usize,
    phantom: PhantomData<B>,
    event_bus: Arc<EventBusInternal>,
}

impl<B> Drop for EventBusConnectionLifetime<B>
where
    B: Broadcastable,
{
    fn drop(&mut self) {
        B::disconnect(&self.event_bus, self.ptr);
    }
}

#[cfg(test)]
#[path = "../tests/unit/event_bus.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
