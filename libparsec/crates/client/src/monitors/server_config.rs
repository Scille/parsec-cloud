// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, sync::Arc};

use libparsec_platform_async::{pretend_future_is_send_on_web, sleep};

use super::Monitor;
use crate::{
    event_bus::{EventBus, EventServerConfigChanged, EventServerConfigNotified},
    Client, ServerConfig,
};

const SERVER_CONFIG_MONITOR_NAME: &str = "server_config";

pub(crate) async fn start_server_config_monitor(
    client: Arc<Client>,
    event_bus: EventBus,
) -> Monitor {
    let future = {
        let task_future = task_future_factory(client, event_bus.clone());
        pretend_future_is_send_on_web(task_future)
    };
    Monitor::start(event_bus, SERVER_CONFIG_MONITOR_NAME, None, future, None).await
}

fn task_future_factory(client: Arc<Client>, event_bus: EventBus) -> impl Future<Output = ()> {
    let events_connection_lifetime = event_bus.connect({
        let event_bus = event_bus.clone();
        move |e: &EventServerConfigNotified| {
            let EventServerConfigNotified {
                active_users_limit,
                user_profile_outsider_allowed,
            } = e;
            let new = ServerConfig {
                active_users_limit: *active_users_limit,
                user_profile_outsider_allowed: *user_profile_outsider_allowed,
            };
            let mut config_has_changed = false;

            client.update_server_config(|config| {
                if *config != new {
                    *config = new;
                    config_has_changed = true;
                }
            });

            if config_has_changed {
                event_bus.send(&EventServerConfigChanged);
            }
        }
    });

    async move {
        // Our coroutine is only here to keep the event connection alive.
        let _events_connection_lifetime = events_connection_lifetime;
        // We want to sleep forever, but we can only do it for `Duration::MAX` (which is
        // very, very long but not infinite). Hence the loop, which is mostly theoretical.
        loop {
            sleep(std::time::Duration::MAX).await;
        }
    }
}
