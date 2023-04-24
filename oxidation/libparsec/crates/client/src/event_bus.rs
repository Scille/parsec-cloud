// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub enum Event {}

#[derive(Debug)]
pub struct EventBus {}

impl EventBus {
    pub fn send(&self, _event: Event) {
        todo!()
    }
}
