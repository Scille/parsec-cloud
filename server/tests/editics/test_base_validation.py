# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from typing import Literal

import httpx_sse
import pytest

from parsec._parsec import DateTime, RealmArchivingConfiguration, VlobID
from parsec.asgi.editics import EditicsHttpStatus
from parsec.components.auth import AuthenticatedToken
from parsec.components.user import UserInfo
from parsec.editics_protocol import (
    EDITICS_API_VERSION,
    EditicsClientEventAuth,
    EditicsClientEventAuthReconnect,
    EditicsParticipantSessionID,
    EditicsTimestampMs,
)
from tests.common import (
    INITIAL_CONTENT_MEDIA_TYPE,
    Backend,
    CoolorgRpcClients,
    EditicsSessionClient,
    alice_gives_profile,
    pack_initial_content_bundle,
    wksp1_alice_update_archiving_config,
)


def _auth_event(
    *,
    mode: Literal["view", "edit"],
    reconnect: tuple[EditicsParticipantSessionID, EditicsTimestampMs | None] | None = None,
) -> EditicsClientEventAuth:
    return EditicsClientEventAuth(
        mode=mode,
        reconnect=EditicsClientEventAuthReconnect(
            participantSessionId=reconnect[0],
            lastOtherSaveTime=reconnect[1],
        )
        if reconnect
        else None,
    )


@pytest.mark.parametrize("bad_api_version", [None, "not a version", "2.0", "1"])
async def test_unsupported_api_version(
    alice_editics: EditicsSessionClient, bad_api_version: str | None
) -> None:
    headers = dict(alice_editics.base_headers)
    if bad_api_version is None:
        del headers["Api-Version"]
    else:
        headers["Api-Version"] = bad_api_version

    # POST initial content

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers=headers,
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == str(EDITICS_API_VERSION)
    # No version could be settled, hence no `Api-Version` reply header
    assert "Api-Version" not in rep.headers

    # GET initial content

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers=headers,
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == str(EDITICS_API_VERSION)
    assert "Api-Version" not in rep.headers

    # Send

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/send",
        params=alice_editics.base_params,
        headers={**headers, "Content-Type": "application/json"},
        content=_auth_event(mode="edit").model_dump_json(),
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == str(EDITICS_API_VERSION)
    # No version could be settled, hence no `Api-Version` reply header
    assert "Api-Version" not in rep.headers

    # Listen

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/listen",
        params=alice_editics.base_params,
        # Note the `Accept` header is required, otherwise the listen route
        # would reply with a 406 before even looking at the API version
        headers={**headers, "Accept": "text/event-stream"},
    )
    assert rep.status_code == EditicsHttpStatus.UnsupportedApiVersion.value
    assert rep.headers["Supported-Api-Versions"] == str(EDITICS_API_VERSION)
    assert "Api-Version" not in rep.headers


async def test_compatible_api_version(alice_editics: EditicsSessionClient) -> None:
    newer_but_compatible_api_version = (
        f"{EDITICS_API_VERSION.version}.{EDITICS_API_VERSION.version + 10}"
    )

    # POST initial content

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={
            **alice_editics.base_headers,
            "Api-Version": newer_but_compatible_api_version,
            "Content-Type": INITIAL_CONTENT_MEDIA_TYPE,
        },
        content=pack_initial_content_bundle(1, [b"<Editor.bin>"]),
    )
    assert rep.status_code == 204
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

    # GET initial content

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Api-Version": newer_but_compatible_api_version},
    )
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)
    assert rep.headers["Content-Type"] == INITIAL_CONTENT_MEDIA_TYPE
    # The fetched bundle is byte-identical to the uploaded one
    assert await rep.aread() == pack_initial_content_bundle(1, [b"<Editor.bin>"])

    # Send

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/send",
        params=alice_editics.base_params,
        headers={
            **alice_editics.base_headers,
            "Content-Type": "application/json",
            "Api-Version": newer_but_compatible_api_version,
        },
        content=_auth_event(mode="view").model_dump_json(),
    )
    assert rep.status_code == 200
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

    # Listen

    async with httpx_sse.aconnect_sse(
        alice_editics.raw_client,
        "GET",
        f"{alice_editics.base_url}/listen",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Api-Version": newer_but_compatible_api_version},
    ) as sse_event_source:
        # A refused request would be answered with a non-SSE error response
        assert sse_event_source.response.status_code == 200
        assert (
            sse_event_source.response.headers["Content-Type"] == "text/event-stream; charset=utf-8"
        )


