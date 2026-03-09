# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    RealmRole,
    RealmRoleCertificate,
    UserProfile,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EventRealmCertificate
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    WorkspaceArchivedOrgRpcClients,
    alice_gives_profile,
    bob_becomes_admin_and_changes_alice,
    get_last_realm_certificate_timestamp,
    wksp1_alice_gives_role,
)


@pytest.mark.parametrize(
    "kind",
    (
        "mallory_manager",
        "bob_contributor",
        "mallory_and_bob_reader",
    ),
)
async def test_authenticated_realm_self_promote_to_owner_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    # In wksp1, Bob is READER and Alice is the active OWNER
    match kind:
        case "mallory_manager":
            # Mallory becomes MANAGER, Bob stays READER
            # Mallory is originally OUTSIDER which cannot becomes MANAGER of a workspace
            await alice_gives_profile(
                coolorg, backend, coolorg.mallory.user_id, UserProfile.STANDARD
            )
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.MANAGER
            )
            await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)
            author = coolorg.mallory

        case "bob_contributor":
            # Bob becomes CONTRIBUTOR
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR
            )
            await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)
            author = coolorg.bob

        case "mallory_and_bob_reader":
            # Bob is already READER, Mallory becomes READER
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)
            author = coolorg.bob

        case unknown:
            assert False, unknown

    certif = RealmRoleCertificate(
        author=author.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=author.user_id,
        role=RealmRole.OWNER,
    )

    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    expected_topics.realms[coolorg.wksp1_id] = certif.timestamp

    with backend.event_bus.spy() as spy:
        rep = await author.realm_self_promote_to_owner(
            realm_role_certificate=certif.dump_and_sign(author.signing_key)
        )
        assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=False,
            )
        )

    author_realms = await backend.realm.get_current_realms_for_user(
        coolorg.organization_id, author.user_id
    )
    assert isinstance(author_realms, dict)
    assert author_realms[coolorg.wksp1_id] == RealmRole.OWNER

    topics = await backend.organization.test_dump_topics(coolorg.organization_id)
    assert topics == expected_topics


@pytest.mark.parametrize(
    "kind",
    (
        "not_a_member",
        "lower_role_than_highest",
        "outsider",
    ),
)
async def test_authenticated_realm_self_promote_to_owner_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "not_a_member":
            # In wksp1, Bob is READER and Alice is the active OWNER
            await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)
            author = coolorg.mallory

        case "lower_role_than_highest":
            # In wksp1, Bob is READER and Alice is the active OWNER
            # Update Bob role twice, this way he used to be higher than Mallory
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR
            )
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.READER)
            await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)
            author = coolorg.bob

        case "outsider":
            # Mallory has OUTSIDER profile and is given the same role than Bob in wksp1
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.mallory.user_id, RealmRole.READER
            )
            # In wksp1, Bob is READER and Alice is the active OWNER
            await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)
            author = coolorg.mallory

        case unknown:
            assert False, unknown

    certif = RealmRoleCertificate(
        author=author.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=author.user_id,
        role=RealmRole.OWNER,
    )
    rep = await author.realm_self_promote_to_owner(
        realm_role_certificate=certif.dump_and_sign(author.signing_key)
    )
    assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepAuthorNotAllowed()


async def test_authenticated_realm_self_promote_to_owner_realm_not_found(
    minimalorg: MinimalorgRpcClients,
) -> None:
    bad_realm_id = VlobID.new()
    certif = RealmRoleCertificate(
        author=minimalorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=bad_realm_id,
        user_id=minimalorg.alice.user_id,
        role=RealmRole.OWNER,
    )
    rep = await minimalorg.alice.realm_self_promote_to_owner(
        realm_role_certificate=certif.dump_and_sign(minimalorg.alice.signing_key)
    )
    assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepRealmNotFound()


async def test_authenticated_realm_self_promote_to_owner_realm_deleted(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients, backend: Backend
) -> None:
    certif = RealmRoleCertificate(
        author=workspace_archived_org.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=workspace_archived_org.wksp_deleted_id,
        user_id=workspace_archived_org.alice.user_id,
        role=RealmRole.OWNER,
    )
    rep = await workspace_archived_org.alice.realm_self_promote_to_owner(
        realm_role_certificate=certif.dump_and_sign(workspace_archived_org.alice.signing_key)
    )
    assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepRealmDeleted()


