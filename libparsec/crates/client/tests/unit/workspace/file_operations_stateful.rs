// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proptest::prelude::*;
use proptest::test_runner::Config;
use proptest_state_machine::{prop_state_machine, ReferenceStateMachine, StateMachineTest};
use std::io::{BufReader, BufWriter, Cursor, Read, SeekFrom, Write};
use std::{collections::HashMap, io::Seek, str::FromStr};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::transactions::{
    prepare_read, prepare_reshape, prepare_resize, prepare_write,
};

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

// Implementation of the reference state machine that drives the test. That is,
// it's used to generate a sequence of transitions the `StateMachineTest`.
impl ReferenceStateMachine for FileOperationOracleStateMachine {
    type State = std::io::Cursor<Vec<u8>>;
    type Transition = Transition;

    fn init_state() -> BoxedStrategy<Self::State> {
        Just(std::io::Cursor::new(vec![])).boxed()
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
            Transition::Read { .. } => (),
            //     c.seek(SeekFrom::Start(*offset)).unwrap();
            //     c.take(size).read_to_end(buf)
            // }
            Transition::Write { content, offset } => {
                state.seek(SeekFrom::Start(*offset)).unwrap();
                state.write(&content).unwrap();
            }
            Transition::Resize { length } => {
                let mut buf = state.into_inner();
                buf.resize(*length as usize, 0);
                state = Cursor::new(buf);
            }
            Transition::Reshape => (),
        }
        state
    }
}

impl StateMachineTest for MyHeap<i32> {
    type SystemUnderTest = Self;
    type Reference = HeapStateMachine;

    fn init_test(
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) -> Self::SystemUnderTest {
        MyHeap::new()
    }

    fn apply(
        mut state: Self::SystemUnderTest,
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
        transition: Transition,
    ) -> Self::SystemUnderTest {
        match transition {
            Transition::Pop => {
                // We read the state before applying the transition.
                let was_empty = state.is_empty();

                // We use the broken implementation of pop, which should be
                // discovered by the test.
                let result = state.pop_wrong();

                // NOTE: To fix the issue that gets found by the state machine,
                // you can comment out the last statement with `pop_wrong` and
                // uncomment this one to see the test pass:
                // let result = state.pop();

                // Check a post-condition.
                match result {
                    Some(value) => {
                        assert!(!was_empty);
                        // The heap must not contain any value which was
                        // greater than the "maximum" we were just given.
                        for in_heap in state.iter() {
                            assert!(
                                value >= *in_heap,
                                "Popped value {:?}, which was less \
                                    than {:?} still in the heap",
                                value,
                                in_heap
                            );
                        }
                    }
                    None => assert!(was_empty),
                }
            }
            Transition::Push(value) => state.push(value),
        }
        state
    }

    fn check_invariants(
        state: &Self::SystemUnderTest,
        _ref_state: &<Self::Reference as ReferenceStateMachine>::State,
    ) {
        // Check that the heap's API gives consistent results
        assert_eq!(0 == state.len(), state.is_empty());
    }
}

// #[parsec_test]
// fn stateful() {

//     prop_state_machine! {
//         #![proptest_config(Config {
//             // Turn failure persistence off for demonstration. This means that no
//             // regression file will be captured.
//             failure_persistence: None,
//             // Enable verbose mode to make the state machine test print the
//             // transitions for each case.
//             verbose: 1,
//             .. Config::default()
//         })]

//         // NOTE: The `#[test]` attribute is commented out in here so we can run it
//         // as an example from the `fn main`.

//         // #[test]
//         fn run_my_heap_test(
//             // This is a macro's keyword - only `sequential` is currently supported.
//             sequential
//             // The number of transitions to be generated for each case. This can
//             // be a single numerical value or a range as in here.
//             1..20
//             // Macro's boilerplate to separate the following identifier.
//             =>
//             // The name of the type that implements `StateMachineTest`.
//             MyHeap<i32>
//         );
//     }

//     // Initialize storage and manifest
//     let mut storage = Storage::default();
//     let blocksize = Blocksize::try_from(16).unwrap();
//     let t1 = DateTime::from_str("2000-01-01 01:00:00 UTC").unwrap();
//     let mut manifest = LocalFileManifest::new(DeviceID::default(), VlobID::default(), t1);
//     manifest.blocksize = blocksize;
//     manifest.base.blocksize = blocksize;
// }
