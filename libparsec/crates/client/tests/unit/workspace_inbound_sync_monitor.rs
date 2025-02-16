// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError, ProxyConfig};
use libparsec_platform_async::prelude::*;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::{
    inbound_sync_monitor_loop, InboundSyncManagerIO, IncomingEvent, WaitForNextIncomingEventOutcome,
};
use crate::event_bus::AnySpiedEvent;
use crate::monitors::workspace_inbound_sync::RealInboundSyncManagerIO;
use crate::workspace::{
    InboundSyncOutcome, WorkspaceExternalInfo, WorkspaceGetNeedInboundSyncEntriesError,
    WorkspaceSyncError,
};
use crate::{
    CertificateOps, ClientConfig, EventBus, MountpointMountStrategy, WorkspaceOps,
    WorkspaceStorageCacheSize,
};

/*
 * Important stuff
 */

/// Optional callback to modify the outcome of the event
type MaybeSideEffect<R> = Option<Box<dyn FnOnce(&mut R) + Send + Sync + 'static>>;
/// Optional callback (taking an additional `expect_xxx` param) to modify the outcome of the event
type MaybeSideEffectP1<P, R> = Option<Box<dyn FnOnce(P, &mut R) + Send + Sync + 'static>>;

#[allow(dead_code)]
enum InboundSyncMonitorEvent {
    WaitForNextIncomingEvent {
        outcome: WaitForNextIncomingEventOutcome,
        side_effect: MaybeSideEffect<WaitForNextIncomingEventOutcome>,
    },
    RetryLaterBusyEntry {
        expected_entry_id: VlobID,
        side_effect: MaybeSideEffectP1<VlobID, ()>,
    },
    EventBusWaitServerOnline {
        side_effect: MaybeSideEffect<()>,
    },
    EventBusSend {
        // No need for `side_effect` here given `assert_event` can be used for that instead
        assert_event: Box<dyn FnOnce(AnySpiedEvent) + Send + Sync + 'static>,
    },
    WorkspaceOpsRefreshRealmCheckpoint {
        outcome: Result<(), WorkspaceSyncError>,
        side_effect: MaybeSideEffect<Result<(), WorkspaceSyncError>>,
    },
    WorkspaceOpsGetNeedInboundSync {
        expected_limit: u32,
        outcome: Result<Vec<VlobID>, WorkspaceGetNeedInboundSyncEntriesError>,
        side_effect:
            MaybeSideEffectP1<u32, Result<Vec<VlobID>, WorkspaceGetNeedInboundSyncEntriesError>>,
    },
    WorkspaceOpsInboundSync {
        expected_entry_id: VlobID,
        outcome: Result<InboundSyncOutcome, WorkspaceSyncError>,
        side_effect: MaybeSideEffectP1<VlobID, Result<InboundSyncOutcome, WorkspaceSyncError>>,
    },
}

enum TestcaseRunOutcome {
    MonitorHasStopped,
    MonitorWasCancelled,
}

/*
 * The actual tests
 */

#[parsec_test]
async fn on_missed_server_events() {
    run_testcase(
        [
            InboundSyncMonitorEvent::WaitForNextIncomingEvent {
                outcome: WaitForNextIncomingEventOutcome::NewEvent(
                    IncomingEvent::MissedServerEvents,
                ),
                side_effect: None,
            },
            InboundSyncMonitorEvent::WorkspaceOpsRefreshRealmCheckpoint {
                outcome: Ok(()),
                side_effect: None,
            },
            InboundSyncMonitorEvent::WorkspaceOpsGetNeedInboundSync {
                expected_limit: u32::MAX,
                outcome: Ok(vec![]),
                side_effect: None,
            },
        ],
        TestcaseRunOutcome::MonitorWasCancelled,
    )
    .await
}

#[parsec_test]
async fn wait_for_next_incoming_event_error() {
    run_testcase(
        [InboundSyncMonitorEvent::WaitForNextIncomingEvent {
            outcome: WaitForNextIncomingEventOutcome::Disconnected,
            side_effect: None,
        }],
        TestcaseRunOutcome::MonitorHasStopped,
    )
    .await
}

