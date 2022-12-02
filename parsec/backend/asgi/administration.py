# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import csv
from functools import wraps
from io import StringIO
from typing import TYPE_CHECKING, Any, Awaitable, Callable, Dict, NoReturn, Tuple, TypeVar

from quart import Blueprint, Response, current_app, g, jsonify, make_response, request
from typing_extensions import ParamSpec

from parsec._parsec import DateTime
from parsec.api.protocol import OrganizationID, UserProfile
from parsec.api.rest import (
    organization_config_rep_serializer,
    organization_create_rep_serializer,
    organization_create_req_serializer,
    organization_stats_rep_serializer,
    organization_update_req_serializer,
    server_stats_rep_serializer,
)
from parsec.backend.organization import (
    OrganizationAlreadyExistsError,
    OrganizationNotFoundError,
    OrganizationStats,
    generate_bootstrap_token,
)
from parsec.serde import SerdePackingError, SerdeValidationError
from parsec.serde.serializer import JSONSerializer

if TYPE_CHECKING:
    from parsec.backend.app import BackendApp

T = TypeVar("T")
P = ParamSpec("P")

CONTENT_TYPE_JSON = "application/json"


administration_bp = Blueprint("administration_api", __name__)


def _convert_server_stats_results_as_csv(stats: Dict[OrganizationID, OrganizationStats]) -> str:
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

        def _find_profile_counts(profile: UserProfile) -> Tuple[int, int]:
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


def administration_authenticated(fn: Callable[P, Awaitable[T]]) -> Callable[P, Awaitable[T]]:
    @wraps(fn)
    async def wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
        authorization = request.headers.get("Authorization")
        if authorization != f"Bearer {g.backend.config.administration_token}":
            await not_allowed_abort()
        return await fn(*args, **kwargs)

    return wrapper


async def not_allowed_abort() -> NoReturn:
    response = await make_response(jsonify({"error": "not_allowed"}), 403)
    current_app.aborter(response)


async def not_found_abort() -> NoReturn:
    response = await make_response(jsonify({"error": "not_found"}), 404)
    current_app.aborter(response)


async def bad_data_abort(reason: str) -> NoReturn:
    response = await make_response(jsonify({"error": "bad_data", "reason": reason}), 400)
    current_app.aborter(response)


async def already_exists_abort() -> NoReturn:
    response = await make_response(jsonify({"error": "already_exists"}), 400)
    current_app.aborter(response)


async def load_req_data(req_serializer: JSONSerializer) -> dict[str, Any]:
    raw = await request.get_data()
    try:
        return req_serializer.loads(raw)  # type: ignore[arg-type]
    except SerdeValidationError as exc:
        await bad_data_abort(reason=str(exc.errors))
    except SerdePackingError:
        await bad_data_abort(reason="Invalid JSON")


def make_rep_response(
    rep_serializer: JSONSerializer, data: dict[str, Any], **kwargs: Any
) -> Response:
    return current_app.response_class(
        rep_serializer.dumps(data), content_type=CONTENT_TYPE_JSON, **kwargs
    )


@administration_bp.route("/administration/organizations", methods=["POST"])
@administration_authenticated
async def administration_create_organizations() -> Response:
    backend: "BackendApp" = current_app.backend  # type: ignore[attr-defined]
    data = await load_req_data(organization_create_req_serializer)

    organization_id = data.pop("organization_id")
    bootstrap_token = generate_bootstrap_token()
    try:
        await backend.organization.create(
            id=organization_id, bootstrap_token=bootstrap_token, created_on=DateTime.now(), **data
        )
    except OrganizationAlreadyExistsError:
        await already_exists_abort()

    return make_rep_response(
        organization_create_rep_serializer, data={"bootstrap_token": bootstrap_token}, status=200
    )


@administration_bp.route(
    "/administration/organizations/<raw_organization_id>", methods=["GET", "PATCH"]
)
@administration_authenticated
async def administration_organization_item(raw_organization_id: str) -> Response:
    backend: "BackendApp" = g.backend
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        await not_found_abort()
    # Check whether the organization actually exists
    try:
        organization = await backend.organization.get(id=organization_id)
    except OrganizationNotFoundError:
        await not_found_abort()

    if request.method == "GET":

        return make_rep_response(
            organization_config_rep_serializer,
            data={
                "is_bootstrapped": organization.is_bootstrapped(),
                "is_expired": organization.is_expired,
                "user_profile_outsider_allowed": organization.user_profile_outsider_allowed,
                "active_users_limit": organization.active_users_limit,
            },
            status=200,
        )

    else:
        assert request.method == "PATCH"

        data = await load_req_data(organization_update_req_serializer)

        try:
            await backend.organization.update(id=organization_id, **data)
        except OrganizationNotFoundError:
            await not_found_abort()

        return make_rep_response(organization_config_rep_serializer, data={}, status=200)


@administration_bp.route(
    "/administration/organizations/<raw_organization_id>/stats", methods=["GET"]
)
@administration_authenticated
async def administration_organization_stat(raw_organization_id: str) -> Response:
    backend: "BackendApp" = g.backend
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        await not_found_abort()

    try:
        stats = await backend.organization.stats(id=organization_id)
    except OrganizationNotFoundError:
        await not_found_abort()

    return make_rep_response(
        organization_stats_rep_serializer,
        data={
            "realms": stats.realms,
            "data_size": stats.data_size,
            "metadata_size": stats.metadata_size,
            "users": stats.users,
            "active_users": stats.active_users,
            "users_per_profile_detail": stats.users_per_profile_detail,
        },
        status=200,
    )


@administration_bp.route("/administration/stats", methods=["GET"])
@administration_authenticated
async def administration_server_stats() -> Response:
    backend: "BackendApp" = g.backend

    if request.args.get("format") not in ("csv", "json"):
        return await bad_data_abort(
            reason=f"Missing/invalid mandatory query argument `format` (expected `csv` or `json`)",
        )

    try:
        raw_at = request.args.get("at")
        at = DateTime.from_rfc3339(raw_at) if raw_at else None
        server_stats = await backend.organization.server_stats(at=at)
    except ValueError:
        return await bad_data_abort(
            reason="Invalid `at` query argument (expected RFC3339 datetime)",
        )

    if request.args["format"] == "csv":
        csv_data = _convert_server_stats_results_as_csv(server_stats)
        return current_app.response_class(
            csv_data,
            content_type="text/csv",
            status=200,
        )

    else:
        return make_rep_response(
            server_stats_rep_serializer,
            data={
                "stats": [
                    {
                        "organization_id": organization_id.str,
                        "data_size": org_stats.data_size,
                        "metadata_size": org_stats.metadata_size,
                        "realms": org_stats.realms,
                        "active_users": org_stats.active_users,
                        "users_per_profile_detail": org_stats.users_per_profile_detail,
                    }
                    for organization_id, org_stats in server_stats.items()
                ]
            },
            status=200,
        )
