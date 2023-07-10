# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, Any

from parsec._parsec import (
    ShamirRecoveryRecipient,
    ShamirRecoverySetup,
    ShamirRevealToken,
    VerifyKey,
)
from parsec.api.protocol import DeviceID, OrganizationID, UserID
from parsec.backend.shamir import BaseShamirComponent

if TYPE_CHECKING:
    from parsec.backend.memory.invite import MemoryInviteComponent
    from parsec.backend.memory.user import MemoryUserComponent


@dataclass
class ShamirRecoveryItem:
    brief_certificate: bytes
    reveal_token: ShamirRevealToken
    threshold: int
    recipients: tuple[ShamirRecoveryRecipient, ...]


class MemoryShamirComponent(BaseShamirComponent):
    def __init__(self) -> None:
        self._shamir_recovery_ciphered_data: dict[ShamirRevealToken, bytes] = {}
        self._shamir_recovery_items: dict[tuple[OrganizationID, UserID], ShamirRecoveryItem] = {}
        self._shamir_recovery_shares: dict[tuple[OrganizationID, UserID], dict[UserID, bytes]] = {}

    def register_components(
        self, user: MemoryUserComponent, invite: MemoryInviteComponent, **other_components: Any
    ) -> None:
        self._user_component = user
        self._invite_component = invite

    async def recovery_others_list(
        self, organization_id: OrganizationID, author: DeviceID
    ) -> list[tuple[bytes, bytes]]:
        result = []
        item_key = (organization_id, author.user_id)
        for user_id, share_certificate in self._shamir_recovery_shares.get(item_key, {}).items():
            other_item_key = (organization_id, user_id)
            item = self._shamir_recovery_items.get(other_item_key)
            if item is None:
                continue
            result.append((item.brief_certificate, share_certificate))
        return result

    async def recovery_self_info(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | None:
        item = self._shamir_recovery_items.get((organization_id, author.user_id))
        if item is None:
            return None
        return item.brief_certificate

    async def recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        setup: ShamirRecoverySetup | None,
    ) -> None:
        assert self._user_component is not None

        # Remove shared recovery device for this user
        if setup is None:
            item_key = (organization_id, author.user_id)
            item = self._shamir_recovery_items.pop(item_key, None)
            if item is not None:
                self._shamir_recovery_ciphered_data.pop(item.reveal_token, None)
            for mapping in self._shamir_recovery_shares.values():
                mapping.pop(author.user_id, None)
            await self._invite_component.delete_shamir_invitation(organization_id, author.user_id)
            return

        # Verify the certificates
        brief_certificate, share_certificates = self._verify_certificates(
            setup,
            author,
            author_verify_key,
        )

        # Get the recipients
        recipients = tuple(
            ShamirRecoveryRecipient(
                user_id,
                self._user_component._get_user(organization_id, user_id).human_handle,
                shares,
            )
            for user_id, shares in brief_certificate.per_recipient_shares.items()
        )

        # Save general information
        item_key = (organization_id, author.user_id)
        item = ShamirRecoveryItem(
            setup.brief, setup.reveal_token, brief_certificate.threshold, recipients
        )
        self._shamir_recovery_items[item_key] = item
        self._shamir_recovery_ciphered_data[(setup.reveal_token)] = setup.ciphered_data

        # Save the shares
        for user_id, raw_certificate in share_certificates.items():
            item_key = organization_id, user_id
            self._shamir_recovery_shares.setdefault(item_key, {})
            self._shamir_recovery_shares[item_key][author.user_id] = raw_certificate

        await self._invite_component.delete_shamir_invitation(organization_id, author.user_id)

    async def recovery_reveal(
        self,
        reveal_token: ShamirRevealToken,
    ) -> bytes | None:
        return self._shamir_recovery_ciphered_data.get(reveal_token)