#[parsec_test]
async fn refresh_realm_checkpoint_offline() {
    run_testcase(
        [
            InboundSyncMonitorEvent::WaitForNextIncomingEvent {
                outcome: WaitForNextIncomingEventOutcome::NewEvent(
                    IncomingEvent::MissedServerEvents,
                ),
                side_effect: None,
            },
            InboundSyncMonitorEvent::WorkspaceOpsRefreshRealmCheckpoint {
                outcome: Err(WorkspaceSyncError::Offline(ConnectionError::NoResponse(
                    None,
                ))),
                side_effect: None,
            },
        ],
        TestcaseRunOutcome::MonitorWasCancelled,
    )
    .await
}

#[parsec_test]
async fn refresh_realm_checkpoint_stopped() {
    run_testcase(
        [
            InboundSyncMonitorEvent::WaitForNextIncomingEvent {
                outcome: WaitForNextIncomingEventOutcome::NewEvent(
                    IncomingEvent::MissedServerEvents,
                ),
                side_effect: None,
            },
            InboundSyncMonitorEvent::WorkspaceOpsRefreshRealmCheckpoint {
                outcome: Err(WorkspaceSyncError::Stopped),
                side_effect: None,
            },
        ],
        TestcaseRunOutcome::MonitorHasStopped,
    )
    .await
}

#[parsec_test]
async fn get_need_inbound_sync_stopped() {
    run_testcase(
        [
            InboundSyncMonitorEvent::WaitForNextIncomingEvent {
                outcome: WaitForNextIncomingEventOutcome::NewEvent(
                    IncomingEvent::MissedServerEvents,
                ),
                side_effect: None,
            },
            InboundSyncMonitorEvent::WorkspaceOpsRefreshRealmCheckpoint {
                outcome: Ok(()),
                side_effect: None,
            },
            InboundSyncMonitorEvent::WorkspaceOpsGetNeedInboundSync {
                expected_limit: u32::MAX,
                outcome: Err(WorkspaceGetNeedInboundSyncEntriesError::Stopped),
                side_effect: None,
            },
        ],
        TestcaseRunOutcome::MonitorHasStopped,
    )
    .await
}

#[parsec_test(testbed = "coolorg")]
async fn real_io_provides_a_starting_event(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.to_owned(),
        data_base_dir: env.discriminant_dir.to_owned(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });
    let event_bus = EventBus::default();
    let cmds = Arc::new(
        AuthenticatedCmds::new(&config.config_dir, alice.clone(), config.proxy.clone()).unwrap(),
    );
    let certificates_ops = Arc::new(
        CertificateOps::start(
            config.clone(),
            alice.clone(),
            event_bus.clone(),
            cmds.clone(),
        )
        .await
        .unwrap(),
    );
    let workspace_ops = Arc::new(
        WorkspaceOps::start(
            config.clone(),
            alice.clone(),
            cmds,
            certificates_ops,
            event_bus.clone(),
            wksp1_id,
            WorkspaceExternalInfo {
                entry: LocalUserManifestWorkspaceEntry {
                    id: wksp1_id,
                    name: "wksp1".parse().unwrap(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: RealmRole::Owner,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                },
                workspace_index: 0,
                total_workspaces: 1,
            },
        )
        .await
        .unwrap(),
    );

    let io = RealInboundSyncManagerIO::new(workspace_ops, event_bus);
    let first_event = io.wait_for_next_incoming_event().await;
    p_assert_matches!(
        first_event,
        WaitForNextIncomingEventOutcome::NewEvent(IncomingEvent::MissedServerEvents)
    );
}

/*
 * Boring stuff: test runner & mocked `InboundSyncManagerIO` implementation
 */

