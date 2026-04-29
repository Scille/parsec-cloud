# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    RealmArchivingCertificate,
    RealmArchivingConfiguration,
    RealmNameCertificate,
    RealmRole,
    VlobID,
    authenticated_cmds,
)
from parsec.events import EventRealmCertificate
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    WorkspaceArchivedOrgRpcClients,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


def _alice_archiving_certificate(
    coolorg: CoolorgRpcClients,
    configuration: RealmArchivingConfiguration | None = None,
    timestamp: DateTime | None = None,
) -> RealmArchivingCertificate:
    return RealmArchivingCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp if timestamp is not None else DateTime.now(),
        realm_id=coolorg.wksp1_id,
        configuration=configuration
        if configuration is not None
        else RealmArchivingConfiguration.AVAILABLE,
    )


async def test_authenticated_realm_update_archiving_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    t0 = DateTime.now()
    t1 = t0.add(seconds=1)
    t2 = t0.add(seconds=2)
    t3 = t0.add(seconds=3)
    t4 = t0.add(seconds=4)
    certifs = [
        _alice_archiving_certificate(coolorg, RealmArchivingConfiguration.AVAILABLE, timestamp=t1),
        # Can provide multiple time the same configuration
        _alice_archiving_certificate(coolorg, RealmArchivingConfiguration.AVAILABLE, timestamp=t2),
        _alice_archiving_certificate(coolorg, RealmArchivingConfiguration.ARCHIVED, timestamp=t3),
        # Deletion_date must be at least 30 days (minimum_archiving_period) in the future
        _alice_archiving_certificate(
            coolorg,
            RealmArchivingConfiguration.deletion_planned(t3.add(days=30, seconds=1)),
            timestamp=t4,
        ),
    ]

    expected_topics = await backend.organization.test_dump_topics(coolorg.organization_id)

    for certif in certifs:
        expected_topics.realms[certif.realm_id] = certif.timestamp

        with backend.event_bus.spy() as spy:
            rep = await coolorg.alice.realm_update_archiving(
                archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
            )
            assert rep == authenticated_cmds.latest.realm_update_archiving.RepOk()
            await spy.wait_event_occurred(
                EventRealmCertificate(
                    organization_id=coolorg.organization_id,
                    timestamp=certif.timestamp,
                    realm_id=certif.realm_id,
                    user_id=coolorg.alice.user_id,
                    role_removed=False,
                )
            )

        topics = await backend.organization.test_dump_topics(coolorg.organization_id)
        assert topics == expected_topics


async def test_authenticated_realm_update_archiving_ok_realm_archived(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
) -> None:
    certif = RealmArchivingCertificate(
        author=workspace_archived_org.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=workspace_archived_org.wksp_archived_id,
        configuration=RealmArchivingConfiguration.AVAILABLE,
    )
    rep = await workspace_archived_org.alice.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(workspace_archived_org.alice.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepOk()


@pytest.mark.parametrize(
    "kind",
    (
        "as_reader",
        "as_contributor",
        "as_manager",
        "no_access",
        "no_longer_allowed",
    ),
)
async def test_authenticated_realm_update_archiving_author_not_allowed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "as_reader":
            # Bob has access to the realm as READER
            author = coolorg.bob

        case "as_contributor":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.bob.user_id, RealmRole.CONTRIBUTOR
            )
            author = coolorg.bob

        case "as_manager":
            await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
            author = coolorg.bob

        case "no_access":
            # Mallory has no access to the realm
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=RealmRole.MANAGER
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    certif = RealmArchivingCertificate(
        author=author.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        configuration=RealmArchivingConfiguration.AVAILABLE,
    )
    rep = await author.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(author.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepAuthorNotAllowed()


async def test_authenticated_realm_update_archiving_realm_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    bad_realm_id = VlobID.new()
    certif = RealmArchivingCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=bad_realm_id,
        configuration=RealmArchivingConfiguration.AVAILABLE,
    )
    rep = await coolorg.alice.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepRealmNotFound()


