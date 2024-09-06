# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import csv
from enum import Enum
from functools import wraps
from io import StringIO
from typing import (
    TYPE_CHECKING,
    Annotated,
    Any,
    Awaitable,
    Callable,
    Literal,
    cast,
)

from fastapi import APIRouter, Depends, HTTPException, Request, Response
from fastapi.responses import JSONResponse
from fastapi.security import HTTPAuthorizationCredentials, HTTPBearer
from pydantic import BaseModel, ConfigDict, field_validator

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    DateTime,
    OrganizationID,
    ParsecOrganizationBootstrapAddr,
    UserID,
    UserProfile,
)
from parsec.components.organization import (
    Organization,
    OrganizationCreateBadOutcome,
    OrganizationGetBadOutcome,
    OrganizationStats,
    OrganizationStatsBadOutcome,
    OrganizationUpdateBadOutcome,
)
from parsec.components.user import UserFreezeUserBadOutcome, UserInfo, UserListUsersBadOutcome
from parsec.events import OrganizationIDField
from parsec.logging import get_logger
from parsec.types import Unset, UnsetType

if TYPE_CHECKING:
    from parsec.backend import Backend

logger = get_logger()

administration_router = APIRouter()
security = HTTPBearer()


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


class CreateOrganizationIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    organization_id: OrganizationIDField
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: ask the server to use its default value for this field
    # - field set to `None`: `None` is a valid value to use for this field
    user_profile_outsider_allowed: bool | Literal[UnsetType.Unset] = Unset
    active_users_limit: ActiveUsersLimit | Literal[UnsetType.Unset] = Unset
    minimum_archiving_period: int | Literal[UnsetType.Unset] = Unset

    @field_validator("active_users_limit", mode="plain")
    @classmethod
    def validate_active_users_limit(cls, v: Any) -> ActiveUsersLimit | Literal[UnsetType.Unset]:
        match v:
            case ActiveUsersLimit():
                return v
            case v if v is Unset:
                return v
            case None:
                return ActiveUsersLimit.NO_LIMIT
            case int() if v >= 0:
                return ActiveUsersLimit.limited_to(v)
            case _:
                raise ValueError("Expected null or positive integer")


