# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import csv
from enum import Enum
from io import StringIO
from typing import (
    TYPE_CHECKING,
    Annotated,
    Any,
    Literal,
    assert_never,
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
    UserProfile,
)
from parsec.components.organization import (
    Organization,
    OrganizationCreateBadOutcome,
    OrganizationGetBadOutcome,
    OrganizationStats,
    OrganizationStatsBadOutcome,
    OrganizationUpdateBadOutcome,
    Unset,
)
from parsec.events import OrganizationIDField

if TYPE_CHECKING:
    from parsec.backend import Backend


administration_router = APIRouter()
security = HTTPBearer()


def check_administration_auth(
    request: Request, credentials: Annotated[HTTPAuthorizationCredentials, Depends(security)]
) -> None:
    if request.app.state.backend.config.administration_token != credentials.credentials:
        raise HTTPException(status_code=403, detail="Bad authorization token")


class CreateOrganizationIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    organization_id: OrganizationIDField
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: ask the backend to use it default value for this field
    # - field set to `None`: `None` is a valid value to use for this field
    user_profile_outsider_allowed: bool | Literal[Unset] = Unset
    active_users_limit: ActiveUsersLimit | Literal[Unset] = Unset

    @field_validator("active_users_limit", mode="plain")
    @classmethod
    def validate_active_users_limit(cls, v: Any) -> ActiveUsersLimit | Literal[Unset]:
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
    bootstrap_token: str


@administration_router.post("/administration/organizations")
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
    )
    match outcome:
        case BootstrapToken() as bootstrap_token:
            pass
        case OrganizationCreateBadOutcome.ORGANIZATION_ALREADY_EXISTS:
            raise HTTPException(
                status_code=400,
                detail="Organization already exists",
            )
        case unknown:
            assert_never(unknown)

    return CreateOrganizationOut(bootstrap_token=bootstrap_token.hex)


class GetOrganizationOut(BaseModel):
    is_bootstrapped: bool
    is_expired: bool
    user_profile_outsider_allowed: bool
    active_users_limit: int | None


@administration_router.get("/administration/organizations/{raw_organization_id}")
async def administration_get_organization(
    raw_organization_id: str,
    request: Request,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> GetOrganizationOut:
    backend: Backend = request.app.state.backend

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        raise HTTPException(
            status_code=404,
            detail="Invalid organization ID",
        )

    # Check whether the organization actually exists
    outcome = await backend.organization.get(id=organization_id)
    match outcome:
        case Organization() as organization:
            pass
        case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case unknown:
            assert_never(unknown)

    return GetOrganizationOut(
        is_bootstrapped=organization.is_bootstrapped,
        is_expired=organization.is_expired,
        user_profile_outsider_allowed=organization.user_profile_outsider_allowed,
        active_users_limit=organization.active_users_limit.to_maybe_int(),
    )


class PatchOrganizationOut(BaseModel):
    pass


class PatchOrganizationIn(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)
    is_expired: bool | Literal[Unset] = Unset
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: ask the backend to use it default value for this field
    # - field set to `None`: `None` is a valid value to use for this field
    user_profile_outsider_allowed: bool | Literal[Unset] = Unset
    active_users_limit: ActiveUsersLimit | Literal[Unset] = Unset

    @field_validator("active_users_limit", mode="plain")
    @classmethod
    def validate_active_users_limit(cls, v: Any) -> ActiveUsersLimit | Literal[Unset]:
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
async def administration_patch_organization(
    raw_organization_id: str,
    body: PatchOrganizationIn,
    request: Request,
    auth: Annotated[None, Depends(check_administration_auth)],
) -> PatchOrganizationOut:
    backend: Backend = request.app.state.backend

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        raise HTTPException(
            status_code=404,
            detail="Invalid organization ID",
        )

    outcome = await backend.organization.update(
        id=organization_id,
        is_expired=body.is_expired,
        active_users_limit=body.active_users_limit,
        user_profile_outsider_allowed=body.user_profile_outsider_allowed,
    )
    match outcome:
        case None:
            pass
        case OrganizationUpdateBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case unknown:
            assert_never(unknown)

    return PatchOrganizationOut()


@administration_router.get("/administration/organizations/{raw_organization_id}/stats")
async def administration_organization_stat(
    raw_organization_id: str,
    auth: Annotated[None, Depends(check_administration_auth)],
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        raise HTTPException(
            status_code=404,
            detail="Invalid organization ID",
        )

    outcome = await backend.organization.organization_stats(organization_id)
    match outcome:
        case OrganizationStats() as stats:
            pass
        case OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND:
            raise HTTPException(status_code=404, detail="Organization not found")
        case unknown:
            assert_never(unknown)

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
async def administration_server_stats(
    request: Request,
    response: Response,
    auth: Annotated[None, Depends(check_administration_auth)],
    format: StatsFormat = StatsFormat.JSON,
    raw_at: str | None = None,
) -> Response:
    backend: Backend = request.app.state.backend

    try:
        at = DateTime.from_rfc3339(raw_at) if raw_at else None
    except ValueError:
        raise HTTPException(
            status_code=400,
            detail="Invalid `at` query argument (expected RFC3339 datetime)",
        )

    server_stats = await backend.organization.server_stats(at=at)

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

        case unknown:
            assert_never(unknown)
