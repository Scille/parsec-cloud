# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Tuple, Union, cast

from quart import Websocket

from parsec._parsec import DateTime
from parsec.api.protocol import (
    APIV1_HandshakeType,
    DeviceID,
    HandshakeType,
    InvitationToken,
    InvitationType,
    OrganizationID,
    ProtocolError,
    ServerHandshake,
)
from parsec.backend.app import BackendApp
from parsec.backend.client_context import (
    APIV1_AnonymousClientContext,
    AuthenticatedClientContext,
    BaseClientContext,
    InvitedClientContext,
)
from parsec.backend.invite import (
    DeviceInvitation,
    InvitationAlreadyDeletedError,
    InvitationError,
    InvitationNotFoundError,
    UserInvitation,
)
from parsec.backend.organization import OrganizationAlreadyExistsError, OrganizationNotFoundError
from parsec.backend.user import UserNotFoundError


async def do_handshake(
    backend: BackendApp, websocket: Websocket
) -> Tuple[BaseClientContext | None, dict[str, object] | None]:
    try:
        handshake = ServerHandshake()
        challenge_req = handshake.build_challenge_req()
        await websocket.send(challenge_req)
        # Websocket can return both bytes or utf8-string messages, we only accept the former
        answer_req: Union[bytes, str] = await websocket.receive()
        if not isinstance(answer_req, bytes):
            raise ProtocolError("Expected bytes message in websocket")

        handshake.process_answer_req(answer_req)
        if handshake.answer_type == HandshakeType.AUTHENTICATED:
            context, result_req, error_infos = await _process_authenticated_answer(
                backend, handshake
            )

        elif handshake.answer_type == HandshakeType.INVITED:
            context, result_req, error_infos = await _process_invited_answer(backend, handshake)

        else:
            assert handshake.answer_type == APIV1_HandshakeType.ANONYMOUS
            context, result_req, error_infos = await _apiv1_process_anonymous_answer(
                backend, handshake
            )

    except ProtocolError as exc:
        context = None
        result_req = handshake.build_bad_protocol_result_req(str(exc))
        error_infos = {"reason": str(exc), "handshake_type": handshake.answer_type}

    await websocket.send(result_req)

    return context, error_infos


async def _process_authenticated_answer(
    backend: BackendApp, handshake: ServerHandshake
) -> Tuple[BaseClientContext | None, bytes, dict[str, object] | None]:
    return await _do_process_authenticated_answer(backend, handshake, HandshakeType.AUTHENTICATED)


async def _do_process_authenticated_answer(
    backend: BackendApp, handshake: ServerHandshake, handshake_type: HandshakeType
) -> Tuple[BaseClientContext | None, bytes, dict[str, object] | None]:

    organization_id = cast(OrganizationID, handshake.answer_data["organization_id"])
    device_id = cast(DeviceID, handshake.answer_data["device_id"])
    expected_rvk = handshake.answer_data["rvk"]

    def _make_error_infos(reason: str) -> dict[str, object]:
        return {
            "reason": reason,
            "handshake_type": handshake_type,
            "organization_id": organization_id,
            "device_id": device_id,
        }

    try:
        organization = await backend.organization.get(organization_id)
        user, device = await backend.user.get_user_with_device(organization_id, device_id)

    except (OrganizationNotFoundError, UserNotFoundError, KeyError) as exc:
        result_req = handshake.build_bad_identity_result_req()
        return None, result_req, _make_error_infos(str(exc))

    if organization.root_verify_key != expected_rvk:
        result_req = handshake.build_rvk_mismatch_result_req()
        return None, result_req, _make_error_infos("Bad root verify key")

    if organization.is_expired:
        result_req = handshake.build_organization_expired_result_req()
        return None, result_req, _make_error_infos("Expired organization")

    if user.revoked_on and user.revoked_on <= DateTime.now():
        result_req = handshake.build_revoked_device_result_req()
        return None, result_req, _make_error_infos("Revoked device")

    context = AuthenticatedClientContext(
        api_version=handshake.backend_api_version,
        organization_id=organization_id,
        device_id=device_id,
        human_handle=user.human_handle,
        device_label=device.device_label,
        profile=user.profile,
        public_key=user.public_key,
        verify_key=device.verify_key,
    )
    result_req = handshake.build_result_req(device.verify_key)
    return context, result_req, None