async fn run_testcase(
    expected_events: impl IntoIterator<Item = InboundSyncMonitorEvent>,
    expected_outcome: TestcaseRunOutcome,
) {
    let task_abort_handle = Arc::new(Mutex::new(Option::<AbortHandle>::None));

    let io = MockedInboundSyncManagerIO {
        internal: Mutex::new(MockedInboundSyncManagerIOInternal {
            expected_events: VecDeque::from_iter(expected_events),
            abort_task: Some({
                let task_abort_handle = task_abort_handle.clone();
                Box::new(move || {
                    task_abort_handle
                        .lock()
                        .expect("Mutex is poisoned")
                        .take()
                        // Given we run the test in a mono-threaded runtime: the task can
                        // only start to run after it creator's coroutine has yield, which
                        // is done *after* `task_abort_handle` is set.
                        .expect("Task not started")
                        .abort();
                })
            }),
        }),
    };

    let realm_id = VlobID::default();
    let task = spawn(inbound_sync_monitor_loop(realm_id, io));
    // Set `task_abort_handle` should be done right after the task is created (and with
    // no await in-between, see above) !
    task_abort_handle
        .lock()
        .expect("Mutex is poisoned")
        .replace(task.abort_handle());

    match (task.await, expected_outcome) {
        (Err(err), _) if err.is_panic() => {
            // Resume the panic on the main task
            std::panic::resume_unwind(err.into_panic());
        }
        (Ok(()), TestcaseRunOutcome::MonitorWasCancelled) => {
            panic!("Expected monitor to be cancelled, but it has returned !")
        }
        (Err(err), TestcaseRunOutcome::MonitorHasStopped) => {
            panic!(
                "Expected monitor to return, but it has been cancelled ({:?}) !",
                err
            )
        }
        _ => (),
    }
}

impl std::fmt::Debug for InboundSyncMonitorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InboundSyncMonitorEvent::WaitForNextIncomingEvent { outcome, .. } => f
                .debug_struct("WaitForNextIncomingEvent")
                .field("outcome", outcome)
                .finish_non_exhaustive(),
            InboundSyncMonitorEvent::RetryLaterBusyEntry {
                expected_entry_id, ..
            } => f
                .debug_struct("RetryLaterBusyEntry")
                .field("expected_entry_id", expected_entry_id)
                .finish_non_exhaustive(),
            Self::EventBusWaitServerOnline { .. } => f
                .debug_struct("EventBusWaitServerOnline")
                .finish_non_exhaustive(),
            Self::EventBusSend { .. } => f.debug_struct("EventBusSend").finish_non_exhaustive(),
            Self::WorkspaceOpsRefreshRealmCheckpoint { outcome, .. } => f
                .debug_struct("RefreshRealmCheckpoint")
                .field("outcome", outcome)
                .finish_non_exhaustive(),
            Self::WorkspaceOpsGetNeedInboundSync {
                expected_limit,
                outcome,
                ..
            } => f
                .debug_struct("GetNeedInboundSync")
                .field("expected_limit", expected_limit)
                .field("outcome", outcome)
                .finish_non_exhaustive(),
            Self::WorkspaceOpsInboundSync {
                expected_entry_id,
                outcome,
                ..
            } => f
                .debug_struct("InboundSync")
                .field("expected_entry_id", expected_entry_id)
                .field("outcome", outcome)
                .finish_non_exhaustive(),
        }
    }
}

struct MockedInboundSyncManagerIOInternal {
    expected_events: VecDeque<InboundSyncMonitorEvent>,
    abort_task: Option<Box<dyn FnOnce() + Send + 'static>>,
}

struct MockedInboundSyncManagerIO {
    internal: Mutex<MockedInboundSyncManagerIOInternal>,
}

impl MockedInboundSyncManagerIO {
    async fn pop_next_expected_event(&self) -> InboundSyncMonitorEvent {
        let next_expected_event = {
            let mut internal = self.internal.lock().expect("Mutex is poisoned");
            internal.expected_events.pop_front()
        };
        match next_expected_event {
            Some(next_expected_event) => next_expected_event,
            None => {
                {
                    let mut internal = self.internal.lock().expect("Mutex is poisoned");
                    let abort_task = internal
                        .abort_task
                        .take()
                        .expect("`abort_task()` already called !");
                    abort_task();
                }
                // Abort is a fire-and-forget call, we then sleep forever and wait for our
                // task to actually get aborted.
                future::pending::<()>().await;
                unreachable!()
            }
        }
    }
}

