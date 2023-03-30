# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from unittest.mock import ANY, Mock
from urllib.error import HTTPError, URLError

import pytest

from parsec.api.data import EntryName
from parsec.api.protocol import RealmID, VlobID
from parsec.backend.sequester import SequesterServiceType
from parsec.core.core_events import CoreEvent
from parsec.core.fs.exceptions import FSServerUploadTemporarilyUnavailableError
from parsec.core.fs.path import FsPath
from tests.common import customize_fixtures, sequester_service_factory


@pytest.mark.trio
@customize_fixtures(
    coolorg_is_sequestered_organization=True,
    alice_initial_remote_user_manifest="not_synced",
    alice_initial_local_user_manifest="speculative_v0",
    bob_initial_remote_user_manifest="not_synced",
    bob_initial_local_user_manifest="non_speculative_v0",
)
async def test_userfs_sequester_sync(
    running_backend, backend, alice_user_fs, bob_user_fs, coolorg, alice, bob
):
    async def _new_sequester_service(label: str):
        service = sequester_service_factory(authority=coolorg.sequester_authority, label=label)
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service.backend_service
        )
        return service

    async def _assert_sequester_dump(service, local_device, expected_versions):
        realm_dump = await backend.sequester.dump_realm(
            organization_id=coolorg.organization_id,
            service_id=service.service_id,
            realm_id=RealmID.from_entry_id(local_device.user_manifest_id),
        )
        user_manifest_vlob_id = VlobID.from_entry_id(local_device.user_manifest_id)

        # Simple check: make sure we retreive the expected items
        items = {(vlob_id, version) for (vlob_id, version, _) in realm_dump}
        expected_items = {(user_manifest_vlob_id, v) for v in expected_versions}
        assert items == expected_items

        # Advanced check: make sure each item contain the valid data
        for _, version, sequestered_blob in realm_dump:
            clear_blob_from_sequester = service.decryption_key.decrypt(sequestered_blob)
            _, blob, _, _, _ = await backend.vlob.read(
                organization_id=coolorg.organization_id,
                author=local_device.device_id,
                encryption_revision=1,
                vlob_id=user_manifest_vlob_id,
                version=version,
            )
            clear_blob = local_device.user_manifest_key.decrypt(blob)

            assert clear_blob == clear_blob_from_sequester

    # 1) Alice sync v1 without sequester services
    await alice_user_fs.sync()

    # Create a sequester service
    s1 = await _new_sequester_service("SequesterService1")

    # 2) Bob sync v1 with a sequester service
    await bob_user_fs.sync()

    # 3) Bob sync v2
    await bob_user_fs.workspace_create(EntryName("bob_w1"))
    await bob_user_fs.sync()

    # New sequester service, next syncs should take it into account
    s2 = await _new_sequester_service("Sequester service 2")

    # 4) Alice sync v2 with a new sequester service
    await alice_user_fs.workspace_create(EntryName("alice_w2"))
    await alice_user_fs.sync()

    # 5) Finally check the sequester services contains valid data

    # S1 is expected to contain Alice v2 and Bob v1&v2
    await _assert_sequester_dump(service=s1, local_device=alice, expected_versions={2})
    await _assert_sequester_dump(service=s1, local_device=bob, expected_versions={1, 2})

    # S2 is expected to contain only Bob v2
    await _assert_sequester_dump(service=s2, local_device=alice, expected_versions={2})
    await _assert_sequester_dump(service=s2, local_device=bob, expected_versions={})


