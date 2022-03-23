// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

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
