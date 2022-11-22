# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import attr
from typing import Any, List, Dict, Tuple, Union
from secrets import token_hex

from parsec._parsec import (
    ClientType,
    DateTime,
    OrganizationStatsReq,
    OrganizationStatsRep,
    OrganizationStatsRepOk,
    OrganizationStatsRepNotAllowed,
    OrganizationStatsRepNotFound,
    OrganizationConfigReq,
    OrganizationConfigRep,
    OrganizationConfigRepOk,
    OrganizationConfigRepNotFound,
    UsersPerProfileDetailItem,
)
from parsec.utils import timestamps_in_the_ballpark
from parsec.crypto import VerifyKey
from parsec.sequester_crypto import SequesterVerifyKeyDer
from parsec.api.data import (
    UserCertificate,
    DeviceCertificate,
    DataError,
    SequesterAuthorityCertificate,
)
from parsec.api.protocol import (
    OrganizationID,
    UserProfile,
    organization_bootstrap_serializer,
    apiv1_organization_bootstrap_serializer,
)
from parsec.backend.utils import (
    catch_protocol_errors,
    api,
    Unset,
    UnsetType,
    api_typed_msg_adapter,
)
from parsec.backend.user import User, Device
from parsec.backend.webhooks import WebhooksComponent
from parsec.backend.config import BackendConfig
from parsec.backend.client_context import (
    AnonymousClientContext,
    APIV1_AnonymousClientContext,
    AuthenticatedClientContext,
)


class OrganizationError(Exception):
    pass


class OrganizationAlreadyExistsError(OrganizationError):
    pass


class OrganizationAlreadyBootstrappedError(OrganizationError):
    pass


class OrganizationNotFoundError(OrganizationError):
    pass


class OrganizationInvalidBootstrapTokenError(OrganizationError):
    pass


class OrganizationFirstUserCreationError(OrganizationError):
    pass


class OrganizationExpiredError(OrganizationError):
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class SequesterAuthority:
    certificate: bytes
    verify_key_der: SequesterVerifyKeyDer


@attr.s(slots=True, frozen=True, auto_attribs=True)
class Organization:
    organization_id: OrganizationID
    bootstrap_token: str
    is_expired: bool
    created_on: DateTime
    bootstrapped_on: DateTime | None
    root_verify_key: VerifyKey | None
    user_profile_outsider_allowed: bool
    active_users_limit: int | None
    sequester_authority: SequesterAuthority | None
    sequester_services_certificates: Tuple[bytes, ...] | None

    def is_bootstrapped(self) -> bool:
        return self.root_verify_key is not None

    def evolve(self, **kwargs: Any) -> Organization:
        return attr.evolve(self, **kwargs)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class OrganizationStats:
    data_size: int
    metadata_size: int
    users: int
    active_users: int
    realms: int
    users_per_profile_detail: Tuple[UsersPerProfileDetailItem, ...]


def generate_bootstrap_token() -> str:
    return token_hex(32)


