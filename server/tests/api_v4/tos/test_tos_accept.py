# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, tos_cmds
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
)


@pytest.mark.parametrize("already_accepted", (False, True))
async def test_tos_tos_accept_ok(
    coolorg: CoolorgRpcClients, backend: Backend, already_accepted: bool
):
    tos_updated_on = DateTime.now()
    outcome = await backend.organization.update(
        now=tos_updated_on,
        id=coolorg.organization_id,
        tos={
            "fr_CA": "https://parsec.invalid/tos_fr.pdf",
            "en_CA": "https://parsec.invalid/tos_en.pdf",
        },
    )
    assert outcome is None

    if already_accepted:
        outcome = await backend.user.accept_tos(
            now=DateTime.now(),
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            tos_updated_on=tos_updated_on,
        )
        assert outcome is None

    rep = await coolorg.alice.tos_accept(tos_updated_on=tos_updated_on)
    assert rep == tos_cmds.latest.tos_accept.RepOk()


@pytest.mark.parametrize("kind", ("too_old", "too_recent"))
async def test_tos_tos_accept_tos_mismatch(coolorg: CoolorgRpcClients, backend: Backend, kind: str):
    tos_updated_on = DateTime.now()
    outcome = await backend.organization.update(
        now=tos_updated_on,
        id=coolorg.organization_id,
        tos={
            "fr_CA": "https://parsec.invalid/tos_fr.pdf",
            "en_CA": "https://parsec.invalid/tos_en.pdf",
        },
    )
    assert outcome is None

    match kind:
        case "too_old":
            bad_tos_updated_on = tos_updated_on.subtract(seconds=1)
        case "too_recent":
            bad_tos_updated_on = tos_updated_on.add(seconds=1)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.tos_accept(tos_updated_on=bad_tos_updated_on)
    assert rep == tos_cmds.latest.tos_accept.RepTosMismatch()


async def test_tos_tos_accept_no_tos(coolorg: CoolorgRpcClients):
    rep = await coolorg.alice.tos_accept(tos_updated_on=DateTime.now())
    assert rep == tos_cmds.latest.tos_accept.RepNoTos()


async def test_tos_tos_accept_http_common_errors(
    coolorg: CoolorgRpcClients, tos_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.tos_accept(tos_updated_on=DateTime.now())

    await tos_http_common_errors_tester(do)
