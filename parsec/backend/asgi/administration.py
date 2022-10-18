# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import csv
from io import StringIO
from typing import NoReturn, Tuple, Dict, TYPE_CHECKING
from functools import wraps
from quart import current_app, Response, Blueprint, abort, request, jsonify, make_response, g

from parsec._parsec import DateTime
from parsec.api.protocol.types import UserProfile
from parsec.serde import SerdeValidationError, SerdePackingError
from parsec.api.protocol import OrganizationID
from parsec.api.rest import (
    organization_config_rep_serializer,
    organization_create_req_serializer,
    organization_create_rep_serializer,
    organization_update_req_serializer,
    organization_stats_rep_serializer,
    server_stats_rep_serializer,
)
from parsec.backend.organization import (
    OrganizationAlreadyExistsError,
    OrganizationNotFoundError,
    OrganizationStats,
    generate_bootstrap_token,
)

if TYPE_CHECKING:
    from parsec.backend.app import BackendApp


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
                "realms_count",
                "users_count",
                "admin_count_active",
                "admin_count_revoked",
                "standard_count_active",
                "standard_count_revoked",
                "outsider_count_active",
                "outsider_count_revoked",
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


def administration_authenticated(fn):
    @wraps(fn)
    async def wrapper(*args, **kwargs):
        authorization = request.headers.get("Authorization")
        if authorization != f"Bearer {g.backend.config.administration_token}":
            await json_abort({"error": "not_allowed"}, 403)
        return await fn(*args, **kwargs)

    return wrapper


async def json_abort(data: dict, status: int) -> NoReturn:
    response = await make_response(jsonify(data), status)
    abort(response)


async def load_req_data(req_serializer) -> dict:
    raw = await request.get_data()
    try:
        return req_serializer.loads(raw)
    except SerdeValidationError as exc:
        await json_abort({"error": "bad_data", "reason": exc.errors}, 400)
    except SerdePackingError:
        await json_abort({"error": "bad_data", "reason": "Invalid JSON"}, 400)


def make_rep_response(rep_serializer, data, **kwargs) -> Response:
    return current_app.response_class(
        rep_serializer.dumps(data), content_type=current_app.config["JSONIFY_MIMETYPE"], **kwargs
    )


@administration_bp.route("/administration/organizations", methods=["POST"])
@administration_authenticated
async def administration_create_organizations():
    backend: "BackendApp" = current_app.backend
    data = await load_req_data(organization_create_req_serializer)

    organization_id = data.pop("organization_id")
    bootstrap_token = generate_bootstrap_token()
    try:
        await backend.organization.create(
            id=organization_id, bootstrap_token=bootstrap_token, **data
        )
    except OrganizationAlreadyExistsError:
        await json_abort({"error": "already_exists"}, 400)

    return make_rep_response(
        organization_create_rep_serializer, data={"bootstrap_token": bootstrap_token}, status=200
    )


@administration_bp.route(
    "/administration/organizations/<raw_organization_id>", methods=["GET", "PATCH"]
)
@administration_authenticated
async def administration_organization_item(raw_organization_id: str):
    backend: "BackendApp" = g.backend
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        await json_abort({"error": "not_found"}, 404)
    # Check whether the organization actually exists
    try:
        organization = await backend.organization.get(id=organization_id)
    except OrganizationNotFoundError:
        await json_abort({"error": "not_found"}, 404)

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
            await json_abort({"error": "not_found"}, 404)

        return make_rep_response(organization_config_rep_serializer, data={}, status=200)


@administration_bp.route(
    "/administration/organizations/<raw_organization_id>/stats", methods=["GET"]
)
@administration_authenticated
async def administration_organization_stat(raw_organization_id: str):
    backend: "BackendApp" = g.backend
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        await json_abort({"error": "not_found"}, 404)

    try:
        stats = await backend.organization.stats(id=organization_id)
    except OrganizationNotFoundError:
        await json_abort({"error": "not_found"}, 404)

    return make_rep_response(
        organization_stats_rep_serializer,
        data={
            "status": "ok",
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
async def administration_server_stats():
    backend: "BackendApp" = g.backend

    if "format" not in request.args or request.args["format"] not in {"csv", "json"}:
        return await json_abort(
            {
                "error": "bad_data",
                "reason": f"Missing/invalid mandatory query argument `format` (expected `csv` or `json`)",
            },
            400,
        )

    try:
        raw_from = request.args.get("from")
        from_date = DateTime.from_rfc3339(raw_from) if raw_from else None
        raw_to = request.args.get("to")
        to_date = DateTime.from_rfc3339(raw_to) if raw_to else None
        server_stats = await backend.organization.server_stats(from_date, to_date)
    except ValueError:
        return await json_abort({"error": "bad_data", "reason": "bad timestamp"}, 400)

    if request.args["format"] == "csv":
        return _convert_server_stats_results_as_csv(server_stats)

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