async def _process_invited_answer(
    backend: BackendApp, handshake: ServerHandshake
) -> Tuple[BaseClientContext | None, bytes, dict[str, object] | None]:
    organization_id = cast(OrganizationID, handshake.answer_data["organization_id"])
    invitation_type = cast(InvitationType, handshake.answer_data["invitation_type"])
    token = handshake.answer_data["token"]

    def _make_error_infos(reason: str) -> dict[str, object]:
        return {
            "reason": reason,
            "handshake_type": HandshakeType.INVITED,
            "organization_id": organization_id,
            "invitation_type": invitation_type,
            "token": token,
        }

    try:
        organization = await backend.organization.get(organization_id)

    except OrganizationNotFoundError:
        result_req = handshake.build_bad_identity_result_req()
        return None, result_req, _make_error_infos("Bad organization")

    if organization.is_expired:
        result_req = handshake.build_organization_expired_result_req()
        return None, result_req, _make_error_infos("Expired organization")

    try:
        invitation = await backend.invite.info(
            organization_id, token=cast(InvitationToken, handshake.answer_data["token"])
        )
    except InvitationAlreadyDeletedError:
        result_req = handshake.build_bad_identity_result_req(
            help="Invalid handshake: Invitation already deleted"
        )
        return None, result_req, _make_error_infos("Bad invitation")

    except InvitationNotFoundError:
        result_req = handshake.build_bad_identity_result_req(
            help="Invalid handshake: Invitation not found"
        )
        return None, result_req, _make_error_infos("Bad invitation")

    except InvitationError:
        result_req = handshake.build_bad_identity_result_req()
        return None, result_req, _make_error_infos("Bad invitation")

    expected_invitation_type = (
        UserInvitation
        if handshake.answer_data["invitation_type"] == InvitationType.USER
        else DeviceInvitation
    )
    if not isinstance(invitation, expected_invitation_type):
        result_req = handshake.build_bad_identity_result_req()
        return None, result_req, _make_error_infos("Bad invitation")

    context = InvitedClientContext(
        api_version=handshake.backend_api_version,
        organization_id=organization_id,
        invitation=invitation,
    )
    result_req = handshake.build_result_req()
    return context, result_req, None


async def _apiv1_process_anonymous_answer(
    backend: BackendApp, handshake: ServerHandshake
) -> Tuple[BaseClientContext | None, bytes, dict[str, object] | None]:
    organization_id = cast(OrganizationID, handshake.answer_data["organization_id"])
    expected_rvk = handshake.answer_data["rvk"]

    def _make_error_infos(reason: str) -> dict[str, object]:
        return {
            "reason": reason,
            "handshake_type": APIV1_HandshakeType.ANONYMOUS,
            "organization_id": organization_id,
        }

    try:
        organization = await backend.organization.get(organization_id)

    except OrganizationNotFoundError:
        if backend.config.organization_spontaneous_bootstrap:
            # Lazy creation of the organization with always the same empty token
            try:
                await backend.organization.create(
                    id=organization_id,
                    bootstrap_token="",
                    created_on=DateTime.now(),
                )
            except OrganizationAlreadyExistsError:
                pass
            organization = await backend.organization.get(organization_id)

        else:
            result_req = handshake.build_bad_identity_result_req()
            return None, result_req, _make_error_infos("Bad organization")

    if organization.is_expired:
        result_req = handshake.build_organization_expired_result_req()
        return None, result_req, _make_error_infos("Expired organization")

    if expected_rvk and organization.root_verify_key != expected_rvk:
        result_req = handshake.build_rvk_mismatch_result_req()
        return None, result_req, _make_error_infos("Bad root verify key")

    context = APIV1_AnonymousClientContext(
        api_version=handshake.backend_api_version, organization_id=organization_id
    )
    result_req = handshake.build_result_req()
    return context, result_req, None
