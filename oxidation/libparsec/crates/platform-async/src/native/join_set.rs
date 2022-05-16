// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use super::task::{spawn, Task};
use futures::future::{join_all, select_all};
use std::future::Future;

pub struct JoinSet<T> {
    tasks: Vec<Task<T>>,
}

impl<T> Default for JoinSet<T> {
    fn default() -> Self {
        Self {
            tasks: Vec::default(),
        }
    }
}

impl<T> JoinSet<T> {
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = T>,
        F: Send + 'static,
        T: Send + 'static,
    {
        let task = spawn(future);
        self.tasks.push(task);
    }

    pub fn cancel_all(&mut self) -> Vec<Option<T>> {
        self.tasks
            .iter()
            .map(|task| task.cancel())
            .collect::<Vec<_>>()
    }

    pub async fn join_all(&mut self) -> Vec<T> {
        join_all(self.tasks.drain(..)).await
    }

    pub async fn join_one(&mut self) -> T {
        let (value, _index, remaining) = select_all(self.tasks.drain(..)).await;
        self.tasks = remaining;
        value
    }
}
