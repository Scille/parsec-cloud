// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_client::{EventBus, EventPing};
use libparsec_tests_fixtures::*;

#[test]
fn debug_format() {
    fn callback(_event: &EventPing) {}

    let eb = EventBus::default();

    // Online/Offline events are allways registered internally
    p_assert_eq!(
        format!("{:?}", eb),
        "EventBus { on_offline_cbs: 1, on_online_cbs: 1 }"
    );

    let _lifetime = eb.connect(callback);

    p_assert_eq!(
        format!("{:?}", eb),
        "EventBus { on_ping_cbs: 1, on_offline_cbs: 1, on_online_cbs: 1 }"
    );
}

#[test]
fn static_callback() {
    fn callback(_event: &EventPing) {}

    let eb = EventBus::default();
    let lifetime = eb.connect(callback);
    eb.send(&EventPing {
        ping: "1".to_owned(),
    });

    // Also test with no events connected at all
    drop(lifetime);
    eb.send(&EventPing {
        ping: "2".to_owned(),
    });
}

#[test]
fn closure() {
    let closure_calls = Arc::new(Mutex::new(vec![]));
    let closure = {
        let closure_calls = closure_calls.clone();
        move |event: &EventPing| {
            let mut guard = closure_calls.lock().unwrap();
            guard.push(event.ping.clone());
        }
    };
    let eb = EventBus::default();

    // Simple connection & events

    let connection_lifetime = eb.connect(closure.clone());

    eb.send(&EventPing {
        ping: "1".to_owned(),
    });
    p_assert_eq!(closure_calls.lock().unwrap().as_ref(), ["1"]);
    eb.send(&EventPing {
        ping: "2".to_owned(),
    });
    eb.send(&EventPing {
        ping: "3".to_owned(),
    });
    p_assert_eq!(closure_calls.lock().unwrap().as_ref(), ["1", "2", "3"]);

    // Disconnect callback

    drop(connection_lifetime);
    eb.send(&EventPing {
        ping: "4".to_owned(),
    });
    p_assert_eq!(*closure_calls.lock().unwrap(), ["1", "2", "3"]);

    // Reconnect

    let _connection_lifetime = eb.connect(closure);
    eb.send(&EventPing {
        ping: "5".to_owned(),
    });
    p_assert_eq!(*closure_calls.lock().unwrap(), ["1", "2", "3", "5"]);

    // Multiple connections
    let closure2 = {
        let closure_calls = closure_calls.clone();
        move |event: &EventPing| {
            let mut guard = closure_calls.lock().unwrap();
            guard.push(format!("closure2:{}", &event.ping));
        }
    };
    let _connection_lifetime2 = eb.connect(closure2);
    eb.send(&EventPing {
        ping: "6".to_owned(),
    });
    p_assert_eq!(
        *closure_calls.lock().unwrap(),
        ["1", "2", "3", "5", "6", "closure2:6"]
    );
}