async def test_authenticated_realm_update_archiving_realm_deleted(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    # First schedule deletion in the "past" (i.e., with a date that has already passed)
    # We do this by directly calling the backend method with a past deletion_date and now
    past_deletion_date = DateTime.now().subtract(days=1)
    past_timestamp = DateTime.now().subtract(days=60)

    certif = RealmArchivingCertificate(
        author=coolorg.alice.device_id,
        timestamp=past_timestamp,
        realm_id=coolorg.wksp1_id,
        configuration=RealmArchivingConfiguration.deletion_planned(past_deletion_date),
    )
    # Use backend directly to bypass the minimum_archiving_period check and set a past deletion
    outcome = await backend.realm.update_archiving(
        now=past_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmArchivingCertificate)

    # Now try to update archiving on a deleted realm
    certif2 = _alice_archiving_certificate(coolorg, RealmArchivingConfiguration.AVAILABLE)
    rep = await coolorg.alice.realm_update_archiving(
        archiving_certificate=certif2.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepRealmDeleted()


async def test_authenticated_realm_update_archiving_archiving_period_too_short(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    timestamp = DateTime.now()
    absolute_minimum_deletion_date = timestamp.add(
        seconds=backend.config.organization_initial_minimum_archiving_period
    )

    certif = _alice_archiving_certificate(
        coolorg,
        RealmArchivingConfiguration.deletion_planned(
            absolute_minimum_deletion_date.subtract(microseconds=1)
        ),
        timestamp=timestamp,
    )
    rep = await coolorg.alice.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepArchivingPeriodTooShort()

    # Ensure the absolute minimal archiving period can be used (i.e. the server uses the certificate
    # timestamp as the starting time instead of the current time to avoid having a check that depends
    # on the request's lag)
    certif = _alice_archiving_certificate(
        coolorg,
        RealmArchivingConfiguration.deletion_planned(absolute_minimum_deletion_date),
        timestamp=timestamp,
    )
    rep = await coolorg.alice.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepOk()


@pytest.mark.parametrize("kind", ("dummy_data", "bad_author"))
async def test_authenticated_realm_update_archiving_invalid_certificate(
    coolorg: CoolorgRpcClients,
    kind: str,
) -> None:
    match kind:
        case "dummy_data":
            certif = b"<dummy data>"
        case "bad_author":
            certif = RealmArchivingCertificate(
                author=coolorg.bob.device_id,
                timestamp=DateTime.now(),
                realm_id=coolorg.wksp1_id,
                configuration=RealmArchivingConfiguration.AVAILABLE,
            ).dump_and_sign(coolorg.bob.signing_key)
        case unknown:
            assert False, unknown

    rep = await coolorg.alice.realm_update_archiving(archiving_certificate=certif)
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepInvalidCertificate()


@pytest.mark.parametrize("kind", ("same_timestamp", "previous_timestamp"))
async def test_authenticated_realm_update_archiving_require_greater_timestamp(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    last_certificate_timestamp = DateTime.now()
    match kind:
        case "same_timestamp":
            realm_archiving_timestamp = last_certificate_timestamp
        case "previous_timestamp":
            realm_archiving_timestamp = last_certificate_timestamp.subtract(seconds=1)
        case unknown:
            assert False, unknown

    # 1) Create some data (e.g. certificate, vlob) with a given timestamp

    rename_certif = RealmNameCertificate(
        author=coolorg.alice.device_id,
        timestamp=last_certificate_timestamp,
        realm_id=coolorg.wksp1_id,
        key_index=1,
        encrypted_name=b"<encrypted name>",
    )
    outcome = await backend.realm.rename(
        now=last_certificate_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_name_certificate=rename_certif.dump_and_sign(coolorg.alice.signing_key),
        initial_name_or_fail=False,
    )
    assert isinstance(outcome, RealmNameCertificate)

    # 2) Create realm archiving certificate where timestamp is clashing with the previous data

    certif = _alice_archiving_certificate(
        coolorg,
        RealmArchivingConfiguration.AVAILABLE,
        timestamp=realm_archiving_timestamp,
    )
    rep = await coolorg.alice.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepRequireGreaterTimestamp(
        strictly_greater_than=last_certificate_timestamp
    )


async def test_authenticated_realm_update_archiving_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    timestamp_out_of_ballpark: DateTime,
) -> None:
    certif = _alice_archiving_certificate(coolorg, timestamp=timestamp_out_of_ballpark)
    rep = await coolorg.alice.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(
        rep, authenticated_cmds.latest.realm_update_archiving.RepTimestampOutOfBallpark
    )
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == timestamp_out_of_ballpark


async def test_authenticated_realm_update_archiving_realm_deleted_from_workspace_archived(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients, backend: Backend
) -> None:
    certif = RealmArchivingCertificate(
        author=workspace_archived_org.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=workspace_archived_org.wksp_deleted_id,
        configuration=RealmArchivingConfiguration.AVAILABLE,
    )
    rep = await workspace_archived_org.alice.realm_update_archiving(
        archiving_certificate=certif.dump_and_sign(workspace_archived_org.alice.signing_key),
    )
    assert rep == authenticated_cmds.latest.realm_update_archiving.RepRealmDeleted()


async def test_authenticated_realm_update_archiving_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        certif = _alice_archiving_certificate(coolorg)
        await coolorg.alice.realm_update_archiving(
            archiving_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        )

    await authenticated_http_common_errors_tester(do)
