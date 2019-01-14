import attr
import pendulum
from typing import Optional

from parsec.types import OrganizationID
from parsec.crypto import generate_token, VerifyKey
from parsec.trustchain import (
    TrustChainError,
    certified_extract_parts,
    validate_payload_certified_user,
    validate_payload_certified_device,
)
from parsec.api.protocole import organization_create_serializer, organization_bootstrap_serializer
from parsec.backend.user import new_user_factory, User
from parsec.backend.utils import catch_protocole_errors, anonymous_api


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


@attr.s(slots=True, frozen=True, auto_attribs=True)
class Organization:
    organization_id: OrganizationID
    bootstrap_token: str
    root_verify_key: Optional[VerifyKey] = None

    def is_bootstrapped(self):
        return self.root_verify_key is not None

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)


class BaseOrganizationComponent:
    def __init__(self, bootstrap_token_size: int = 32):
        self.bootstrap_token_size = bootstrap_token_size

    @catch_protocole_errors
    async def api_organization_create(self, client_ctx, msg):
        msg = organization_create_serializer.req_load(msg)

        bootstrap_token = generate_token(self.bootstrap_token_size)
        try:
            await self.create(msg["organization_id"], bootstrap_token=bootstrap_token)

        except OrganizationAlreadyExistsError:
            return {"status": "already_exists"}

        return organization_create_serializer.rep_dump(
            {"bootstrap_token": bootstrap_token, "status": "ok"}
        )

    @anonymous_api
    @catch_protocole_errors
    async def api_organization_bootstrap(self, client_ctx, msg):
        msg = organization_bootstrap_serializer.req_load(msg)
        root_verify_key = msg["root_verify_key"]

        try:
            u_certifier_id, u_payload = certified_extract_parts(msg["certified_user"])
            d_certifier_id, d_payload = certified_extract_parts(msg["certified_device"])

        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        try:
            now = pendulum.now()
            u_data = validate_payload_certified_user(root_verify_key, u_payload, now)
            d_data = validate_payload_certified_device(root_verify_key, d_payload, now)
        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if u_data["user_id"] != d_data["device_id"].user_id:
            return {
                "status": "invalid_data",
                "reason": "Device and user must have the same user ID.",
            }

        if u_data["timestamp"] != d_data["timestamp"]:
            return {
                "status": "invalid_data",
                "reason": "Device and user must have the same timestamp.",
            }

        user = new_user_factory(
            device_id=d_data["device_id"],
            is_admin=True,
            certifier=None,
            certified_user=msg["certified_user"],
            certified_device=msg["certified_device"],
        )
        try:
            await self.bootstrap(
                client_ctx.organization_id, user, msg["bootstrap_token"], root_verify_key
            )

        except OrganizationAlreadyBootstrappedError:
            return {"status": "already_bootstrapped"}

        except (OrganizationNotFoundError, OrganizationInvalidBootstrapTokenError):
            return {"status": "not_found"}

        # Note: we let OrganizationFirstUserCreationError bobbles up given
        # it should not occurs under normal circumstances

        return organization_bootstrap_serializer.rep_dump({"status": "ok"})

    async def create(self, id: OrganizationID, bootstrap_token: str) -> None:
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
        self, id: OrganizationID, user: User, bootstrap_token: str, root_verify_key: VerifyKey
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
            OrganizationAlreadyBootstrappedError
            OrganizationInvalidBootstrapTokenError
            OrganizationFirstUserCreationError
        """
        raise NotImplementedError()
