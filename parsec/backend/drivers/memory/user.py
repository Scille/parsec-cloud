# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from typing import Tuple, Optional, Dict
from collections import defaultdict

from parsec.types import UserID, DeviceID, DeviceName, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.user import (
    user_get_revoked_on,
    user_is_revoked,
    BaseUserComponent,
    User,
    Device,
    UserInvitation,
    DeviceInvitation,
    UserAlreadyExistsError,
    UserAlreadyRevokedError,
    UserNotFoundError,
)


@attr.s
class OrganizationStore:
    _users = attr.ib(factory=dict)
    _devices = attr.ib(factory=lambda: defaultdict(dict))
    _invitations = attr.ib(factory=dict)
    _device_configuration_tries = attr.ib(factory=dict)
    _unconfigured_devices = attr.ib(factory=dict)


class MemoryUserComponent(BaseUserComponent):
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self._organizations = defaultdict(OrganizationStore)

    async def create_user(
        self, organization_id: OrganizationID, user: User, first_device: Device
    ) -> None:
        org = self._organizations[organization_id]

        if user.user_id in org._users:
            raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

        org._users[user.user_id] = user
        org._devices[first_device.user_id][first_device.device_name] = first_device
        self.event_bus.send(
            "user.created",
            organization_id=organization_id,
            user_id=user.user_id,
            first_device_id=first_device.device_id,
        )

    async def create_device(
        self, organization_id: OrganizationID, device: Device, encrypted_answer: bytes = b""
    ) -> None:
        org = self._organizations[organization_id]

        if device.user_id not in org._users:
            raise UserNotFoundError(f"User `{device.user_id}` doesn't exists")

        user_devices = org._devices[device.user_id]
        if device.device_name in user_devices:
            raise UserAlreadyExistsError(f"Device `{device.device_id}` already exists")

        user_devices[device.device_name] = device
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
            device = await self._get_device(organization_id, device_id)
            trustchain[device_id] = device
            await _recursive_extract_creators(device.device_certifier)
            await _recursive_extract_creators(device.revoked_device_certifier)

        for device_id in devices_ids:
            await _recursive_extract_creators(device_id)

        return tuple(trustchain.values())

    def _get_user(self, organization_id: OrganizationID, user_id: UserID) -> User:
        org = self._organizations[organization_id]

        try:
            return org._users[user_id]

        except KeyError:
            raise UserNotFoundError(user_id)

    async def get_user(self, organization_id: OrganizationID, user_id: UserID) -> User:
        return self._get_user(organization_id, user_id)

    async def get_user_with_trustchain(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Tuple[Device]]:
        user = self._get_user(organization_id, user_id)
        trustchain = await self._get_trustchain(organization_id, user.user_certifier)
        return user, trustchain

    async def get_user_with_devices_and_trustchain(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Tuple[Device], Tuple[Device]]:
        user = self._get_user(organization_id, user_id)
        user_devices = self._get_user_devices(organization_id, user_id)
        user_devices = tuple(user_devices.values())
        trustchain = await self._get_trustchain(
            organization_id,
            user.user_certifier,
            *[device.device_certifier for device in user_devices],
            *[device.revoked_device_certifier for device in user_devices],
        )
        return user, user_devices, trustchain

    async def _get_device(self, organization_id: OrganizationID, device_id: DeviceID) -> Device:
        org = self._organizations[organization_id]

        try:
            return org._devices[device_id.user_id][device_id.device_name]

        except KeyError:
            raise UserNotFoundError(device_id)

    def _get_user_devices(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Dict[DeviceName, Device]:
        org = self._organizations[organization_id]
        # Make sure user exists
        self._get_user(organization_id, user_id)
        return org._devices[user_id]

    async def get_user_with_device(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[User, Device]:
        user = self._get_user(organization_id, device_id.user_id)
        device = await self._get_device(organization_id, device_id)
        return user, device

    async def find(
        self,
        organization_id: OrganizationID,
        query: str = None,
        page: int = 0,
        per_page: int = 100,
        omit_revoked: bool = False,
    ):
        org = self._organizations[organization_id]
        users = org._users

        if query:
            results = [user_id for user_id in users.keys() if user_id.startswith(query)]
        else:
            results = users.keys()

        if omit_revoked:
            results = [
                user_id
                for user_id in results
                if not user_is_revoked(org._devices[user_id].values())
            ]

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

        user_devices = self._get_user_devices(organization_id, invitation.device_id.user_id)
        if invitation.device_id.device_name in user_devices:
            raise UserAlreadyExistsError(f"Device `{invitation.device_id}` already exists")

        org._invitations[invitation.device_id] = invitation

    async def get_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> DeviceInvitation:
        org = self._organizations[organization_id]

        user_devices = self._get_user_devices(organization_id, device_id.user_id)
        if device_id.device_name in user_devices:
            raise UserAlreadyExistsError(device_id)

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
        revoked_device_certificate: bytes,
        revoked_device_certifier: DeviceID,
        revoked_on: pendulum.Pendulum = None,
    ) -> Optional[pendulum.Pendulum]:
        user_devices = self._get_user_devices(organization_id, device_id.user_id)
        try:
            if user_devices[device_id.device_name].revoked_on:
                raise UserAlreadyRevokedError()

        except KeyError:
            raise UserNotFoundError(device_id)

        user_devices[device_id.device_name] = user_devices[device_id.device_name].evolve(
            revoked_on=revoked_on or pendulum.now(),
            revoked_device_certificate=revoked_device_certificate,
            revoked_device_certifier=revoked_device_certifier,
        )
        return user_get_revoked_on(user_devices.values())
