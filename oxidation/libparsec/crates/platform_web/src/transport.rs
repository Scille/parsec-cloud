// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub struct Transport;

impl Transport {
    pub async fn init_for_client<T: AsRef<str>>(_uri: T) -> Self {
        unimplemented!()
    }
    pub async fn send(&mut self, _data: Vec<u8>) {
        unimplemented!()
    }
    pub async fn recv(&mut self) -> Option<Vec<u8>> {
        unimplemented!()
    }
}
