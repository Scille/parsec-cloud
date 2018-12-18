import trio
import attr
import pendulum
from typing import Tuple, Optional
from async_generator import asynccontextmanager

from parsec.types import DeviceID, UserID
from parsec.event_bus import EventBus
from parsec.crypto import PublicKey, VerifyKey
from parsec.trustchain import certify_user, certify_device
from parsec.core.invite_claim import extract_user_encrypted_claim
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.backend_connection import (
    backend_cmds_pool_factory,
    backend_listen_events,
    backend_monitor_connection,
)
from parsec.core.encryption_manager import EncryptionManager
from parsec.core.mountpoint import mountpoint_manager_factory
from parsec.core.beacons_monitor import monitor_beacons
from parsec.core.messages_monitor import monitor_messages
from parsec.core.sync_monitor import SyncMonitor
from parsec.core.fs import FS
from parsec.core.local_db import LocalDB


@attr.s(frozen=True, slots=True)
class LoggedCore:
    config = attr.ib()
    device = attr.ib()
    local_db = attr.ib()
    event_bus = attr.ib()
    encryption_manager = attr.ib()
    mountpoint_manager = attr.ib()
    backend_cmds = attr.ib()
    fs = attr.ib()

    # Helper functions

    async def user_invite(self, user_id: UserID) -> Tuple[DeviceID, PublicKey, VerifyKey]:
        """
        Raises:
            core.backend_connection.BackendConnectionError
            core.trustchain.TrustChainError
        """
        encrypted_claim = await self.backend_cmds.user_invite(user_id)

        claim = extract_user_encrypted_claim(self.device.private_key, encrypted_claim)
        return claim["device_id"], claim["public_key"], claim["verify_key"]

    async def user_create(
        self, device_id: UserID, public_key: PublicKey, verify_key: VerifyKey
    ) -> None:
        """
        Raises:
            core.backend_connection.BackendConnectionError
            core.trustchain.TrustChainError
        """
        now = pendulum.now()
        certified_user = certify_user(
            self.device.device_id, self.device.signing_key, device_id.user_id, public_key, now=now
        )
        certified_device = certify_device(
            self.device.device_id, self.device.signing_key, device_id, verify_key, now=now
        )
        await self.backend_cmds.user_create(certified_user, certified_device)

    async def user_claim(backend_addr: str, device_id: DeviceID, token: str):
        public_key = PublicKey.generate()
        signing_key = SigningKey.generate()

        async with backend_anonymous_cmds_factory(backend_addr) as cmds:
            invitation_creator = await cmds.user_get_invitation_creator(device_id.user_id)

            encrypted_claim = generate_user_encrypted_claim(
                invitation_creator.public_key, token, device_id, public_key, verify_key
            )
            await cmds.user_claim(device_id.user_id, encrypted_claim)

            if pkcs11:
                save_device_with_password(config_dir, device_id, token_id, key_id, pin)
            else:
                save_device_with_password(config_dir, device_id, password)

        async with backend_anonymous_cmds_factory(backend_addr) as cmds:
            invitation_creator = await cmds.user_get_invitation_creator(mallory.user_id)
            assert isinstance(invitation_creator, RemoteUser)

            encrypted_claim = generate_user_encrypted_claim(
                invitation_creator.public_key,
                mallory.device_id,
                mallory.public_key,
                mallory.verify_key,
            )
            await cmds.user_claim(mallory.user_id, encrypted_claim)


@asynccontextmanager
async def logged_core_factory(
    config: CoreConfig, device: LocalDevice, event_bus: Optional[EventBus] = None
):
    event_bus = event_bus or EventBus()

    # Plenty of nested scope to order components init/teardown
    async with trio.open_nursery() as root_nursery:

        # Monitor connection must be first given it will watch on
        # other monitors' events
        await root_nursery.start(backend_monitor_connection, event_bus)

        async with trio.open_nursery() as backend_conn_nursery:
            await backend_conn_nursery.start(backend_listen_events, device, event_bus)

            async with backend_cmds_pool_factory(
                device.backend_addr,
                device.device_id,
                device.signing_key,
                config.backend_max_connections,
            ) as backend_cmds_pool:

                local_db = LocalDB(config.data_dir, device)
                encryption_manager = EncryptionManager(device, local_db, backend_cmds_pool)

                async with trio.open_nursery() as monitor_nursery:

                    fs = FS(device, local_db, backend_cmds_pool, encryption_manager, event_bus)

                    # Finally start monitoring coroutines
                    await monitor_nursery.start(monitor_beacons, device, fs, event_bus)
                    await monitor_nursery.start(monitor_messages, fs, event_bus)
                    # TODO: replace SyncMonitor by a function
                    sync_monitor = SyncMonitor(fs, event_bus)
                    await monitor_nursery.start(sync_monitor.run)

                    mountpoint_manager = mountpoint_manager_factory(fs, event_bus)
                    # TODO: rework mountpoint manager to avoid init/teardown
                    await mountpoint_manager.init(monitor_nursery)

                    try:
                        yield LoggedCore(
                            config=config,
                            device=device,
                            local_db=local_db,
                            event_bus=event_bus,
                            encryption_manager=encryption_manager,
                            mountpoint_manager=mountpoint_manager,
                            backend_cmds=backend_cmds_pool,
                            fs=fs,
                        )
                        root_nursery.cancel_scope.cancel()

                    finally:
                        await mountpoint_manager.teardown()
