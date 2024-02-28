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


class ParsecAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = (
        "|addr: libparsec::ParsecAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"
    )


class ParsecOrganizationAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecOrganizationAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecOrganizationAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParsecOrganizationBootstrapAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParsecInvitationAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParsecOrganizationFileLinkAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecOrganizationFileLinkAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecOrganizationFileLinkAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParsecPkiEnrollmentAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecPkiEnrollmentAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecPkiEnrollmentAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParseBackendAddrError(ErrorVariant):
    class InvalidUrl:
        pass


class ParsedParsecAddr(Variant):
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


def parse_backend_addr(url: Ref[str]) -> Result[ParsedParsecAddr, ParseBackendAddrError]:
    raise NotImplementedError


def build_backend_organization_bootstrap_addr(
    addr: ParsecAddr, organization_id: OrganizationID
) -> ParsecOrganizationBootstrapAddr:
    raise NotImplementedError
