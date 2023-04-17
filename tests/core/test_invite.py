# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import contextmanager

import pytest
import trio

from parsec import FEATURE_FLAGS
from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    InvitationType,
    InviteAlreadyUsedError,
    InviteListRepOk,
)
from parsec.api.data import EntryName
from parsec.api.protocol import (
    DeviceLabel,
    HumanHandle,
    InvitationDeletedReason,
    InvitationStatus,
    UserProfile,
)
from parsec.backend.backend_events import BackendEvent
from parsec.core.backend_connection import (
    BackendInvitationAlreadyUsed,
    backend_authenticated_cmds_factory,
    backend_invited_cmds_factory,
)
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.core.invite import (
    DeviceClaimInitialCtx,
    DeviceGreetInitialCtx,
    InviteActiveUsersLimitReachedError,
    InvitePeerResetError,
    UserClaimInitialCtx,
    UserGreetInitialCtx,
    claimer_retrieve_info,
)
from parsec.core.types import BackendInvitationAddr, LocalDevice, WorkspaceRole
from tests.common import real_clock_timeout


@pytest.mark.trio
@pytest.mark.parametrize("with_labels", [False, True], ids=["without_labels", "with_labels"])
async def test_good_device_claim(
    backend,
    running_backend,
    alice,
    bob,
    alice_backend_cmds,
    user_fs_factory,
    with_labels,
    alice_new_device_invitation,
):
    if with_labels:
        requested_device_label = DeviceLabel("Foo's label")
        granted_device_label = DeviceLabel("Bar's label")
    else:
        requested_device_label = None
        granted_device_label = None
    new_device = None

    # Simulate out-of-bounds canal
    oob_send, oob_recv = trio.open_memory_channel(0)

    async def _run_claimer():
        async with backend_invited_cmds_factory(addr=alice_new_device_invitation) as cmds:
            initial_ctx = await claimer_retrieve_info(cmds)
            assert isinstance(initial_ctx, DeviceClaimInitialCtx)
            assert initial_ctx.greeter_user_id == alice.user_id
            assert initial_ctx.greeter_human_handle == alice.human_handle

            in_progress_ctx = await initial_ctx.do_wait_peer()

            choices = in_progress_ctx.generate_greeter_sas_choices(size=4)
            assert len(choices) == 4
            assert in_progress_ctx.greeter_sas in choices

            greeter_sas = await oob_recv.receive()
            assert greeter_sas == in_progress_ctx.greeter_sas

            in_progress_ctx = await in_progress_ctx.do_signify_trust()
            await oob_send.send(in_progress_ctx.claimer_sas)

            in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()

            nonlocal new_device
            new_device = await in_progress_ctx.do_claim_device(
                requested_device_label=requested_device_label
            )
            assert isinstance(new_device, LocalDevice)

    async def _run_greeter():
        initial_ctx = DeviceGreetInitialCtx(
            cmds=alice_backend_cmds, token=alice_new_device_invitation.token
        )

        in_progress_ctx = await initial_ctx.do_wait_peer()

        await oob_send.send(in_progress_ctx.greeter_sas)

        in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()

        choices = in_progress_ctx.generate_claimer_sas_choices(size=5)
        assert len(choices) == 5
        assert in_progress_ctx.claimer_sas in choices

        claimer_sas = await oob_recv.receive()
        assert claimer_sas == in_progress_ctx.claimer_sas

        in_progress_ctx = await in_progress_ctx.do_signify_trust()

        in_progress_ctx = await in_progress_ctx.do_get_claim_requests()

        assert in_progress_ctx.requested_device_label == requested_device_label

        await in_progress_ctx.do_create_new_device(author=alice, device_label=granted_device_label)

    async with real_clock_timeout():
        async with trio.open_nursery() as nursery:
            nursery.start_soon(_run_claimer)
            nursery.start_soon(_run_greeter)

    assert new_device is not None
    assert new_device.user_id == alice.user_id
    assert new_device.device_name != alice.device_name
    assert new_device.device_label == granted_device_label
    assert new_device.human_handle == alice.human_handle
    assert new_device.private_key == alice.private_key
    assert new_device.signing_key != alice.signing_key
    assert new_device.profile == alice.profile
    assert new_device.user_manifest_id == alice.user_manifest_id
    assert new_device.user_manifest_key == alice.user_manifest_key
    # Make sure greeter&claimer data are not mixed
    assert new_device.local_symkey != alice.local_symkey

    # Now invitation should have been deleted
    rep = await alice_backend_cmds.invite_list()
    assert rep == InviteListRepOk([])

    # Verify user&device data in backend
    _, device = await backend.user.get_user_with_device(
        new_device.organization_id, new_device.device_id
    )
    assert device.device_label == granted_device_label
    if with_labels:
        assert device.device_certificate != device.redacted_device_certificate
    else:
        assert device.device_certificate == device.redacted_device_certificate

    # Test the behavior of this new device
    async with user_fs_factory(bob) as bob_fs:
        async with user_fs_factory(alice) as alice_fs:
            async with user_fs_factory(new_device) as new_fs:
                # New device should start with a speculative user manifest
                um = new_fs.get_user_manifest()
                assert um.is_placeholder
                assert um.speculative

                # Old device modify user manifest
                await alice_fs.workspace_create(EntryName("wa"))
                await alice_fs.sync()

                # New sharing from other user
                wb_id = await bob_fs.workspace_create(EntryName("wb"))
                await bob_fs.workspace_share(wb_id, alice.user_id, WorkspaceRole.CONTRIBUTOR)

                # Test new device get access to both new workspaces
                await new_fs.process_last_messages()
                await new_fs.sync()
                new_fs_um = new_fs.get_user_manifest()

                # Make sure new and old device have the same view on data
                await alice_fs.sync()
                alice_fs_um = alice_fs.get_user_manifest()
                assert new_fs_um == alice_fs_um


