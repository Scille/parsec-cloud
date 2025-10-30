# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .addr import ParsecAddr, ParsecOrganizationBootstrapAddr
from .common import (
    DateTime,
    EmailAddress,
    ErrorVariant,
    HumanHandle,
    KeyDerivation,
    OrganizationID,
    Path,
    Ref,
    Result,
)


class TestbedError(ErrorVariant):
    class Disabled:
        pass

    class Internal:
        pass


async def test_new_testbed(
    template: Ref[str], test_server: Ref[ParsecAddr] | None
) -> Result[Path, TestbedError]:
    raise NotImplementedError


def test_get_testbed_organization_id(
    discriminant_dir: Ref[Path],
) -> Result[OrganizationID | None, TestbedError]:
    raise NotImplementedError


def test_get_testbed_bootstrap_organization_addr(
    discriminant_dir: Ref[Path],
) -> Result[ParsecOrganizationBootstrapAddr | None, TestbedError]:
    raise NotImplementedError


async def test_drop_testbed(path: Ref[Path]) -> Result[None, TestbedError]:
    raise NotImplementedError


async def test_check_mailbox(
    server_addr: Ref[ParsecAddr],
    email: Ref[EmailAddress],
) -> Result[list[tuple[EmailAddress, DateTime, str]], TestbedError]:
    raise NotImplementedError


async def test_new_account(
    server_addr: Ref[ParsecAddr],
) -> Result[tuple[HumanHandle, KeyDerivation], TestbedError]:
    raise NotImplementedError
