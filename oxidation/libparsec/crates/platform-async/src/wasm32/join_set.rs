use super::{spawn, Task};
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
        F: 'static,
        T: 'static,
    {
        let task = spawn(future);

        self.tasks.push(task);
    }

    pub fn abort_all(&mut self) -> Vec<Option<T>> {
        self.tasks.iter().map(Task::abort).collect::<Vec<_>>()
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
