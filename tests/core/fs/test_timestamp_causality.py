# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import initialize, rule
from pendulum import now

from parsec import UNSTABLE_OXIDATION
from parsec.api.data import EntryName
from parsec.api.protocol import RealmRole, UserProfile
from parsec.core.fs.exceptions import FSReadOnlyError, FSWorkspaceNoWriteAccess


@pytest.fixture
def shift_now(monkeypatch):
    testing = True
    current_delay_us = 0

    def get_real_now():
        nonlocal testing
        try:
            testing = False
            return now()
        finally:
            testing = True

    def _shift_now(delay):
        nonlocal current_delay_us
        current_delay_us += int(delay * 1_000_000)

    monkeypatch.setattr("pendulum.has_test_now", lambda: testing)
    monkeypatch.setattr(
        "pendulum.get_test_now", lambda: get_real_now().add(microseconds=current_delay_us)
    )

    return _shift_now


@pytest.fixture
def set_device_time_offset(monkeypatch):
    per_device_time_offsets_us = {}

    def _set_device_time_offset(device, offset):
        per_device_time_offsets_us[device.device_id] = int(offset * 1_000_000)

    monkeypatch.setattr(
        "parsec.core.types.LocalDevice.timestamp",
        # We're using `now()` plus an offset to control each device time.
        # Using `now()` allows for a realistic progress of time, only shifted for each device.
        # Then the `shift_now` fixture can be used to flash-forward.
        lambda local_device: now().add(
            microseconds=per_device_time_offsets_us[local_device.device_id]
        ),
    )

    return _set_device_time_offset


