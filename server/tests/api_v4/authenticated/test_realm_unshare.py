# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, RealmRole, RealmRoleCertificate, authenticated_cmds
from parsec.events import EventRealmCertificate
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_realm_share_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=None,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_unshare(
            realm_role_certificate=certif,
        )
        assert rep == authenticated_cmds.v4.realm_unshare.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=timestamp,
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.device_id.user_id,
                role_removed=True,
            )
        )

    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.device_id.user_id
    )
    assert isinstance(bob_realms, dict)
    assert coolorg.wksp1_id not in bob_realms


@pytest.mark.parametrize("kind", ("dummy_certif", "invalid_role", "self_unshare"))
async def test_authenticated_realm_unshare_invalid_certificate(
    coolorg: CoolorgRpcClients, kind: str
) -> None:
    timestamp = DateTime.now()

    match kind:
        case "dummy_certif":
            certif = b"<dummy>"
        case "invalid_role":
            certif = RealmRoleCertificate(
                author=coolorg.alice.device_id,
                timestamp=timestamp,
                realm_id=coolorg.wksp1_id,
                role=RealmRole.READER,
                user_id=coolorg.bob.device_id.user_id,
            ).dump_and_sign(coolorg.alice.signing_key)
        case "self_unshare":
            certif = RealmRoleCertificate(
                author=coolorg.alice.device_id,
                timestamp=timestamp,
                realm_id=coolorg.wksp1_id,
                role=None,
                user_id=coolorg.alice.device_id.user_id,
            ).dump_and_sign(coolorg.alice.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_unshare(
        realm_role_certificate=certif,
    )
    assert rep == authenticated_cmds.v4.realm_unshare.RepInvalidCertificate()


# TODO: test unshare on revoked user
# TODO: test unshare with certificate containing a non-null role