async def test_bad_auth(coolorg: CoolorgRpcClients, alice_editics: EditicsSessionClient) -> None:
    raw_bad_token = AuthenticatedToken.generate_raw(
        device_id=coolorg.alice.device_id,
        timestamp=coolorg.alice.now_factory(),
        key=coolorg.bob.signing_key,  # Here!
        scope=f"editics/{alice_editics.realm_id.hex}/{alice_editics.document_id.hex}".encode(),
    )
    # Unscoped token (i.e. a regular authenticated API token)
    raw_unscoped_token = AuthenticatedToken.generate_raw(
        device_id=coolorg.alice.device_id,
        timestamp=coolorg.alice.now_factory(),
        key=coolorg.alice.signing_key,
    )
    # Token scoped to another document
    raw_other_scope_token = AuthenticatedToken.generate_raw(
        device_id=coolorg.alice.device_id,
        timestamp=coolorg.alice.now_factory(),
        key=coolorg.alice.signing_key,
        scope=f"editics/{alice_editics.realm_id.hex}/{VlobID.new().hex}".encode(),
    )

    for bad_authorization_value, expected_status_code in [
        # Missing authorization header
        (None, EditicsHttpStatus.MissingAuthenticationInfo),
        # Malformed authorization header value
        ("Dummy", EditicsHttpStatus.MissingAuthenticationInfo),
        # Malformed authorization token
        ("Bearer bad", EditicsHttpStatus.MissingAuthenticationInfo),
        # Invalid authorization token
        (f"Bearer {raw_bad_token.decode('ascii')}", EditicsHttpStatus.BadAuthenticationInfo),
        # Unscoped token (i.e. a regular authenticated API token)
        (
            f"Bearer {raw_unscoped_token.decode('ascii')}",
            EditicsHttpStatus.MissingAuthenticationInfo,
        ),
        # Token scoped to another document
        (
            f"Bearer {raw_other_scope_token.decode('ascii')}",
            EditicsHttpStatus.MissingAuthenticationInfo,
        ),
    ]:
        base_headers = alice_editics.base_headers.copy()
        if bad_authorization_value is None:
            del base_headers["Authorization"]
        else:
            base_headers["Authorization"] = bad_authorization_value

        # POST initial content

        rep = await alice_editics.raw_client.post(
            f"{alice_editics.base_url}/initial_content",
            params=alice_editics.base_params,
            headers={
                **base_headers,
                "Content-Type": INITIAL_CONTENT_MEDIA_TYPE,
            },
            content=pack_initial_content_bundle(1, [b"<Editor.bin>"]),
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # GET initial content

        rep = await alice_editics.raw_client.get(
            f"{alice_editics.base_url}/initial_content",
            params=alice_editics.base_params,
            headers={
                **base_headers,
                "Accept": "text/event-stream",
            },
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # Send

        rep = await alice_editics.raw_client.post(
            f"{alice_editics.base_url}/send",
            params=alice_editics.base_params,
            headers={
                **base_headers,
                "Content-Type": "application/json",
            },
            content=_auth_event(mode="edit").model_dump_json(),
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # Listen

        rep = await alice_editics.raw_client.get(
            f"{alice_editics.base_url}/listen",
            params=alice_editics.base_params,
            headers={
                **base_headers,
                "Accept": "text/event-stream",
            },
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)


async def test_user_revoked_frozen(
    backend: Backend, coolorg: CoolorgRpcClients, bob_editics: EditicsSessionClient
) -> None:
    async def _assert_requests(expected_status_code: EditicsHttpStatus) -> None:
        # POST initial content

        rep = await bob_editics.raw_client.post(
            f"{bob_editics.base_url}/initial_content",
            params=bob_editics.base_params,
            headers={
                **bob_editics.base_headers,
                "Content-Type": INITIAL_CONTENT_MEDIA_TYPE,
            },
            content=pack_initial_content_bundle(1, [b"<Editor.bin>"]),
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # GET initial content

        rep = await bob_editics.raw_client.get(
            f"{bob_editics.base_url}/initial_content",
            params=bob_editics.base_params,
            headers={
                **bob_editics.base_headers,
                "Accept": "text/event-stream",
            },
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # Send

        rep = await bob_editics.raw_client.post(
            f"{bob_editics.base_url}/send",
            params=bob_editics.base_params,
            headers={
                **bob_editics.base_headers,
                "Content-Type": "application/json",
            },
            content=_auth_event(mode="edit").model_dump_json(),
        )
        assert rep.status_code == expected_status_code.value

        # Listen

        rep = await bob_editics.raw_client.get(
            f"{bob_editics.base_url}/listen",
            params=bob_editics.base_params,
            headers={
                **bob_editics.base_headers,
                "Accept": "text/event-stream",
            },
        )
        assert rep.status_code == expected_status_code.value

    # Frozen user

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.bob.user_id,
        user_email=None,
        frozen=True,
    )
    assert isinstance(outcome, UserInfo)

    await _assert_requests(EditicsHttpStatus.UserFrozen)

    # Revoked user

    await alice_gives_profile(coolorg, backend, recipient=coolorg.bob.user_id, new_profile=None)

    await _assert_requests(EditicsHttpStatus.UserRevoked)


async def test_realm_not_found_is_indistinguishable_from_realm_without_role(
    coolorg: CoolorgRpcClients,
) -> None:
    outcomes = []
    for realm_id in (VlobID.new(), coolorg.wksp1_id):
        mallory_editics = EditicsSessionClient(
            who=coolorg.mallory,
            realm_id=realm_id,
            document_id=coolorg.wksp1_id,
        )

        # POST initial content

        rep = await mallory_editics.raw_client.post(
            f"{mallory_editics.base_url}/initial_content",
            params=mallory_editics.base_params,
            headers={
                **mallory_editics.base_headers,
                "Content-Type": INITIAL_CONTENT_MEDIA_TYPE,
            },
            content=pack_initial_content_bundle(1, [b"<Editor.bin>"]),
        )
        outcomes.append(("send", rep.status_code, rep.json()))

        # GET initial content

        rep = await mallory_editics.raw_client.get(
            f"{mallory_editics.base_url}/initial_content",
            params=mallory_editics.base_params,
            headers={
                **mallory_editics.base_headers,
                "Accept": "text/event-stream",
            },
        )
        outcomes.append(("send", rep.status_code, rep.json()))

        # Send

        rep = await mallory_editics.raw_client.post(
            f"{mallory_editics.base_url}/send",
            params=mallory_editics.base_params,
            headers={**mallory_editics.base_headers, "Content-Type": "application/json"},
            content=_auth_event(mode="view").model_dump_json(),
        )
        outcomes.append(("send", rep.status_code, rep.json()))

        # Listen

        rep = await mallory_editics.raw_client.get(
            f"{mallory_editics.base_url}/listen",
            params=mallory_editics.base_params,
            headers={**mallory_editics.base_headers, "Accept": "text/event-stream"},
        )
        outcomes.append(("listen", rep.status_code, rep.json()))

    (
        post_initial_content_nonexistent,
        fetch_initial_content_nonexistent,
        send_nonexistent,
        listen_nonexistent,
        post_initial_content_without_role,
        fetch_initial_content_without_role,
        send_without_role,
        listen_without_role,
    ) = outcomes

    # Information hiding: an organization member must not learn whether a realm
    # it has no access to exists or not
    assert post_initial_content_nonexistent == post_initial_content_without_role
    assert fetch_initial_content_nonexistent == fetch_initial_content_without_role
    assert send_nonexistent == send_without_role
    assert listen_nonexistent == listen_without_role
    assert post_initial_content_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value
    assert fetch_initial_content_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value
    assert send_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value
    assert listen_nonexistent[1] == EditicsHttpStatus.UserNoRealmRole.value


async def test_bad_content_type_and_accept_headers(alice_editics: EditicsSessionClient) -> None:
    # POST initial content (Content-Type)

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/initial_content",
        params=alice_editics.base_params,
        headers={
            **alice_editics.base_headers,
            "Content-Type": "application/dummy",
        },
        content=pack_initial_content_bundle(1, [b"<Editor.bin>"]),
    )
    assert rep.status_code == EditicsHttpStatus.BadContentTypeOrInvalidBody.value
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)
    assert rep.json()["detail"] == {"detail": "Bad content type or invalid body"}

    # GET initial content not tested: doesn't use Content-Type nor Accept headers

    # Send (Content-Type)

    rep = await alice_editics.raw_client.post(
        f"{alice_editics.base_url}/send",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Content-Type": "application/dummy"},
        content=_auth_event(mode="edit").model_dump_json(),
    )
    assert rep.status_code == EditicsHttpStatus.BadContentTypeOrInvalidBody.value
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)
    assert rep.json()["detail"] == {"detail": "Bad content type or invalid body"}

    # Listen (Accept)

    rep = await alice_editics.raw_client.get(
        f"{alice_editics.base_url}/listen",
        params=alice_editics.base_params,
        headers={**alice_editics.base_headers, "Accept": "application/dummy"},
    )
    assert rep.status_code == EditicsHttpStatus.BadAcceptType.value
    assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)
    assert rep.json()["detail"] == {"detail": "Bad accept type"}


