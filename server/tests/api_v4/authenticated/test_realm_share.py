# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, RealmRole, RealmRoleCertificate, authenticated_cmds
from parsec.events import EventRealmCertificate
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_realm_share_ok_new_sharing(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.device_id.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert coolorg.wksp1_id not in mallory_realms

    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.READER,
        user_id=coolorg.mallory.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_share(
            key_index=1,
            realm_role_certificate=certif,
            # Keys bundle access is not redable by the server, so we can put anything here
            recipient_keys_bundle_access=b"<mallory keys bundle access>",
        )
        assert rep == authenticated_cmds.v4.realm_share.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=timestamp,
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.mallory.device_id.user_id,
                role_removed=False,
            )
        )

    mallory_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.mallory.device_id.user_id
    )
    assert isinstance(mallory_realms, dict)
    assert mallory_realms[coolorg.wksp1_id] == RealmRole.READER


async def test_authenticated_realm_share_ok_update_sharing_role(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.device_id.user_id
    )
    assert isinstance(bob_realms, dict)
    assert bob_realms[coolorg.wksp1_id] == RealmRole.READER

    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.MANAGER,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_share(
            key_index=1,
            realm_role_certificate=certif,
            # Keys bundle access is not redable by the server, so we can put anything here
            recipient_keys_bundle_access=b"<bob keys bundle access>",
        )
        assert rep == authenticated_cmds.v4.realm_share.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=timestamp,
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.device_id.user_id,
                role_removed=False,
            )
        )

    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.device_id.user_id
    )
    assert isinstance(bob_realms, dict)
    assert bob_realms[coolorg.wksp1_id] == RealmRole.MANAGER


async def test_authenticated_realm_share_role_already_granted(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bob_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, coolorg.bob.device_id.user_id
    )
    assert isinstance(bob_realms, dict)
    assert bob_realms[coolorg.wksp1_id] == RealmRole.READER

    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.READER,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif,
        # Keys bundle access is not redable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepRoleAlreadyGranted()


async def test_authenticated_realm_share_invalid_certificate_role_none(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    timestamp = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=coolorg.wksp1_id,
        role=None,
        user_id=coolorg.bob.device_id.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=certif,
        # Keys bundle access is not redable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepInvalidCertificate()


async def test_authenticated_realm_share_invalid_certificate_dummy_data(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.alice.realm_share(
        key_index=1,
        realm_role_certificate=b"<dummy>",
        # Keys bundle access is not redable by the server, so we can put anything here
        recipient_keys_bundle_access=b"<bob keys bundle access>",
    )
    assert rep == authenticated_cmds.v4.realm_share.RepInvalidCertificate()


# TODO: test share on revoked user