@pytest.mark.slow
@pytest.mark.skipif(UNSTABLE_OXIDATION, reason="No persistent_mockup")
def test_timestamp_causality(
    user_fs_online_state_machine,
    coolorg,
    local_device_factory,
    backend_data_binder_factory,
    set_device_time_offset,
    shift_now,
):
    small_offset = st.integers(min_value=0, max_value=5)
    alice = local_device_factory("alice-causality@dev1", profile=UserProfile.ADMIN)
    bob = local_device_factory("bob-causality@dev1", profile=UserProfile.STANDARD)
    carl = local_device_factory("carl-causality@dev1", profile=UserProfile.STANDARD)
    diana = local_device_factory("diana-causality@dev1", profile=UserProfile.STANDARD)

    class TimestampCausality(user_fs_online_state_machine):
        """
        Let's make sure no corruption occurs while the following operations are performed repeatedely
        and concurrently, despite the devices running with shifted clocks:

        1. Bob tries to upload new manifests to a workspace (if it has write rights)
        2. Alice downloads and checks Bob's manifests (alice always have read rights)
        3. Carl grants Bob's write rights (carl always have owner rights)
        4. Diana revokes Bob's write rights (diana always have owner rights)

        Note that the operations 1, 3 and 4 are going to produce timestamps that will affect the checking
        in operation 2, which means operation 2 is the one we expect to fail.
        """

        @property
        def alice_fs(self):
            return self.alice_controller.user_fs

        @property
        def bob_fs(self):
            return self.bob_controller.user_fs

        @property
        def carl_fs(self):
            return self.carl_controller.user_fs

        @property
        def diana_fs(self):
            return self.diana_controller.user_fs

        async def reset_all(self):
            for device in (alice, bob, carl, diana):
                set_device_time_offset(device, 0.0)
            for name in ["alice", "bob", "carl", "diana"]:
                controller_name = f"{name}_controller"
                try:
                    controller = getattr(self, controller_name)
                except AttributeError:
                    pass
                else:
                    await controller.stop()
                    delattr(self, controller_name)
            await super().reset_all()

        @initialize(
            alice_offset=small_offset,
            bob_offset=small_offset,
            carl_offset=small_offset,
            diana_offset=small_offset,
        )
        async def init(self, alice_offset, bob_offset, carl_offset, diana_offset):
            # Reset all
            await self.reset_all()

            # Start backend
            await self.start_backend(populated=False)

            self.alice = self.backend_controller.server.correct_addr(alice)
            self.bob = self.backend_controller.server.correct_addr(bob)
            self.carl = self.backend_controller.server.correct_addr(carl)
            self.diana = self.backend_controller.server.correct_addr(diana)

            # Carl and diana are not bound to the default organization
            binder = backend_data_binder_factory(self.backend)
            await binder.bind_organization(coolorg, self.alice)
            for local_device in (self.bob, self.carl, self.diana):
                await binder.bind_device(local_device)

            # Start user FS
            self.alice_controller = await self.start_user_fs(self.alice)
            self.bob_controller = await self.start_user_fs(self.bob)
            self.carl_controller = await self.start_user_fs(self.carl)
            self.diana_controller = await self.start_user_fs(self.diana)

            # Carl
            self.wid = await self.carl_fs.workspace_create(EntryName("w"))
            await self.carl_fs.workspace_share(self.wid, self.alice.user_id, RealmRole.READER)
            await self.carl_fs.workspace_share(self.wid, self.bob.user_id, RealmRole.CONTRIBUTOR)
            await self.carl_fs.workspace_share(self.wid, self.diana.user_id, RealmRole.OWNER)
            await self.carl_fs.sync()

            # Diana
            await self.diana_fs.process_last_messages()
            await self.diana_fs.sync()

            # Bob
            await self.bob_fs.process_last_messages()
            await self.bob_fs.sync()
            self.bob_workspace = self.bob_fs.get_workspace(self.wid)
            await self.bob_workspace.touch("/file.txt")
            await self.bob_workspace.sync()

            # Alice
            await self.alice_fs.process_last_messages()
            await self.alice_fs.sync()
            self.alice_workspace = self.alice_fs.get_workspace(self.wid)
            await self.alice_workspace.sync()
            assert await self.alice_workspace.read_bytes("/file.txt") == b""
            self.current_length = 0

            # Clear Alice's cache since the offsets have not been applied yet
            self.alice_fs.remote_loader.clear_realm_role_certificate_cache()

            # Set device time offsets
            set_device_time_offset(self.alice, alice_offset)
            set_device_time_offset(self.bob, bob_offset)
            set_device_time_offset(self.carl, carl_offset)
            set_device_time_offset(self.diana, diana_offset)

        @rule()
        async def bob_updates_the_file(self):
            # Write a single character to the file
            try:
                async with await self.bob_workspace.open_file("/file.txt", "ab") as file_txt:
                    await file_txt.write(b"a")
                    length = file_txt.tell()
            # Maybe Diana revoked our write rights and we know about it
            except FSReadOnlyError:
                return
            # Synchronize the workspace
            try:
                await self.bob_workspace.sync()
            # Maybe Diana revoked our write and we didn't know about it
            except FSWorkspaceNoWriteAccess:
                return
            # Upload is successful, update the expected file length
            self.current_length = length

        @rule()
        async def alice_clears_her_cache_certificate(self):
            self.alice_fs.remote_loader.clear_realm_role_certificate_cache()

        @rule()
        async def alice_reads_the_file(self):
            """This is the rule that is the most likely to fail in the scenarions we want to test.
            This is because the timestamps produced by other devices in the other rules will affect
            the checking performed here when alice checks the validity of the manifest they've just
            downloaded.
            """
            await self.alice_workspace.sync()
            assert await self.alice_workspace.read_bytes("/file.txt") == b"a" * self.current_length

        @rule()
        async def carl_grants_bob_write_rights(self):
            await self.carl_fs.workspace_share(self.wid, bob.user_id, RealmRole.CONTRIBUTOR)
            await self.carl_fs.sync()

        @rule()
        async def bob_synchronizes(self):
            await self.bob_fs.process_last_messages()
            await self.bob_fs.sync()

        @rule()
        async def diana_revokes_bob_write_rights(self):
            await self.diana_fs.workspace_share(self.wid, bob.user_id, RealmRole.READER)
            await self.diana_fs.sync()

        @rule(seconds=st.integers(min_value=1, max_value=60))
        async def a_few_seconds_goes_by(self, seconds):
            shift_now(seconds)

        async def teardown(self):
            await self.reset_all()

    TimestampCausality.run_as_test()
