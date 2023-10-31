// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use paste::paste;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use crate::certificates_ops::InvalidMessageError;

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
    (@munch ( $event:ident, $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            #[derive(Debug, Clone)]
            pub struct [< Event $event >];
            impl_broadcastable!($event, [< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [$event, [< Event $event >], [< on_ $event:lower _cbs >]]));
        }
    };

    // e.g. Foo(u64)
    (@munch ( $event:ident ( $($ty:ty),* $(,)? ), $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            #[derive(Debug, Clone)]
            pub struct [< Event $event>]( $(pub $ty),* );
            impl_broadcastable!($event, [< Event $event >], [< on_ $event:lower _cbs >]);
            impl_events!(@munch ($($tail)*) -> ($($output)* [$event, [< Event $event >], [< on_ $event:lower _cbs >]]));
        }
    };

    // e.g. Foo{ bar: u64 }
    (@munch ( $event:ident { $($id:ident: $ty:ty),* $(,)? }, $($tail:tt)* ) -> ($($output:tt)*)) => {
        paste!{
            #[derive(Debug, Clone)]
            pub struct [< Event $event>] {
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
pub enum IncompatibleServerReason {
    UnsupportedApiVersion {
        api_version: ApiVersion,
        supported_api_versions: Vec<ApiVersion>,
    },
    Unexpected(Arc<anyhow::Error>),
}

// All those items will be named with a `Event` prefix (e.g. `Foo` => `EventFoo`)
// Note all errors in the events are wrapped with an `Arc`, this due to`anyhow::Error`
// not being `Clone`. If you wonder, performance impact should be minimal because:
// 1) Error-containing events are not often fired
// 2) `anyhow::Error` itself is just a pointer, so converting it to an Arc is cheap
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
    InvalidMessage {
        index: IndexInt,
        sender: DeviceID,
        reason: InvalidMessageError,
    },
    UserOpsSynced,
    UserOpsNeedSync,
    UserOpsWorkspaceCreated {
        name: EntryName,
        id: VlobID,
    },
    WorkspaceOpsOutboundSyncNeeded {
        realm_id: VlobID,
        entry_id: VlobID,
    },
    // Events related to monitors
    CertificatesMonitorCrashed(Arc<anyhow::Error>),
    MessagesMonitorCrashed(Arc<anyhow::Error>),
    InvalidCertificate(crate::certificates_ops::InvalidCertificateError),
    UserSyncMonitorCrashed(Arc<anyhow::Error>),
    WorkspaceInboundSyncMonitorCrashed(Arc<anyhow::Error>),
    WorkspaceOutboundSyncMonitorCrashed(Arc<anyhow::Error>),
    // Re-publishing of `events_listen`
    CertificatesUpdated { index: IndexInt },
    MessageReceived { index: IndexInt },
    InviteStatusChanged {
        invitation_status: InvitationStatus,
        token: InvitationToken
    },
    RealmMaintenanceStarted {
        encryption_revision: IndexInt,
        realm_id: VlobID
    },
    RealmMaintenanceFinished {
        encryption_revision: IndexInt,
        realm_id: VlobID
    },
    RealmVlobsUpdated {
        checkpoint: IndexInt,
        realm_id: VlobID,
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

    struct EventBusSpyInternal {
        events: Vec<AnySpiedEvent>,
        on_new_event: Option<Event>,
    }

    #[derive(Clone)]
    pub struct EventBusSpy {
        internal: Arc<Mutex<EventBusSpyInternal>>,
    }

    impl EventBusSpy {
        pub(super) fn new() -> Self {
            Self {
                internal: Arc::new(Mutex::new(EventBusSpyInternal {
                    events: Default::default(),
                    on_new_event: None,
                })),
            }
        }
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
            spy: EventBusSpy::new(),
        }
    }
}

impl EventBus {
    pub fn send(&self, event: &impl Broadcastable) {
        #[cfg(test)]
        {
            self.spy.add(event.to_any_spied_event());
        }
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
