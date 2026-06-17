# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import csv
from collections.abc import Awaitable, Callable
from enum import StrEnum
from functools import wraps
from io import StringIO
from typing import (
    TYPE_CHECKING,
    Annotated,
    Literal,
    cast,
)

from fastapi import APIRouter, Depends, HTTPException, Request, Response, status
from fastapi.security import HTTPAuthorizationCredentials, HTTPBearer
from pydantic import BaseModel, ConfigDict, Field, NonNegativeInt

from parsec._parsec import (
    AccessToken,
    DateTime,
    OrganizationID,
    ParsecOrganizationBootstrapAddr,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    UserProfile,
)
from parsec.components.organization import (
    Organization,
    OrganizationCreateBadOutcome,
    OrganizationGetBadOutcome,
    OrganizationStats,
    OrganizationStatsBadOutcome,
    OrganizationUpdateBadOutcome,
    TosLocale,
    TosUrl,
)
from parsec.components.sequester import (
    RequireGreaterTimestamp,
    SequesterCreateServiceStoreBadOutcome,
    SequesterCreateServiceValidateBadOutcome,
    SequesterGetOrganizationServicesBadOutcome,
    SequesterRevokeServiceStoreBadOutcome,
    SequesterRevokeServiceValidateBadOutcome,
    SequesterServiceConfig,
    SequesterServiceType,
    SequesterUpdateConfigForServiceStoreBadOutcome,
    WebhookSequesterService,
)
from parsec.components.totp import TOTPResetBadOutcome
from parsec.components.user import UserFreezeUserBadOutcome, UserInfo, UserListActiveUsersBadOutcome
from parsec.events import ActiveUsersLimitField, DateTimeField, OrganizationIDField, UserIDField
from parsec.logging import get_logger
from parsec.types import (
    Base64BytesField,
    EmailAddressField,
    SequesterServiceIDField,
    Unset,
    UnsetType,
)

if TYPE_CHECKING:
    from parsec.backend import Backend


logger = get_logger()


administration_router = APIRouter()


# TODO: Use 401 Unauthorized instead of 403 Forbidden on failed authentication
# Before FastAPI version 0.122.0, when the integrated security utilities returned an error to the client
# after a failed authentication, they used the HTTP status code 403 Forbidden.
#
# Starting with FastAPI version 0.122.0, they use the more appropriate HTTP status code 401 Unauthorized,
# and return a sensible WWW-Authenticate header in the response, following the HTTP specifications, RFC 7235, RFC 9110.
#
# This is a "hack" suggested in FastAPI docs to keep the old behavior.
# See: https://fastapi.tiangolo.com/how-to/authentication-error-status-code/
class HTTPBearer403(HTTPBearer):
    def make_not_authenticated_error(self) -> HTTPException:
        return HTTPException(status_code=status.HTTP_403_FORBIDDEN, detail="Not authenticated")


security = HTTPBearer403()


def check_administration_auth(
    request: Request, credentials: Annotated[HTTPAuthorizationCredentials, Depends(security)]
) -> None:
    if request.app.state.backend.config.administration_token != credentials.credentials:
        raise HTTPException(status_code=403, detail="Bad authorization token")


# This function is a workaround for FastAPI's broken custom type in query parameters
# (see https://github.com/tiangolo/fastapi/issues/10259)
def parse_organization_id_or_die(raw_organization_id: str) -> OrganizationID:
    try:
        return OrganizationID(raw_organization_id)
    except ValueError:
        raise HTTPException(
            status_code=404,
            detail="Invalid organization ID",
        )