@pytest.mark.trio
@pytest.mark.parametrize("with_labels", [False, True], ids=["without_labels", "with_labels"])
async def test_good_user_claim(
    backend,
    running_backend,
    data_base_dir,
    alice,
    alice_backend_cmds,
    user_fs_factory,
    with_labels,
    zack_new_user_invitation,
):
    if with_labels:
        # Let's pretent we invited a Fortnite player...
        requested_human_handle = HumanHandle(email="ZACK@example.com", label="xXx_Z4ck_xXx")
        requested_device_label = DeviceLabel("Ultr4_B00st")
        granted_human_handle = HumanHandle(email="zack@example.com", label="Zack")
        granted_device_label = DeviceLabel("Desktop")
    else:
        requested_human_handle = None
        requested_device_label = None
        granted_human_handle = None
        granted_device_label = None
    granted_profile = UserProfile.STANDARD
    new_device = None

    # Simulate out-of-bounds canal
    oob_send, oob_recv = trio.open_memory_channel(0)

    async def _run_claimer():
        async with backend_invited_cmds_factory(addr=zack_new_user_invitation) as cmds:
            initial_ctx = await claimer_retrieve_info(cmds)
            assert isinstance(initial_ctx, UserClaimInitialCtx)
            assert initial_ctx.claimer_email == "zack@example.com"
            assert initial_ctx.greeter_user_id == alice.user_id
            assert initial_ctx.greeter_human_handle == alice.human_handle

            in_progress_ctx = await initial_ctx.do_wait_peer()

            choices = in_progress_ctx.generate_greeter_sas_choices(size=4)
            assert len(choices) == 4
            assert in_progress_ctx.greeter_sas in choices

            greeter_sas = await oob_recv.receive()
            assert greeter_sas == in_progress_ctx.greeter_sas

            in_progress_ctx = await in_progress_ctx.do_signify_trust()
            await oob_send.send(in_progress_ctx.claimer_sas)

            in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()

            nonlocal new_device
            new_device = await in_progress_ctx.do_claim_user(
                requested_device_label=requested_device_label,
                requested_human_handle=requested_human_handle,
            )
            assert isinstance(new_device, LocalDevice)
            # User storage should be populated with non-speculative user manifest
            # before save the device
            await user_storage_non_speculative_init(data_base_dir=data_base_dir, device=new_device)

    async def _run_greeter():
        initial_ctx = UserGreetInitialCtx(
            cmds=alice_backend_cmds, token=zack_new_user_invitation.token
        )

        in_progress_ctx = await initial_ctx.do_wait_peer()

        await oob_send.send(in_progress_ctx.greeter_sas)

        in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()

        choices = in_progress_ctx.generate_claimer_sas_choices(size=5)
        assert len(choices) == 5
        assert in_progress_ctx.claimer_sas in choices

        claimer_sas = await oob_recv.receive()
        assert claimer_sas == in_progress_ctx.claimer_sas

        in_progress_ctx = await in_progress_ctx.do_signify_trust()

        in_progress_ctx = await in_progress_ctx.do_get_claim_requests()

        assert in_progress_ctx.requested_device_label == requested_device_label
        assert in_progress_ctx.requested_human_handle == requested_human_handle

        await in_progress_ctx.do_create_new_user(
            author=alice,
            device_label=granted_device_label,
            human_handle=granted_human_handle,
            profile=granted_profile,
        )

    async with real_clock_timeout():
        async with trio.open_nursery() as nursery:
            nursery.start_soon(_run_claimer)
            nursery.start_soon(_run_greeter)

    assert new_device is not None
    assert new_device.device_id != alice.device_id
    assert new_device.device_label == granted_device_label
    # Label is normally ignored when comparing HumanLabel
    if with_labels:
        assert new_device.human_handle.label == granted_human_handle.label
        assert new_device.human_handle.email == granted_human_handle.email
    else:
        assert new_device.human_handle is None
    assert new_device.profile == granted_profile
    # Extra check to make sure claimer&greeter data are not mixed
    assert new_device.user_manifest_id != alice.user_manifest_id
    assert new_device.user_manifest_key != alice.user_manifest_key
    assert new_device.local_symkey != alice.local_symkey

    # Now invitation should have been deleted
    rep = await alice_backend_cmds.invite_list()
    assert rep == InviteListRepOk([])

    # Verify user&device data in backend
    user, device = await backend.user.get_user_with_device(
        new_device.organization_id, new_device.device_id
    )
    assert user.profile == granted_profile
    assert user.human_handle == granted_human_handle
    assert device.device_label == granted_device_label
    if with_labels:
        assert user.user_certificate != user.redacted_user_certificate
        assert device.device_certificate != device.redacted_device_certificate
    else:
        assert user.user_certificate == user.redacted_user_certificate
        assert device.device_certificate == device.redacted_device_certificate

    # Test the behavior of this new user device
    async with user_fs_factory(alice) as alice_fs:
        async with user_fs_factory(new_device) as new_fs:
            # New user should start with a non-speculative user manifest
            um = new_fs.get_user_manifest()
            assert um.is_placeholder
            assert not um.speculative

            # Share a workspace with new user
            aw_id = await alice_fs.workspace_create(EntryName("alice_workspace"))
            await alice_fs.workspace_share(aw_id, new_device.user_id, WorkspaceRole.CONTRIBUTOR)

            # New user cannot create a new workspace
            zw_id = await new_fs.workspace_create(EntryName("zack_workspace"))
            await new_fs.workspace_share(zw_id, alice.user_id, WorkspaceRole.READER)

            # Now both users should have the same workspaces
            await alice_fs.process_last_messages()
            await new_fs.process_last_messages()
            await new_fs.sync()  # Not required, but just to make sure it works

            alice_um = alice_fs.get_user_manifest()
            zack_um = new_fs.get_user_manifest()

            assert {(w.id, w.key.secret) for w in alice_um.workspaces} == {
                (w.id, w.key.secret) for w in zack_um.workspaces
            }


