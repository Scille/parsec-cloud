# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import BackendAddr, BackendOrganizationBootstrapAddr, OrganizationID, Path, Ref


async def test_new_testbed(template: Ref[str], test_server: Optional[Ref[BackendAddr]]) -> Path:
    raise NotImplementedError


def test_get_testbed_organization_id(discriminant_dir: Ref[Path]) -> Optional[OrganizationID]:
    raise NotImplementedError


def test_get_testbed_bootstrap_organization_addr(
    discriminant_dir: Ref[Path],
) -> Optional[BackendOrganizationBootstrapAddr]:
    raise NotImplementedError


async def test_drop_testbed(path: Ref[Path]) -> None:
    raise NotImplementedError
