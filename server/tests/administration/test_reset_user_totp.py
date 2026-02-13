# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Any

import httpx
import pytest

from parsec._parsec import ParsecTOTPResetAddr
from parsec.components.email import SendEmailBadOutcome
from tests.common import (
    AdminUnauthErrorsTester,
    Backend,
    CoolorgRpcClients,
    LetterBox,
    MinimalorgRpcClients,
    alice_gives_profile,
)


async def test_bad_auth(
    minimalorg: MinimalorgRpcClients,
    administration_route_unauth_errors_tester: AdminUnauthErrorsTester,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    async def do(client: httpx.AsyncClient):
        return await client.post(url)

    await administration_route_unauth_errors_tester(do)


@pytest.mark.parametrize(
    "kind",
    (
        "invalid_json",
        "bad_type_user_id",
        "bad_value_user_id",
        "bad_type_user_email",
        "bad_type_send_email",
    ),
)
async def test_bad_data(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
    kind: str,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    body_args: dict[str, Any]
    match kind:
        case "invalid_json":
            body_args = {"content": b"<dummy>"}
        case "bad_type_user_id":
            body_args = {"json": {"user_id": 42}}
        case "bad_value_user_id":
            body_args = {"json": {"user_id": "<>"}}
        case "bad_type_user_email":
            body_args = {"json": {"user_email": 42}}
        case "bad_type_send_email":
            body_args = {"json": {"user_id": minimalorg.alice.user_id.hex, "send_email": "yes"}}
        case unknown:
            assert False, unknown

    response = await administration_client.post(
        url,
        **body_args,
    )
    assert response.status_code == 422, response.content


async def test_bad_method(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    response = await administration_client.patch(
        url,
        json={"user_id": minimalorg.alice.user_id.hex},
    )
    assert response.status_code == 405, response.content


async def test_ok_by_user_id(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
    email_totp_reset_letterbox: LetterBox,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    response = await administration_client.post(
        url,
        json={"user_id": minimalorg.alice.user_id.hex},
    )
    assert response.status_code == 200, response.content
    result = response.json()
    assert result["user_id"] == minimalorg.alice.user_id.hex
    assert result["user_email"] == minimalorg.alice.human_handle.email.str
    totp_reset_url = ParsecTOTPResetAddr.from_url(result["totp_reset_url"])
    assert totp_reset_url.user_id == minimalorg.alice.user_id
    assert result["totp_reset_url_as_http_redirection"].startswith("http")
    assert (
        ParsecTOTPResetAddr.from_url(
            result["totp_reset_url_as_http_redirection"], allow_http_redirection=True
        )
        == totp_reset_url
    )
    assert result["email_sent_status"] == "NOT_SENT_AS_REQUESTED"
    assert email_totp_reset_letterbox.count() == 0


async def test_ok_by_user_email(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
    email_totp_reset_letterbox: LetterBox,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    response = await administration_client.post(
        url,
        json={"user_email": str(minimalorg.alice.human_handle.email), "send_email": False},
    )
    assert response.status_code == 200, response.content
    result = response.json()
    assert result["user_id"] == minimalorg.alice.user_id.hex
    assert result["user_email"] == minimalorg.alice.human_handle.email.str
    totp_reset_url = ParsecTOTPResetAddr.from_url(result["totp_reset_url"])
    assert totp_reset_url.user_id == minimalorg.alice.user_id
    assert result["totp_reset_url_as_http_redirection"].startswith("http")
    assert (
        ParsecTOTPResetAddr.from_url(
            result["totp_reset_url_as_http_redirection"], allow_http_redirection=True
        )
        == totp_reset_url
    )
    assert result["email_sent_status"] == "NOT_SENT_AS_REQUESTED"
    assert email_totp_reset_letterbox.count() == 0


async def test_ok_send_email(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
    email_totp_reset_letterbox: LetterBox,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    response = await administration_client.post(
        url,
        json={"user_id": minimalorg.alice.user_id.hex, "send_email": True},
    )
    assert response.status_code == 200, response.content
    result = response.json()
    assert result["email_sent_status"] == "SENT_AS_REQUESTED"
    assert email_totp_reset_letterbox.count() == 1

    # Check that the email was sent
    to_addr, message = await email_totp_reset_letterbox.get_next()
    assert to_addr == minimalorg.alice.human_handle.email
    message_as_str = message.as_string()
    assert f"Your MFA has been reset for {minimalorg.organization_id}" in message_as_str
    assert result["totp_reset_url_as_http_redirection"] in message_as_str


async def test_both_user_id_and_email(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    response = await administration_client.post(
        url,
        json={
            "user_id": minimalorg.alice.user_id.hex,
            "user_email": str(minimalorg.alice.human_handle.email),
        },
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Both `user_id` and `user_email` fields are provided"}


async def test_no_user_id_nor_email(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    response = await administration_client.post(
        url,
        json={},
    )
    assert response.status_code == 400, response.content
    assert response.json() == {"detail": "Missing either `user_id` or `user_email` field"}


async def test_unknown_organization(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = "http://parsec.invalid/administration/organizations/DummyOrg/users/reset_totp"
    response = await administration_client.post(
        url,
        json={"user_id": minimalorg.alice.user_id.hex},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "Organization not found"}


async def test_unknown_user_id(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"
    response = await administration_client.post(
        url,
        json={"user_id": "d51589e233c0451e9d2fa1c7b9a8b08b"},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User not found"}


async def test_unknown_email(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
) -> None:
    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"
    response = await administration_client.post(
        url,
        json={"user_email": "dummy@example.invalid"},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User not found"}


async def test_revoked_user(
    administration_client: httpx.AsyncClient,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    await alice_gives_profile(coolorg, backend, recipient=coolorg.bob.user_id, new_profile=None)
    url = f"http://parsec.invalid/administration/organizations/{coolorg.organization_id.str}/users/reset_totp"

    # Access by user ID
    response = await administration_client.post(
        url,
        json={"user_id": coolorg.bob.user_id.hex},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User has been revoked"}

    # Access by email
    response = await administration_client.post(
        url,
        json={"user_email": coolorg.bob.human_handle.email.str},
    )
    assert response.status_code == 404, response.content
    assert response.json() == {"detail": "User not found"}


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.RECIPIENT_REFUSED,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_ok_send_email_bad_outcome(
    administration_client: httpx.AsyncClient,
    minimalorg: MinimalorgRpcClients,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.totp.send_email", _mocked_send_email)

    url = f"http://parsec.invalid/administration/organizations/{minimalorg.organization_id.str}/users/reset_totp"

    response = await administration_client.post(
        url,
        json={"user_id": minimalorg.alice.user_id.hex, "send_email": True},
    )
    assert response.status_code == 200, response.content
    result = response.json()

    match bad_outcome:
        case SendEmailBadOutcome.BAD_SMTP_CONFIG:
            expected_email_sent_status = "BAD_SMTP_CONFIG"
        case SendEmailBadOutcome.RECIPIENT_REFUSED:
            expected_email_sent_status = "RECIPIENT_REFUSED"
        case SendEmailBadOutcome.SERVER_UNAVAILABLE:
            expected_email_sent_status = "SERVER_UNAVAILABLE"

    assert result["email_sent_status"] == expected_email_sent_status