def log_request[**P, T: BaseModel | Response](
    func: Callable[P, Awaitable[T]],
) -> Callable[P, Awaitable[T]]:
    @wraps(func)
    async def wrapped(*args: P.args, **kwargs: P.kwargs) -> T:
        request = cast(Request, kwargs["request"])
        body = cast(BaseModel | None, kwargs.get("body"))
        body_dict = {} if body is None else body.model_dump()
        logger.debug(f"{request.method} {request.url.path} request", **body_dict)
        try:
            result = await func(*args, **kwargs)
        except HTTPException as e:
            logger.info(
                f"{request.method} {request.url.path} HTTP error",
                status_code=e.status_code,
                detail=e.detail,
            )
            raise
        except Exception as e:
            logger.error(f"{request.method} {request.url.path} exception", exc_info=e)
            raise
        except BaseException as e:
            logger.debug(f"{request.method} {request.url.path} base exception", exc_info=e)
            raise
        if isinstance(result, Response):
            body = result.body
            if isinstance(body, memoryview):
                body = bytes(body)
            debug_extra = {
                "status_code": result.status_code,
                "body": body.decode("utf-8"),
            }
            logger.info_with_debug_extra(
                f"{request.method} {request.url.path} response", debug_extra=debug_extra
            )
        if isinstance(result, BaseModel):
            logger.info_with_debug_extra(
                f"{request.method} {request.url.path} reply", debug_extra=result.model_dump()
            )
        return result

    return wrapped


tos_example = {
    "fr_FR": "https://example.com/tos_fr.html",
    "en_US": "https://example.com/tos_en.html",
}


class CreateOrganizationIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    organization_id: OrganizationIDField
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: ask the server to use its default value for this field
    # - field set to `None`: `None` is a valid value to use for this field
    user_profile_outsider_allowed: bool | UnsetType = Field(Unset, examples=[True])
    active_users_limit: ActiveUsersLimitField | UnsetType = Field(Unset, examples=[50])
    realm_minimum_archiving_period_before_deletion: NonNegativeInt | UnsetType = Field(
        Unset, examples=[2592000]
    )
    tos: dict[TosLocale, TosUrl] | UnsetType = Field(Unset, examples=[tos_example])


class CreateOrganizationOut(BaseModel):
    bootstrap_url: str = Field(
        examples=[
            # cspell: disable-next-line
            "parsec3://parsec.invalid/MyOrganization?a=bootstrap_organization&p=xBDz8e8SAIBia36yYIolxIM_"
        ]
    )
    bootstrap_url_as_http_redirection: str = Field(
        examples=[
            # cspell: disable-next-line
            "https://parsec.invalid/redirect/MyOrganization?a=bootstrap_organization&p=xBDz8e8SAIBia36yYIolxIM_"
        ]
    )


