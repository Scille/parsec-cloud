// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::future::Future;

pub trait Taskable<T>: Future<Output = T> {
    /// Cancels the task.
    fn cancel(&self) -> Option<T>;

    /// Detaches the task to let it keep running in the backgroud.
    fn detach(self);

    /// Return `true` if the current task is finished.
    fn is_finised(&self) -> bool;

    /// Return `true` if the current task is canceled.
    fn is_canceled(&self) -> bool;
}
