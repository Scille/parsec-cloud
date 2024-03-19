# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .addr import ParsecAddr, ParsecOrganizationBootstrapAddr
from .common import OrganizationID, Path, Ref, ErrorVariant, Result


class TestbedError(ErrorVariant):
    class Disabled:
        pass

    class Internal:
        pass


async def test_new_testbed(
    template: Ref[str], test_server: Optional[Ref[ParsecAddr]]
) -> Result[Path, TestbedError]:
    raise NotImplementedError


def test_get_testbed_organization_id(
    discriminant_dir: Ref[Path],
) -> Result[Optional[OrganizationID], TestbedError]:
    raise NotImplementedError


def test_get_testbed_bootstrap_organization_addr(
    discriminant_dir: Ref[Path],
) -> Result[Optional[ParsecOrganizationBootstrapAddr], TestbedError]:
    raise NotImplementedError


async def test_drop_testbed(path: Ref[Path]) -> Result[None, TestbedError]:
    raise NotImplementedError