async def test_authenticated_realm_self_promote_to_owner_active_owner_already_exists(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    async def _do_self_promote() -> authenticated_cmds.latest.realm_self_promote_to_owner.Rep:
        certif = RealmRoleCertificate(
            author=coolorg.bob.device_id,
            timestamp=DateTime.now(),
            realm_id=coolorg.wksp1_id,
            user_id=coolorg.bob.user_id,
            role=RealmRole.OWNER,
        )
        return await coolorg.bob.realm_self_promote_to_owner(
            realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key)
        )

    # In wksp1, Bob is READER and Alice is the active OWNER

    last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
        coolorg.testbed_template, coolorg.wksp1_id
    )
    rep = await _do_self_promote()
    assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepActiveOwnerAlreadyExists(
        last_realm_certificate_timestamp=last_realm_certificate_timestamp
    )

    # Same test, but after a successful self-promotion

    await wksp1_alice_gives_role(coolorg, backend, coolorg.mallory.user_id, RealmRole.READER)
    await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)

    certif = RealmRoleCertificate(
        author=coolorg.bob.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.bob.user_id,
        role=RealmRole.OWNER,
    )
    outcome = await backend.realm.self_promote_to_owner(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.bob.device_id,
        author_verify_key=coolorg.bob.signing_key.verify_key,
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key),
    )
    assert isinstance(outcome, RealmRoleCertificate)
    last_realm_certificate_timestamp = outcome.timestamp

    rep = await _do_self_promote()
    assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepActiveOwnerAlreadyExists(
        last_realm_certificate_timestamp=last_realm_certificate_timestamp
    )


@pytest.mark.parametrize(
    "kind",
    (
        "dummy_certif",
        "role_not_owner",
        "user_id_mismatch",
        "wrong_signing_key",
    ),
)
async def test_authenticated_realm_self_promote_to_owner_invalid_certificate(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    # In wksp1, Bob is READER and Alice is the active OWNER
    await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)

    match kind:
        case "dummy_certif":
            certif_bytes = b"<dummy>"

        case "role_not_owner":
            certif_bytes = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=RealmRole.MANAGER,
            ).dump_and_sign(coolorg.bob.signing_key)

        case "user_id_mismatch":
            # Certificate says user_id=alice but is signed by bob
            certif_bytes = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.alice.user_id,
                role=RealmRole.OWNER,
            ).dump_and_sign(coolorg.bob.signing_key)

        case "wrong_signing_key":
            # Signed by alice's key but claims bob as author
            certif_bytes = RealmRoleCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                user_id=coolorg.bob.user_id,
                role=RealmRole.OWNER,
            ).dump_and_sign(coolorg.alice.signing_key)

        case unknown:
            assert False, unknown

    rep = await coolorg.bob.realm_self_promote_to_owner(realm_role_certificate=certif_bytes)
    assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepInvalidCertificate()


async def test_authenticated_realm_self_promote_to_owner_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    # In wksp1, Bob is READER and Alice is the active OWNER
    await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)

    certif = RealmRoleCertificate(
        author=coolorg.bob.device_id,
        timestamp=timestamp_out_of_ballpark,
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.bob.user_id,
        role=RealmRole.OWNER,
    )
    rep = await coolorg.bob.realm_self_promote_to_owner(
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key)
    )
    assert isinstance(
        rep, authenticated_cmds.latest.realm_self_promote_to_owner.RepTimestampOutOfBallpark
    )
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


@pytest.mark.parametrize("kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_realm_self_promote_to_owner_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    # In wksp1, Bob is READER and Alice is the active OWNER
    _, (alice_revoke_certif, _) = await bob_becomes_admin_and_changes_alice(
        coolorg, backend, new_alice_profile=None
    )
    last_timestamp = alice_revoke_certif.timestamp

    match kind:
        case "same_timestamp":
            self_promote_timestamp = last_timestamp
        case "previous_timestamp":
            self_promote_timestamp = last_timestamp.subtract(seconds=1)
        case unknown:
            assert False, unknown

    certif = RealmRoleCertificate(
        author=coolorg.bob.device_id,
        timestamp=self_promote_timestamp,
        realm_id=coolorg.wksp1_id,
        user_id=coolorg.bob.user_id,
        role=RealmRole.OWNER,
    )
    rep = await coolorg.bob.realm_self_promote_to_owner(
        realm_role_certificate=certif.dump_and_sign(coolorg.bob.signing_key)
    )
    assert rep == authenticated_cmds.latest.realm_self_promote_to_owner.RepRequireGreaterTimestamp(
        strictly_greater_than=last_timestamp
    )


async def test_authenticated_realm_self_promote_to_owner_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        certif = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=DateTime.now(),
            realm_id=coolorg.wksp1_id,
            user_id=coolorg.alice.user_id,
            role=RealmRole.OWNER,
        )
        await coolorg.alice.realm_self_promote_to_owner(
            realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )

    await authenticated_http_common_errors_tester(do)
