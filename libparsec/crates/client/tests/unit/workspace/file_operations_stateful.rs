// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proptest::prelude::*;
use proptest::test_runner::Config;
use proptest_state_machine::{prop_state_machine, ReferenceStateMachine, StateMachineTest};
use std::collections::HashSet;
use std::io::{Cursor, Read, SeekFrom, Write};
use std::{io::Seek, str::FromStr};

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
                    state.cursor.write(&content).unwrap();
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
}

impl FileOperationStateMachine {
    fn read(&self, size: u64, offset: u64, expected: &[u8]) {
        let data = self.storage.read(&self.manifest, size, offset);
        assert_eq!(data, expected);
    }
    fn resize(&mut self, length: u64) {
        let timestamp = DateTime::from_str("2000-01-01 01:00:00 UTC").unwrap();
        self.storage.resize(&mut self.manifest, length, timestamp);
    }

    fn write(&mut self, content: &[u8], offset: u64) {
        let timestamp = DateTime::from_str("2000-01-01 01:00:00 UTC").unwrap();
        self.storage
            .write(&mut self.manifest, content, offset, timestamp);
    }

    fn reshape(&mut self) {
        self.storage.reshape(&mut self.manifest);
        assert!(self.manifest.is_reshaped());
    }
}

impl StateMachineTest for FileOperationStateMachine {
    type SystemUnderTest = Self;
    type Reference = FileOperationOracleStateMachine;

    fn init_test(
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        let mut manifest = LocalFileManifest::new(
            DeviceID::default(),
            VlobID::default(),
            DateTime::from_str("2000-01-01 01:00:00 UTC").unwrap(),
        );
        manifest.blocksize = Blocksize::try_from(8).unwrap();
        manifest.base.blocksize = manifest.blocksize;
        FileOperationStateMachine {
            storage: Storage::default(),
            manifest,
        }
    }

    fn apply(
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

    fn check_invariants(
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
                .to_remote(
                    DeviceID::default(),
                    DateTime::from_str("2000-01-01 01:00:00 UTC").unwrap(),
                )
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
        // Turn failure persistence off for demonstration. This means that no
        // regression file will be captured.
        failure_persistence: None,
        // Enable verbose mode to make the state machine test print the
        // transitions for each case.
        verbose: 0,
        .. Config::default()
    })]

    // NOTE: The `#[test]` attribute is commented out in here so we can run it
    // as an example from the `fn main`.

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