@pytest.mark.trio
async def test_claimer_handle_reset(
    backend, running_backend, alice, alice_backend_cmds, alice_new_device_invitation
):
    async with backend_invited_cmds_factory(addr=alice_new_device_invitation) as claimer_cmds:
        claimer_in_progress_ctx = None
        greeter_in_progress_ctx = None

        # Step 1
        async with real_clock_timeout():
            async with trio.open_nursery() as nursery:

                async def _do_claimer():
                    nonlocal claimer_in_progress_ctx
                    claimer_in_progress_ctx = await claimer_initial_ctx.do_wait_peer()

                async def _do_greeter():
                    nonlocal greeter_in_progress_ctx
                    greeter_in_progress_ctx = await greeter_initial_ctx.do_wait_peer()

                greeter_initial_ctx = UserGreetInitialCtx(
                    cmds=alice_backend_cmds, token=alice_new_device_invitation.token
                )
                claimer_initial_ctx = await claimer_retrieve_info(claimer_cmds)

                nursery.start_soon(_do_claimer)
                nursery.start_soon(_do_greeter)

        # Claimer restart the conduit while greeter try to do step 2
        async with real_clock_timeout():
            async with trio.open_nursery() as nursery:

                async def _do_claimer():
                    nonlocal claimer_in_progress_ctx
                    claimer_in_progress_ctx = await claimer_initial_ctx.do_wait_peer()

                # Reinitialize claimer_initial_ctx because it has been consumed
                claimer_initial_ctx = await claimer_retrieve_info(claimer_cmds)

                nursery.start_soon(_do_claimer)
                with pytest.raises(InvitePeerResetError):
                    await greeter_in_progress_ctx.do_wait_peer_trust()

                # Reinitialize greeter_initial_ctx because it has been consumed
                greeter_initial_ctx = UserGreetInitialCtx(
                    cmds=alice_backend_cmds, token=alice_new_device_invitation.token
                )

                # Greeter redo step 1
                greeter_in_progress_ctx = await greeter_initial_ctx.do_wait_peer()

        # Now do the other way around: greeter restart conduit while claimer try step 2
        async with real_clock_timeout():
            async with trio.open_nursery() as nursery:

                async def _do_greeter():
                    nonlocal greeter_in_progress_ctx
                    greeter_in_progress_ctx = await greeter_initial_ctx.do_wait_peer()

                # Reinitialize greeter_initial_ctx because it has been consumed
                greeter_initial_ctx = UserGreetInitialCtx(
                    cmds=alice_backend_cmds, token=alice_new_device_invitation.token
                )

                nursery.start_soon(_do_greeter)
                with pytest.raises(InvitePeerResetError):
                    await claimer_in_progress_ctx.do_signify_trust()

                # Reinitialize claimer_initial_ctx because it has been consumed
                claimer_initial_ctx = await claimer_retrieve_info(claimer_cmds)

                # Claimer redo step 1
                claimer_in_progress_ctx = await claimer_initial_ctx.do_wait_peer()


