# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pendulum
import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import initialize, rule

from parsec.api.protocol import RealmRole
from parsec.core.fs.exceptions import FSReadOnlyError, FSWorkspaceNoWriteAccess

from tests.common import freeze_time


@pytest.mark.slow
def test_timestamp_causality(user_fs_online_state_machine, alice, bob, carl, diana):
    small_offset = st.integers(min_value=0, max_value=5)

    class TimestampCausality(user_fs_online_state_machine):
        """
        Let's make sure no corruption occurs in the following scenario, despite running with shifted clocks:

        - Bob tries to upload new manifests to a workspace
        - Alice downloads and checks Bob's manifests
        - Carl grants Bob's write rights
        - Diana revokes Bob's write rights
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
            for name in ["alice", "bob", "carl", "diana"]:
                controller_name = f"{name}_controller"
                try:
                    controller = getattr(self, controller_name)
                except AttributeError:
                    pass
                else:
                    controller.stop()
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

            # Start all
            await self.start_backend()
            self.alice_controller = await self.start_user_fs(alice)
            self.bob_controller = await self.start_user_fs(bob)
            self.carl_controller = await self.start_user_fs(carl)
            self.diana_controller = await self.start_user_fs(diana)

            # Carl
            self.wid = await self.carl_fs.workspace_create("w")
            await self.carl_fs.workspace_share(self.wid, alice.user_id, RealmRole.READER)
            await self.carl_fs.workspace_share(self.wid, bob.user_id, RealmRole.CONTRIBUTOR)
            await self.carl_fs.workspace_share(self.wid, diana.user_id, RealmRole.OWNER)
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

            # Set the time shifts in seconds
            self.alice_offset = alice_offset
            self.bob_offset = bob_offset
            self.carl_offset = carl_offset
            self.diana_offset = diana_offset

        @rule()
        async def bob_updates_the_file(self):
            with freeze_time(pendulum.now().add(seconds=self.bob_offset)):
                try:
                    async with await self.bob_workspace.open_file("/file.txt", "ab") as file_txt:
                        await file_txt.write(b"a")
                        length = file_txt.tell()
                except FSReadOnlyError:
                    return
                try:
                    await self.bob_workspace.sync()
                except FSWorkspaceNoWriteAccess:
                    return
                self.current_length = length

        @rule()
        async def alice_clears_her_cache_certificate(self):
            self.alice_fs.remote_loader.clear_realm_role_certificate_cache()

        @rule()
        async def alice_reads_the_file(self):
            with freeze_time(pendulum.now().add(seconds=self.alice_offset)):
                await self.alice_workspace.sync()
                assert (
                    await self.alice_workspace.read_bytes("/file.txt") == b"a" * self.current_length
                )

        @rule()
        async def carl_grants_bob_write_rights(self):
            with freeze_time(pendulum.now().add(seconds=self.carl_offset)):
                await self.carl_fs.workspace_share(self.wid, bob.user_id, RealmRole.CONTRIBUTOR)
                await self.carl_fs.sync()

        @rule()
        async def bob_synchronizes(self):
            with freeze_time(pendulum.now().add(seconds=self.bob_offset)):
                await self.bob_fs.process_last_messages()
                await self.bob_fs.sync()

        @rule()
        async def diana_revokes_bob_write_rights(self):
            with freeze_time(pendulum.now().add(seconds=self.diana_offset)):
                await self.diana_fs.workspace_share(self.wid, bob.user_id, RealmRole.READER)
                await self.diana_fs.sync()

        @rule()
        async def a_second_goes_by(self):
            self.alice_offset += 1
            self.bob_offset += 1
            self.carl_offset += 1
            self.diana_offset += 1

    TimestampCausality.run_as_test()