@pytest.mark.parametrize("kind", ("archived", "deletion_planned"))
async def test_realm_archived(
    backend: Backend,
    coolorg: CoolorgRpcClients,
    alice_editics: EditicsSessionClient,
    kind: str,
) -> None:
    match kind:
        case "archived":
            await wksp1_alice_update_archiving_config(
                coolorg, backend, RealmArchivingConfiguration.ARCHIVED
            )
        case "deletion_planned":
            # Note the deletion date must be far enough in the future given the
            # organization's minimum archiving period before deletion (30 days)
            await wksp1_alice_update_archiving_config(
                coolorg,
                backend,
                RealmArchivingConfiguration.deletion_planned(DateTime.now().add(days=31)),
            )
        case unknown:
            assert False, unknown

    async def _assert_requests(
        editics_client: EditicsSessionClient, expected_status_code: EditicsHttpStatus
    ) -> None:
        # POST initial content

        rep = await editics_client.raw_client.post(
            f"{editics_client.base_url}/initial_content",
            params=editics_client.base_params,
            headers={
                **editics_client.base_headers,
                "Content-Type": INITIAL_CONTENT_MEDIA_TYPE,
            },
            content=pack_initial_content_bundle(1, [b"<Editor.bin>"]),
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # GET initial content

        rep = await editics_client.raw_client.get(
            f"{editics_client.base_url}/initial_content",
            params=editics_client.base_params,
            headers={**editics_client.base_headers, "Accept": "text/event-stream"},
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # Send

        rep = await editics_client.raw_client.post(
            f"{editics_client.base_url}/send",
            params=editics_client.base_params,
            headers={**editics_client.base_headers, "Content-Type": "application/json"},
            content=_auth_event(mode="view").model_dump_json(),
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

        # Listen

        rep = await editics_client.raw_client.get(
            f"{editics_client.base_url}/listen",
            params=editics_client.base_params,
            headers={**editics_client.base_headers, "Accept": "text/event-stream"},
        )
        assert rep.status_code == expected_status_code.value
        assert rep.headers["Api-Version"] == str(EDITICS_API_VERSION)

    # A realm member is told the realm is archived (or deletion planned)

    await _assert_requests(alice_editics, EditicsHttpStatus.RealmArchived)

    # Information hiding: a user without role on the realm must not learn
    # the realm exists (and is archived!) from the archived-specific error

    mallory_editics = EditicsSessionClient(
        who=coolorg.mallory,
        realm_id=coolorg.wksp1_id,
        document_id=alice_editics.document_id,
    )
    await _assert_requests(mallory_editics, EditicsHttpStatus.UserNoRealmRole)
