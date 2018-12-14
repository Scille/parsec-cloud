import pendulum
from typing import Tuple, Dict

from parsec.types import UserID, DeviceID
from parsec.crypto import VerifyKey
from parsec.event_bus import EventBus
from parsec.backend.user import (
    BaseUserComponent,
    User,
    Device,
    DevicesMapping,
    UserInvitation,
    DeviceInvitation,
    UserAlreadyExistsError,
    UserAlreadyRevokedError,
    UserNotFoundError,
)


class MemoryUserComponent(BaseUserComponent):
    def __init__(self, root_verify_key: VerifyKey, event_bus: EventBus):
        super().__init__(root_verify_key, event_bus)
        self._users = {}
        self._invitations = {}
        self._device_configuration_tries = {}
        self._unconfigured_devices = {}

    async def create_user(self, user: User) -> None:
        if user.user_id in self._users:
            raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

        self._users[user.user_id] = user

    async def create_device(self, device: Device) -> None:
        if device.user_id not in self._users:
            raise UserNotFoundError(f"User `{device.user_id}` doesn't exists")

        user = self._users[device.user_id]
        if device.device_name in user.devices:
            raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

        self._users[device.user_id] = user.evolve(
            devices=DevicesMapping(*user.devices.values(), device)
        )

    async def _get_trustchain(self, *devices_ids):
        trustchain = {}

        async def _recursive_extract_creators(device_id):
            if not device_id or device_id in trustchain:
                return
            device = await self.get_device(device_id)
            trustchain[device_id] = device
            await _recursive_extract_creators(device.device_certifier)
            await _recursive_extract_creators(device.revocation_certifier)

        for device_id in devices_ids:
            await _recursive_extract_creators(device_id)
        return trustchain

    async def get_user(self, user_id: UserID) -> User:
        try:
            return self._users[user_id]

        except KeyError:
            raise UserNotFoundError(user_id)

    async def get_user_with_trustchain(
        self, user_id: UserID
    ) -> Tuple[User, Dict[DeviceID, Device]]:
        user = await self.get_user(user_id)
        trustchain = await self._get_trustchain(
            user.user_certifier,
            *[device.device_certifier for device in user.devices.values()],
            *[device.revocation_certifier for device in user.devices.values()],
        )
        return user, trustchain

    async def get_device(self, device_id: DeviceID) -> Device:
        user = await self.get_user(device_id.user_id)
        try:
            return user.devices[device_id.device_name]

        except KeyError:
            raise UserNotFoundError(device_id)

    async def get_device_with_trustchain(
        self, device_id: DeviceID
    ) -> Tuple[Device, Dict[DeviceID, Device]]:
        device = await self.get_device(device_id)
        trustchain = await self._get_trustchain(
            device.device_certifier, device.revocation_certifier
        )
        return device, trustchain

    async def find(self, query: str = None, page: int = 0, per_page: int = 100):
        if query:
            results = [user_id for user_id in self._users.keys() if user_id.startswith(query)]
        else:
            results = list(self._users.keys())
        # PostgreSQL does case insensitive sort
        sorted_results = sorted(results, key=lambda s: s.lower())
        return sorted_results[(page - 1) * per_page : page * per_page], len(results)

    async def create_user_invitation(self, invitation: UserInvitation) -> None:
        if invitation.user_id in self._users:
            raise UserAlreadyExistsError(f"User `{invitation.user_id}` already exists")
        self._invitations[invitation.user_id] = invitation

    async def get_user_invitation(self, user_id: UserID) -> UserInvitation:
        if user_id in self._users:
            raise UserNotFoundError(user_id)
        try:
            return self._invitations[user_id]
        except KeyError:
            raise UserNotFoundError(user_id)

    async def user_cancel_invitation(self, user_id: UserID) -> None:
        self._invitations.pop(user_id, None)

    async def create_device_invitation(self, invitation: DeviceInvitation) -> None:
        user = await self.get_user(invitation.device_id.user_id)
        if invitation.device_id.device_name in user.devices:
            raise UserAlreadyExistsError(f"Device `{invitation.device_id}` already exists")
        self._invitations[invitation.device_id] = invitation

    async def get_device_invitation(self, device_id: UserID) -> DeviceInvitation:
        try:
            self._users[device_id.user_id].devices[device_id.device_name]
            raise UserNotFoundError(device_id)
        except KeyError:
            pass
        try:
            return self._invitations[device_id]
        except KeyError:
            raise UserNotFoundError(device_id)

    async def device_cancel_invitation(self, device_id: DeviceID) -> None:
        self._invitations.pop(device_id, None)

    async def revoke_device(
        self, device_id: DeviceID, certified_revocation: bytes, revocation_certifier: DeviceID
    ) -> None:
        user = await self.get_user(device_id.user_id)
        try:
            if user.devices[device_id.device_name].revocated_on:
                raise UserAlreadyRevokedError()

        except KeyError:
            raise UserNotFoundError(device_id)

        patched_devices = []
        for device in user.devices.values():
            if device.device_id == device_id:
                device = device.evolve(
                    revocated_on=pendulum.now(),
                    certified_revocation=certified_revocation,
                    revocation_certifier=revocation_certifier,
                )
            patched_devices.append(device)

        self._users[device_id.user_id] = user.evolve(devices=DevicesMapping(*patched_devices))
