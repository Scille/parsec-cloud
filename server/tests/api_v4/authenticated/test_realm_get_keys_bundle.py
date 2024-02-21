# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    DateTime,
    HashAlgorithm,
    RealmKeyRotationCertificate,
    RealmRole,
    RealmRoleCertificate,
    SecretKey,
    SecretKeyAlgorithm,
    authenticated_cmds,
)
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_realm_get_keys_bundle_ok(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.realm_get_keys_bundle(
        realm_id=coolorg.wksp1_id,
        key_index=1,
    )
    expected_key_bundle = coolorg.key_bundle(coolorg.wksp1_id, 1)
    expected_key_bundle_access = coolorg.key_bundle_access(
        coolorg.wksp1_id, 1, coolorg.alice.user_id
    )
    assert rep == authenticated_cmds.v4.realm_get_keys_bundle.RepOk(
        keys_bundle=expected_key_bundle,
        keys_bundle_access=expected_key_bundle_access,
    )


async def test_authenticated_realm_get_keys_bundle_access_not_available_for_author(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # 1) Do a key rotation, so the workspace now is at key index 2

    t1 = DateTime.now()
    key = SecretKey.generate()
    key_canary = key.encrypt(b"")
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=t1,
        realm_id=coolorg.wksp1_id,
        key_index=2,
        encryption_algorithm=SecretKeyAlgorithm.XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=key_canary,
    ).dump_and_sign(coolorg.alice.signing_key)
    outcome = await backend.realm.rotate_key(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_key_rotation_certificate=certif,
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice keys bundle 2 access>",
            coolorg.bob.user_id: b"<bob keys bundle 2 access>",
        },
        keys_bundle=b"<keys bundle 2>",
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    # 2) Then share the realm with Mallory, which hence only has access to key index 2

    t2 = DateTime.now()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=t2,
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.mallory.user_id,
        role=RealmRole.READER,
    ).dump_and_sign(coolorg.alice.signing_key)
    outcome = await backend.realm.share(
        now=t2,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        key_index=2,
        realm_role_certificate=certif,
        recipient_keys_bundle_access=b"<mallory keys bundle 2 access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    # 3) Mallory cannot access key index 1

    rep = await coolorg.mallory.realm_get_keys_bundle(
        realm_id=coolorg.wksp1_id,
        key_index=1,
    )
    assert rep == authenticated_cmds.v4.realm_get_keys_bundle.RepAccessNotAvailableForAuthor()


async def test_authenticated_realm_get_keys_bundle_access_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.mallory.realm_get_keys_bundle(
        realm_id=coolorg.wksp1_id,
        key_index=1,
    )
    assert rep == authenticated_cmds.v4.realm_get_keys_bundle.RepAuthorNotAllowed()


async def test_authenticated_realm_get_keys_bundle_access_bad_key_index(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.realm_get_keys_bundle(
        realm_id=coolorg.wksp1_id,
        key_index=10,
    )
    assert rep == authenticated_cmds.v4.realm_get_keys_bundle.RepBadKeyIndex()