@pytest.mark.trio
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_workspacefs_sequester_sync(running_backend, backend, alice_user_fs, coolorg, alice):
    async def _new_sequester_service(label: str):
        service = sequester_service_factory(authority=coolorg.sequester_authority, label=label)
        await backend.sequester.create_service(
            organization_id=coolorg.organization_id, service=service.backend_service
        )
        return service

    async def _assert_sequester_dump(service, workspace, expected_items):
        realm_id = RealmID.from_entry_id(workspace.workspace_id)
        realm_dump = await backend.sequester.dump_realm(
            organization_id=coolorg.organization_id,
            service_id=service.service_id,
            realm_id=realm_id,
        )

        # Simple check: make sure we retreive the expected items
        items = {(vlob_id, version) for (vlob_id, version, _) in realm_dump}
        expected_items = {
            (VlobID.from_entry_id(entry_id), version) for (entry_id, version) in expected_items
        }
        assert items == expected_items

        # Advanced check: make sure each item contain the valid data
        for vlob_id, version, sequestered_blob in realm_dump:
            clear_blob_from_sequester = service.decryption_key.decrypt(sequestered_blob)
            _, blob, _, _, _ = await backend.vlob.read(
                organization_id=coolorg.organization_id,
                author=alice.device_id,
                encryption_revision=1,
                vlob_id=vlob_id,
                version=version,
            )
            clear_blob = workspace.get_workspace_entry().key.decrypt(blob)

            assert clear_blob == clear_blob_from_sequester

    # 1) No sequester services, sync w1@v1 and w1f1@v1
    w1_id = await alice_user_fs.workspace_create(EntryName("w1"))
    w1 = alice_user_fs.get_workspace(w1_id)
    await w1.touch("/w1f1")
    await w1.sync()

    # Create a sequester service
    s1 = await _new_sequester_service("SequesterService1")

    # 2) Sync w2@v1 with a sequester service
    w2_id = await alice_user_fs.workspace_create(EntryName("w2"))
    w2 = alice_user_fs.get_workspace(w2_id)
    await w2.sync()

    # 3) Sync w2@v2, w2f1@v1
    await w2.touch("/w2f1")
    w2f1_id = await w2.path_id("/w2f1")
    await w2.sync()

    # New sequester service, next syncs should take it into account
    s2 = await _new_sequester_service("Sequester service 2")

    # 4) Sync w1@v2 with a new sequester service
    await w1.unlink("/w1f1")
    await w1.sync()

    # 5) Finally check the sequester services contains valid data

    # S1 is expected to contain w2@v1, w2@v2, w2f1@v1, w1@v2
    await _assert_sequester_dump(service=s1, workspace=w1, expected_items={(w1_id, 2)})
    await _assert_sequester_dump(
        service=s1, workspace=w2, expected_items={(w2_id, 1), (w2_id, 2), (w2f1_id, 1)}
    )

    # S1 is expected to contain only w1@v2
    await _assert_sequester_dump(service=s2, workspace=w1, expected_items={(w1_id, 2)})
    await _assert_sequester_dump(service=s2, workspace=w2, expected_items={})


