# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from typing import Tuple, Dict, Optional
from collections import defaultdict

from parsec.types import UserID, DeviceID, OrganizationID
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


@attr.s
class OrganizationStore:
    _users = attr.ib(factory=dict)
    _invitations = attr.ib(factory=dict)
    _device_configuration_tries = attr.ib(factory=dict)
    _unconfigured_devices = attr.ib(factory=dict)


class MemoryUserComponent(BaseUserComponent):
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self._organizations = defaultdict(OrganizationStore)

    async def set_user_admin(
        self, organization_id: OrganizationID, user_id: UserID, is_admin: bool
    ) -> None:
        org = self._organizations[organization_id]

        user = await self.get_user(organization_id, user_id)
        org._users[user_id] = user.evolve(is_admin=is_admin)

    async def create_user(self, organization_id: OrganizationID, user: User) -> None:
        org = self._organizations[organization_id]

        if user.user_id in org._users:
            raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

        org._users[user.user_id] = user
        self.event_bus.send("user.created", organization_id=organization_id, user_id=user.user_id)

    async def create_device(
        self, organization_id: OrganizationID, device: Device, encrypted_answer: bytes = b""
    ) -> None:
        org = self._organizations[organization_id]

        if device.user_id not in org._users:
            raise UserNotFoundError(f"User `{device.user_id}` doesn't exists")

        user = org._users[device.user_id]
        if device.device_name in user.devices:
            raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

        org._users[device.user_id] = user.evolve(
            devices=DevicesMapping(*user.devices.values(), device)
        )
        self.event_bus.send(
            "device.created",
            organization_id=organization_id,
            device_id=device.device_id,
            encrypted_answer=encrypted_answer,
        )

    async def _get_trustchain(self, organization_id, *devices_ids):
        trustchain = {}

        async def _recursive_extract_creators(device_id):
            if not device_id or device_id in trustchain:
                return
            device = await self.get_device(organization_id, device_id)
            trustchain[device_id] = device
            await _recursive_extract_creators(device.device_certifier)
            await _recursive_extract_creators(device.revocation_certifier)

        for device_id in devices_ids:
            await _recursive_extract_creators(device_id)
        return trustchain

    async def get_user(self, organization_id: OrganizationID, user_id: UserID) -> User:
        org = self._organizations[organization_id]

        try:
            return org._users[user_id]

        except KeyError:
            raise UserNotFoundError(user_id)

    async def get_user_with_trustchain(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Dict[DeviceID, Device]]:
        user = await self.get_user(organization_id, user_id)
        trustchain = await self._get_trustchain(
            organization_id,
            user.user_certifier,
            *[device.device_certifier for device in user.devices.values()],
            *[device.revocation_certifier for device in user.devices.values()],
        )
        return user, trustchain

    async def get_device(self, organization_id: OrganizationID, device_id: DeviceID) -> Device:
        user = await self.get_user(organization_id, device_id.user_id)
        try:
            return user.devices[device_id.device_name]

        except KeyError:
            raise UserNotFoundError(device_id)

    async def get_device_with_trustchain(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[Device, Dict[DeviceID, Device]]:
        device = await self.get_device(organization_id, device_id)
        trustchain = await self._get_trustchain(
            organization_id, device.device_certifier, device.revocation_certifier
        )
        return device, trustchain

    async def find(
        self,
        organization_id: OrganizationID,
        query: str = None,
        page: int = 0,
        per_page: int = 100,
        omit_revocated: bool = False,
    ):
        org = self._organizations[organization_id]
        users = org._users

        if query:
            results = [user_id for user_id in users.keys() if user_id.startswith(query)]
        else:
            results = users.keys()

        if omit_revocated:
            results = [user_id for user_id in results if not users[user_id].is_revocated()]

        # PostgreSQL does case insensitive sort
        sorted_results = sorted(results, key=lambda s: s.lower())
        return sorted_results[(page - 1) * per_page : page * per_page], len(results)

    async def create_user_invitation(
        self, organization_id: OrganizationID, invitation: UserInvitation
    ) -> None:
        org = self._organizations[organization_id]

        if invitation.user_id in org._users:
            raise UserAlreadyExistsError(f"User `{invitation.user_id}` already exists")
        org._invitations[invitation.user_id] = invitation

    async def get_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> UserInvitation:
        org = self._organizations[organization_id]

        if user_id in org._users:
            raise UserAlreadyExistsError(user_id)
        try:
            return org._invitations[user_id]
        except KeyError:
            raise UserNotFoundError(user_id)

    async def claim_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        invitation = await self.get_user_invitation(organization_id, user_id)
        self.event_bus.send(
            "user.claimed",
            organization_id=organization_id,
            user_id=invitation.user_id,
            encrypted_claim=encrypted_claim,
        )
        return invitation

    async def cancel_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> None:
        org = self._organizations[organization_id]

        if org._invitations.pop(user_id, None):
            self.event_bus.send(
                "user.invitation.cancelled", organization_id=organization_id, user_id=user_id
            )

    async def create_device_invitation(
        self, organization_id: OrganizationID, invitation: DeviceInvitation
    ) -> None:
        org = self._organizations[organization_id]

        user = await self.get_user(organization_id, invitation.device_id.user_id)
        if invitation.device_id.device_name in user.devices:
            raise UserAlreadyExistsError(f"Device `{invitation.device_id}` already exists")
        org._invitations[invitation.device_id] = invitation

    async def get_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> DeviceInvitation:
        org = self._organizations[organization_id]

        try:
            org._users[device_id.user_id].devices[device_id.device_name]
            raise UserAlreadyExistsError(device_id)
        except KeyError:
            pass
        try:
            return org._invitations[device_id]
        except KeyError:
            raise UserNotFoundError(device_id)

    async def claim_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        invitation = await self.get_device_invitation(organization_id, device_id)
        self.event_bus.send(
            "device.claimed",
            organization_id=organization_id,
            device_id=invitation.device_id,
            encrypted_claim=encrypted_claim,
        )
        return invitation

    async def cancel_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> None:
        org = self._organizations[organization_id]

        if org._invitations.pop(device_id, None):
            self.event_bus.send(
                "device.invitation.cancelled", organization_id=organization_id, device_id=device_id
            )

    async def revoke_device(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        certified_revocation: bytes,
        revocation_certifier: DeviceID,
        now: pendulum.Pendulum = None,
    ) -> Optional[pendulum.Pendulum]:
        now = now or pendulum.now()
        org = self._organizations[organization_id]

        user = await self.get_user(organization_id, device_id.user_id)
        try:
            if user.devices[device_id.device_name].revocated_on:
                raise UserAlreadyRevokedError()

        except KeyError:
            raise UserNotFoundError(device_id)

        patched_devices = []
        for device in user.devices.values():
            if device.device_id == device_id:
                device = device.evolve(
                    revocated_on=now,
                    certified_revocation=certified_revocation,
                    revocation_certifier=revocation_certifier,
                )
            patched_devices.append(device)

        user = org._users[device_id.user_id] = user.evolve(devices=DevicesMapping(*patched_devices))
        return user.get_revocated_on()