class CreateOrganizationOut(BaseModel):
    bootstrap_url: str


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
            debug_extra = {
                "status_code": result.status_code,
                "body": result.body.decode("utf-8"),
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


@administration_router.post("/administration/organizations")
@log_request
async def administration_create_organizations(
    request: Request,
    body: CreateOrganizationIn,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> CreateOrganizationOut:
    backend: Backend = request.app.state.backend

    outcome = await backend.organization.create(
        now=DateTime.now(),
        id=body.organization_id,
        user_profile_outsider_allowed=body.user_profile_outsider_allowed,
        active_users_limit=body.active_users_limit,
        minimum_archiving_period=body.minimum_archiving_period,
    )
    match outcome:
        case BootstrapToken() as bootstrap_token:
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
    return CreateOrganizationOut(bootstrap_url=bootstrap_url.to_url())


class GetOrganizationOut(BaseModel):
    is_bootstrapped: bool
    is_expired: bool
    user_profile_outsider_allowed: bool
    active_users_limit: int | None
    minimum_archiving_period: int


@administration_router.get("/administration/organizations/{raw_organization_id}")
@log_request
async def administration_get_organization(
    raw_organization_id: str,
    request: Request,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> GetOrganizationOut:
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
        minimum_archiving_period=organization.minimum_archiving_period,
    )


class PatchOrganizationOut(BaseModel):
    pass


class PatchOrganizationIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    is_expired: bool | Literal[UnsetType.Unset] = Unset
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: ask the server to use its default value for this field
    # - field set to `None`: `None` is a valid value to use for this field
    user_profile_outsider_allowed: bool | Literal[UnsetType.Unset] = Unset
    active_users_limit: ActiveUsersLimit | Literal[UnsetType.Unset] = Unset
    minimum_archiving_period: Literal[UnsetType.Unset] | int = Unset

    @field_validator("active_users_limit", mode="plain")
    @classmethod
    def validate_active_users_limit(cls, v: Any) -> ActiveUsersLimit | Literal[UnsetType.Unset]:
        match v:
            case ActiveUsersLimit():
                return v
            case v if v is Unset:
                return v
            case None:
                return ActiveUsersLimit.NO_LIMIT
            case int() if v >= 0:
                return ActiveUsersLimit.limited_to(v)
            case _:
                raise ValueError("Expected null or positive integer")


@administration_router.patch("/administration/organizations/{raw_organization_id}")
@log_request
async def administration_patch_organization(
    raw_organization_id: str,
    body: PatchOrganizationIn,
    request: Request,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> PatchOrganizationOut:
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.organization.update(
        id=organization_id,
        is_expired=body.is_expired,
        active_users_limit=body.active_users_limit,
        user_profile_outsider_allowed=body.user_profile_outsider_allowed,
        minimum_archiving_period=body.minimum_archiving_period,
    )
    match outcome:
        case None:
            pass
        case OrganizationUpdateBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")

    return PatchOrganizationOut()


@administration_router.get("/administration/organizations/{raw_organization_id}/stats")
@log_request
async def administration_organization_stat(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.organization.organization_stats(organization_id)
    match outcome:
        case OrganizationStats() as stats:
            pass
        case OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")

    return JSONResponse(
        status_code=200,
        content={
            "realms": stats.realms,
            "data_size": stats.data_size,
            "metadata_size": stats.metadata_size,
            "users": stats.users,
            "active_users": stats.active_users,
            "users_per_profile_detail": {
                detail.profile.str: {
                    "active": detail.active,
                    "revoked": detail.revoked,
                }
                for detail in stats.users_per_profile_detail
            },
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


class StatsFormat(str, Enum):
    CSV = "csv"
    JSON = "json"


@administration_router.get("/administration/stats")
@log_request
async def administration_server_stats(
    request: Request,
    response: Response,
    auth: Annotated[None, Depends(check_administration_auth)],
    format: StatsFormat = StatsFormat.JSON,
    at: str | None = None,
) -> Response:
    backend: Backend = request.app.state.backend

    try:
        typed_at = DateTime.from_rfc3339(at) if at else None
    except ValueError:
        raise HTTPException(
            status_code=400,
            detail="Invalid `at` query argument (expected RFC3339 datetime)",
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
            return JSONResponse(
                status_code=200,
                content={
                    "stats": [
                        {
                            "organization_id": organization_id.str,
                            "data_size": org_stats.data_size,
                            "metadata_size": org_stats.metadata_size,
                            "realms": org_stats.realms,
                            "users": org_stats.users,
                            "active_users": org_stats.active_users,
                            "users_per_profile_detail": {
                                detail.profile.str: {
                                    "active": detail.active,
                                    "revoked": detail.revoked,
                                }
                                for detail in org_stats.users_per_profile_detail
                            },
                        }
                        for organization_id, org_stats in server_stats.items()
                    ]
                },
            )


@administration_router.get("/administration/organizations/{raw_organization_id}/users")
@log_request
async def administration_organization_users(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    organization_id = parse_organization_id_or_die(raw_organization_id)

    outcome = await backend.user.list_users(organization_id)
    match outcome:
        case list() as users:
            pass
        case UserListUsersBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")

    return JSONResponse(
        status_code=200,
        content={
            "users": [
                {
                    "user_id": user.user_id.hex,
                    "user_email": user.human_handle.email,
                    "user_name": user.human_handle.label,
                    "frozen": user.frozen,
                }
                for user in users
            ]
        },
    )


class UserFreezeIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    frozen: bool
    user_email: str | None = None
    user_id: UserID | None = None

    @field_validator("user_id", mode="plain")
    @classmethod
    def validate_user_id(cls, v: Any) -> UserID | None:
        match v:
            case UserID():
                return v
            case None:
                return None
            case raw:
                return UserID.from_hex(raw)


@administration_router.post("/administration/organizations/{raw_organization_id}/users/freeze")
@log_request
async def administration_organization_users_freeze(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    body: UserFreezeIn,
    request: Request,
) -> Response:
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
        case UserFreezeUserBadOutcome.BOTH_USER_ID_AND_EMAIL:
            raise HTTPException(
                status_code=400, detail="Both `user_id` and `user_email` fields are provided"
            )
        case UserFreezeUserBadOutcome.NO_USER_ID_NOR_EMAIL:
            raise HTTPException(
                status_code=400, detail="Missing either `user_id` or `user_email` field"
            )

    return JSONResponse(
        status_code=200,
        content={
            "user_id": user.user_id.hex,
            "user_email": user.human_handle.email,
            "user_name": user.human_handle.label,
            "frozen": user.frozen,
        },
    )
