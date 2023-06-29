# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from typing import TYPE_CHECKING, Any

from parsec._parsec import (
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryRecipient,
    ShamirRecoverySetup,
    ShamirRevealToken,
)
from parsec.api.protocol import DeviceID, OrganizationID, UserID
from parsec.backend.shamir import BaseShamirComponent

if TYPE_CHECKING:
    from parsec.backend.memory.user import MemoryUserComponent


class MemoryShamirComponent(BaseShamirComponent):
    def __init__(self) -> None:
        self._shamir_recovery_ciphered_data: dict[ShamirRevealToken, bytes] = {}
        self._shamir_recovery_brief_certs: dict[tuple[OrganizationID, UserID], bytes] = {}
        self._shamir_recovery_reveal: dict[tuple[OrganizationID, UserID], ShamirRevealToken] = {}
        self._shamir_recovery_shares_certs: dict[
            tuple[OrganizationID, UserID], tuple[bytes, ...]
        ] = {}
        self.thresholds: dict[tuple[OrganizationID, UserID], int] = {}
        self.recipients: dict[
            tuple[OrganizationID, UserID], tuple[ShamirRecoveryRecipient, ...]
        ] = {}

    def register_components(self, user: MemoryUserComponent, **other_components: Any) -> None:
        self._user_component = user

    async def recovery_others_list(self, organization_id: OrganizationID) -> tuple[bytes, ...]:
        return tuple(
            cert
            for (
                current_organization_id,
                _,
            ), cert in self._shamir_recovery_brief_certs.items()
            if current_organization_id == organization_id
        )

    async def recovery_self_info(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | None:
        return self._shamir_recovery_brief_certs.get((organization_id, author.user_id))

    async def recovery_setup(
        self, organization_id: OrganizationID, author: DeviceID, setup: ShamirRecoverySetup | None
    ) -> None:
        def map_recipients(x: tuple[UserID, int]) -> ShamirRecoveryRecipient:
            assert self._user_component is not None
            return ShamirRecoveryRecipient(
                user_id=x[0],
                human_handle=self._user_component._get_user(organization_id, x[0]).human_handle,
                shares=x[1],
            )

        if setup is None:
            self.thresholds.pop((organization_id, author.user_id), None)
            self.recipients.pop((organization_id, author.user_id), None)
            self._shamir_recovery_brief_certs.pop((organization_id, author.user_id), None)
            self._shamir_recovery_shares_certs.pop((organization_id, author.user_id), None)
            reveal_token = self._shamir_recovery_reveal.pop((organization_id, author.user_id), None)
            if reveal_token is not None:
                self._shamir_recovery_ciphered_data.pop(reveal_token, None)

        else:
            self._shamir_recovery_brief_certs[(organization_id, author.user_id)] = setup.brief
            self._shamir_recovery_shares_certs[(organization_id, author.user_id)] = setup.shares
            self._shamir_recovery_reveal[(organization_id, author.user_id)] = setup.reveal_token
            self._shamir_recovery_ciphered_data[(setup.reveal_token)] = setup.ciphered_data

            brief = ShamirRecoveryBriefCertificate.unsecure_load(setup.brief)
            self.thresholds[(organization_id, author.user_id)] = brief.threshold
            self.recipients[(organization_id, author.user_id)] = tuple(
                map(map_recipients, brief.per_recipient_shares.items())
            )

    async def recovery_reveal(
        self,
        reveal_token: ShamirRevealToken,
    ) -> bytes | None:
        return self._shamir_recovery_ciphered_data.get(reveal_token)