@contextmanager
def patch_rs_invitation_already_used_error_usage():
    # For sad legacy reasons, we use both `BackendInvitationAlreadyUsed` and `InviteAlreadyUsedError`
    # in the Python codebase (typically `BackendInvitationAlreadyUsed` is raised during
    # server connection handshake and bubbles up, while `InviteAlreadyUsedError` is raised
    # if the server answered a specific status to our command)
    # Long story short we should in theory fix this by catching `BackendInvitationAlreadyUsed`
    # (all `BackendConnectionError` in fact) and wrap them with invite-specific exception.
    # However this is cumbersome to do, so we are changing this in the new Rust codebase.
    # Hence this patch function that on the fly catch the new but correct use of
    # `InviteAlreadyUsedError` to convert it into the legacy `BackendInvitationAlreadyUsed`
    if FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
        try:
            yield
        except InviteAlreadyUsedError as exc:
            assert str(exc) == "Invitation already used"
            raise BackendInvitationAlreadyUsed(
                "Invalid handshake: Invitation already deleted"
            ) from exc
    else:
        yield


@pytest.mark.trio
@pytest.mark.parametrize(
    "fail_on_step", ["wait_peer", "signify_trust", "wait_peer_trust", "claim_device"]
)
async def test_claimer_handle_cancel_event(
    backend, running_backend, alice, alice_backend_cmds, fail_on_step, alice_new_device_invitation
):
    async def _cancel_invitation():
        await backend.invite.delete(
            organization_id=alice.organization_id,
            greeter=alice.user_id,
            token=alice_new_device_invitation.token,
            on=DateTime.now(),
            reason=InvitationDeletedReason.CANCELLED,
        )

    async with backend_invited_cmds_factory(addr=alice_new_device_invitation) as claimer_cmds:
        greeter_initial_ctx = UserGreetInitialCtx(
            cmds=alice_backend_cmds, token=alice_new_device_invitation.token
        )
        claimer_initial_ctx = await claimer_retrieve_info(claimer_cmds)

        claimer_in_progress_ctx = None
        greeter_in_progress_ctx = None

        async def _do_claimer():
            nonlocal claimer_in_progress_ctx
            if fail_on_step == "wait_peer":
                return
            claimer_in_progress_ctx = await claimer_initial_ctx.do_wait_peer()
            if fail_on_step == "signify_trust":
                return
            claimer_in_progress_ctx = await claimer_in_progress_ctx.do_signify_trust()
            if fail_on_step == "wait_peer_trust":
                return
            claimer_in_progress_ctx = await claimer_in_progress_ctx.do_wait_peer_trust()

        async def _do_greeter():
            nonlocal greeter_in_progress_ctx
            if fail_on_step == "wait_peer":
                return
            greeter_in_progress_ctx = await greeter_initial_ctx.do_wait_peer()
            if fail_on_step == "signify_trust":
                return
            greeter_in_progress_ctx = await greeter_in_progress_ctx.do_wait_peer_trust()
            if fail_on_step == "wait_peer_trust":
                return
            greeter_in_progress_ctx = await greeter_in_progress_ctx.do_signify_trust()

        async with real_clock_timeout():
            async with trio.open_nursery() as nursery:
                nursery.start_soon(_do_claimer)
                nursery.start_soon(_do_greeter)

        async with real_clock_timeout():
            async with trio.open_nursery() as nursery:

                async def _do_claimer_wait_peer():
                    with pytest.raises(BackendInvitationAlreadyUsed) as exc_info:
                        with patch_rs_invitation_already_used_error_usage():
                            await claimer_initial_ctx.do_wait_peer()
                    assert str(exc_info.value) == "Invalid handshake: Invitation already deleted"

                async def _do_claimer_signify_trust():
                    with pytest.raises(BackendInvitationAlreadyUsed) as exc_info:
                        with patch_rs_invitation_already_used_error_usage():
                            await claimer_in_progress_ctx.do_signify_trust()
                    assert str(exc_info.value) == "Invalid handshake: Invitation already deleted"

                async def _do_claimer_wait_peer_trust():
                    with pytest.raises(BackendInvitationAlreadyUsed) as exc_info:
                        with patch_rs_invitation_already_used_error_usage():
                            await claimer_in_progress_ctx.do_wait_peer_trust()
                    assert str(exc_info.value) == "Invalid handshake: Invitation already deleted"

                async def _do_claimer_claim_device():
                    with pytest.raises(BackendInvitationAlreadyUsed) as exc_info:
                        with patch_rs_invitation_already_used_error_usage():
                            await claimer_in_progress_ctx.do_claim_device(
                                requested_device_label=DeviceLabel("TheSecretDevice")
                            )
                    assert str(exc_info.value) == "Invalid handshake: Invitation already deleted"

                steps = {
                    "wait_peer": _do_claimer_wait_peer,
                    "signify_trust": _do_claimer_signify_trust,
                    "wait_peer_trust": _do_claimer_wait_peer_trust,
                    "claim_device": _do_claimer_claim_device,
                }
                _do_claimer = steps[fail_on_step]

                with backend.event_bus.listen() as spy:
                    nursery.start_soon(_do_claimer)
                    # Be sure that _do_claimer got valid invitations before cancelation
                    await spy.wait_with_timeout(BackendEvent.INVITE_CONDUIT_UPDATED)
                    await _cancel_invitation()
                    await spy.wait_with_timeout(BackendEvent.INVITE_STATUS_CHANGED)


