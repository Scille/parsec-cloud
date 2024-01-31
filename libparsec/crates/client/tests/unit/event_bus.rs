// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    cell::Cell,
    sync::{Arc, Mutex},
};

use libparsec_tests_fixtures::*;

use crate::{EventBus, EventOffline, EventOnline, EventPing};

#[test]
fn debug_format() {
    fn callback(_event: &EventPing) {}

    let eb = EventBus::default();

    // Online/Offline events are always registered internally
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

#[parsec_test]
async fn wait_server_online() {
    let eb = EventBus::default();

    for _ in 0..2 {
        let switched_to_online = Cell::new(false);
        libparsec_platform_async::future::join(
            // Event bus starts by considering we are offline...
            eb.wait_server_online(),
            async {
                // Yield multiple times so the other future can settle
                for _ in 0..10 {
                    libparsec_platform_async::sleep(std::time::Duration::from_millis(0)).await;
                }
                // ...until the first online event is received
                eb.send(&EventOnline);
                switched_to_online.set(true);
            },
        )
        .await;
        // Make sure `wait_server_online` resolved thanks to our event
        assert!(switched_to_online.get());

        // Now we are online
        eb.wait_server_online().await;

        // ...and switch back to offline and retry from the beginning !
        eb.send(&EventOffline);
    }
}
