// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proptest::prelude::*;
use proptest::test_runner::Config;
use proptest_state_machine::{prop_state_machine, ReferenceStateMachine, StateMachineTest};
use std::collections::HashSet;
use std::io::Seek;
use std::io::{Cursor, Read, SeekFrom, Write};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::file_operations::Storage;

const MAX_SIZE: u64 = 64;

pub struct FileOperationOracleStateMachine;

/// The possible transitions of the state machine.
#[derive(Clone, Debug)]
pub enum Transition {
    Read { size: u64, offset: u64 },
    Write { content: Vec<u8>, offset: u64 },
    Resize { length: u64 },
    Reshape,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ReferenceState {
    cursor: std::io::Cursor<Vec<u8>>,
    data: Vec<u8>,
}

// Implementation of the reference state machine that drives the test. That is,
// it's used to generate a sequence of transitions the `StateMachineTest`.
impl ReferenceStateMachine for FileOperationOracleStateMachine {
    type State = ReferenceState;
    type Transition = Transition;

    fn init_state() -> BoxedStrategy<Self::State> {
        Just(ReferenceState::default()).boxed()
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
            Just(Transition::Reshape),
        ]
        .boxed()
    }

    fn apply(mut state: Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            Transition::Read { size, offset } => {
                state.data.resize(*size as usize, 0);
                state.cursor.seek(SeekFrom::Start(*offset)).unwrap();
                let n = state.cursor.read(&mut state.data).unwrap();
                state.data.truncate(n);
            }
            Transition::Write { content, offset } => {
                if !content.is_empty() {
                    state.cursor.seek(SeekFrom::Start(*offset)).unwrap();
                    state.cursor.write_all(content).unwrap();
                }
            }
            Transition::Resize { length } => {
                let mut buf = state.cursor.into_inner();
                buf.resize(*length as usize, 0);
                state.cursor = Cursor::new(buf);
            }
            Transition::Reshape => (),
        }
        state.cursor.seek(SeekFrom::End(0)).unwrap();
        state
    }
}

pub struct FileOperationStateMachine {
    storage: Storage,
    manifest: LocalFileManifest,
    device_id: DeviceID,
    time_provider: TimeProvider,
}

impl FileOperationStateMachine {
    fn read(&self, size: u64, offset: u64, expected: &[u8]) {
        let data = self.storage.read(&self.manifest, size, offset);
        assert_eq!(data, expected);
    }
    fn resize(&mut self, length: u64) {
        let timestamp = self.time_provider.now();
        self.storage.resize(&mut self.manifest, length, timestamp);
    }

    fn write(&mut self, content: &[u8], offset: u64) {
        let timestamp = self.time_provider.now();
        self.storage
            .write(&mut self.manifest, content, offset, timestamp);
    }

    fn reshape(&mut self) {
        self.storage.reshape(&mut self.manifest);
        assert!(self.manifest.is_reshaped());
    }
}

pub trait AsyncStateMachineTest {
    /// The concrete state, that is the system under test (SUT).
    type SystemUnderTest;

    /// The abstract state machine that implements [`ReferenceStateMachine`]
    /// drives the generation of the state machine's transitions.
    type Reference: ReferenceStateMachine;