class BaseOrganizationComponent:
    def __init__(self, webhooks: WebhooksComponent, config: BackendConfig):
        self.webhooks = webhooks
        self._config = config

    @api("organization_config")
    @catch_protocol_errors
    @api_typed_msg_adapter(OrganizationConfigReq, OrganizationConfigRep)
    async def api_authenticated_organization_config(
        self, client_ctx: AuthenticatedClientContext, req: OrganizationConfigReq
    ) -> OrganizationConfigRep:
        organization_id = client_ctx.organization_id
        try:
            organization = await self.get(organization_id)

        except OrganizationNotFoundError:
            return OrganizationConfigRepNotFound()

        if organization.sequester_authority:
            sequester_authority_certificate: bytes | None = (
                organization.sequester_authority.certificate
            )
            sequester_services_certificates: List[bytes] | None = (
                list(organization.sequester_services_certificates)
                if organization.sequester_services_certificates
                else []
            )
        else:
            sequester_authority_certificate = None
            sequester_services_certificates = None

        return OrganizationConfigRepOk(
            user_profile_outsider_allowed=organization.user_profile_outsider_allowed,
            active_users_limit=organization.active_users_limit,
            sequester_authority_certificate=sequester_authority_certificate,
            sequester_services_certificates=sequester_services_certificates,
        )

    @api("organization_stats")
    @catch_protocol_errors
    @api_typed_msg_adapter(OrganizationStatsReq, OrganizationStatsRep)
    async def api_authenticated_organization_stats(
        self, client_ctx: AuthenticatedClientContext, req: OrganizationStatsReq
    ) -> OrganizationStatsRep:
        if client_ctx.profile != UserProfile.ADMIN:
            return OrganizationStatsRepNotAllowed(None)
        # Get organization of the user
        organization_id = client_ctx.organization_id
        try:
            stats = await self.stats(organization_id)

        except OrganizationNotFoundError:
            return OrganizationStatsRepNotFound()

        return OrganizationStatsRepOk(
            data_size=stats.data_size,
            metadata_size=stats.metadata_size,
            realms=stats.realms,
            users=stats.users,
            active_users=stats.active_users,
            users_per_profile_detail=list(stats.users_per_profile_detail),
        )

    @api(
        "organization_bootstrap",
        client_types=[ClientType.APIV1_ANONYMOUS, ClientType.ANONYMOUS],
    )
    @catch_protocol_errors
    async def api_organization_bootstrap(
        self,
        client_ctx: Union[APIV1_AnonymousClientContext, AnonymousClientContext],
        msg: dict[str, object],
    ) -> dict[str, object]:
        # Use the correct serializer depending on the API
        if isinstance(client_ctx, APIV1_AnonymousClientContext):
            serializer = apiv1_organization_bootstrap_serializer
        else:
            assert isinstance(client_ctx, AnonymousClientContext)
            serializer = organization_bootstrap_serializer

        msg = serializer.req_load(msg)

        bootstrap_token = msg["bootstrap_token"]
        root_verify_key = msg["root_verify_key"]

        try:
            u_data = UserCertificate.verify_and_load(
                msg["user_certificate"],
                author_verify_key=root_verify_key,
                expected_author=None,
            )
            d_data = DeviceCertificate.verify_and_load(
                msg["device_certificate"],
                author_verify_key=root_verify_key,
                expected_author=None,
            )

            ru_data = rd_data = None
            # TODO: Remove this `if` statement once APIv1 is no longer supported
            if "redacted_user_certificate" in msg:
                ru_data = UserCertificate.verify_and_load(
                    msg["redacted_user_certificate"],
                    author_verify_key=root_verify_key,
                    expected_author=None,
                )
            # TODO: Remove this `if` statement once APIv1 is no longer supported
            if "redacted_device_certificate" in msg:
                rd_data = DeviceCertificate.verify_and_load(
                    msg["redacted_device_certificate"],
                    author_verify_key=root_verify_key,
                    expected_author=None,
                )

        except DataError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }
        if u_data.profile != UserProfile.ADMIN:
            return {
                "status": "invalid_data",
                "reason": "Bootstrapping user must have admin profile.",
            }

        if u_data.timestamp != d_data.timestamp:
            return {
                "status": "invalid_data",
                "reason": "Device and user certificates must have the same timestamp.",
            }

        if u_data.user_id != d_data.device_id.user_id:
            return {
                "status": "invalid_data",
                "reason": "Device and user must have the same user ID.",
            }

        now = DateTime.now()
        if not timestamps_in_the_ballpark(u_data.timestamp, now):
            return serializer.timestamp_out_of_ballpark_rep_dump(
                backend_timestamp=now, client_timestamp=u_data.timestamp
            )

        if ru_data:
            if ru_data.evolve(human_handle=u_data.human_handle) != u_data:
                return {
                    "status": "invalid_data",
                    "reason": "Redacted User certificate differs from User certificate.",
                }
            if ru_data.human_handle:
                return {
                    "status": "invalid_data",
                    "reason": "Redacted User certificate must not contain a human_handle field.",
                }

        if rd_data:
            if rd_data.evolve(device_label=d_data.device_label) != d_data:
                return {
                    "status": "invalid_data",
                    "reason": "Redacted Device certificate differs from Device certificate.",
                }
            if rd_data.device_label:
                return {
                    "status": "invalid_data",
                    "reason": "Redacted Device certificate must not contain a device_label field.",
                }

        if (rd_data and not ru_data) or (ru_data and not rd_data):
            return {
                "status": "invalid_data",
                "reason": "Redacted user&device certificate must be provided together",
            }

        # Sequester can not be set with APIV1
        if isinstance(client_ctx, APIV1_AnonymousClientContext):
            sequester_authority = None

        else:
            sequester_authority_certificate = msg["sequester_authority_certificate"]
            if sequester_authority_certificate is None:
                sequester_authority = None

            else:
                try:
                    sequester_authority_certif_data = SequesterAuthorityCertificate.verify_and_load(
                        sequester_authority_certificate,
                        author_verify_key=root_verify_key,
                        expected_author=None,
                    )

                except DataError:
                    return {
                        "status": "invalid_data",
                        "reason": "Invalid signature for sequester authority certificate",
                    }

                if not timestamps_in_the_ballpark(sequester_authority_certif_data.timestamp, now):
                    return serializer.timestamp_out_of_ballpark_rep_dump(
                        backend_timestamp=now,
                        client_timestamp=sequester_authority_certif_data.timestamp,
                    )

                if sequester_authority_certif_data.timestamp != u_data.timestamp:
                    return {
                        "status": "invalid_data",
                        "reason": "Device, user and sequester authority certificates must have the same timestamp.",
                    }

                sequester_authority = SequesterAuthority(
                    certificate=sequester_authority_certificate,
                    verify_key_der=sequester_authority_certif_data.verify_key_der,
                )

        user = User(
            user_id=u_data.user_id,
            human_handle=u_data.human_handle,
            profile=u_data.profile,
            user_certificate=msg["user_certificate"],
            # TODO: Remove this `get` method once APIv1 is no longer supported
            redacted_user_certificate=msg.get("redacted_user_certificate", msg["user_certificate"]),
            user_certifier=u_data.author,
            created_on=u_data.timestamp,
        )
        first_device = Device(
            device_id=d_data.device_id,
            device_label=d_data.device_label,
            device_certificate=msg["device_certificate"],
            # TODO: Remove this `get` method once APIv1 is no longer supported
            redacted_device_certificate=msg.get(
                "redacted_device_certificate", msg["device_certificate"]
            ),
            device_certifier=d_data.author,
            created_on=d_data.timestamp,
        )
        try:
            await self.bootstrap(
                id=client_ctx.organization_id,
                user=user,
                first_device=first_device,
                bootstrap_token=bootstrap_token,
                root_verify_key=root_verify_key,
                bootstrapped_on=now,
                sequester_authority=sequester_authority,
            )

        except OrganizationAlreadyBootstrappedError:
            return {"status": "already_bootstrapped"}

        except (OrganizationNotFoundError, OrganizationInvalidBootstrapTokenError):
            return {"status": "not_found"}

        # Note: we let OrganizationFirstUserCreationError bubbles up given
        # it should not occurs under normal circumstances

        # Finally notify webhook
        await self.webhooks.on_organization_bootstrap(
            organization_id=client_ctx.organization_id,
            device_id=first_device.device_id,
            device_label=first_device.device_label,
            human_email=user.human_handle.email if user.human_handle else None,
            human_label=user.human_handle.label if user.human_handle else None,
        )

        return serializer.rep_dump({"status": "ok"})

    async def create(
        self,
        id: OrganizationID,
        bootstrap_token: str,
        # `None` is a valid value for some of those params, hence it cannot be used
        # as "param not set" marker and we use a custom `Unset` singleton instead.
        # `None` stands for "no limit"
        active_users_limit: Union[UnsetType, int | None] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
        created_on: DateTime | None = None,
    ) -> None:
        """
        Raises:
            OrganizationAlreadyExistsError
        """
        raise NotImplementedError()

    async def get(self, id: OrganizationID) -> Organization:
        """
        Raises:
            OrganizationNotFoundError
        """
        raise NotImplementedError()

    async def bootstrap(
        self,
        id: OrganizationID,
        user: User,
        first_device: Device,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
        bootstrapped_on: DateTime | None = None,
        sequester_authority: SequesterAuthority | None = None,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
            OrganizationAlreadyBootstrappedError
            OrganizationInvalidBootstrapTokenError
            OrganizationFirstUserCreationError
        """
        raise NotImplementedError()

    async def stats(
        self,
        id: OrganizationID,
        at: DateTime | None = None,
    ) -> OrganizationStats:
        """
        Raises:
            OrganizationNotFoundError

        Note: also raises `OrganizationNotFoundError` if the organization is
        present in the database but has been created after `at` datetime.
        """
        raise NotImplementedError()

    async def server_stats(
        self, at: DateTime | None = None
    ) -> Dict[OrganizationID, OrganizationStats]:
        """
        Raises: Nothing !
        """
        raise NotImplementedError()

    async def update(
        self,
        id: OrganizationID,
        # `None` is a valid value for some of those params, hence it cannot be used
        # as "param not set" marker and we use a custom `Unset` singleton instead.
        is_expired: Union[UnsetType, bool] = Unset,
        # `None` stands for "no limit"
        active_users_limit: Union[UnsetType, int | None] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
        """
        raise NotImplementedError()
