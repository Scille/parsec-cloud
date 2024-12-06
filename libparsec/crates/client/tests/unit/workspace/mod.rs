// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(unix)]
use proptest_state_machine::ReferenceStateMachine;

mod base;
mod create_file;
mod create_folder;
mod create_folder_all;
mod fd_close;
mod fd_flush;
mod fd_read;
mod fd_write;
mod file_operations;
mod file_operations_stateful;
mod folder_transactions;
mod history;
mod inbound_sync_file;
mod inbound_sync_folder;
mod inbound_sync_root;
mod link;
mod merge_file;
mod merge_folder;
mod move_entry;
mod open_file;
mod outbound_sync_file;
mod outbound_sync_folder;
mod outbound_sync_root;
mod read_folder;
mod remove_entry;
mod resolve_path;
mod retrieve_path_from_id;
mod stat_entry;
mod store;
mod utils;
mod watch_entry;

#[cfg(unix)]
mod file_transactions_stateful;

#[cfg(unix)]
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

#[macro_export]
macro_rules! impl_async_state_machine {
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
                rt.block_on(async {
                    <Self as AsyncStateMachineTest>::teardown(state).await;
                });
            }
        }
    };
}
