# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import urlsafe_b64encode

import httpx
import pytest

from parsec._parsec import DateTime, anonymous_cmds, authenticated_cmds, invited_cmds
from parsec.ballpark import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET
from parsec.components.auth import AuthenticatedToken
from tests.common import CoolorgRpcClients, RpcTransportError


@pytest.mark.parametrize("family", ("anonymous", "authenticated", "invited"))
async def test_unknown_org(family: str, client: httpx.AsyncClient) -> None:
    headers = {
        "Content-Type": "application/msgpack",
        "Api-Version": "4.0",
    }

    match family:
        case "anonymous":
            pass
        case "authenticated":
            headers["Authorization"] = (
                "Bearer PARSEC-SIGN-ED25519.YWxpY2VAZGV2MQ==.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
        case "invited":
            headers["Authorization"] = "Bearer 6f56a8579fc4425c82a71f9fc8531b77"
        case unknown:
            assert False, unknown

    response = await client.post(
        f"http://parsec.invalid/{family}/CoolOrg",
        content=anonymous_cmds.latest.ping.Req(ping="hello").dump(),
        headers=headers,
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
        "content_type_header_missing",
        "content_type_header_invalid",
        "authorization_header_missing",
        "authorization_header_invalid",
        "authorization_header_invalid_method",
        "authorization_header_invalid_token",
        "authorization_header_author_unknown",
        "authorization_header_author_not_b64",
        "authorization_header_author_not_utf8",
        "authorization_header_bad_timestamp",
        "authorization_header_signature_not_b64",
        "authorization_header_bad_signature",
        "authorization_header_timestamp_too_late",
        "authorization_header_timestamp_too_early",
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
            client.url += "\xc0"
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
        case "authorization_header_missing":
            vanilla_post = client.raw_client.post

            async def wrapper_post(url, headers, content):
                del headers["Authorization"]
                return await vanilla_post(url, headers=headers, content=content)

            client.raw_client.post = wrapper_post  # type: ignore
            expected_status_code = 401
        case "authorization_header_invalid":
            client.headers["Authorization"] = "<dummy>"
            expected_status_code = 401
        case "authorization_header_invalid_method":
            client.headers["Authorization"] = (
                "Dummy PARSEC-SIGN-ED25519.YWxpY2VAZGV2MQ==.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_invalid_token":
            client.headers["Authorization"] = "Bearer <dummy>"
            expected_status_code = 401
        case "authorization_header_author_unknown":
            b64_uknown_author = urlsafe_b64encode(b"dummy@dev1").decode()
            client.headers["Authorization"] = (
                f"Bearer PARSEC-SIGN-ED25519.{b64_uknown_author}.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 403
        case "authorization_header_author_not_b64":
            client.headers["Authorization"] = (
                "Bearer PARSEC-SIGN-ED25519.<dummy>.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_author_not_utf8":
            b64_bad_author = urlsafe_b64encode(b"alice\xc0@dev1").decode()
            client.headers["Authorization"] = (
                f"Bearer PARSEC-SIGN-ED25519.{b64_bad_author}.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_bad_timestamp":
            client.headers["Authorization"] = (
                "Bearer PARSEC-SIGN-ED25519.YWxpY2VAZGV2MQ==.<dummy>.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_signature_not_b64":
            client.headers["Authorization"] = (
                "Bearer PARSEC-SIGN-ED25519.YWxpY2VAZGV2MQ==.1708687856.<dummy>"  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_bad_signature":
            vanilla_post = client.raw_client.post

            async def wrapper_post(url, headers, content):
                without_signature, _ = headers["Authorization"].rsplit(".", 1)
                headers["Authorization"] = (
                    f"{without_signature}.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
                )
                return await vanilla_post(url, headers=headers, content=content)

            client.raw_client.post = wrapper_post  # type: ignore
            expected_status_code = 403
        case "authorization_header_timestamp_too_late":
            client.now_factory = lambda: DateTime.now().subtract(
                seconds=BALLPARK_CLIENT_LATE_OFFSET + 1
            )
            expected_status_code = 498
        case "authorization_header_timestamp_too_early":
            client.now_factory = lambda: DateTime.now().add(
                seconds=BALLPARK_CLIENT_EARLY_OFFSET + 1
            )
            expected_status_code = 498
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
        "authorization_header_invalid_method",
        "authorization_header_invalid_token",
        "authorization_header_author_unknown",
        "authorization_header_author_not_b64",
        "authorization_header_author_not_utf8",
        "authorization_header_bad_timestamp",
        "authorization_header_signature_not_b64",
        "authorization_header_bad_signature",
        "authorization_header_timestamp_too_late",
        "authorization_header_timestamp_too_early",
    ),
)
async def test_authenticated_bad_auth_sse(coolorg: CoolorgRpcClients, kind: str) -> None:
    client = coolorg.alice

    client.url += "/events"
    client.headers["Accept"] = "text/event-stream"
    token = AuthenticatedToken.generate_raw(
        device_id=client.device_id, timestamp=DateTime.now(), key=client.signing_key
    )
    client.headers["Authorization"] = f"Bearer {token.decode()}"

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
            client.url += "\xc0"
            expected_status_code = 404
        case "organization_unknown":
            base_url = client.url.rsplit("/", 1)[0]
            client.url = f"{base_url}/DummyOrg"
            expected_status_code = 404
        case "authorization_header_missing":
            vanilla_get = client.raw_client.get

            async def wrapper_get(url, headers):
                del headers["Authorization"]
                return await vanilla_get(url, headers=headers)

            client.raw_client.get = wrapper_get  # type: ignore
            expected_status_code = 401
        case "authorization_header_invalid":
            client.headers["Authorization"] = "<dummy>"
            expected_status_code = 401
        case "authorization_header_invalid_method":
            client.headers["Authorization"] = (
                "Dummy PARSEC-SIGN-ED25519.YWxpY2VAZGV2MQ==.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_invalid_token":
            client.headers["Authorization"] = "Bearer <dummy>"
            expected_status_code = 401
        case "authorization_header_author_unknown":
            b64_uknown_author = urlsafe_b64encode(b"dummy@dev1").decode()
            client.headers["Authorization"] = (
                f"Bearer PARSEC-SIGN-ED25519.{b64_uknown_author}.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 403
        case "authorization_header_author_not_b64":
            client.headers["Authorization"] = (
                "Bearer PARSEC-SIGN-ED25519.<dummy>.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_author_not_utf8":
            b64_bad_author = urlsafe_b64encode(b"alice\xc0@dev1").decode()
            client.headers["Authorization"] = (
                f"Bearer PARSEC-SIGN-ED25519.{b64_bad_author}.1708687856.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_bad_timestamp":
            client.headers["Authorization"] = (
                "Bearer PARSEC-SIGN-ED25519.YWxpY2VAZGV2MQ==.<dummy>.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_signature_not_b64":
            client.headers["Authorization"] = (
                "Bearer PARSEC-SIGN-ED25519.YWxpY2VAZGV2MQ==.1708687856.<dummy>"  # cspell:disable-line
            )
            expected_status_code = 401
        case "authorization_header_bad_signature":
            vanilla_get = client.raw_client.get

            async def wrapper_get(url, headers):
                without_signature, _ = headers["Authorization"].rsplit(".", 1)
                headers["Authorization"] = (
                    f"{without_signature}.akCqHmz2O8bQzz7i0ECmH8F3_mD7pwDds1eW_NzatTHfvAuwr_7obK5qSrCHFlN0XtVAJKaIZWDFLKNf7iGOAg=="  # cspell:disable-line
                )
                return await vanilla_get(url, headers=headers)

            client.raw_client.get = wrapper_get  # type: ignore
            expected_status_code = 403
        case "authorization_header_timestamp_too_late":
            too_old = DateTime.now().subtract(seconds=BALLPARK_CLIENT_LATE_OFFSET + 1)
            token = AuthenticatedToken.generate_raw(
                device_id=client.device_id, timestamp=too_old, key=client.signing_key
            )
            client.headers["Authorization"] = f"Bearer {token.decode()}"
            expected_status_code = 498
        case "authorization_header_timestamp_too_early":
            too_new = DateTime.now().add(seconds=BALLPARK_CLIENT_EARLY_OFFSET + 1)
            token = AuthenticatedToken.generate_raw(
                device_id=client.device_id, timestamp=too_new, key=client.signing_key
            )
            client.headers["Authorization"] = f"Bearer {token.decode()}"
            expected_status_code = 498
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
        "authorization_header_missing",
        "authorization_header_invalid",
        "authorization_header_invalid_method",
        "authorization_header_invalid_token",
        "authorization_header_unknown_token",
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
            client.url += "\xc0"
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
            expected_status_code = 401
        case "authorization_header_invalid_method":
            client.headers["Authorization"] = "Dummy 6f56a8579fc4425c82a71f9fc8531b77"
            expected_status_code = 401
        case "authorization_header_invalid_token":
            client.headers["Authorization"] = "Bearer <dummy>"
            expected_status_code = 401
        case "authorization_header_unknown_token":
            client.headers["Authorization"] = "Bearer 6f56a8579fc4425c82a71f9fc8531b77"
            expected_status_code = 403
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
            client.url += "\xc0"
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
