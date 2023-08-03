// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    todo!()
}

#[derive(Clone, Debug)]
pub struct RecvError;

#[derive(Clone, Debug)]
pub enum TryRecvError {
    Empty,
    Closed,
}

pub struct Receiver<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> Receiver<T> {
    pub fn blocking_recv(self) -> Result<T, RecvError> {
        todo!()
    }
    pub fn close(&mut self) {
        todo!()
    }
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        todo!()
    }
}

impl<T> crate::future::Future for Receiver<T> {
    type Output = std::result::Result<T, RecvError>;

    #[inline(always)]
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Sender<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> Sender<T> {
    pub fn send(self, _t: T) -> Result<(), T> {
        todo!()
    }
    pub async fn closed(&mut self) {
        todo!()
    }
    pub fn poll_closed(&mut self, _cx: &mut core::task::Context<'_>) -> core::task::Poll<()> {
        todo!()
    }
}
