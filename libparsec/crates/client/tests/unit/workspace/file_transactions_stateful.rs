// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proptest::{prelude::*, test_runner::Config};
use proptest_state_machine::{prop_state_machine, ReferenceStateMachine};
use std::{fs::File, os::unix::prelude::FileExt, path::PathBuf, sync::Arc};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    workspace::{
        tests::{utils::workspace_ops_factory, AsyncStateMachineTest},
        OpenOptions,
    },
    WorkspaceOps,
};

const MAX_SIZE: u64 = 64;

pub struct FileTransactionOracleStateMachine;

/// The possible transitions of the state machine.
#[derive(Clone, Debug)]
pub enum Transition {
    Read { size: u64, offset: u64 },
    Write { content: Vec<u8>, offset: u64 },
    Resize { length: u64 },
    Reopen,
}

pub struct FileTransactionStateMachine {
    ops: WorkspaceOps,
    fd: FileDescriptor,
    file: File,
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ReferenceState;

// Implementation of the reference state machine that drives the test. That is,
// it's used to generate a sequence of transitions the `StateMachineTest`.
impl ReferenceStateMachine for FileTransactionOracleStateMachine {
    type State = ReferenceState;
    type Transition = Transition;

    fn init_state() -> BoxedStrategy<Self::State> {
        Just(ReferenceState).boxed()
    }

    fn transitions(_state: &Self::State) -> BoxedStrategy<Self::Transition> {
        prop_oneof![
            (0..MAX_SIZE).prop_flat_map(|size| {
                (0..MAX_SIZE).prop_map(move |offset| Transition::Read { size, offset })
            }),
            (0..MAX_SIZE).prop_flat_map(|offset| {
                any_with::<Vec<u8>>(prop::collection::size_range(0..MAX_SIZE as usize).lift())
                    .prop_map(move |content| Transition::Write { content, offset })
            }),
            (0..MAX_SIZE).prop_map(|length| Transition::Resize { length }),
            Just(Transition::Reopen),
        ]
        .boxed()
    }

    fn apply(state: Self::State, _transition: &Self::Transition) -> Self::State {
        state
    }
}

impl FileTransactionStateMachine {
    async fn read(&self, size: u64, offset: u64) {
        // Oracle
        let mut expected = vec![0; size as usize];
        let expected_len = self.file.read_at(&mut expected, offset).unwrap();
        expected.truncate(expected_len);

        // SUT
        let mut data = Vec::with_capacity(size as usize);
        let data_len = self
            .ops
            .fd_read(self.fd, offset, size, &mut data)
            .await
            .unwrap();
        data.truncate(data_len as usize);

        p_assert_eq!(data_len, expected_len as u64);
        p_assert_eq!(data, expected);
    }

    async fn write(&self, content: &[u8], offset: u64) {
        // Oracle
        if !content.is_empty() {
            self.file.write_all_at(content, offset).unwrap();
        }

        // SUT
        let written_len = self.ops.fd_write(self.fd, offset, content).await.unwrap();

        p_assert_eq!(written_len, content.len() as u64);
    }

    async fn resize(&self, length: u64) {
        // Oracle
        self.file.set_len(length).unwrap();

        // SUT
        self.ops.fd_resize(self.fd, length, false).await.unwrap();
    }

    async fn reopen(&mut self) {
        // Oracle
        self.file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .unwrap();

        // SUT
        self.ops.fd_close(self.fd).await.unwrap();
        self.fd = self
            .ops
            .open_file(
                "/oracle-test.txt".parse().unwrap(),
                OpenOptions {
                    read: true,
                    write: true,
                    truncate: false,
                    create: false,
                    create_new: false,
                },
            )
            .await
            .unwrap();
    }
}

crate::impl_async_state_machine!(FileTransactionStateMachine);

impl AsyncStateMachineTest for FileTransactionStateMachine {
    type SystemUnderTest = (TestbedScope, Self);
    type Reference = FileTransactionOracleStateMachine;

    async fn init_test(
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        let testbed = TestbedScope::start("minimal", Run::WithoutServer)
            .await
            .unwrap();

        let url = ParsecOrganizationAddr::from_any(
            // cspell:disable-next-line
            "parsec3://test.invalid:6770/Org?no_ssl=true&p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA"
        ).unwrap();
        let device = LocalDevice::generate_new_device(
            url,
            UserProfile::Admin,
            HumanHandle::from_raw("alice@dev1", "alice").unwrap(),
            "alice label".parse().unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let realm_id = VlobID::default();

        let path = PathBuf::from(format!("/tmp/oracle-test-{realm_id}.txt",));
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();

        let mut r = Self {
            ops: workspace_ops_factory(&testbed.env.discriminant_dir, &Arc::new(device), realm_id)
                .await,
            fd: FileDescriptor(1),
            file,
            path,
        };

        let vlob_id = r
            .ops
            .create_file("/oracle-test.txt".parse().unwrap())
            .await
            .unwrap();

        let fd = r
            .ops
            .open_file_by_id(
                vlob_id,
                OpenOptions {
                    read: true,
                    write: true,
                    truncate: false,
                    create: false,
                    create_new: true,
                },
            )
            .await
            .unwrap();

        r.fd = fd;

        (testbed, r)
    }

    async fn apply(
        mut state: Self::SystemUnderTest,
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        transition: Transition,
    ) -> Self::SystemUnderTest {
        match transition {
            Transition::Reopen => {
                state.1.reopen().await;
            }
            Transition::Resize { length } => {
                state.1.resize(length).await;
            }
            Transition::Read { size, offset } => {
                state.1.read(size, offset).await;
            }
            Transition::Write { content, offset } => {
                state.1.write(&content, offset).await;
            }
        }
        state
    }

    async fn teardown(state: Self::SystemUnderTest) {
        state.1.ops.fd_close(state.1.fd).await.unwrap();
        std::fs::remove_file(state.1.path).unwrap();
        state.0.stop().await;
    }
}

prop_state_machine! {
    #![proptest_config(Config {
        cases: 200, // About 5 seconds
        ..Config::default()
    })]
    #[parsec_test]
    fn file_transactions_stateful_test(
        // This is a macro's keyword - only `sequential` is currently supported.
        sequential
        // The number of transitions to be generated for each case. This can
        // be a single numerical value or a range as in here.
        1..10
        // Macro's boilerplate to separate the following identifier.
        =>
        // The name of the type that implements `StateMachineTest`.
        FileTransactionStateMachine
    );
}
