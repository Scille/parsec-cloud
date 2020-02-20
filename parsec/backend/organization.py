# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from typing import Optional
from secrets import token_hex

from pendulum import Pendulum

from parsec.utils import timestamps_in_the_ballpark
from parsec.crypto import VerifyKey
from parsec.api.protocol import (
    OrganizationID,
    organization_create_serializer,
    organization_bootstrap_serializer,
    organization_stats_serializer,
    organization_status_serializer,
    organization_update_serializer,
)
from parsec.api.data import UserCertificateContent, DeviceCertificateContent, DataError
from parsec.backend.user import new_user_factory, User, Device
from parsec.backend.utils import catch_protocol_errors, anonymous_api


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
class Organization:
    organization_id: OrganizationID
    bootstrap_token: str
    expiration_date: Optional[Pendulum] = None
    root_verify_key: Optional[VerifyKey] = None

    def is_bootstrapped(self):
        return self.root_verify_key is not None

    def is_expired(self):
        return self.expiration_date < Pendulum.now()

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class OrganizationStats:
    data_size: int
    metadata_size: int
    users: int


class BaseOrganizationComponent:
    def __init__(self, bootstrap_token_size: int = 32):
        self.bootstrap_token_size = bootstrap_token_size

    @catch_protocol_errors
    async def api_organization_create(self, client_ctx, msg):
        msg = organization_create_serializer.req_load(msg)

        bootstrap_token = token_hex(self.bootstrap_token_size)
        expiration_date = msg.get("expiration_date", None)
        try:
            await self.create(
                msg["organization_id"],
                bootstrap_token=bootstrap_token,
                expiration_date=expiration_date,
            )

        except OrganizationAlreadyExistsError:
            return {"status": "already_exists"}

        rep = {"bootstrap_token": bootstrap_token, "status": "ok"}
        if expiration_date:
            rep["expiration_date"] = expiration_date

        return organization_create_serializer.rep_dump(rep)

    @catch_protocol_errors
    async def api_organization_status(self, client_ctx, msg):
        msg = organization_status_serializer.req_load(msg)

        try:
            organization = await self.get(msg["organization_id"])

        except OrganizationNotFoundError:
            return {"status": "not_found"}

        return organization_status_serializer.rep_dump(
            {
                "is_bootstrapped": organization.is_bootstrapped(),
                "expiration_date": organization.expiration_date,
                "status": "ok",
            }
        )

    @catch_protocol_errors
    async def api_organization_stats(self, client_ctx, msg):
        msg = organization_stats_serializer.req_load(msg)

        try:
            stats = await self.stats(msg["organization_id"])

        except OrganizationNotFoundError:
            return {"status": "not_found"}

        return organization_stats_serializer.rep_dump(
            {
                "status": "ok",
                "users": stats.users,
                "data_size": stats.data_size,
                "metadata_size": stats.metadata_size,
            }
        )

    @catch_protocol_errors
    async def api_organization_update(self, client_ctx, msg):
        msg = organization_update_serializer.req_load(msg)

        try:
            await self.set_expiration_date(
                msg["organization_id"], expiration_date=msg["expiration_date"]
            )

        except OrganizationNotFoundError:
            return {"status": "not_found"}

        return organization_update_serializer.rep_dump({"status": "ok"})

    @anonymous_api
    @catch_protocol_errors
    async def api_organization_bootstrap(self, client_ctx, msg):
        msg = organization_bootstrap_serializer.req_load(msg)
        root_verify_key = msg["root_verify_key"]

        try:
            u_data = UserCertificateContent.verify_and_load(
                msg["user_certificate"], author_verify_key=root_verify_key, expected_author=None
            )
            d_data = DeviceCertificateContent.verify_and_load(
                msg["device_certificate"], author_verify_key=root_verify_key, expected_author=None
            )

        except DataError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
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

        now = pendulum.now()
        if not timestamps_in_the_ballpark(u_data.timestamp, now):
            return {
                "status": "invalid_certification",
                "reason": f"Invalid timestamp in certification.",
            }

        user, first_device = new_user_factory(
            device_id=d_data.device_id,
            is_admin=True,
            certifier=None,
            user_certificate=msg["user_certificate"],
            device_certificate=msg["device_certificate"],
        )
        try:
            await self.bootstrap(
                client_ctx.organization_id,
                user,
                first_device,
                msg["bootstrap_token"],
                root_verify_key,
            )

        except OrganizationAlreadyBootstrappedError:
            return {"status": "already_bootstrapped"}

        except (OrganizationNotFoundError, OrganizationInvalidBootstrapTokenError):
            return {"status": "not_found"}

        # Note: we let OrganizationFirstUserCreationError bobbles up given
        # it should not occurs under normal circumstances

        return organization_bootstrap_serializer.rep_dump({"status": "ok"})

    async def create(
        self, id: OrganizationID, bootstrap_token: str, expiration_date: Optional[Pendulum]
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
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
            OrganizationAlreadyBootstrappedError
            OrganizationInvalidBootstrapTokenError
            OrganizationFirstUserCreationError
        """
        raise NotImplementedError()

    async def stats(self, id: OrganizationID) -> OrganizationStats:
        """
        Raises:
            OrganizationNotFoundError
        """
        raise NotImplementedError()

    async def set_expiration_date(self, id: OrganizationID, expiration_date: Pendulum = None):
        """
        Raises:
            OrganizationNotFoundError
        """
        raise NotImplementedError()