    /// Initialize the state of SUT.
    ///
    /// If the reference state machine is generated from a non-constant
    /// strategy, ensure to use it to initialize the SUT to a corresponding
    /// state.
    async fn init_test(
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest;

    /// Apply a transition in the SUT state and check post-conditions.
    /// The post-conditions are properties of your state machine that you want
    /// to assert.
    ///
    /// Note that the `ref_state` is the state *after* this `transition` is
    /// applied. You can use it to compare it with your SUT after you apply
    /// the transition.
    async fn apply(
        state: Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        transition: <Self::Reference as ReferenceStateMachine>::Transition,
    ) -> Self::SystemUnderTest;

    /// Check some invariant on the SUT state after every transition.
    ///
    /// Note that just like in [`StateMachineTest::apply`] you can use
    /// the `ref_state` to compare it with your SUT.
    async fn check_invariants(
        state: &Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) {
        // This is to avoid `unused_variables` warning
        let _ = (state, ref_state);
    }

    /// Override this function to add some teardown logic on the SUT state
    /// at the end of each test case. The default implementation simply drops
    /// the state.
    async fn teardown(state: Self::SystemUnderTest) {
        // This is to avoid `unused_variables` warning
        let _ = state;
    }
}

macro_rules! impl_state_machine_for_async {
    ($t:ty) => {
        impl StateMachineTest for $t {
            type SystemUnderTest = (
                tokio::runtime::Runtime,
                <Self as AsyncStateMachineTest>::SystemUnderTest,
            );
            type Reference = <Self as AsyncStateMachineTest>::Reference;

            fn init_test(
                ref_state: &<Self::Reference as ReferenceStateMachine>::State,
            ) -> Self::SystemUnderTest {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                let state = rt.block_on(async {
                    <Self as AsyncStateMachineTest>::init_test(ref_state).await
                });
                (rt, state)
            }

            fn apply(
                state: Self::SystemUnderTest,
                ref_state: &<Self::Reference as ReferenceStateMachine>::State,
                transition: <Self::Reference as ReferenceStateMachine>::Transition,
            ) -> Self::SystemUnderTest {
                let (rt, state) = state;
                let state = rt.block_on(async {
                    <Self as AsyncStateMachineTest>::apply(state, ref_state, transition).await
                });
                (rt, state)
            }

            fn check_invariants(
                state: &Self::SystemUnderTest,
                ref_state: &<Self::Reference as ReferenceStateMachine>::State,
            ) {
                let (rt, state) = state;
                rt.block_on(async {
                    <Self as AsyncStateMachineTest>::check_invariants(state, ref_state).await
                });
            }

            fn teardown(state: Self::SystemUnderTest) {
                let (rt, state) = state;
                rt.block_on(async { <Self as AsyncStateMachineTest>::teardown(state).await });
            }
        }
    };
}

impl_state_machine_for_async!(FileOperationStateMachine);

impl AsyncStateMachineTest for FileOperationStateMachine {
    type SystemUnderTest = Self;
    type Reference = FileOperationOracleStateMachine;

    async fn init_test(
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        let time_provider = TimeProvider::default();
        let device_id = DeviceID::default();
        let mut manifest =
            LocalFileManifest::new(device_id.clone(), VlobID::default(), time_provider.now());
        manifest.blocksize = Blocksize::try_from(8).unwrap();
        manifest.base.blocksize = manifest.blocksize;
        FileOperationStateMachine {
            storage: Storage::default(),
            manifest,
            device_id,
            time_provider,
        }
    }

    async fn apply(
        mut state: Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        transition: Transition,
    ) -> Self::SystemUnderTest {
        match transition {
            Transition::Reshape => {
                state.reshape();
            }
            Transition::Resize { length } => {
                state.resize(length);
            }
            Transition::Read { size, offset } => {
                state.read(size, offset, &ref_state.data);
            }
            Transition::Write { content, offset } => {
                state.write(&content, offset);
            }
        }
        state
    }

    async fn check_invariants(
        state: &Self::SystemUnderTest,
        ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) {
        // 1. Manifest integrity
        state.manifest.assert_integrity();
        // 2. Same size for the manifest and the cursor
        assert_eq!(ref_state.cursor.position(), state.manifest.size);
        // 3. Remote conversion is OK
        if state.manifest.is_reshaped() {
            let remote = state
                .manifest
                .to_remote(state.device_id.clone(), state.time_provider.now())
                .unwrap();
            LocalFileManifest::from_remote(remote).assert_integrity();
        }
        // 3. No corruption or leaks in the storage
        let manifest_ids: HashSet<_> = state
            .manifest
            .blocks
            .iter()
            .flat_map(|b| b.iter().map(|c| c.id))
            .collect();
        let storage_ids: HashSet<_> = state.storage.0.keys().cloned().collect();
        assert_eq!(manifest_ids, storage_ids);
    }
}

prop_state_machine! {
    #![proptest_config(Config {
        // Enable verbose mode to make the state machine test print the
        // transitions for each case.
        verbose: 0,
        .. Config::default()
    })]
    #[parsec_test]
    fn file_operations_stateful_test(
        // This is a macro's keyword - only `sequential` is currently supported.
        sequential
        // The number of transitions to be generated for each case. This can
        // be a single numerical value or a range as in here.
        1..20
        // Macro's boilerplate to separate the following identifier.
        =>
        // The name of the type that implements `StateMachineTest`.
        FileOperationStateMachine
    );
}
