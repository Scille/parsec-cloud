// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use super::{spawn, Task};
use futures::future::{join_all, select_all};
use std::future::Future;

/// A collection of tasks spawned.
///
/// A `JoinSet` can be used to await the completion of some or all of the tasks in the set.
/// The set is not ordered, and the tasks will be returned in the order they complete.
///
/// All of the tasks must have the same return type `T`.
///
/// When the `JoinSet` is dropped, all the tasks in the `JoinSet` are immediately aborted.
#[derive(Default)]
pub struct JoinSet<T> {
    tasks: Vec<Task<T>>,
}

impl<T> JoinSet<T> {
    /// Spawn the provided task on the `JoinSet`.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = T>,
        F: 'static,
        T: 'static,
    {
        let task = spawn(future);

        self.tasks.push(task);
    }

    /// Returns the number of tasks currently in the `JoinSet`.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Returns whether the `JoinSet` is empty
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Aborts all tasks on this `JoinSet`.
    ///
    /// This does not remove the tasks from the `JoinSet`
    pub fn abort_all(&self) -> Vec<Option<T>> {
        self.tasks.iter().map(Task::abort).collect::<Vec<_>>()
    }

    /// Waits until all of the tasks in the set completes and returns their outputs.
    pub async fn join_all(&mut self) -> Vec<T> {
        join_all(self.tasks.drain(..)).await
    }

    /// Waits until one of the tasks in the set completes and returns its output.
    /// Return `None` if the set is empty
    pub async fn join_one(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let (value, _index, remaining) = select_all(self.tasks.drain(..)).await;
            self.tasks = remaining;
            Some(value)
        }
    }
}

impl<T> Drop for JoinSet<T> {
    fn drop(&mut self) {
        self.abort_all();
    }
}