@pytest.mark.trio
@pytest.mark.parametrize(
    "fail_on_step", ["wait_peer", "signify_trust", "wait_peer_trust", "claim_device"]
)
async def test_claimer_handle_command_failure(
    backend,
    running_backend,
    alice,
    alice_backend_cmds,
    monkeypatch,
    fail_on_step,
    alice_new_device_invitation,
):
    async def _cancel_invitation():
        await backend.invite.delete(
            organization_id=alice.organization_id,
            greeter=alice.user_id,
            token=alice_new_device_invitation.token,
            on=DateTime.now(),
            reason=InvitationDeletedReason.CANCELLED,
        )

    async with backend_invited_cmds_factory(addr=alice_new_device_invitation) as claimer_cmds:
        greeter_initial_ctx = UserGreetInitialCtx(
            cmds=alice_backend_cmds, token=alice_new_device_invitation.token
        )
        claimer_initial_ctx = await claimer_retrieve_info(claimer_cmds)

        claimer_in_progress_ctx = None
        greeter_in_progress_ctx = None

        async def _do_claimer():
            nonlocal claimer_in_progress_ctx
            if fail_on_step == "wait_peer":
                return
            claimer_in_progress_ctx = await claimer_initial_ctx.do_wait_peer()
            if fail_on_step == "signify_trust":
                return
            claimer_in_progress_ctx = await claimer_in_progress_ctx.do_signify_trust()
            if fail_on_step == "wait_peer_trust":
                return
            claimer_in_progress_ctx = await claimer_in_progress_ctx.do_wait_peer_trust()

        async def _do_greeter():
            nonlocal greeter_in_progress_ctx
            if fail_on_step == "wait_peer":
                return
            greeter_in_progress_ctx = await greeter_initial_ctx.do_wait_peer()
            if fail_on_step == "signify_trust":
                return
            greeter_in_progress_ctx = await greeter_in_progress_ctx.do_wait_peer_trust()
            if fail_on_step == "wait_peer_trust":
                return
            greeter_in_progress_ctx = await greeter_in_progress_ctx.do_signify_trust()

        async with real_clock_timeout():
            async with trio.open_nursery() as nursery:
                nursery.start_soon(_do_claimer)
                nursery.start_soon(_do_greeter)

        deleted_event = trio.Event()

        async def _send_event(*args, **kwargs):
            if BackendEvent.INVITE_STATUS_CHANGED in args and (
                kwargs.get("status") == InvitationStatus.DELETED
            ):
                deleted_event.set()
            await trio.sleep(0)

        backend.invite._send_event = _send_event
        monkeypatch.setattr("parsec.backend.postgresql.invite.send_signal", _send_event)

        async with real_clock_timeout():
            await _cancel_invitation()
            await deleted_event.wait()
            with pytest.raises((BackendInvitationAlreadyUsed, InviteAlreadyUsedError)) as exc_info:
                if fail_on_step == "wait_peer":
                    await claimer_initial_ctx.do_wait_peer()
                elif fail_on_step == "signify_trust":
                    await claimer_in_progress_ctx.do_signify_trust()
                elif fail_on_step == "wait_peer_trust":
                    await claimer_in_progress_ctx.do_wait_peer_trust()
                elif fail_on_step == "claim_device":
                    await claimer_in_progress_ctx.do_claim_device(
                        requested_device_label=DeviceLabel("TheSecretDevice")
                    )
                else:
                    raise AssertionError(f"Unknown step {fail_on_step}")
            if FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
                assert str(exc_info.value) == "Invitation already used"
            else:  # Legacy, Backend Error found instead of Invite Error
                assert str(exc_info.value) == "Invalid handshake: Invitation already deleted"


