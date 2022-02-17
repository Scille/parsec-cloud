// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod local_device;
mod local_device_file;
mod local_manifest;

pub use local_device::*;
pub use local_device_file::*;
pub use local_manifest::*;

#[macro_export]
macro_rules! set {
    {} => (
        std::collections::HashSet::new()
    );
    {$($x: expr),+ $(,)?} => ({
        let mut set = std::collections::HashSet::new();
        $(set.insert($x);)+
        set
    });
}

#[macro_export]
macro_rules! map {
    {} => (
        std::collections::HashMap::new()
    );
    {$($x: expr => $y: expr),+ $(,)?} => ({
        let mut map = std::collections::HashMap::new();
        $(map.insert($x, $y);)+
        map
    });
}
