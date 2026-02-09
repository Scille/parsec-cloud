// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use paste::paste;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::sleep;
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

#[derive(Debug, Clone)]
pub enum ClientErrorResponseType {
    BadAcceptType,
    BadContent,
    BadAuthenticationInfo,
    MissingAuthenticationInfo,
    AuthenticationTokenExpired,
    InvitationAlreadyUsedOrDeleted,
}
impl std::fmt::Display for ClientErrorResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ClientErrorResponseType::BadAcceptType => write!(f, "BadAcceptType"),
            ClientErrorResponseType::BadContent => write!(f, "BadContent"),
            ClientErrorResponseType::BadAuthenticationInfo => write!(f, "BadAuthenticationInfo"),
            ClientErrorResponseType::MissingAuthenticationInfo => {
                write!(f, "MissingAuthenticationInfo")
            }
            ClientErrorResponseType::AuthenticationTokenExpired => {
                write!(f, "AuthenticationTokenExpired")
            }
            ClientErrorResponseType::InvitationAlreadyUsedOrDeleted => {
                write!(f, "InvitationAlreadyUsedOrDeleted")
            }
        }
    }
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
    /// The server has informed us that the organization was not found.
    OrganizationNotFound,
    /// This event is fired by the connection monitor.
    ///
    /// The server has informed us that the invitation was already used or deleted.
    InvitationAlreadyUsedOrDeleted,
    /// This event is fired by the connection monitor.
    ///
    /// The server has informed us that the authenticated user has been revoked.
    RevokedSelfUser,
    /// This event is fired by the connection monitor.
    ///
    /// The server has informed us that the authenticated user has been frozen
    /// (i.e. the user is temporary not allowed to connect to the server).
    FrozenSelfUser,
    /// This event is fired by the connection monitor.
    ///
    /// The sever has informed us that the organization's configuration doesn't
    /// allow web clients (i.e. clients with `User-Agent` not starting with
    /// `Parsec-client/`).
    WebClientNotAllowedByOrganization,
    /// This event is fired by the connection monitor.
    ///
    /// The server has informed us that the authenticated user has not accepted
    /// the Terms of Service (TOS) yet.
    MustAcceptTos,
    /// This event is fired by the connection monitor.
    ///
    /// A connection has been established between the client and the server, but
    /// cannot settle on a common API to communicate.
    IncompatibleServer{
        api_version: ApiVersion,
        supported_api_version: Vec<ApiVersion>
    },
    /// This event is fired by the connection monitor.
    ///
    /// The server returned a client error response status code (4xx)
    ClientErrorResponse{
        error_type: ClientErrorResponseType
    },
    /// This event is fired by the connection monitor.
    ///
    /// The server returned an unexpected status code
    ServerInvalidResponseStatus{
        status_code: String
    },
    /// This event is fired by the connection monitor.
    ///
    /// The server returned an unexpected response content
    ServerInvalidResponseContent{
        protocol_decode_error: String
    },
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
    /// The invitation status has changed on server side (i.e the invitation
    /// has been created, cancelled or completed).
    ///
    /// Note this event will be fired (i.e. the server pushes it to us) even if
    /// we are at the origin of the change (e.g. we cancelled the invitation).
    InvitationChanged {
        status: InvitationStatus,
        token: AccessToken,
    },

    /// This event is fired by the connection monitor.
    ///
    /// It is used to inform the user that a claimer is waiting to be greeted.
    /// More precisely, this event is sent each time the claimer polls the first step
    /// (WAIT_PEER) of the corresponding greeting attempt.
    GreetingAttemptReady {
        token: AccessToken,
        greeting_attempt: GreetingAttemptID,
    },

    /// This event is fired by the connection monitor.
    ///
    /// It is used to inform the user that a greeting attempt has been cancelled.
    /// It can be used to invalidate a previous `GreetingAttemptReady` event,
    /// since this greeting attempt can no longer be joined.
    ///
    /// Note this event will be fired (i.e. the server pushes it to us) even if
    /// we are at the origin of the change (e.g. we cancelled the greeting attempt).
    GreetingAttemptCancelled {
        token: AccessToken,
        greeting_attempt: GreetingAttemptID,
    },

    /// This event is fired by the connection monitor.
    ///
    /// It is used to inform the user that a greeting attempt has been joined.
    /// It can be used to invalidate a previous `GreetingAttemptReady` event,
    /// since this greeting attempt can no longer be joined.
    ///
    /// Note this event will be fired (i.e. the server pushes it to us) even if
    /// we are at the origin of the change (e.g. we cancelled the greeting attempt).
    GreetingAttemptJoined {
        token: AccessToken,
        greeting_attempt: GreetingAttemptID,
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

    /// This event is fired by the connection monitor.
    ///
    /// An asynchronous enrollment request has changed on server side (i.e. a new
    /// enrollment is available, or an existing one has been accepted/rejected/cancelled).
    ///
    /// Notes:
    /// - This event will be fired (i.e. the server pushes it to us) even if
    ///   we are at the origin of the change (e.g. we accepted/rejected the enrollment).
    /// - This event is received by all users with ADMIN profile.
    AsyncEnrollmentUpdated,

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
    /// This event is also used to implement `WorkspaceOps::watch_entry_oneshot`.
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
    /// This event is used to implement `WorkspaceOps::watch_entry_oneshot`.
    WorkspaceOpsInboundSyncDone {
        realm_id: VlobID,
        entry_id: VlobID,
        parent_id: VlobID
    },
    /// This event is fired by the workspace ops when a manifest that is watched gets
    /// modified on local or remote (see `WorkspaceOps::watch_entry_oneshot`).
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

/// Keeps track of some state that are propagated by events.
///
/// This is typically useful to know if the server is online or not, and to wait
/// for it to be online.
struct ServerStateListener {
    internal: Arc<Mutex<ServerStateInternal>>,
    _lifetimes: (
        EventBusConnectionLifetime<EventOnline>,
        EventBusConnectionLifetime<EventOffline>,
        EventBusConnectionLifetime<EventExpiredOrganization>,
        EventBusConnectionLifetime<EventMustAcceptTos>,
    ),
}

struct ServerStateInternal {
    /// Based on `EventOnline` and `EventOffline` events.
    is_server_online: bool,
    /// Based on `EventExpiredOrganization` and `EventOnline` events (the latter event because
    /// a client cannot successfully connect to the server if the organization is expired).
    is_organization_expired: bool,
    /// Based on `EventMustAcceptTos` and `EventOnline` events (the latter event because
    /// a client can only successfully connect to the server once the TOS has been accepted).
    must_accept_tos: bool,
    on_changed: libparsec_platform_async::event::Event,
}

impl ServerStateListener {
    fn new(event_bus: &Arc<EventBusInternal>) -> Self {
        let internal = Arc::new(Mutex::new(ServerStateInternal {
            is_server_online: false,
            is_organization_expired: false,
            must_accept_tos: false,
            on_changed: libparsec_platform_async::event::Event::new(),
        }));

        let lifetimes = (
            {
                let internal = internal.clone();
                EventOnline::connect(
                    event_bus.clone(),
                    Box::new(move |_| {
                        let mut guard = internal.lock().expect("Mutex is poisoned");
                        guard.is_server_online = true;
                        guard.is_organization_expired = false;
                        guard.must_accept_tos = false;
                        guard.on_changed.notify(usize::MAX);
                    }),
                )
            },
            {
                let internal = internal.clone();
                EventOffline::connect(
                    event_bus.clone(),
                    Box::new(move |_| {
                        let mut guard = internal.lock().expect("Mutex is poisoned");
                        guard.is_server_online = false;
                        guard.on_changed.notify(usize::MAX);
                    }),
                )
            },
            {
                let internal = internal.clone();
                EventExpiredOrganization::connect(
                    event_bus.clone(),
                    Box::new(move |_| {
                        let mut guard = internal.lock().expect("Mutex is poisoned");
                        guard.is_organization_expired = true;
                        guard.on_changed.notify(usize::MAX);
                    }),
                )
            },
            {
                let internal = internal.clone();
                EventMustAcceptTos::connect(
                    event_bus.clone(),
                    Box::new(move |_| {
                        let mut guard = internal.lock().expect("Mutex is poisoned");
                        guard.must_accept_tos = true;
                        guard.on_changed.notify(usize::MAX);
                    }),
                )
            },
        );

        Self {
            internal,
            _lifetimes: lifetimes,
        }
    }

    pub fn is_server_online(&self) -> bool {
        let guard = self.internal.lock().expect("Mutex is poisoned");
        guard.is_server_online
    }

    pub fn is_organization_expired(&self) -> bool {
        let guard = self.internal.lock().expect("Mutex is poisoned");
        guard.is_organization_expired
    }

    pub fn must_accept_tos(&self) -> bool {
        let guard = self.internal.lock().expect("Mutex is poisoned");
        guard.must_accept_tos
    }

    pub async fn wait_server_online(&self) {
        loop {
            let listener = {
                let guard = self.internal.lock().expect("Mutex is poisoned");
                if guard.is_server_online {
                    return;
                }

                // The server is currently offline, must wait !
                guard.on_changed.listen()
            };

            listener.await
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

        /// Return all events that occurred so far, useful for dump as last ditch attempt
        /// during debugging.
        ///
        /// In tests, use `EventBusSpy::start_expecting` instead.
        pub fn events(&self) -> Vec<AnySpiedEvent> {
            let guard = self.internal.lock().expect("Mutex is poisoned");
            guard.events.clone()
        }

        /// Convenient way to ensure some events occurred in at a given place.
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
                        None => panic!("Unexpected event: {any_event:?}"),
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
                    "Event spy expect context hasn't acknowledge all events: {not_acknowledged:#?}"
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
                        "Event spy expect context hasn't acknowledge all events: {not_acknowledged:#?}"
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
    server_state: Arc<ServerStateListener>,
    #[cfg(test)]
    pub spy: EventBusSpy,
}

impl Default for EventBus {
    fn default() -> Self {
        let internal = Arc::new(EventBusInternal::default());
        let server_state = Arc::new(ServerStateListener::new(&internal));
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
        log::debug!("Sending event: {event:?}");
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

    pub fn must_accept_tos(&self) -> bool {
        self.server_state.must_accept_tos()
    }

    pub fn is_server_online(&self) -> bool {
        self.server_state.is_server_online()
    }

    pub fn is_organization_expired(&self) -> bool {
        self.server_state.is_organization_expired()
    }

    pub async fn wait_server_online(&self) {
        self.server_state.wait_server_online().await
    }

    /// This method is used in the monitors when a request has failed due
    /// to a connection error. That means we also expect the SSE connection
    /// to be down, or that it will be down soon. Waiting for a couple of
    /// seconds for the server state to be offline allows us to avoid hard
    /// polling the server in case the connection error only affects the RPC
    /// but not the SSE connection.
    pub async fn wait_server_reconnect(&self) {
        sleep(std::time::Duration::from_secs(3)).await;
        self.wait_server_online().await;
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
