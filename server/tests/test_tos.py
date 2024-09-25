# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, authenticated_cmds, tos_cmds
from parsec.events import EventOrganizationTosUpdated

from .common import Backend, MinimalorgRpcClients, RpcTransportError


async def test_auth_tos_then_authenticated(backend: Backend, minimalorg: MinimalorgRpcClients):
    # 1) Configure TOS

    outcome = await backend.organization.update(
        now=DateTime.now(),
        id=minimalorg.organization_id,
        tos={
            "fr_CA": "https://parsec.invalid/tos_fr.pdf",
            "en_CA": "https://parsec.invalid/tos_en.pdf",
        },
    )
    assert outcome is None

    # 2) User authenticate to the `tos` family...

    rep = await minimalorg.alice.tos_get()
    assert isinstance(rep, tos_cmds.latest.tos_get.RepOk)

    # 3) ...but this shouldn't change the fact the `authenticated` family cmds are
    # not accessible since the TOS hasn't been accepted yet !

    with pytest.raises(RpcTransportError) as err:
        await minimalorg.alice.ping(ping="foo")
    assert err.value.rep.status_code == 463


async def test_auth_tos_accepted_then_removed(backend: Backend, minimalorg: MinimalorgRpcClients):
    tos_updated_on = DateTime.now()
    # 1) Configure TOS

    outcome = await backend.organization.update(
        now=tos_updated_on,
        id=minimalorg.organization_id,
        tos={
            "fr_CA": "https://parsec.invalid/tos_fr.pdf",
            "en_CA": "https://parsec.invalid/tos_en.pdf",
        },
    )
    assert outcome is None

    # 2) User accept the TOS

    rep = await minimalorg.alice.tos_accept(tos_updated_on=tos_updated_on)
    assert rep == tos_cmds.latest.tos_accept.RepOk()

    # 3) Sanity check to ensure the user can now use the `authenticated` family cmds

    rep = await minimalorg.alice.ping(ping="foo")
    assert rep == authenticated_cmds.latest.ping.RepOk(pong="foo")

    # 4) Then the TOS gets removed

    outcome = await backend.organization.update(
        now=tos_updated_on, id=minimalorg.organization_id, tos=None
    )
    assert outcome is None

    # 5) This shouldn't impact the user capacity to use the `authenticated` family cmds

    rep = await minimalorg.alice.ping(ping="foo")
    assert rep == authenticated_cmds.latest.ping.RepOk(pong="foo")


async def test_auth_authenticated_then_tos_added(
    backend: Backend, minimalorg: MinimalorgRpcClients
):
    tos_updated_on = DateTime.now()
    # 1) User authenticate to an Organization without TOS

    rep = await minimalorg.alice.ping(ping="foo")
    assert rep == authenticated_cmds.latest.ping.RepOk(pong="foo")

    # 2) Configure TOS

    with backend.event_bus.spy() as spy:
        outcome = await backend.organization.update(
            now=tos_updated_on,
            id=minimalorg.organization_id,
            tos={
                "fr_CA": "https://parsec.invalid/tos_fr.pdf",
                "en_CA": "https://parsec.invalid/tos_en.pdf",
            },
        )
        assert outcome is None

        await spy.wait_event_occurred(
            EventOrganizationTosUpdated(organization_id=minimalorg.organization_id)
        )

    # 3) User is now required to accept TOS

    with pytest.raises(RpcTransportError) as err:
        await minimalorg.alice.ping(ping="foo")
    assert err.value.rep.status_code == 463