impl InboundSyncManagerIO for MockedInboundSyncManagerIO {
    async fn wait_for_next_incoming_event(&self) -> super::WaitForNextIncomingEventOutcome {
        println!(">>> wait_for_next_incoming_event()");
        let next_expected_events = self.pop_next_expected_event().await;
        match next_expected_events {
            InboundSyncMonitorEvent::WaitForNextIncomingEvent {
                mut outcome,
                side_effect,
            } => {
                if let Some(side_effect) = side_effect {
                    side_effect(&mut outcome);
                }
                outcome
            }
            expected => panic!("The unexpected occured ! Expected {:?}", expected),
        }
    }

    async fn retry_later_busy_entry(&mut self, entry_id: VlobID) {
        println!(">>> retry_later_busy_entry({:?})", entry_id);
        let next_expected_events = self.pop_next_expected_event().await;
        match next_expected_events {
            InboundSyncMonitorEvent::RetryLaterBusyEntry {
                expected_entry_id,
                side_effect,
            } => {
                p_assert_eq!(entry_id, expected_entry_id);
                if let Some(side_effect) = side_effect {
                    side_effect(entry_id, &mut ());
                }
            }
            expected => panic!("The unexpected occured ! Expected {:?}", expected),
        }
    }
    async fn event_bus_wait_server_reconnect(&self) {
        println!(">>> event_bus_wait_server_reconnect()");
        let next_expected_events = self.pop_next_expected_event().await;
        match next_expected_events {
            InboundSyncMonitorEvent::EventBusWaitServerOnline { side_effect } => {
                if let Some(side_effect) = side_effect {
                    side_effect(&mut ());
                }
            }
            expected => panic!("The unexpected occured ! Expected {:?}", expected),
        }
    }

    async fn event_bus_send(&self, event: &impl crate::Broadcastable) {
        println!(">>> event_bus_send({:?})", event);
        let next_expected_events = self.pop_next_expected_event().await;
        match next_expected_events {
            InboundSyncMonitorEvent::EventBusSend { assert_event } => {
                assert_event(event.to_any_spied_event());
            }
            expected => panic!("The unexpected occured ! Expected {:?}", expected),
        }
    }

    async fn workspace_ops_refresh_realm_checkpoint(&self) -> Result<(), WorkspaceSyncError> {
        println!(">>> workspace_ops_refresh_realm_checkpoint()");
        let next_expected_events = self.pop_next_expected_event().await;
        match next_expected_events {
            InboundSyncMonitorEvent::WorkspaceOpsRefreshRealmCheckpoint {
                mut outcome,
                side_effect,
            } => {
                if let Some(side_effect) = side_effect {
                    side_effect(&mut outcome);
                }
                outcome
            }
            expected => panic!("The unexpected occured ! Expected {:?}", expected),
        }
    }

    async fn workspace_ops_get_need_inbound_sync(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, WorkspaceGetNeedInboundSyncEntriesError> {
        println!(">>> workspace_ops_get_need_inbound_sync({})", limit);
        let next_expected_events = self.pop_next_expected_event().await;
        match next_expected_events {
            InboundSyncMonitorEvent::WorkspaceOpsGetNeedInboundSync {
                expected_limit,
                mut outcome,
                side_effect,
            } => {
                p_assert_eq!(limit, expected_limit);
                if let Some(side_effect) = side_effect {
                    side_effect(limit, &mut outcome);
                }
                outcome
            }
            expected => panic!("The unexpected occured ! Expected {:?}", expected),
        }
    }

    async fn workspace_ops_inbound_sync(
        &self,
        entry_id: VlobID,
    ) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
        println!(">>> workspace_ops_inbound_sync({:?})", entry_id);
        let next_expected_events = self.pop_next_expected_event().await;
        match next_expected_events {
            InboundSyncMonitorEvent::WorkspaceOpsInboundSync {
                expected_entry_id,
                mut outcome,
                side_effect,
            } => {
                p_assert_eq!(entry_id, expected_entry_id);
                if let Some(side_effect) = side_effect {
                    side_effect(entry_id, &mut outcome);
                }
                outcome
            }
            expected => panic!("The unexpected occured ! Expected {:?}", expected),
        }
    }
}
