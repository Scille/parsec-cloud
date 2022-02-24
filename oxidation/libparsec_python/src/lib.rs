// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pyo3::prelude::*;

mod addrs;
mod binding_utils;
mod crypto;
mod ids;
mod invite;
mod manifest;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _libparsec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<crypto::HashDigest>()?;
    m.add_class::<crypto::SigningKey>()?;
    m.add_class::<crypto::VerifyKey>()?;
    m.add_class::<crypto::SecretKey>()?;
    m.add_class::<crypto::PrivateKey>()?;
    m.add_class::<crypto::PublicKey>()?;
    m.add_class::<addrs::BackendAddr>()?;
    m.add_class::<addrs::BackendOrganizationAddr>()?;
    m.add_class::<addrs::BackendActionAddr>()?;
    m.add_class::<addrs::BackendOrganizationBootstrapAddr>()?;
    m.add_class::<addrs::BackendOrganizationFileLinkAddr>()?;
    m.add_class::<addrs::BackendInvitationAddr>()?;
    m.add_class::<ids::OrganizationID>()?;
    m.add_class::<ids::EntryID>()?;
    m.add_class::<ids::BlockID>()?;
    m.add_class::<ids::RealmID>()?;
    m.add_class::<ids::VlobID>()?;
    m.add_class::<ids::HumanHandle>()?;
    m.add_class::<ids::DeviceID>()?;
    m.add_class::<ids::DeviceName>()?;
    m.add_class::<ids::DeviceLabel>()?;
    m.add_class::<ids::UserID>()?;
    m.add_class::<invite::InvitationToken>()?;
    m.add_class::<manifest::EntryName>()?;
    m.add_class::<manifest::WorkspaceEntry>()?;
    m.add_class::<manifest::BlockAccess>()?;
    m.add_class::<manifest::FileManifest>()?;
    m.add_class::<manifest::FolderManifest>()?;
    m.add_class::<manifest::WorkspaceEntry>()?;
    m.add_class::<manifest::WorkspaceManifest>()?;
    Ok(())
}