@administration_router.post(
    "/administration/organizations",
    summary="Create an Organization with the specified configuration",
    tags=["Organization"],
)
@log_request
async def administration_create_organizations(
    request: Request,
    body: CreateOrganizationIn,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> CreateOrganizationOut:
    """
    The `organization_id` is the name of the organization (see [About Organization names](https://docs.parsec.cloud/en/latest/userguide/new-organization.html#about-organization-names)).

    Configuration options:

    - `user_profile_outsider_allowed`: Whether to allow (True) or disallow (False) users with External profile.
    - `active_users_limit`: The maximum number of allowed active users (e.g. non-revoked).
    - `realm_minimum_archiving_period_before_deletion`: The amount of time (in seconds) that must elapse before a workspace scheduled for deletion is actually deleted.
    - `tos`: Links to custom Term of Services for this organization

    If the organization was created successfully, the *Bootstrap URL* is returned both as a Parsec URL and as an HTTP URL.
    """
    backend: Backend = request.app.state.backend

    outcome = await backend.organization.create(
        now=DateTime.now(),
        id=body.organization_id,
        user_profile_outsider_allowed=body.user_profile_outsider_allowed,
        active_users_limit=body.active_users_limit,
        realm_minimum_archiving_period_before_deletion=body.realm_minimum_archiving_period_before_deletion,
        tos=body.tos,
    )
    match outcome:
        case AccessToken() as bootstrap_token:
            pass
        case OrganizationCreateBadOutcome.ORGANIZATION_ALREADY_EXISTS:
            raise HTTPException(
                status_code=400,
                detail="Organization already exists",
            )

    assert request.url.hostname is not None
    bootstrap_url = ParsecOrganizationBootstrapAddr(
        body.organization_id,
        bootstrap_token,
        hostname=request.url.hostname,
        port=request.url.port,
        use_ssl=request.url.scheme == "https",
    )
    return CreateOrganizationOut(
        bootstrap_url=bootstrap_url.to_url(),
        bootstrap_url_as_http_redirection=bootstrap_url.to_http_redirection_url(),
    )


class GetOrganizationOutTos(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    per_locale_urls: dict[TosLocale, TosUrl] = Field(examples=[tos_example])
    updated_on: DateTimeField


class GetOrganizationOut(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    is_bootstrapped: bool
    is_expired: bool
    user_profile_outsider_allowed: bool
    active_users_limit: int | None
    realm_minimum_archiving_period_before_deletion: NonNegativeInt
    tos: GetOrganizationOutTos | None


@administration_router.get(
    "/administration/organizations/{raw_organization_id}",
    summary="Get an Organization status and configuration",
    tags=["Organization"],
)
@log_request
async def administration_get_organization(
    raw_organization_id: str,
    request: Request,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> GetOrganizationOut:
    """
    The organization status is described by:

    - `is_bootstrapped` whether the organization has been bootstrapped or not
    - `is_expired` whether the organization has been expired or not. Users are not allowed to connect to an expired organization.

    The organization configuration is described by the same options used during organization creation.
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    # Check whether the organization actually exists
    outcome = await backend.organization.get(id=organization_id)
    match outcome:
        case Organization() as organization:
            pass
        case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")

    return GetOrganizationOut(
        is_bootstrapped=organization.is_bootstrapped,
        is_expired=organization.is_expired,
        user_profile_outsider_allowed=organization.user_profile_outsider_allowed,
        active_users_limit=organization.active_users_limit.to_maybe_int(),
        realm_minimum_archiving_period_before_deletion=organization.realm_minimum_archiving_period_before_deletion,
        tos=None
        if organization.tos is None
        else GetOrganizationOutTos(
            updated_on=organization.tos.updated_on,
            per_locale_urls=organization.tos.per_locale_urls,
        ),
    )


class PatchOrganizationOut(BaseModel):
    pass


class PatchOrganizationIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    is_expired: bool | UnsetType = Field(Unset, examples=[False])
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: ask the server to use its default value for this field
    # - field set to `None`: `None` is a valid value to use for this field
    user_profile_outsider_allowed: bool | UnsetType = Field(Unset, examples=[True])
    active_users_limit: ActiveUsersLimitField | UnsetType = Field(Unset, examples=[50])
    realm_minimum_archiving_period_before_deletion: NonNegativeInt | UnsetType = Field(
        Unset, examples=[2592000]
    )
    tos: dict[TosLocale, TosUrl] | UnsetType | None = Field(Unset, examples=[tos_example])


@administration_router.patch(
    "/administration/organizations/{raw_organization_id}",
    summary="Update Organization status and configuration",
    tags=["Organization"],
)
@log_request
async def administration_patch_organization(
    raw_organization_id: str,
    body: PatchOrganizationIn,
    request: Request,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> PatchOrganizationOut:
    """
    The organization status can be updated with:

    - `is_expired` whether the organization has been expired or not. Users are not allowed to connect to an expired organization.

    The organization configuration is described by the same options used during organization creation.

    Fields that you do not want to be updated should be omitted from the request.
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.organization.update(
        now=DateTime.now(),
        id=organization_id,
        is_expired=body.is_expired,
        active_users_limit=body.active_users_limit,
        user_profile_outsider_allowed=body.user_profile_outsider_allowed,
        realm_minimum_archiving_period_before_deletion=body.realm_minimum_archiving_period_before_deletion,
        tos=body.tos,
    )
    match outcome:
        case None:
            pass
        case OrganizationUpdateBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")

    return PatchOrganizationOut()


org_stats_example = {
    "realms": 10,
    "data_size": 18333,
    "metadata_size": 1158,
    "users": 9,
    "active_users": 6,
    "users_per_profile_detail": {
        "ADMIN": {"active": 2, "revoked": 1},
        "STANDARD": {"active": 2, "revoked": 1},
        "OUTSIDER": {"active": 2, "revoked": 1},
    },
}


class UsersPerProfile(BaseModel):
    active: NonNegativeInt
    revoked: NonNegativeInt


class GetOrganizationStatsOut(BaseModel):
    model_config = ConfigDict(strict=True)
    realms: NonNegativeInt
    data_size: NonNegativeInt
    metadata_size: NonNegativeInt
    users: NonNegativeInt
    active_users: NonNegativeInt
    users_per_profile_detail: dict[str, UsersPerProfile]


@administration_router.get(
    "/administration/organizations/{raw_organization_id}/stats",
    summary="Get Organization usage statistics",
    tags=["Stats"],
    responses={
        200: {
            "content": {
                "application/json": {"example": org_stats_example},
            },
        },
    },
)
@log_request
async def administration_organization_stat(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> GetOrganizationStatsOut:
    """
    The organization usage statistics includes:

    - `data_size`: size of stored data blocks (S3)
    - `metadata_size`: size of stored metadata (PostgreSQL database)
    - `realms`: number of realms created (workspaces)
    - `active_users`: number of active users (e.g. non-revoked)
    - `users_per_profile_detail`: number of active/revoked per profile:
       - `ADMIN`: number of active/revoked users with **ADMINISTRATOR** profile
       - `STANDARD`: number of active/revoked users with **MEMBER** profile
       - `OUTSIDER`: number of active/revoked users with **EXTERNAL** profile
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.organization.organization_stats(organization_id)
    match outcome:
        case OrganizationStats() as stats:
            pass
        case OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")

    return GetOrganizationStatsOut(
        realms=stats.realms,
        data_size=stats.data_size,
        metadata_size=stats.metadata_size,
        users=stats.users,
        active_users=stats.active_users,
        users_per_profile_detail={
            detail.profile.str: UsersPerProfile(
                active=detail.active,
                revoked=detail.revoked,
            )
            for detail in stats.users_per_profile_detail
        },
    )


def _convert_server_stats_results_as_csv(stats: dict[OrganizationID, OrganizationStats]) -> str:
    # Use `newline=""` to let the CSV writer handles the newlines
    with StringIO(newline="") as memory_file:
        writer = csv.writer(memory_file)
        # Header
        writer.writerow(
            [
                "organization_id",
                "data_size",
                "metadata_size",
                "realms",
                "active_users",
                "admin_users_active",
                "admin_users_revoked",
                "standard_users_active",
                "standard_users_revoked",
                "outsider_users_active",
                "outsider_users_revoked",
            ]
        )

        def _find_profile_counts(profile: UserProfile) -> tuple[int, int]:
            detail = next(x for x in org_stats.users_per_profile_detail if x.profile == profile)
            return (detail.active, detail.revoked)

        for organization_id, org_stats in stats.items():
            csv_row = [
                organization_id.str,
                org_stats.data_size,
                org_stats.metadata_size,
                org_stats.realms,
                org_stats.active_users,
                *_find_profile_counts(UserProfile.ADMIN),
                *_find_profile_counts(UserProfile.STANDARD),
                *_find_profile_counts(UserProfile.OUTSIDER),
            ]
            writer.writerow(csv_row)

        return memory_file.getvalue()


class StatsFormat(StrEnum):
    CSV = "csv"
    JSON = "json"


class GetServerStatsOrganizationDetail(BaseModel):
    model_config = ConfigDict(strict=True)
    organization_id: str
    realms: NonNegativeInt
    data_size: NonNegativeInt
    metadata_size: NonNegativeInt
    users: NonNegativeInt
    active_users: NonNegativeInt
    users_per_profile_detail: dict[str, UsersPerProfile]


class GetServerStatsOut(BaseModel):
    model_config = ConfigDict(strict=True)
    stats: list[GetServerStatsOrganizationDetail]


server_stats_example_json = {
    "stats": [dict({"organization_id": "MyOrganization"}, **org_stats_example)]
}
server_stats_example_csv = """
organization_id,realms,data_size,metadata_size,users,active_users,admin_users_active,admin_users_revoked,standard_users_active,standard_users_revoked,outsider_users_active,outsider_users_revoked
"MyOrganization, 10,18333, 1158, 9, 6,  2, 1, 2, 1, 2, 1
"""


@administration_router.get(
    "/administration/stats",
    summary="Get Server usage statistics",
    tags=["Stats"],
    response_model=None,
    responses={
        200: {
            "content": {
                "application/json": {"example": server_stats_example_json},
                "text/csv": {"example": server_stats_example_csv},
            },
        },
    },
)
@log_request
async def administration_server_stats(
    request: Request,
    response: Response,
    auth: Annotated[None, Depends(check_administration_auth)],
    format: StatsFormat = StatsFormat.JSON,
    at: str | None = None,
) -> GetServerStatsOut | Response:
    """
    The following parameters control the export:

    - `format`: whether the information is exported in **JSON** or **CSV** (with headers)
    - `at`: The extraction date in RFC 3339 format. Everything after the date provided will be ignored.

    The server usage statistics *for each organization* includes:

    - `organization_id`: the organization name
    - `realms`: number of realms created (workspaces)
    - `data_size`: size of stored data blocks (S3)
    - `metadata_size`: size of stored metadata (PostgreSQL database)
    - `active_users`: number of active users (e.g. non-revoked)
    - `users_per_profile_detail`: number of active/revoked per profile
       - `ADMIN`: number of active/revoked users with **ADMINISTRATOR** profile
       - `STANDARD`: number of active/revoked users with **MEMBER** profile
       - `OUTSIDER`: number of active/revoked users with **EXTERNAL** profile
    """
    backend: Backend = request.app.state.backend

    try:
        typed_at = DateTime.from_rfc3339(at) if at else None
    except ValueError:
        raise HTTPException(
            status_code=400,
            detail="Invalid `at` query argument (expected RFC 3339 datetime)",
        )

    server_stats = await backend.organization.server_stats(at=typed_at)

    match format:
        case StatsFormat.CSV:
            csv_data = _convert_server_stats_results_as_csv(server_stats)
            return Response(
                status_code=200,
                media_type="text/csv",
                content=csv_data,
            )

        case StatsFormat.JSON:
            return GetServerStatsOut(
                stats=[
                    GetServerStatsOrganizationDetail(
                        organization_id=organization_id.str,
                        realms=org_stats.realms,
                        data_size=org_stats.data_size,
                        metadata_size=org_stats.metadata_size,
                        users=org_stats.users,
                        active_users=org_stats.active_users,
                        users_per_profile_detail={
                            detail.profile.str: UsersPerProfile(
                                active=detail.active,
                                revoked=detail.revoked,
                            )
                            for detail in org_stats.users_per_profile_detail
                        },
                    )
                    for organization_id, org_stats in server_stats.items()
                ]
            )


class OrganizationUserDetail(BaseModel):
    user_id: str
    user_email: str
    user_name: str
    frozen: bool


class GetOrganizationUsersOut(BaseModel):
    users: list[OrganizationUserDetail]


@administration_router.get(
    "/administration/organizations/{raw_organization_id}/users",
    summary="Get organization Users",
    tags=["Users"],
)
@log_request
async def administration_organization_users(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> GetOrganizationUsersOut:
    """
    Get the list of all Users in the organization
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.user.list_active_users(organization_id)
    match outcome:
        case list() as users:
            pass
        case UserListActiveUsersBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")

    return GetOrganizationUsersOut(
        users=[
            OrganizationUserDetail(
                user_id=user.user_id.hex,
                user_email=str(user.human_handle.email),
                user_name=user.human_handle.label,
                frozen=user.frozen,
            )
            for user in users
        ]
    )


class UserFreezeIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    frozen: bool
    user_email: EmailAddressField | None = None
    user_id: UserIDField | None = None


class UserFreezeOut(BaseModel):
    user_id: str
    user_email: str
    user_name: str
    frozen: bool


@administration_router.post(
    "/administration/organizations/{raw_organization_id}/users/freeze",
    summary="Update User frozen status",
    tags=["Users"],
)
@log_request
async def administration_organization_users_freeze(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    body: UserFreezeIn,
    request: Request,
) -> UserFreezeOut:
    """
    Set `frozen` to `True` to freeze the user.

    A frozen user will be blocked from connecting to Parsec server.
    Unlike revocation, this operation can be reverted by setting `frozen` to `False`.

    See [Freeze Users](https://docs.parsec.cloud/en/latest/hosting/administration/freeze-users.html).
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.user.freeze_user(
        organization_id, user_id=body.user_id, user_email=body.user_email, frozen=body.frozen
    )
    match outcome:
        case UserInfo() as user:
            pass
        case UserFreezeUserBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case UserFreezeUserBadOutcome.USER_NOT_FOUND:
            raise HTTPException(status_code=404, detail="User not found")
        case UserFreezeUserBadOutcome.USER_REVOKED:
            raise HTTPException(status_code=404, detail="User has been revoked")
        case UserFreezeUserBadOutcome.BOTH_USER_ID_AND_EMAIL:
            raise HTTPException(
                status_code=400, detail="Both `user_id` and `user_email` fields are provided"
            )
        case UserFreezeUserBadOutcome.NO_USER_ID_NOR_EMAIL:
            raise HTTPException(
                status_code=400, detail="Missing either `user_id` or `user_email` field"
            )

    return UserFreezeOut(
        user_id=user.user_id.hex,
        user_email=str(user.human_handle.email),
        user_name=user.human_handle.label,
        frozen=user.frozen,
    )


class UserResetTOTPIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    user_email: EmailAddressField | None = None
    user_id: UserIDField | None = None
    send_email: bool = False


class UserResetTOTPOut(BaseModel):
    user_id: str
    user_email: str
    totp_reset_url: str
    totp_reset_url_as_http_redirection: str
    email_sent_status: str


@administration_router.post(
    "/administration/organizations/{raw_organization_id}/users/reset_totp",
    summary="Reset User TOTP setup for MFA",
    tags=["Users"],
)
@log_request
async def administration_organization_users_reset_totp(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    body: UserResetTOTPIn,
    request: Request,
) -> UserResetTOTPOut:
    """
    Reset TOTP setup allows the user to define a new TOTP setup.

    If `send_email` is set to True, an email will be sent to the user with the link to
    configure a new TOTP setup for MFA.

    See [MFA Setup reset](https://docs.parsec.cloud/en/latest/hosting/administration/user-authentication.html#mfa-setup-reset).
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.totp.reset(
        organization_id,
        user_id=body.user_id,
        user_email=body.user_email,
        send_email=body.send_email,
    )
    match outcome:
        case (user_id, user_email, totp_reset_url, None):
            if body.send_email:
                email_sent_status = "SENT_AS_REQUESTED"
            else:
                email_sent_status = "NOT_SENT_AS_REQUESTED"
        case (user_id, user_email, totp_reset_url, email_sent_status):
            email_sent_status = email_sent_status.name
        case TOTPResetBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case TOTPResetBadOutcome.USER_NOT_FOUND:
            raise HTTPException(status_code=404, detail="User not found")
        case TOTPResetBadOutcome.USER_REVOKED:
            raise HTTPException(status_code=404, detail="User has been revoked")
        case TOTPResetBadOutcome.BOTH_USER_ID_AND_EMAIL:
            raise HTTPException(
                status_code=400, detail="Both `user_id` and `user_email` fields are provided"
            )
        case TOTPResetBadOutcome.NO_USER_ID_NOR_EMAIL:
            raise HTTPException(
                status_code=400, detail="Missing either `user_id` or `user_email` field"
            )

    return UserResetTOTPOut(
        user_id=user_id.hex,
        user_email=user_email.str,
        totp_reset_url=totp_reset_url.to_url(),
        totp_reset_url_as_http_redirection=totp_reset_url.to_http_redirection_url(),
        email_sent_status=email_sent_status,
    )


class StorageSequesterServiceDetail(BaseModel):
    service_id: str
    service_label: str
    created_on: str
    revoked_on: str | None
    type: str


class WebhookSequesterServiceDetail(BaseModel):
    service_id: str
    service_label: str
    created_on: str
    revoked_on: str | None
    type: str
    webhook_url: str


class GetSequesterServiceOut(BaseModel):
    services: list[StorageSequesterServiceDetail | WebhookSequesterServiceDetail]


@administration_router.get(
    "/administration/organizations/{raw_organization_id}/sequester/services",
    summary="Get Sequester Services",
    tags=["Sequester"],
)
@log_request
async def administration_organization_sequester_services(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> GetSequesterServiceOut:
    """
    Get the list of all sequester services configured in the server.
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.sequester.get_organization_services(organization_id)
    match outcome:
        case list() as services:
            cooked_services = [
                WebhookSequesterServiceDetail(
                    service_id=service.service_id.hex,
                    service_label=service.service_label,
                    created_on=service.created_on.to_rfc3339(),
                    revoked_on=service.revoked_on.to_rfc3339() if service.revoked_on else None,
                    type=service.service_type.value,
                    webhook_url=service.webhook_url,
                )
                if isinstance(service, WebhookSequesterService)
                else StorageSequesterServiceDetail(
                    service_id=service.service_id.hex,
                    service_label=service.service_label,
                    created_on=service.created_on.to_rfc3339(),
                    revoked_on=service.revoked_on.to_rfc3339() if service.revoked_on else None,
                    type=service.service_type.value,
                )
                for service in services
            ]

        case SequesterGetOrganizationServicesBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case SequesterGetOrganizationServicesBadOutcome.SEQUESTER_DISABLED:
            raise HTTPException(status_code=400, detail="Sequester disabled")

    return GetSequesterServiceOut(services=cooked_services)


class SequesterServiceConfigInStorage(BaseModel):
    model_config = ConfigDict(strict=True)
    type: Literal["storage"]

    @property
    def cooked(self) -> SequesterServiceConfig:
        return SequesterServiceType.STORAGE


class SequesterServiceConfigInWebhook(BaseModel):
    model_config = ConfigDict(strict=True)
    type: Literal["webhook"]
    webhook_url: str

    @property
    def cooked(self) -> SequesterServiceConfig:
        return (SequesterServiceType.WEBHOOK, self.webhook_url)


SequesterServiceConfigField = Annotated[
    SequesterServiceConfigInStorage | SequesterServiceConfigInWebhook,
    Field(
        discriminator="type",
    ),
]


class CreateSequesterServiceIn(BaseModel):
    model_config = ConfigDict(strict=True)
    service_certificate: Base64BytesField
    config: SequesterServiceConfigField


class CreateSequesterServiceOut(BaseModel):
    pass


@administration_router.post(
    "/administration/organizations/{raw_organization_id}/sequester/services",
    summary="Create a Sequester Service",
    tags=["Sequester"],
)
@log_request
async def administration_organization_sequester_service_create(
    raw_organization_id: str,
    body: CreateSequesterServiceIn,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> CreateSequesterServiceOut:
    """
    Create a sequester service with the specified configuration.
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.sequester.create_service(
        now=DateTime.now(),
        organization_id=organization_id,
        service_certificate=body.service_certificate,
        config=body.config.cooked,
    )
    match outcome:
        case SequesterServiceCertificate():
            pass
        case SequesterCreateServiceValidateBadOutcome.INVALID_CERTIFICATE:
            raise HTTPException(status_code=400, detail="Invalid certificate")
        case SequesterCreateServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case SequesterCreateServiceStoreBadOutcome.SEQUESTER_DISABLED:
            raise HTTPException(status_code=400, detail="Sequester disabled")
        case SequesterCreateServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_EXISTS:
            raise HTTPException(status_code=400, detail="Sequester service already exists")
        case RequireGreaterTimestamp() as error:
            raise HTTPException(
                status_code=400,
                detail={
                    "msg": "Require greater timestamp",
                    "strictly_greater_than": error.strictly_greater_than.to_rfc3339(),
                },
            )

    return CreateSequesterServiceOut()


class RevokeSequesterServiceIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    revoked_service_certificate: Base64BytesField


class RevokeSequesterServiceOut(BaseModel):
    pass


@administration_router.post(
    "/administration/organizations/{raw_organization_id}/sequester/services/revoke",
    summary="Revoke a Sequester Service",
    tags=["Sequester"],
)
@log_request
async def administration_organization_sequester_service_revoke(
    raw_organization_id: str,
    body: RevokeSequesterServiceIn,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> RevokeSequesterServiceOut:
    """
    Revoke an existing sequester service.
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.sequester.revoke_service(
        now=DateTime.now(),
        organization_id=organization_id,
        revoked_service_certificate=body.revoked_service_certificate,
    )
    match outcome:
        case SequesterRevokedServiceCertificate():
            pass
        case SequesterRevokeServiceValidateBadOutcome.INVALID_CERTIFICATE:
            raise HTTPException(status_code=400, detail="Invalid certificate")
        case SequesterRevokeServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case SequesterRevokeServiceStoreBadOutcome.SEQUESTER_DISABLED:
            raise HTTPException(status_code=400, detail="Sequester disabled")
        case SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_REVOKED:
            raise HTTPException(status_code=400, detail="Sequester service already revoked")
        case SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Sequester service not found")
        case RequireGreaterTimestamp() as error:
            raise HTTPException(
                status_code=400,
                detail={
                    "msg": "Require greater timestamp",
                    "strictly_greater_than": error.strictly_greater_than.to_rfc3339(),
                },
            )

    return RevokeSequesterServiceOut()


class PutSequesterServiceIn(BaseModel):
    model_config = ConfigDict(strict=True)
    service_id: SequesterServiceIDField
    config: SequesterServiceConfigField


class PutSequesterServiceOut(BaseModel):
    pass


@administration_router.put(
    "/administration/organizations/{raw_organization_id}/sequester/services/config",
    summary="Update a Sequester Service",
    tags=["Sequester"],
)
@log_request
async def administration_organization_sequester_service_update_config(
    raw_organization_id: str,
    body: PutSequesterServiceIn,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> PutSequesterServiceOut:
    """
    Update the sequester service with the specified configuration.
    """
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.sequester.update_config_for_service(
        organization_id=organization_id,
        service_id=body.service_id,
        config=body.config.cooked,
    )
    match outcome:
        case None:
            pass
        case SequesterUpdateConfigForServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_DISABLED:
            raise HTTPException(status_code=400, detail="Sequester disabled")
        case SequesterUpdateConfigForServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Sequester service not found")

    return PutSequesterServiceOut()
