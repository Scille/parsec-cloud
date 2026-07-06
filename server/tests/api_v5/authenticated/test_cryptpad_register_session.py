# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, VlobID, authenticated_cmds
from parsec.config import CryptpadConfig
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    WorkspaceArchivedOrgRpcClients,
    get_last_realm_certificate_timestamp,
)

COMMON_CERTIFICATE_TIMESTAMP = DateTime(2000, 1, 6)


@pytest.fixture
def with_cryptpad_configured(backend: Backend) -> None:
    backend.config.cryptpad_config = CryptpadConfig(server_url="http://cryptpad.invalid")


async def test_authenticated_cryptpad_register_session_ok_create_with_only_read_access(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    realm_id = coolorg.wksp1_id
    document_id = VlobID.new()
    last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=realm_id,
    )

    t1 = DateTime.now()
    t2 = t1.add(seconds=1)
    t3 = t2.add(seconds=1)

    # Create with a READER

    rep = await coolorg.bob.cryptpad_register_session(
        realm_id=realm_id,
        document_id=document_id,
        key_index=1,
        timestamp=t1,
        encrypted_candidate_view_key=b"<view_key_1_from_bob>",
        encrypted_candidate_edit_key=None,
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepOk(
        author=coolorg.bob.device_id,
        timestamp=t1,
        key_index=1,
        encrypted_view_key=b"<view_key_1_from_bob>",
        encrypted_edit_key=None,
        needed_common_certificate_timestamp=COMMON_CERTIFICATE_TIMESTAMP,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )

    # Get the session as a READER

    rep = await coolorg.bob.cryptpad_register_session(
        realm_id=realm_id,
        document_id=document_id,
        key_index=1,
        timestamp=t2,
        encrypted_candidate_view_key=b"<view_key_2_from_bob>",
        encrypted_candidate_edit_key=None,
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepOk(
        author=coolorg.bob.device_id,
        timestamp=t1,
        key_index=1,
        encrypted_view_key=b"<view_key_1_from_bob>",
        encrypted_edit_key=None,
        needed_common_certificate_timestamp=COMMON_CERTIFICATE_TIMESTAMP,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )

    # Replace the session since a non-READER tries to get it

    rep = await coolorg.alice.cryptpad_register_session(
        realm_id=realm_id,
        document_id=document_id,
        key_index=1,
        timestamp=t3,
        encrypted_candidate_view_key=b"<view_key_1_from_alice>",
        encrypted_candidate_edit_key=b"<edit_key_1_from_alice>",
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepOk(
        author=coolorg.alice.device_id,
        timestamp=t3,
        key_index=1,
        encrypted_view_key=b"<view_key_1_from_alice>",
        encrypted_edit_key=b"<edit_key_1_from_alice>",
        needed_common_certificate_timestamp=COMMON_CERTIFICATE_TIMESTAMP,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )


async def test_authenticated_cryptpad_register_session_ok_create_with_write_access(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    realm_id = coolorg.wksp1_id
    document_id = VlobID.new()
    last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=realm_id,
    )

    t1 = DateTime.now()
    t2 = t1.add(seconds=1)
    t3 = t2.add(seconds=1)

    # Create with a non-READER

    rep = await coolorg.alice.cryptpad_register_session(
        realm_id=realm_id,
        document_id=document_id,
        key_index=1,
        timestamp=t1,
        encrypted_candidate_view_key=b"<view_key_1_from_alice>",
        encrypted_candidate_edit_key=b"<edit_key_1_from_alice>",
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepOk(
        author=coolorg.alice.device_id,
        timestamp=t1,
        key_index=1,
        encrypted_view_key=b"<view_key_1_from_alice>",
        encrypted_edit_key=b"<edit_key_1_from_alice>",
        needed_common_certificate_timestamp=COMMON_CERTIFICATE_TIMESTAMP,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )

    # Get the session as a non-READER

    rep = await coolorg.alice.cryptpad_register_session(
        realm_id=realm_id,
        document_id=document_id,
        key_index=1,
        timestamp=t2,
        encrypted_candidate_view_key=b"<view_key_2_from_alice>",
        encrypted_candidate_edit_key=b"<edit_key_2_from_alice>",
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepOk(
        author=coolorg.alice.device_id,
        timestamp=t1,
        key_index=1,
        encrypted_view_key=b"<view_key_1_from_alice>",
        encrypted_edit_key=b"<edit_key_1_from_alice>",
        needed_common_certificate_timestamp=COMMON_CERTIFICATE_TIMESTAMP,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )

    # Get the session as a READER

    rep = await coolorg.bob.cryptpad_register_session(
        realm_id=realm_id,
        document_id=document_id,
        key_index=1,
        timestamp=t3,
        encrypted_candidate_view_key=b"<view_key_1_from_bob>",
        encrypted_candidate_edit_key=None,
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepOk(
        author=coolorg.alice.device_id,
        timestamp=t1,
        key_index=1,
        encrypted_view_key=b"<view_key_1_from_alice>",
        encrypted_edit_key=None,
        needed_common_certificate_timestamp=COMMON_CERTIFICATE_TIMESTAMP,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )


async def test_authenticated_cryptpad_register_session_realm_not_found(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    rep = await coolorg.alice.cryptpad_register_session(
        realm_id=VlobID.new(),
        document_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=b"<edit_key>",
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepRealmNotFound()


async def test_authenticated_cryptpad_register_session_realm_deleted(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    rep = await workspace_archived_org.alice.cryptpad_register_session(
        realm_id=workspace_archived_org.wksp_deleted_id,
        document_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=b"<edit_key>",
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepRealmDeleted()


async def test_authenticated_cryptpad_register_session_timestamp_out_of_ballpark(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    t0 = DateTime.now().subtract(seconds=3600)
    rep = await coolorg.alice.cryptpad_register_session(
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
        key_index=1,
        timestamp=t0,
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=b"<edit_key>",
    )
    assert isinstance(
        rep, authenticated_cmds.latest.cryptpad_register_session.RepTimestampOutOfBallpark
    )
    assert rep.ballpark_client_early_offset == 300.0
    assert rep.ballpark_client_late_offset == 320.0
    assert rep.client_timestamp == t0


async def test_authenticated_cryptpad_register_session_author_not_allowed_not_part_of_realm(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    rep = await coolorg.mallory.cryptpad_register_session(
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=b"<edit_key>",
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepAuthorNotAllowed()


async def test_authenticated_cryptpad_register_session_author_not_allowed_with_only_read_access(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    rep = await coolorg.bob.cryptpad_register_session(
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=b"<edit_key>",  # Not allowed to provide the edit key!
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepAuthorNotAllowed()


async def test_authenticated_cryptpad_register_session_author_not_allowed_with_right_access(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    rep = await coolorg.alice.cryptpad_register_session(
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=None,  # Must provide an edit key!
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepAuthorNotAllowed()


async def test_authenticated_cryptpad_register_session_bad_key_index(
    coolorg: CoolorgRpcClients,
    with_cryptpad_configured: None,
) -> None:
    last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=coolorg.wksp1_id,
    )

    rep = await coolorg.alice.cryptpad_register_session(
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
        key_index=10,
        timestamp=DateTime.now(),
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=b"<edit_key>",
    )
    assert rep == authenticated_cmds.latest.cryptpad_register_session.RepBadKeyIndex(
        last_realm_certificate_timestamp=last_realm_certificate_timestamp
    )


async def test_authenticated_cryptpad_register_session_cryptpad_unavailable(
    coolorg: CoolorgRpcClients,
) -> None:
    # Mallory is not part of the workspace
    rep = await coolorg.mallory.cryptpad_register_session(
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
        key_index=1,
        timestamp=DateTime.now(),
        encrypted_candidate_view_key=b"<view_key>",
        encrypted_candidate_edit_key=b"<edit_key>",
    )
    assert isinstance(
        rep, authenticated_cmds.latest.cryptpad_register_session.RepCryptpadUnavailable
    )


async def test_authenticated_cryptpad_register_session_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.cryptpad_register_session(
            realm_id=coolorg.wksp1_id,
            document_id=VlobID.new(),
            key_index=1,
            timestamp=DateTime.now(),
            encrypted_candidate_view_key=b"<view_key>",
            encrypted_candidate_edit_key=b"<edit_key>",
        )

    await authenticated_http_common_errors_tester(do)
