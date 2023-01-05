// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// An empty implementation of tokio::sync::watch
// TODO: write it

pub struct Receiver<T>(T);
pub struct Sender<T>(T);

pub fn channel<T: Clone>(init: T) -> (Sender<T>, Receiver<T>) {
    let tx = Sender(init.clone());
    let rx = Receiver(init.clone());
    (tx, rx)
}

impl<T> Receiver<T> {
    pub async fn changed(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

impl<T: Clone> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver(self.0.clone())
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, _value: T) -> Result<(), ()> {
        Ok(())
    }
}