@pytest.mark.trio
async def test_user_claim_but_active_users_limit_reached(backend, running_backend, alice):
    # Organization has reached active user limit
    await backend.organization.update(
        alice.organization_id, active_users_limit=ActiveUsersLimit.LimitedTo(1)
    )

    # Invitation is still ok...
    invitation = await backend.invite.new_for_user(
        organization_id=alice.organization_id,
        greeter_user_id=alice.user_id,
        claimer_email="zack@example.com",
    )
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=alice.organization_addr.get_backend_addr(),
        organization_id=alice.organization_id,
        invitation_type=InvitationType.USER,
        token=invitation.token,
    )

    async def _run_greeter():
        async with backend_authenticated_cmds_factory(
            alice.organization_addr, alice.device_id, alice.signing_key
        ) as alice_backend_cmds:
            initial_ctx = UserGreetInitialCtx(cmds=alice_backend_cmds, token=invitation_addr.token)
            in_progress_ctx = await initial_ctx.do_wait_peer()
            in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()
            in_progress_ctx = await in_progress_ctx.do_signify_trust()
            in_progress_ctx = await in_progress_ctx.do_get_claim_requests()

            # ...this is where the limit should be enforced
            with pytest.raises(InviteActiveUsersLimitReachedError):
                await in_progress_ctx.do_create_new_user(
                    author=alice,
                    device_label=in_progress_ctx.requested_device_label,
                    human_handle=in_progress_ctx.requested_human_handle,
                    profile=UserProfile.STANDARD,
                )

    async def _run_claimer():
        async with backend_invited_cmds_factory(addr=invitation_addr) as cmds:
            initial_ctx = await claimer_retrieve_info(cmds)
            in_progress_ctx = await initial_ctx.do_wait_peer()
            in_progress_ctx = await in_progress_ctx.do_signify_trust()
            in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()

            await in_progress_ctx.do_claim_user(
                requested_device_label=None, requested_human_handle=None
            )

    async with real_clock_timeout():
        async with trio.open_nursery() as nursery:
            nursery.start_soon(_run_claimer)
            await _run_greeter()
            # Claimer is not notified that the greeter has failed so we
            # must explicitly cancel it
            nursery.cancel_scope.cancel()
