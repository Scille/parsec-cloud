# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Test the editics routes' API version negotiation and HTTP error statuses
(see `parsec/asgi/editics.py`).
"""

from __future__ import annotations

from uuid import uuid4

import pytest

from parsec._parsec import VlobID
from parsec.editics_protocol import EDITICS_API_VERSION, EditicsClientEventAuth
from tests.common import Backend, CoolorgRpcClients, EditicsSessionClient

API_VERSION = str(EDITICS_API_VERSION)
AUTH_EVENT = EditicsClientEventAuth(vlobVersion=1, reconnect=None, mode="view")


@pytest.fixture
def send_url_and_authorization(backend: Backend, coolorg: CoolorgRpcClients) -> tuple[str, str]:
    _listen_url, send_url, authorization = EditicsSessionClient.build_urls_and_authorization(
        coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
        participant_id=uuid4().hex,
    )
    return send_url, authorization


async def _post_send_event(
    alice: CoolorgRpcClients,
    send_url: str,
    authorization: str,
    api_version: str | None,
    **extra_headers: str,
):
    headers = {
        "Authorization": authorization,
        "Content-Type": "application/json",
        **extra_headers,
    }
    if api_version is not None:
        headers["Api-Version"] = api_version
    return await alice.raw_client.post(
        send_url, headers=headers, content=AUTH_EVENT.model_dump_json()
    )


@pytest.mark.parametrize("bad_api_version", [None, "not a version", "2.0", "1"])
async def test_unsupported_api_version(
    coolorg: CoolorgRpcClients, send_url_and_authorization: tuple[str, str], bad_api_version: str
) -> None:
    send_url, authorization = send_url_and_authorization

    rep = await _post_send_event(coolorg.alice, send_url, authorization, bad_api_version)
    assert rep.status_code == 422
    assert rep.headers["Supported-Api-Versions"] == API_VERSION
    # No version could be settled, hence no `Api-Version` reply header
    assert "Api-Version" not in rep.headers


async def test_compatible_api_version(
    coolorg: CoolorgRpcClients, send_url_and_authorization: tuple[str, str]
) -> None:
    send_url, authorization = send_url_and_authorization

    # Only the major version breaks compatibility, so a client using a more
    # recent revision is accepted: the server replies with its own version.
    rep = await _post_send_event(coolorg.alice, send_url, authorization, "1.42")
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == API_VERSION


async def test_missing_authorization(
    coolorg: CoolorgRpcClients, send_url_and_authorization: tuple[str, str]
) -> None:
    send_url, _authorization = send_url_and_authorization

    rep = await _post_send_event(
        coolorg.alice, send_url, authorization="Bearer bad", api_version="1.0"
    )
    assert rep.status_code == 401
    assert rep.headers["Api-Version"] == API_VERSION
    assert rep.json()["detail"] == {
        "code": "missing_authentication_info",
        "detail": "missing authentication info",
    }


async def test_realm_not_found_is_indistinguishable_from_bad_realm_id(
    coolorg: CoolorgRpcClients, send_url_and_authorization: tuple[str, str]
) -> None:
    send_url, authorization = send_url_and_authorization

    outcomes = []
    for raw_realm_id in ("not-a-vlob-id", VlobID.new().hex):
        send_url = send_url.replace(coolorg.wksp1_id.hex, raw_realm_id)
        rep = await _post_send_event(coolorg.alice, send_url, authorization, "1.0")
        outcomes.append((rep.status_code, rep.json()))

    # Information hiding: an organization member must not learn whether a realm
    # it has no access to exists or is deleted (see `EditicsHttpStatus.NotFound`)
    assert outcomes[0] == outcomes[1]
    assert outcomes[0][0] == 404
    assert outcomes[0][1]["detail"]["code"] == "not_found"


async def test_bad_content_type(
    coolorg: CoolorgRpcClients, send_url_and_authorization: tuple[str, str]
) -> None:
    send_url, authorization = send_url_and_authorization

    rep = await _post_send_event(
        coolorg.alice, send_url, authorization, "1.0", **{"Content-Type": "text/plain"}
    )
    assert rep.status_code == 415
    assert rep.json()["detail"]["code"] == "bad_content_type"
