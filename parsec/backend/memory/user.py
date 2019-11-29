# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from typing import Tuple, Dict
from collections import defaultdict

from parsec.api.protocol import UserID, DeviceID, DeviceName, OrganizationID
from parsec.backend.user import (
    BaseUserComponent,
    User,
    Device,
    Trustchain,
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
    def __init__(self, send_event, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._send_event = send_event
        self._organizations = defaultdict(OrganizationStore)

    def register_components(self, **other_components):
        pass

    async def create_user(
        self, organization_id: OrganizationID, user: User, first_device: Device
    ) -> None:
        org = self._organizations[organization_id]

        if user.user_id in org._users:
            raise UserAlreadyExistsError(f"User `{user.user_id}` already exists")

        org._users[user.user_id] = user
        org._devices[first_device.user_id][first_device.device_name] = first_device
        await self._send_event(
            "user.created",
            organization_id=organization_id,
            user_id=user.user_id,
            user_certificate=user.user_certificate,
            first_device_id=first_device.device_id,
            first_device_certificate=first_device.device_certificate,
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
        await self._send_event(
            "device.created",
            organization_id=organization_id,
            device_id=device.device_id,
            device_certificate=device.device_certificate,
            encrypted_answer=encrypted_answer,
        )

    async def _get_trustchain(self, organization_id, *devices_ids):
        trustchain_devices = set()
        trustchain_users = set()
        trustchain_revoked_users = set()
        in_trustchain = set()

        async def _recursive_extract_creators(device_id):
            if not device_id or device_id in in_trustchain:
                return
            in_trustchain.add(device_id)
            user = self._get_user(organization_id, device_id.user_id)
            device = self._get_device(organization_id, device_id)
            trustchain_devices.add(device.device_certificate)
            trustchain_users.add(user.user_certificate)
            if user.revoked_user_certificate:
                trustchain_revoked_users.add(user.revoked_user_certificate)
            await _recursive_extract_creators(device.device_certifier)
            await _recursive_extract_creators(user.revoked_user_certifier)
            await _recursive_extract_creators(user.user_certifier)

        for device_id in devices_ids:
            await _recursive_extract_creators(device_id)

        return Trustchain(
            devices=tuple(trustchain_devices),
            users=tuple(trustchain_users),
            revoked_users=tuple(trustchain_revoked_users),
        )

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
    ) -> Tuple[User, Trustchain]:
        user = self._get_user(organization_id, user_id)
        trustchain = await self._get_trustchain(
            organization_id, user.user_certifier, user.revoked_user_certifier
        )
        return user, trustchain

    async def get_user_with_device_and_trustchain(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[User, Device, Trustchain]:
        user = self._get_user(organization_id, device_id.user_id)
        user_device = self._get_device(organization_id, device_id)
        trustchain = await self._get_trustchain(
            organization_id,
            user.user_certifier,
            user.revoked_user_certifier,
            user_device.device_certifier,
        )
        return user, user_device, trustchain

    async def get_user_with_devices_and_trustchain(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Tuple[Device], Trustchain]:
        user = self._get_user(organization_id, user_id)
        user_devices = self._get_user_devices(organization_id, user_id)
        user_devices = tuple(user_devices.values())
        trustchain = await self._get_trustchain(
            organization_id,
            user.user_certifier,
            user.revoked_user_certifier,
            *[device.device_certifier for device in user_devices],
        )
        return user, user_devices, trustchain

    def _get_device(self, organization_id: OrganizationID, device_id: DeviceID) -> Device:
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
        device = self._get_device(organization_id, device_id)
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
            try:
                UserID(query)
            except ValueError:
                # Contains invalid caracters, no need to go further
                return ([], 0)

            results = [
                user_id for user_id in users.keys() if user_id.lower().startswith(query.lower())
            ]

        else:
            results = users.keys()

        if omit_revoked:
            now = pendulum.now()

            def _user_is_revoked(user_id):
                revoked_on = org._users[user_id].revoked_on
                return revoked_on is not None and revoked_on <= now

            results = [user_id for user_id in results if not _user_is_revoked(user_id)]

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
        await self._send_event(
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
            await self._send_event(
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
        await self._send_event(
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
            await self._send_event(
                "device.invitation.cancelled", organization_id=organization_id, device_id=device_id
            )

    async def revoke_user(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        revoked_user_certificate: bytes,
        revoked_user_certifier: DeviceID,
        revoked_on: pendulum.Pendulum = None,
    ) -> None:
        org = self._organizations[organization_id]

        try:
            user = org._users[user_id]

        except KeyError:
            raise UserNotFoundError(user_id)

        if user.revoked_on:
            raise UserAlreadyRevokedError()

        org._users[user_id] = user.evolve(
            revoked_on=revoked_on or pendulum.now(),
            revoked_user_certificate=revoked_user_certificate,
            revoked_user_certifier=revoked_user_certifier,
        )

        await self._send_event("user.revoked", organization_id=organization_id, user_id=user_id)
