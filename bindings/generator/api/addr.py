# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .common import (
    U16,
    AccessToken,
    ErrorVariant,
    IndexInt,
    OrganizationID,
    Ref,
    Result,
    StrBasedType,
    UserID,
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


class ParsecWorkspacePathAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecWorkspacePathAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecWorkspacePathAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParsecPkiEnrollmentAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecPkiEnrollmentAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecPkiEnrollmentAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParsecAsyncEnrollmentAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecAsyncEnrollmentAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecAsyncEnrollmentAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParsecTOTPResetAddr(StrBasedType):
    custom_from_rs_string = "|s: String| -> Result<_, String> { libparsec::ParsecTOTPResetAddr::from_any(&s).map_err(|e| e.to_string()) }"
    custom_to_rs_string = "|addr: libparsec::ParsecTOTPResetAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) }"


class ParseParsecAddrError(ErrorVariant):
    class InvalidUrl:
        pass


class ParsedParsecAddr(Variant):
    class Server:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool

    class Organization:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID

    class OrganizationBootstrap:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID
        token: str | None

    class WorkspacePath:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID
        workspace_id: VlobID
        key_index: IndexInt
        encrypted_path: bytes

    class InvitationUser:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID
        token: AccessToken

    class InvitationDevice:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID
        token: AccessToken

    class InvitationShamirRecovery:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID
        token: AccessToken

    class PkiEnrollment:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID

    class AsyncEnrollment:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID

    class TOTPReset:
        hostname: str
        port: U16
        is_default_port: bool
        use_ssl: bool
        organization_id: OrganizationID
        user_id: UserID
        token: AccessToken


def parse_parsec_addr(url: Ref[str]) -> Result[ParsedParsecAddr, ParseParsecAddrError]:
    raise NotImplementedError


def build_parsec_organization_bootstrap_addr(
    addr: ParsecAddr, organization_id: OrganizationID
) -> ParsecOrganizationBootstrapAddr:
    raise NotImplementedError


def build_parsec_addr(
    hostname: str,
    port: U16 | None,
    use_ssl: bool,
) -> ParsecAddr:
    raise NotImplementedError
