use std::{
    cell::UnsafeCell,
    future::Future,
    marker::PhantomPinned,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll, Waker},
};

pub struct Notify {
    state: Mutex<NotifyState>,
    waiters: Mutex<Vec<Waiter>>,
}

#[derive(Default)]
struct NotifyState {
    waiting: bool,
    notified: bool,
}

struct Waiter {
    /// Waiting task's waker.
    waker: Option<Waker>,

    /// `true` if the notification has been assigned to this waiter.
    notified: bool,

    /// Should not be `Unpin`.
    _p: PhantomPinned,
}

impl Notify {
    /// Wait for a notification.
    pub fn notified(&self) -> Notified<'_> {
        Notified {
            notify: self,
            state: State::Init(0),
            waiter: UnsafeCell::new(Waiter {
                waker: None,
                notified: false,
                _p: PhantomPinned,
            }),
        }
    }

    /// Notifies a waiting task.
    pub fn notify_one(&self) {
        if let Some(waiter) = self.waiters.lock().expect("mutex poisoned").pop() {
            todo!("take value from waiters if any");
        } else {
            todo!("if no waiter set state as waiting");
        }
    }
}

impl Default for Notify {
    fn default() -> Self {
        Self {
            state: Mutex::default(),
            waiters: Mutex::default(),
        }
    }
}

pub struct Notified<'a> {
    notify: &'a Notify,
    state: State,
    waiter: UnsafeCell<Waiter>,
}

impl<'a> Future for Notified<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut s = self.as_mut();
        match s.state {
            State::Init(id) => {
                todo!("register is waker in the poll");
                s.state = State::Waiting;
                Poll::Pending;
            }
            State::Waiting => {
                todo!("wait to be waked");
                Poll::Pending;
            }
            State::Done => {
                todo!("done");
                Poll::Ready(())
            }
        }
    }
}

enum State {
    Init(usize),
    Waiting,
    Done,
}
