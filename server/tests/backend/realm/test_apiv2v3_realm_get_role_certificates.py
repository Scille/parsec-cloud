# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    DateTime,
    authenticated_cmds,
)
from parsec.api.data import RealmRoleCertificate
from parsec.api.protocol import (
    RealmID,
    RealmRole,
    VlobID,
)
from parsec.backend.realm import RealmGrantedRole
from tests.backend.common import apiv2v3_realm_get_role_certificates, realm_update_roles
from tests.common import freeze_time

RealmGetRoleCertificatesRepOk = authenticated_cmds.v3.realm_get_role_certificates.RepOk
RealmGetRoleCertificatesRepNotAllowed = (
    authenticated_cmds.v3.realm_get_role_certificates.RepNotAllowed
)
RealmGetRoleCertificatesRepNotFound = authenticated_cmds.v3.realm_get_role_certificates.RepNotFound
RealmUpdateRolesRepInMaintenance = authenticated_cmds.v3.realm_update_roles

NOW = DateTime(2000, 1, 1)
VLOB_ID = VlobID.from_hex("00000000000000000000000000000001")
REALM_ID = RealmID.from_hex("0000000000000000000000000000000A")


@pytest.mark.trio
async def test_get_roles_not_found(alice_ws):
    rep = await apiv2v3_realm_get_role_certificates(alice_ws, REALM_ID)
    # The reason is no longer generated
    assert isinstance(rep, RealmGetRoleCertificatesRepNotFound)


@pytest.fixture
def realm_generate_certif_and_update_roles_or_fail(next_timestamp):
    async def _realm_generate_certif_and_update_roles_or_fail(
        ws, author, realm_id, user_id, role, timestamp=None
    ):
        certif = RealmRoleCertificate(
            author=author.device_id,
            timestamp=timestamp or next_timestamp(),
            realm_id=realm_id,
            user_id=user_id,
            role=role,
        ).dump_and_sign(author.signing_key)
        return await realm_update_roles(ws, certif, check_rep=False)

    return _realm_generate_certif_and_update_roles_or_fail


@pytest.fixture
def backend_realm_generate_certif_and_update_roles(next_timestamp):
    async def _backend_realm_generate_certif_and_update_roles(
        backend, author, realm_id, user_id, role, timestamp=None
    ):
        now = timestamp or next_timestamp()
        certif = RealmRoleCertificate(
            author=author.device_id, timestamp=now, realm_id=realm_id, user_id=user_id, role=role
        ).dump_and_sign(author.signing_key)
        await backend.realm.update_roles(
            author.organization_id,
            RealmGrantedRole(
                certificate=certif,
                realm_id=realm_id,
                user_id=user_id,
                role=role,
                granted_by=author.device_id,
                granted_on=now,
            ),
        )
        return certif

    return _backend_realm_generate_certif_and_update_roles


@pytest.mark.trio
async def test_get_role_certificates_multiple(
    backend, alice, bob, adam, bob_ws, realm, backend_realm_generate_certif_and_update_roles
):
    # Realm is created on 2000-01-02

    with freeze_time("2000-01-03"):
        c3 = await backend_realm_generate_certif_and_update_roles(
            backend, alice, realm, bob.user_id, RealmRole.OWNER
        )

    with freeze_time("2000-01-04"):
        c4 = await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, adam.user_id, RealmRole.MANAGER
        )

    with freeze_time("2000-01-05"):
        c5 = await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, alice.user_id, RealmRole.READER
        )

    with freeze_time("2000-01-06"):
        c6 = await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, alice.user_id, None
        )

    rep = await apiv2v3_realm_get_role_certificates(bob_ws, realm)
    assert isinstance(rep, RealmGetRoleCertificatesRepOk)
    assert rep.certificates[1:] == [c3, c4, c5, c6]


@pytest.mark.trio
async def test_get_role_certificates_no_longer_allowed(
    backend, alice, bob, alice_ws, realm, backend_realm_generate_certif_and_update_roles
):
    # Realm is created on 2000-01-02

    with freeze_time("2000-01-03"):
        await backend_realm_generate_certif_and_update_roles(
            backend, alice, realm, bob.user_id, RealmRole.OWNER
        )

    with freeze_time("2000-01-04"):
        await backend_realm_generate_certif_and_update_roles(
            backend, bob, realm, alice.user_id, None
        )

    rep = await apiv2v3_realm_get_role_certificates(alice_ws, realm)
    assert isinstance(rep, RealmGetRoleCertificatesRepNotAllowed)


@pytest.mark.trio
async def test_role_access_during_maintenance(
    backend, alice, bob, alice_ws, realm, realm_generate_certif_and_update_roles_or_fail
):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        DateTime(2000, 1, 2),
    )

    # Get roles allowed...
    roles = await backend.realm.get_current_roles(alice.organization_id, realm)
    assert roles == {alice.user_id: RealmRole.OWNER}

    rep = await apiv2v3_realm_get_role_certificates(alice_ws, realm)
    assert isinstance(rep, RealmGetRoleCertificatesRepOk)
