// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use paste::paste;

macro_rules! impl_event_bus_internal_and_event_bus_debug {
    ($([$event_struct:ident, $field_on_event_cbs:ident])*) => {

        pub struct EventBusInternal {
            $(
                $field_on_event_cbs: Mutex<Vec<Box<dyn Fn(&$event_struct)>>>,
            )*
        }

        impl std::fmt::Debug for EventBus {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut f = f.debug_struct("EventBus");
                $(
                let count = self.0.$field_on_event_cbs
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

        impl Default for EventBus {
            fn default() -> Self {
                Self(Arc::new(EventBusInternal {
                    $(
                    $field_on_event_cbs: Mutex::new(vec![]),
                    )*
                }))
            }
        }
    };
}

macro_rules! impl_broadcastable {
    ($event_struct:ident, $field_on_event_cbs:ident) => {
        impl Broadcastable for $event_struct {
            fn send(&self, event_bus: &EventBus) {
                let guard = event_bus
                    .0
                    .$field_on_event_cbs
                    .lock()
                    .expect("Mutex is poisoned");
                for callback in guard.iter() {
                    callback(self);
                }
            }
            fn connect(
                event_bus: &EventBus,
                callback: Box<dyn Fn(&Self)>,
            ) -> EventBusConnectionLifetime<Self> {
                let mut guard = event_bus
                    .0
                    .$field_on_event_cbs
                    .lock()
                    .expect("Mutex is poisoned");
                let fatptr: *const _ = callback.as_ref();
                let ptr = fatptr as *const () as usize;
                guard.push(callback);
                EventBusConnectionLifetime {
                    ptr,
                    phantom: PhantomData,
                    event_bus: event_bus.0.clone(),
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
                        // So the solution is to manualy transform the fat pointer into
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

impl_events!(
    Ping { ping: String },
    Offline,
    Online,
    ExpiredOrganization,
    RevokedUser,
);

pub trait Broadcastable
where
    Self: Sized,
{
    fn send(&self, event_bus: &EventBus);
    fn connect(
        event_bus: &EventBus,
        callback: Box<dyn Fn(&Self)>,
    ) -> EventBusConnectionLifetime<Self>;
    fn disconnect(event_bus: &EventBusInternal, ptr: usize);
}

#[derive(Clone)]
pub struct EventBus(Arc<EventBusInternal>);

impl EventBus {
    pub fn send(&self, event: &impl Broadcastable) {
        event.send(self);
    }

    #[must_use]
    pub fn connect<F, B>(&self, callback: F) -> EventBusConnectionLifetime<B>
    where
        F: Fn(&B) + Send + 'static,
        B: Broadcastable + 'static,
    {
        B::connect(self, Box::new(callback))
    }
}

pub struct EventBusConnectionLifetime<B>
where
    B: Broadcastable,
{
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
