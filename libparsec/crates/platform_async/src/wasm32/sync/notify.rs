// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

#[derive(Default)]
pub struct Notify {
    state: Arc<Mutex<NotifyState>>,
    waiters: Mutex<Vec<Arc<Mutex<Waiter>>>>,
}

#[derive(Default)]
struct NotifyState {
    /// `true` when no waiter was registered and we have a pending notification.
    notified: bool,
}

struct Waiter {
    /// Waiting task's waker.
    waker: Option<Waker>,

    /// `true` if the notification has been assigned to this waiter.
    notified: bool,
}

impl Notify {
    /// Wait for a notification.
    pub fn notified(&self) -> Notified {
        Notified {
            notify: self,
            waiting: false,
            waiter: Arc::new(Mutex::new(Waiter {
                waker: None,
                notified: false,
            })),
        }
    }

    /// Notifies a waiting task.
    pub fn notify_one(&self) {
        // Lock state to prevent waiter to read it.
        let mut state = self.state.lock().expect("mutex poisoned");

        if let Some(waiter) = self.waiters.lock().expect("mutex poisoned").pop() {
            let mut waiter = waiter.lock().expect("mutex poisoned");

            waiter.notified = true;
            waiter
                .waker
                .as_ref()
                .expect("precondition: Notified set its waker before add it's waiter")
                .wake_by_ref();
            state.notified = false;
        } else {
            state.notified = true;
        }
    }
}

pub struct Notified<'a> {
    notify: &'a Notify,
    waiting: bool,
    waiter: Arc<Mutex<Waiter>>,
}

impl<'a> Future for Notified<'a> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut s = self.as_mut();
        let mut state = s.notify.state.lock().expect("mutex poisoned");
        let mut waiters = s.notify.waiters.lock().expect("mutex poisoned");
        let mut waiter = s.waiter.lock().expect("mutex poisoned");

        waiter.waker = Some(cx.waker().clone());

        if state.notified || waiter.notified {
            state.notified = false;
            return Poll::Ready(());
        } else if !s.waiting {
            waiters.push(s.waiter.clone());
            drop(waiter);
            s.waiting = true;
        }

        Poll::Pending
    }
}
