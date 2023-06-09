// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use paste::paste;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use libparsec_types::prelude::*;

use crate::certificates_ops::InvalidMessageError;

macro_rules! impl_event_bus_internal_and_event_bus_debug {
    ($([$event_struct:ident, $field_on_event_cbs:ident])*) => {

        pub struct EventBusInternal {
            $(
                $field_on_event_cbs: Mutex<Vec<Box<dyn Fn(&$event_struct) + Send>>>,
            )*
        }

        impl Default for EventBusInternal {
            fn default() -> Self {
                Self{
                    $(
                    $field_on_event_cbs: Mutex::new(vec![]),
                    )*
                }
            }
        }

        impl std::fmt::Debug for EventBus {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    ($event_struct:ident, $field_on_event_cbs:ident) => {
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
                    let fatptr: *const _ = callback.as_ref();
                    let ptr = fatptr as *const () as usize;
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
                        let e_fatptr: *const _ = e.as_ref();
                        let e_thinptr = e_fatptr as *const () as usize;
                        e_thinptr == ptr
                    })
                    .map(|index| guard.swap_remove(index));
            }
        }
    };
}

macro_rules! impl_events {
    // Final step (output contains the list of all events at this point)
    (@munch () -> ($([$event_struct:ident, $field_on_event_cbs:ident])*)) => {
        impl_event_bus_internal_and_event_bus_debug!($([$event_struct, $field_on_event_cbs])*);
    };

    // e.g. Foo
    (@munch ( $event:ident, $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            pub struct [< Event $event >];
            impl_broadcastable!([< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [[< Event $event >], [< on_ $event:lower _cbs >]]));
        }
    };

    // e.g. Foo(u64)
    (@munch ( $event:ident ( $($ty:ty),* $(,)? ), $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            pub struct [< Event $event>]( $(pub $ty),* );
            impl_broadcastable!([< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [[< Event $event >], [< on_ $event:lower _cbs >]]));
        }
    };

    // e.g. Foo{ bar: u64 }
    (@munch ( $event:ident { $($id:ident: $ty:ty),* $(,)? }, $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            pub struct [< Event $event>] {
                $(pub $id:$ty),*
            }
            impl_broadcastable!([< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [[< Event $event >], [< on_ $event:lower _cbs >]]));
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

pub enum IncompatibleServerReason {
    UnsupportedApiVersion {
        api_version: ApiVersion,
        supported_api_versions: Vec<ApiVersion>,
    },
    Unexpected(anyhow::Error),
}

pub enum UnprocessableMessageReason {
    InvalidMessage(InvalidMessageError),
    // The message is lying about the fact our role got revoked
    SharingRevokedButNoCorrespondingCertificate,
    SharingGrantedButNoCorrespondingCertificate,
}

impl_events!(
    // Dummy event for tests only
    Ping { ping: String },
    // Events related to server connection
    Offline,
    Online,
    MissedServerEvents,
    TooMuchDriftWithServerClock {
        backend_timestamp: DateTime,
        ballpark_client_early_offset: Float,
        ballpark_client_late_offset: Float,
        client_timestamp: DateTime,
    },
    ExpiredOrganization,
    RevokedUser,
    IncompatibleServer(IncompatibleServerReason),
    // Events related to ops
    UnprocessableMessage {
        index: IndexInt,
        sender: DeviceID,
        reason: UnprocessableMessageReason
    },
    // Events related to monitors
    CertificatesMonitorCrashed(anyhow::Error),
    InvalidCertificate(crate::certificates_ops::InvalidCertificateError),
    // Re-publishing of `events_listen`
    CertificatesUpdated { index: IndexInt },
    MessageReceived { index: IndexInt },
    InviteStatusChanged {
        invitation_status: InvitationStatus,
        token: InvitationToken
    },
    RealmMaintenanceStarted {
        encryption_revision: IndexInt,
        realm_id: RealmID
    },
    RealmMaintenanceFinished {
        encryption_revision: IndexInt,
        realm_id: RealmID
    },
    RealmVlobsUpdated {
        checkpoint: IndexInt,
        realm_id: RealmID,
        src_id: VlobID,
        src_version: VersionInt
    },
    PkiEnrollmentUpdated,
);

pub trait Broadcastable: Send
where
    Self: Sized,
{
    fn send(&self, event_bus: &EventBusInternal);
    fn connect(
        event_bus: Arc<EventBusInternal>,
        callback: Box<dyn Fn(&Self) + Send>,
    ) -> EventBusConnectionLifetime<Self>;
    fn disconnect(event_bus: &EventBusInternal, ptr: usize);
}

enum ServerState {
    Online,
    Offline,
}

struct ServerStateListener {
    watch: tokio::sync::watch::Receiver<ServerState>,
    _lifetimes: (
        EventBusConnectionLifetime<EventOnline>,
        EventBusConnectionLifetime<EventOffline>,
    ),
}

impl ServerStateListener {
    fn new(event_bus: &Arc<EventBusInternal>) -> Self {
        let (tx, rx) = tokio::sync::watch::channel(ServerState::Offline);
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

#[derive(Clone)]
pub struct EventBus {
    internal: Arc<EventBusInternal>,
    server_state: Arc<tokio::sync::Mutex<ServerStateListener>>,
}

impl Default for EventBus {
    fn default() -> Self {
        let internal = Arc::new(EventBusInternal::default());
        let server_state = Arc::new(tokio::sync::Mutex::new(ServerStateListener::new(&internal)));
        Self {
            internal,
            server_state,
        }
    }
}

impl EventBus {
    pub fn send(&self, event: &impl Broadcastable) {
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
