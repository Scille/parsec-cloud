# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    BlockID,
    DateTime,
    RealmRole,
    RealmRoleCertificate,
    VlobID,
    authenticated_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    generate_realm_role_certificate,
)
from tests.common.data import wksp1_alice_gives_role


async def test_authenticated_realm_list_info_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.alice.realm_list_info()
    assert isinstance(rep, authenticated_cmds.latest.realm_list_info.RepOk)

    # Coolorg has 3 realms: `wksp1` (shared between alice and bob), and the
    # personal realms of alice and bob
    assert len(rep.realms) == 3
    realms_per_id = {realm.realm_id: realm for realm in rep.realms}

    # `wksp1` is shared between alice (owner) and bob (reader)
    wksp1 = realms_per_id[coolorg.wksp1_id]
    assert wksp1.users_per_role == {
        RealmRole.OWNER: 1,
        RealmRole.MANAGER: 0,
        RealmRole.CONTRIBUTOR: 0,
        RealmRole.READER: 1,
    }
    assert wksp1.data_size == 0

    vlobs = await backend.vlob.test_dump_vlobs(coolorg.organization_id)
    expected_metadata_size = sum(
        len(blob) for vlob_atoms in vlobs[coolorg.wksp1_id].values() for _, _, blob in vlob_atoms
    )
    assert wksp1.metadata_size == expected_metadata_size

    # The personal realms only contain a single owner
    for realm_id, realm in realms_per_id.items():
        if realm_id == coolorg.wksp1_id:
            continue
        assert realm.users_per_role == {
            RealmRole.OWNER: 1,
            RealmRole.MANAGER: 0,
            RealmRole.CONTRIBUTOR: 0,
            RealmRole.READER: 0,
        }

    # Bob's role change should be reflected in the per-role counts
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)

    rep = await coolorg.alice.realm_list_info()
    assert isinstance(rep, authenticated_cmds.latest.realm_list_info.RepOk)
    realms_per_id = {realm.realm_id: realm for realm in rep.realms}
    wksp1 = realms_per_id[coolorg.wksp1_id]
    assert wksp1.users_per_role == {
        RealmRole.OWNER: 1,
        RealmRole.MANAGER: 1,
        RealmRole.CONTRIBUTOR: 0,
        RealmRole.READER: 0,
    }

    # Blocks should be taken into account in the data size
    outcome = await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        block_id=BlockID.new(),
        key_index=1,
        block=b"<block content>",
    )
    assert outcome is None

    rep = await coolorg.alice.realm_list_info()
    assert isinstance(rep, authenticated_cmds.latest.realm_list_info.RepOk)
    realms_per_id = {realm.realm_id: realm for realm in rep.realms}
    wksp1 = realms_per_id[coolorg.wksp1_id]
    assert wksp1.data_size == len(b"<block content>")

    # Realms the author has no access to are also listed: Bob (STANDARD profile)
    # creates his own realm, that alice (ADMIN profile) has no access to
    certif = generate_realm_role_certificate(
        coolorg,
        author=coolorg.bob.device_id,
        user_id=coolorg.bob.user_id,
        role=RealmRole.OWNER,
        realm_id=VlobID.new(),
    )
    outcome = await backend.realm.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.bob.device_id,
        author_verify_key=coolorg.bob.signing_key.verify_key,
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key),
    )
    assert isinstance(outcome, RealmRoleCertificate)

    rep = await coolorg.alice.realm_list_info()
    assert isinstance(rep, authenticated_cmds.latest.realm_list_info.RepOk)
    assert len(rep.realms) == 4
    realms_per_id = {realm.realm_id: realm for realm in rep.realms}
    bob_realm = realms_per_id[certif.realm_id]
    assert bob_realm.users_per_role == {
        RealmRole.OWNER: 1,
        RealmRole.MANAGER: 0,
        RealmRole.CONTRIBUTOR: 0,
        RealmRole.READER: 0,
    }
    assert bob_realm.metadata_size == 0
    assert bob_realm.data_size == 0


async def test_authenticated_realm_list_info_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    # Bob has STANDARD profile and mallory has OUTSIDER profile
    rep = await coolorg.bob.realm_list_info()
    assert rep == authenticated_cmds.latest.realm_list_info.RepAuthorNotAllowed()

    rep = await coolorg.mallory.realm_list_info()
    assert rep == authenticated_cmds.latest.realm_list_info.RepAuthorNotAllowed()


async def test_authenticated_realm_list_info_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.realm_list_info()

    await authenticated_http_common_errors_tester(do)