@pytest.mark.trio
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_webhook_timeout_and_rejected(
    monkeypatch, unused_tcp_port, running_backend, alice_user_fs, coolorg
):
    webhook_calls = 0

    def _mock_webhook_response(outcome):
        async def _mocked_http_request(**kwargs):
            nonlocal webhook_calls
            webhook_calls += 1
            if isinstance(outcome, Exception):
                raise outcome
            else:
                return outcome

        monkeypatch.setattr("parsec.backend.vlob.http_request", _mocked_http_request)

    # Create a workspace & make sure evrything is sync so far
    wid = await alice_user_fs.workspace_create(EntryName("w"))
    await alice_user_fs.sync()
    alice_workspace = alice_user_fs.get_workspace(wid)

    # Now add a sequester service that will work... unhelpfully ;-)
    webhook_url = f"https://localhost:{unused_tcp_port}/webhook"
    s1 = sequester_service_factory(
        authority=coolorg.sequester_authority,
        label="SequesterService1",
        service_type=SequesterServiceType.WEBHOOK,
        webhook_url=webhook_url,
    )
    await running_backend.backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s1.backend_service
    )

    ################################
    # 1) First test workspacefs
    ################################

    # 1.a) Test sequester timeout

    await alice_workspace.write_bytes("/test.txt", b"v1")
    # Cannot sync the change given webhook is not available
    _mock_webhook_response(outcome=URLError("[Errno -2] Name or service not known"))
    with pytest.raises(FSServerUploadTemporarilyUnavailableError):
        await alice_workspace.sync()
    # Now webhook is back online
    _mock_webhook_response(outcome=b"")
    await alice_workspace.sync()

    # So we called the webhook 4 times:
    # - first time failed
    # - second time for the file manifest minimal sync
    # - third time for the parent folder manifest sync
    # - fourth time for the actual file manifest sync
    assert webhook_calls == 4
    root_info = await alice_workspace.path_info("/")
    assert root_info["need_sync"] is False
    assert root_info["base_version"] == 2
    file_info = await alice_workspace.path_info("/test.txt")
    assert file_info["need_sync"] is False
    assert file_info["base_version"] == 2

    # 1.b) Test sequester rejection

    fp = Mock()
    fp.read.return_value = b'{"reason": "some_error_from_service"}'
    _mock_webhook_response(outcome=HTTPError(webhook_url, 400, "", None, fp))

    await alice_workspace.write_bytes("/test.txt", b"v2 with virus !")
    with alice_user_fs.event_bus.listen() as spy:
        await alice_workspace.sync()

        sync_rejected_events = [
            e.kwargs
            for e in spy.events
            if e.event == CoreEvent.FS_ENTRY_SYNC_REJECTED_BY_SEQUESTER_SERVICE
        ]
        assert sync_rejected_events == [
            {
                "service_id": s1.service_id,
                "service_label": "SequesterService1",
                "reason": "some_error_from_service",
                "workspace_id": wid,
                "entry_id": ANY,
                "file_path": FsPath("/test.txt"),
            }
        ]

    # The sync operation went fine, but in fact no sync occured...
    file_info = await alice_workspace.path_info("/test.txt")
    assert file_info["need_sync"] is True
    assert file_info["base_version"] == 2

    # ...so if we modify again the file everything should be synced fine
    _mock_webhook_response(outcome=b"")
    await alice_workspace.write_bytes("/test.txt", b"v2")
    await alice_workspace.sync()

    file_info = await alice_workspace.path_info("/test.txt")
    assert file_info["need_sync"] is False
    assert file_info["base_version"] == 3

    ################################
    # 2) Now test userfs
    ################################

    webhook_calls = 0

    # 2.a) Test sequester service timeout

    await alice_user_fs.workspace_rename(wid, EntryName("new_name"))
    # Cannot sync the change given webhook is not available
    _mock_webhook_response(outcome=URLError("[Errno -2] Name or service not known"))
    with pytest.raises(FSServerUploadTemporarilyUnavailableError):
        await alice_user_fs.sync()
    # Now webhook is back online
    _mock_webhook_response(outcome=b"")
    await alice_user_fs.sync()

    # So we called the webhook 4 times:
    # - first time failed
    # - second successufly synced the user manifest
    assert webhook_calls == 2

    # 2.b) Test sequester service rejection

    fp = Mock()
    fp.read.return_value = b'{"reason": "some_error_from_service"}'
    _mock_webhook_response(outcome=HTTPError(webhook_url, 400, "", None, fp))

    await alice_user_fs.workspace_rename(wid, EntryName("new_new_name"))
    with alice_user_fs.event_bus.listen() as spy:
        await alice_user_fs.sync()

        sync_rejected_events = [
            e.kwargs
            for e in spy.events
            if e.event == CoreEvent.USERFS_SYNC_REJECTED_BY_SEQUESTER_SERVICE
        ]
        assert sync_rejected_events == [
            {
                "service_id": s1.service_id,
                "service_label": "SequesterService1",
                "reason": "some_error_from_service",
            }
        ]

    # The sync operation went fine, but in fact no sync occured...
    um = alice_user_fs.get_user_manifest()
    assert um.need_sync is True

    # ...so if we modify again the file everything should be synced fine
    _mock_webhook_response(outcome=b"")
    await alice_user_fs.workspace_rename(wid, EntryName("new_new_name"))
    await alice_user_fs.sync()
    um = alice_user_fs.get_user_manifest()
    assert um.need_sync is False
