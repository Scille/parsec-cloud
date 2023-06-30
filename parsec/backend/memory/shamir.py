# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, Any

from parsec._parsec import (
    DataError,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryRecipient,
    ShamirRecoverySetup,
    ShamirRecoveryShareCertificate,
    ShamirRevealToken,
    VerifyKey,
)
from parsec.api.protocol import DeviceID, OrganizationID, UserID
from parsec.backend.shamir import BaseShamirComponent

if TYPE_CHECKING:
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
        self._shamir_recovery_shares: dict[
            tuple[OrganizationID, UserID], list[tuple[UserID, bytes]]
        ] = {}

    def register_components(self, user: MemoryUserComponent, **other_components: Any) -> None:
        self._user_component = user

    async def recovery_others_list(self, organization_id: OrganizationID) -> tuple[bytes, ...]:
        return tuple(
            item.brief_certificate
            for (
                current_organization_id,
                _,
            ), item in self._shamir_recovery_items.items()
            if current_organization_id == organization_id
        )

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
            return

        # Verify the certificates
        share_certificates: dict[UserID, bytes] = {}
        brief_certificate = ShamirRecoveryBriefCertificate.verify_and_load(
            setup.brief,
            author_verify_key,
            expected_author=author,
        )
        for raw_share in setup.shares:
            share_certificate = ShamirRecoveryShareCertificate.verify_and_load(
                raw_share, author_verify_key, expected_author=author
            )
            if share_certificate.recipient not in brief_certificate.per_recipient_shares:
                raise DataError(
                    f"Recipient {share_certificate.recipient.str} does not in appear in the brief certificate"
                )
            if share_certificate.recipient in share_certificates:
                raise DataError(
                    f"Recipient {share_certificate.recipient.str} appears more than once"
                )
            if share_certificate.recipient == author.user_id:
                raise DataError(f"Author {author.user_id} included themselves in the recipients")
            share_certificates[share_certificate.recipient] = raw_share
        delta = set(brief_certificate.per_recipient_shares) - set(share_certificates)
        if delta:
            missing = ", ".join(user_id.str for user_id in delta)
            raise DataError(f"The following shares are missing: {missing}")

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
            self._shamir_recovery_shares.setdefault(item_key, []).append(
                (author.user_id, raw_certificate)
            )

    async def recovery_reveal(
        self,
        reveal_token: ShamirRevealToken,
    ) -> bytes | None:
        return self._shamir_recovery_ciphered_data.get(reveal_token)
