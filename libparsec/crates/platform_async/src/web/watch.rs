// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

pub fn channel<T>(init: T) -> (Sender<T>, Receiver<T>) {
    let state = Arc::new(Mutex::new(State {
        current: init,
        version: 0,
        notify_on_new_version: crate::event::Event::new(),
        senders: 1,
        receivers: 1,
    }));
    (
        Sender {
            state: state.clone(),
        },
        Receiver {
            state,
            last_known_version: 0,
        },
    )
}

#[derive(Debug)]
struct State<T> {
    current: T,
    version: usize,
    notify_on_new_version: crate::event::Event,
    senders: usize,
    receivers: usize,
}

#[derive(Debug)]
pub struct SendError<T>(T);

#[derive(Debug)]
pub struct Sender<T> {
    state: Arc<Mutex<State<T>>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        let mut state = self.state.lock().expect("Mutex is poisoned");
        // Receiver has been dropped
        if state.receivers == 0 {
            return Err(SendError(value));
        }
        state.current = value;
        state.version += 1;
        state.notify_on_new_version.notify(usize::MAX); // Notify all listeners
        Ok(())
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut guard = self.state.lock().expect("Mutex is poisoned");
        guard.senders += 1;
        Self {
            state: self.state.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut guard = self.state.lock().expect("Mutex is poisoned");
        guard.senders -= 1;
    }
}

#[derive(Debug)]
pub struct RecvError;

#[derive(Debug)]
pub struct Receiver<T> {
    last_known_version: usize,
    state: Arc<Mutex<State<T>>>,
}

pub struct Ref<'a, T> {
    has_changed: bool,
    // value: &'a T,
    guard: std::sync::MutexGuard<'a, State<T>>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard.deref().current
    }
}

impl<T> Ref<'_, T> {
    pub fn has_changed(&self) -> bool {
        self.has_changed
    }
}

impl<T> Receiver<T> {
    pub async fn changed(&mut self) -> Result<(), RecvError> {
        let listener = {
            let state = self.state.lock().expect("Mutex is poisoned");
            // Sender has been dropped
            if state.senders == 0 {
                return Err(RecvError);
            }
            if state.version > self.last_known_version {
                self.last_known_version = state.version;
                return Ok(());
            } else {
                state.notify_on_new_version.listen()
            }
        };
        listener.await;
        let state = self.state.lock().expect("Mutex is poisoned");
        // Sender has been dropped
        if state.senders == 0 {
            return Err(RecvError);
        }
        assert!(state.version > self.last_known_version);
        self.last_known_version = state.version;
        Ok(())
    }

    pub fn has_changed(&self) -> Result<bool, RecvError> {
        let state = self.state.lock().expect("Mutex is poisoned");
        // Sender has been dropped
        if state.senders == 0 {
            return Err(RecvError);
        }
        Ok(self.last_known_version != state.version)
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        let state = self.state.lock().expect("Mutex is poisoned");
        Ref {
            has_changed: self.last_known_version != state.version,
            guard: state,
        }
    }

    pub fn borrow_and_update(&mut self) -> Ref<'_, T> {
        let state = self.state.lock().expect("Mutex is poisoned");
        let has_changed = self.last_known_version != state.version;
        self.last_known_version = state.version;
        Ref {
            has_changed,
            guard: state,
        }
    }

    pub async fn wait_for(
        &mut self,
        mut f: impl FnMut(&T) -> bool,
    ) -> Result<Ref<'_, T>, RecvError> {
        loop {
            let listener = {
                let state = self.state.lock().expect("Mutex is poisoned");
                // Sender has been dropped
                if state.senders == 0 {
                    return Err(RecvError);
                }
                let has_changed = self.last_known_version != state.version;
                if has_changed && f(&state.current) {
                    return Ok(Ref {
                        guard: state,
                        has_changed: true,
                    });
                }
                // Need to wait for the next change
                state.notify_on_new_version.listen()
            };
            listener.await;
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        let mut guard = self.state.lock().expect("Mutex is poisoned");
        guard.receivers += 1;
        Self {
            state: self.state.clone(),
            last_known_version: self.last_known_version,
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let mut guard = self.state.lock().expect("Mutex is poisoned");
        guard.receivers -= 1;
    }
}
