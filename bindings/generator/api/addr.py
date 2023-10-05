# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    U32,
    ErrorVariant,
    InvitationToken,
    OrganizationID,
    Ref,
    Result,
    StrBasedType,
    Variant,
    VlobID,
)


class BackendAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendOrganizationAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendOrganizationAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendOrganizationAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendOrganizationBootstrapAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendInvitationAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendInvitationAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendInvitationAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendOrganizationFileLinkAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendOrganizationFileLinkAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendOrganizationFileLinkAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class BackendPkiEnrollmentAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::BackendPkiEnrollmentAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::BackendPkiEnrollmentAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParseBackendAddrError(ErrorVariant):
    class InvalidUrl:
        pass


class ParsedBackendAddr(Variant):
    class Server:
        hostname: str
        port: U32
        use_ssl: bool

    class Organization:
        hostname: str
        port: U32
        use_ssl: bool
        organization_id: OrganizationID

    class OrganizationBootstrap:
        hostname: str
        port: U32
        use_ssl: bool
        organization_id: OrganizationID
        token: Optional[str]

    class OrganizationFileLink:
        hostname: str
        port: U32
        use_ssl: bool
        organization_id: OrganizationID
        workspace_id: VlobID
        encrypted_path: bytes
        encrypted_timestamp: Optional[bytes]

    class InvitationUser:
        hostname: str
        port: U32
        use_ssl: bool
        organization_id: OrganizationID
        token: InvitationToken

    class InvitationDevice:
        hostname: str
        port: U32
        use_ssl: bool
        organization_id: OrganizationID
        token: InvitationToken

    class PkiEnrollment:
        hostname: str
        port: U32
        use_ssl: bool
        organization_id: OrganizationID


def parse_backend_addr(url: Ref[str]) -> Result[ParsedBackendAddr, ParseBackendAddrError]:
    raise NotImplementedError


def build_backend_organization_bootstrap_addr(
    addr: BackendAddr, organization_id: OrganizationID
) -> BackendOrganizationBootstrapAddr:
    raise NotImplementedError
