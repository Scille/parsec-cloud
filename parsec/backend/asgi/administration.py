# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import re
import csv
from io import StringIO
from typing import NoReturn, TYPE_CHECKING
from functools import wraps
from quart import current_app, Response, Blueprint, abort, request, jsonify, make_response, g

from parsec._parsec import DateTime
from parsec.serde import SerdeValidationError, SerdePackingError
from parsec.api.protocol import OrganizationID
from parsec.api.rest import (
    organization_config_rep_serializer,
    organization_create_req_serializer,
    organization_create_rep_serializer,
    organization_update_req_serializer,
    organization_stats_rep_serializer,
)
from parsec.backend.organization import (
    OrganizationAlreadyExistsError,
    OrganizationNotFoundError,
    generate_bootstrap_token,
)

if TYPE_CHECKING:
    from parsec.backend.app import BackendApp


administration_bp = Blueprint("administration_api", __name__)


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

    def _date_from_str(input: str) -> DateTime:
        regex = re.compile(r"(\d{4})[-/](\d{1,2})[-/](\d{1,2})")
        matched = regex.match(input)

        if not matched:
            raise ValueError(f"Provided date is not a valid timestamp {input}")

        return DateTime(
            int(matched.group(1)),
            int(matched.group(2)),
            int(matched.group(3)),
            hour=0,
            minute=0,
            second=0,
        )

    for arg in ["format", "from"]:
        if arg not in request.args:
            return await json_abort({"error": f"missing query argument '{arg}'"}, 400)

    if request.args["format"] not in {"csv", "json"}:
        return await json_abort(
            {"error": f"bad format '{request.args['format']}' expected one of ['json', 'csv']"}, 400
        )

    try:
        from_date = _date_from_str(request.args["from"])
        to_date = _date_from_str(request.args["to"]) if "to" in request.args else DateTime.now()
        assert from_date < to_date
        results = await backend.organization.server_stats(from_date, to_date)
    except ValueError:
        return await json_abort({"error": "bad timestamp"}, 400)
    except AssertionError:
        return await json_abort({"error": f"{from_date} to {to_date} is not a valid range"}, 400)

    if request.args["format"] == "csv":
        print(results)
        with StringIO(newline="") as memory_file:
            writer = csv.writer(memory_file)
            # Header
            writer.writerow(
                [
                    "id",
                    "data_size",
                    "metadata_size",
                    "realms",
                    "users",
                    "admin_count_active",
                    "standard_count_active",
                    "outsider_count_active",
                    "admin_count_revoked",
                    "standard_count_revoked",
                    "outsider_count_revoked",
                ]
            )

            for row in results:
                user_values = list()
                for value in row["user_per_profiles"].values():
                    user_values.extend(value.values())
                row.pop("user_per_profiles")
                writer.writerow(
                    [row["id"], row["data_size"], row["metadata_size"], row["realms"], row["users"]]
                    + user_values
                )
            return memory_file.getvalue()

    return {"stats": results}
