# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import b64encode

import httpx
import pytest

from parsec._parsec import anonymous_cmds, authenticated_cmds, invited_cmds
from tests.common import CoolorgRpcClients, RpcTransportError


@pytest.mark.parametrize("family", ("anonymous", "authenticated", "invited"))
async def test_unknown_org(family: str, client: httpx.AsyncClient) -> None:
    response = await client.post(
        f"http://parsec.invalid/{family}/CoolOrg",
        content=anonymous_cmds.latest.ping.Req(ping="hello").dump(),
        headers={
            "Content-Type": "application/msgpack",
            "Api-Version": "4.0",
            "Invitation-Token": "6f56a8579fc4425c82a71f9fc8531b77",
            "Authorization": "PARSEC-SIGN-ED25519",
            "Author": "d2FsZG9Ad2hlcmU=",  # spell-checker: disable-line
            "Signature": "NDI=",
        },
    )
    assert response.status_code == 404, response.content


async def test_good_org_authenticated(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.ping(ping="hello")
    assert rep == authenticated_cmds.latest.ping.RepOk(pong="hello")


async def test_good_org_anonymous(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.anonymous.ping(ping="hello")
    assert rep == anonymous_cmds.latest.ping.RepOk(pong="hello")


async def test_good_org_invited(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.invited_zack.ping(ping="hello")
    assert rep == invited_cmds.latest.ping.RepOk(pong="hello")


@pytest.mark.parametrize(
    "kind",
    (
        "api_version_missing",
        "api_version_invalid",
        "api_version_not_supported",
        "organization_not_utf8",
        "organization_unknown",
        "authorization_header_missing",
        "authorization_header_invalid",
        "content_type_header_missing",
        "content_type_header_invalid",
        "author_header_missing",
        "author_header_not_b64",
        "author_header_not_utf8",
        "signature_header_missing",
        "signature_header_invalid",
    ),
)
async def test_authenticated_bad_auth_cmd(coolorg: CoolorgRpcClients, kind: str) -> None:
    client = coolorg.alice
    match kind:
        case "api_version_missing":
            del client.headers["Api-Version"]
            expected_status_code = 422
        case "api_version_invalid":
            client.headers["Api-Version"] = "<dummy>"
            expected_status_code = 422
        case "api_version_not_supported":
            client.headers["Api-Version"] = "3.99"
            expected_status_code = 422
        case "organization_not_utf8":
            client.url += "\xC0"
            expected_status_code = 404
        case "organization_unknown":
            base_url = client.url.rsplit("/", 1)[0]
            client.url = f"{base_url}/DummyOrg"
            expected_status_code = 404
        case "authorization_header_missing":
            del client.headers["Authorization"]
            expected_status_code = 401
        case "authorization_header_invalid":
            client.headers["Authorization"] = "<dummy>"
            expected_status_code = 403
        case "content_type_header_missing":
            del client.headers["Content-Type"]
            expected_status_code = 415
        case "content_type_header_invalid":
            client.headers["Content-Type"] = "<dummy>"
            expected_status_code = 415
        case "author_header_missing":
            del client.headers["Author"]
            expected_status_code = 401
        case "author_header_not_b64":
            client.headers["Author"] = "<dummy>"
            expected_status_code = 403
        case "author_header_not_utf8":
            client.headers["Author"] = b64encode(b"alice\xC0").decode()
            expected_status_code = 403
        case "signature_header_missing":
            vanilla_post = client.raw_client.post

            async def wrapper_post(url, headers, content):
                del headers["Signature"]
                return await vanilla_post(url, headers=headers, content=content)

            client.raw_client.post = wrapper_post  # type: ignore
            expected_status_code = 401
        case "signature_header_invalid":
            vanilla_post = client.raw_client.post

            async def wrapper_post(url, headers, content):
                headers["Signature"] = "<dummy>"
                return await vanilla_post(url, headers=headers, content=content)

            client.raw_client.post = wrapper_post  # type: ignore
            expected_status_code = 403
        case unknown:
            assert False, unknown

    with pytest.raises(RpcTransportError) as exc:
        await client.ping(ping="hello")
    assert exc.value.rep.status_code == expected_status_code


@pytest.mark.parametrize(
    "kind",
    (
        "api_version_missing",
        "api_version_invalid",
        "api_version_not_supported",
        "accept_header_missing",
        "accept_header_invalid",
        "organization_not_utf8",
        "organization_unknown",
        "authorization_header_missing",
        "authorization_header_invalid",
        "author_header_missing",
        "author_header_not_b64",
        "author_header_not_utf8",
        "signature_header_missing",
        "signature_header_invalid",
    ),
)
async def test_authenticated_bad_auth_sse(coolorg: CoolorgRpcClients, kind: str) -> None:
    client = coolorg.alice

    client.url += "/events"
    client.headers["Accept"] = "text/event-stream"
    signature = b64encode(client.signing_key.sign_only_signature(b"")).decode()
    client.headers["Signature"] = signature

    match kind:
        case "api_version_missing":
            del client.headers["Api-Version"]
            expected_status_code = 422
        case "api_version_invalid":
            client.headers["Api-Version"] = "<dummy>"
            expected_status_code = 422
        case "api_version_not_supported":
            client.headers["Api-Version"] = "3.99"
            expected_status_code = 422
        case "accept_header_missing":
            del client.headers["Accept"]
            expected_status_code = 406
        case "accept_header_invalid":
            client.headers["Accept"] = "application/json"
            expected_status_code = 406
        case "organization_not_utf8":
            client.url += "\xC0"
            expected_status_code = 404
        case "organization_unknown":
            base_url = client.url.rsplit("/", 1)[0]
            client.url = f"{base_url}/DummyOrg"
            expected_status_code = 404
        case "authorization_header_missing":
            del client.headers["Authorization"]
            expected_status_code = 401
        case "authorization_header_invalid":
            client.headers["Authorization"] = "<dummy>"
            expected_status_code = 403
        case "author_header_missing":
            del client.headers["Author"]
            expected_status_code = 401
        case "author_header_not_b64":
            client.headers["Author"] = "<dummy>"
            expected_status_code = 403
        case "author_header_not_utf8":
            client.headers["Author"] = b64encode(b"alice\xC0").decode()
            expected_status_code = 403
        case "signature_header_missing":
            del client.headers["Signature"]
            expected_status_code = 401
        case "signature_header_invalid":
            client.headers["Signature"] = "<dummy>"
            expected_status_code = 403
        case unknown:
            assert False, unknown

    rep = await client.raw_client.get(client.url, headers=client.headers)
    assert rep.status_code == expected_status_code


@pytest.mark.parametrize(
    "kind",
    (
        "api_version_missing",
        "api_version_invalid",
        "api_version_not_supported",
        "organization_not_utf8",
        "organization_unknown",
        "invitation_token_header_missing",
        "invitation_token_header_invalid",
        "content_type_header_missing",
        "content_type_header_invalid",
    ),
)
async def test_invited_bad_auth(coolorg: CoolorgRpcClients, kind: str) -> None:
    client = coolorg.invited_zack
    match kind:
        case "api_version_missing":
            del client.headers["Api-Version"]
            expected_status_code = 422
        case "api_version_invalid":
            client.headers["Api-Version"] = "<dummy>"
            expected_status_code = 422
        case "api_version_not_supported":
            client.headers["Api-Version"] = "3.99"
            expected_status_code = 422
        case "organization_not_utf8":
            client.url += "\xC0"
            expected_status_code = 404
        case "organization_unknown":
            base_url = client.url.rsplit("/", 1)[0]
            client.url = f"{base_url}/DummyOrg"
            expected_status_code = 404
        case "invitation_token_header_missing":
            del client.headers["Invitation-Token"]
            expected_status_code = 415
        case "invitation_token_header_invalid":
            client.headers["Invitation-Token"] = "<dummy>"
            expected_status_code = 415
        case "content_type_header_missing":
            del client.headers["Content-Type"]
            expected_status_code = 415
        case "content_type_header_invalid":
            client.headers["Content-Type"] = "<dummy>"
            expected_status_code = 415
        case unknown:
            assert False, unknown

    with pytest.raises(RpcTransportError) as exc:
        await client.ping(ping="hello")
    assert exc.value.rep.status_code == expected_status_code


@pytest.mark.parametrize(
    "kind",
    (
        "api_version_missing",
        "api_version_invalid",
        "api_version_not_supported",
        "organization_not_utf8",
        "organization_unknown",
        "content_type_header_missing",
        "content_type_header_invalid",
    ),
)
async def test_anonymous_bad_auth(coolorg: CoolorgRpcClients, kind: str) -> None:
    client = coolorg.anonymous
    match kind:
        case "api_version_missing":
            del client.headers["Api-Version"]
            expected_status_code = 422
        case "api_version_invalid":
            client.headers["Api-Version"] = "<dummy>"
            expected_status_code = 422
        case "api_version_not_supported":
            client.headers["Api-Version"] = "3.99"
            expected_status_code = 422
        case "organization_not_utf8":
            client.url += "\xC0"
            expected_status_code = 404
        case "organization_unknown":
            base_url = client.url.rsplit("/", 1)[0]
            client.url = f"{base_url}/DummyOrg"
            expected_status_code = 404
        case "content_type_header_missing":
            del client.headers["Content-Type"]
            expected_status_code = 415
        case "content_type_header_invalid":
            client.headers["Content-Type"] = "<dummy>"
            expected_status_code = 415
        case unknown:
            assert False, unknown

    with pytest.raises(RpcTransportError) as exc:
        await client.ping(ping="hello")
    assert exc.value.rep.status_code == expected_status_code
