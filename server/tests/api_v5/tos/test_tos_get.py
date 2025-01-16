# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, tos_cmds
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
)


@pytest.mark.parametrize("accepted", (False, True))
async def test_tos_tos_get_ok(coolorg: CoolorgRpcClients, backend: Backend, accepted: bool):
    t0 = DateTime.now()
    outcome = await backend.organization.update(
        now=t0,
        id=coolorg.organization_id,
        tos={
            # Provide various locale to make sure their order is preserved
            "fr_CA": "https://parsec.invalid/tos_fr.pdf",
            "en_CA": "https://parsec.invalid/tos_en.pdf",
            "fr": "https://parsec.invalid/tos_fr.pdf",
            "en": "https://parsec.invalid/tos_en.pdf",
        },
    )
    assert outcome is None

    if accepted:
        outcome = await backend.user.accept_tos(
            now=DateTime.now(),
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            tos_updated_on=t0,
        )
        assert outcome is None

    rep = await coolorg.alice.tos_get()
    assert rep == tos_cmds.latest.tos_get.RepOk(
        per_locale_urls={
            "fr_CA": "https://parsec.invalid/tos_fr.pdf",
            "en_CA": "https://parsec.invalid/tos_en.pdf",
            "fr": "https://parsec.invalid/tos_fr.pdf",
            "en": "https://parsec.invalid/tos_en.pdf",
        },
        updated_on=t0,
    )


async def test_tos_tos_get_no_tos(coolorg: CoolorgRpcClients):
    rep = await coolorg.alice.tos_get()
    assert rep == tos_cmds.latest.tos_get.RepNoTos()


async def test_tos_tos_get_http_common_errors(
    coolorg: CoolorgRpcClients, tos_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.tos_get()

    await tos_http_common_errors_tester(do)
