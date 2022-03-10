# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Tuple, Optional, cast, Dict, Sequence, Union
from enum import Enum
from secrets import token_bytes

import pendulum
from pendulum.datetime import DateTime

from parsec.crypto import SigningKey, VerifyKey, CryptoError
from parsec.serde import BaseSchema, OneOfSchema, fields, validate
from parsec.utils import (
    BALLPARK_CLIENT_EARLY_OFFSET,
    BALLPARK_CLIENT_LATE_OFFSET,
    BALLPARK_CLIENT_TOLERANCE,
    timestamps_in_the_ballpark,
)
from parsec.api.protocol.base import ProtocolError, InvalidMessageError, serializer_factory
from parsec.api.protocol.types import OrganizationID, DeviceID, OrganizationIDField, DeviceIDField
from parsec.api.protocol.invite import (
    InvitationToken,
    InvitationTokenField,
    InvitationType,
    InvitationTypeField,
)
from parsec.api.version import ApiVersion, API_V1_VERSION, API_V2_VERSION


class HandshakeError(ProtocolError):
    pass


class HandshakeFailedChallenge(HandshakeError):
    pass


class HandshakeBadAdministrationToken(HandshakeError):
    pass


class HandshakeBadIdentity(HandshakeError):
    pass


class HandshakeOrganizationExpired(HandshakeError):
    pass


class HandshakeRVKMismatch(HandshakeError):
    pass


class HandshakeRevokedDevice(HandshakeError):
    pass


class HandshakeOutOfBallparkError(HandshakeError):
    pass


class HandshakeAPIVersionError(HandshakeError):
    def __init__(
        self, backend_versions: Sequence[ApiVersion], client_versions: Sequence[ApiVersion] = []
    ):
        self.client_versions = client_versions
        self.backend_versions = backend_versions
        client_versions_str = "{" + ", ".join(map(str, client_versions)) + "}"
        backend_versions_str = "{" + ", ".join(map(str, backend_versions)) + "}"
        self.message = (
            f"No overlap between client API versions {client_versions_str} "
            f"and backend API versions {backend_versions_str}"
        )

    def __str__(self) -> str:
        return self.message


class HandshakeType(Enum):
    AUTHENTICATED = "AUTHENTICATED"
    INVITED = "INVITED"
    NOT_INITIALIZED = "NOT_INITIALIZED"


HandshakeTypeField = fields.enum_field_factory(HandshakeType)


class APIV1_HandshakeType(Enum):
    ANONYMOUS = "anonymous"


APIV1_HandshakeTypeField = fields.enum_field_factory(APIV1_HandshakeType)


def _settle_compatible_versions(
    backend_versions: Sequence[ApiVersion], client_versions: Sequence[ApiVersion]
) -> Tuple[ApiVersion, ApiVersion]:
    # Try to use the newest version first
    for cv in reversed(sorted(client_versions)):
        # No need to compare `revision` because only `version` field breaks compatibility
        bv = next((bv for bv in backend_versions if bv.version == cv.version), None)
        if bv:
            return bv, cv
    raise HandshakeAPIVersionError(backend_versions, client_versions)


class ApiVersionField(fields.Tuple):
    def __init__(self, **kwargs: object):
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        revision = fields.Integer(required=True, validate=validate.Range(min=0))
        super().__init__(version, revision, **kwargs)

    def _deserialize(self, *args: object, **kwargs: object) -> ApiVersion:
        result = super()._deserialize(*args, **kwargs)
        return ApiVersion(*result)


class HandshakeChallengeSchema(BaseSchema):
    handshake = fields.CheckedConstant("challenge", required=True)
    challenge = fields.Bytes(required=True)
    supported_api_versions = fields.List(ApiVersionField(), required=True)
    # Those fields have been added to API version 2.4
    # They are provided to the client in order to allow them to detect whether
    # their system clock is out of sync and let them close the connection.
    # They will be missing for older backend so they cannot be strictly required.
    # TODO: This backward compatibility should be removed once Parsec < 2.4 support is dropped
    ballpark_client_early_offset = fields.Float(required=False, missing=None)
    ballpark_client_late_offset = fields.Float(required=False, missing=None)
    backend_timestamp = fields.DateTime(required=False, missing=None)


handshake_challenge_serializer = serializer_factory(HandshakeChallengeSchema)


class HandshakeAnswerVersionOnlySchema(BaseSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    client_api_version = ApiVersionField(required=True)


handshake_answer_version_only_serializer = serializer_factory(HandshakeAnswerVersionOnlySchema)


class AnswerSchema(BaseSchema):
    type = fields.CheckedConstant("signed_answer", required=True)
    answer = fields.Bytes(required=True)


answer_serializer = serializer_factory(AnswerSchema)


class HandshakeAuthenticatedAnswerSchema(BaseSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.EnumCheckedConstant(HandshakeType.AUTHENTICATED, required=True)
    client_api_version = ApiVersionField(required=True)
    organization_id = OrganizationIDField(required=True)
    device_id = DeviceIDField(required=True)
    rvk = fields.VerifyKey(required=True)
    answer = fields.Bytes(required=True)


class HandshakeInvitedAnswerSchema(BaseSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.EnumCheckedConstant(HandshakeType.INVITED, required=True)
    client_api_version = ApiVersionField(required=True)
    organization_id = OrganizationIDField(required=True)
    invitation_type = InvitationTypeField(required=True)
    token = InvitationTokenField(required=True)


class HandshakeAnswerSchema(OneOfSchema):
    type_field = "type"
    type_schemas = {
        HandshakeType.AUTHENTICATED: HandshakeAuthenticatedAnswerSchema(),
        HandshakeType.INVITED: HandshakeInvitedAnswerSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> HandshakeType:
        return cast(HandshakeType, obj["type"])


handshake_answer_serializer = serializer_factory(HandshakeAnswerSchema)


class APIV1_HandshakeAnonymousAnswerSchema(BaseSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.EnumCheckedConstant(APIV1_HandshakeType.ANONYMOUS, required=True)
    client_api_version = ApiVersionField(required=True)
    organization_id = OrganizationIDField(required=True)
    # Cannot provide rvk during organization bootstrap
    rvk = fields.VerifyKey(missing=None)


class APIV1_HandshakeAnswerSchema(OneOfSchema):
    type_field = "type"
    type_schemas = {APIV1_HandshakeType.ANONYMOUS: APIV1_HandshakeAnonymousAnswerSchema()}

    def get_obj_type(self, obj: Dict[str, object]) -> APIV1_HandshakeType:
        return cast(APIV1_HandshakeType, obj["type"])


apiv1_handshake_answer_serializer = serializer_factory(APIV1_HandshakeAnswerSchema)


class HandshakeResultSchema(BaseSchema):
    handshake = fields.CheckedConstant("result", required=True)
    result = fields.String(required=True)
    help = fields.String(missing=None)


handshake_result_serializer = serializer_factory(HandshakeResultSchema)


class ServerHandshake:
    # Class attribute
    SUPPORTED_API_VERSIONS = (API_V2_VERSION, API_V1_VERSION)

    def __init__(self, challenge_size: int = 48):
        # Challenge
        self.challenge_size = challenge_size
        self.challenge: bytes

        # Once support APIV1 is dropped, we can do much better than exposing the answer data as
        # a dictionary of arbitrary object. Instead, it could be deserialize as a dedicated and
        # properly typed `HandshakeAnswer` object.
        self.answer_data: Dict[str, object]
        self.answer_type: Union[HandshakeType, APIV1_HandshakeType] = HandshakeType.NOT_INITIALIZED

        # API version
        self.client_api_version: ApiVersion
        self.backend_api_version: ApiVersion

        # State
        self.state = "stalled"

    def build_challenge_req(self) -> bytes:
        if not self.state == "stalled":
            raise HandshakeError("Invalid state.")

        self.challenge = token_bytes(self.challenge_size)
        self.state = "challenge"

        return handshake_challenge_serializer.dumps(
            {
                "handshake": "challenge",
                "challenge": self.challenge,
                "supported_api_versions": self.SUPPORTED_API_VERSIONS,
                "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
                "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
                "backend_timestamp": pendulum.now(),
            }
        )

    def process_answer_req(self, req: bytes) -> None:
        if not self.state == "challenge":
            raise HandshakeError("Invalid state.")

        # Retrieve the version used by client first
        data = handshake_answer_version_only_serializer.loads(req)
        client_api_version = data["client_api_version"]
        if client_api_version.version == 2:
            serializer = handshake_answer_serializer
        elif client_api_version.version == 1:
            serializer = apiv1_handshake_answer_serializer
        else:
            # Imcompatible version !
            raise HandshakeAPIVersionError(self.SUPPORTED_API_VERSIONS, [client_api_version])

        # Now we know how to deserialize the rest of the data
        data = serializer.loads(req)

        data.pop("handshake")
        self.answer_type = data.pop("type")
        self.answer_data = data
        self.state = "answer"

        # API version matching
        self.backend_api_version, self.client_api_version = _settle_compatible_versions(
            self.SUPPORTED_API_VERSIONS, [client_api_version]
        )

    def build_bad_protocol_result_req(self, help: str = "Invalid params") -> bytes:
        if self.state not in ("answer", "challenge"):
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "bad_protocol", "help": help}
        )

    def build_bad_administration_token_result_req(
        self, help: str = "Invalid administration token"
    ) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "bad_admin_token", "help": help}
        )

    def build_bad_identity_result_req(self, help: str = "Invalid handshake information") -> bytes:
        """
        We should keep the help for this result voluntarily broad otherwise
        an attacker could use it to brute force informations.
        """
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "bad_identity", "help": help}
        )

    def build_organization_expired_result_req(
        self, help: str = "Trial organization has expired"
    ) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "organization_expired", "help": help}
        )

    def build_rvk_mismatch_result_req(self, help: Optional[str] = None) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        help = help or ("Root verify key for organization differs between client and server")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "rvk_mismatch", "help": help}
        )

    def build_revoked_device_result_req(self, help: str = "Device has been revoked") -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "revoked_device", "help": help}
        )

    def build_result_req(self, verify_key: Optional[VerifyKey] = None) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        if self.answer_type == HandshakeType.AUTHENTICATED:
            if not verify_key:
                raise HandshakeError(
                    "`verify_key` param must be provided for authenticated handshake"
                )

            try:
                answer = self.answer_data["answer"]
                assert isinstance(answer, bytes)

                if self.client_api_version >= (2, 5):
                    returned_challenge = answer_serializer.loads(verify_key.verify(answer))[
                        "answer"
                    ]
                else:
                    returned_challenge = verify_key.verify(answer)

                if returned_challenge != self.challenge:
                    raise HandshakeFailedChallenge("Invalid returned challenge")

            except CryptoError as exc:
                raise HandshakeFailedChallenge("Invalid answer signature") from exc

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "ok"})


class BaseClientHandshake:
    SUPPORTED_API_VERSIONS: Sequence[ApiVersion]  # Overwritten by subclasses

    def __init__(self) -> None:
        self.challenge_data: Dict[str, object]
        self.backend_api_version: ApiVersion
        self.client_api_version: ApiVersion
        self.client_timestamp = self.timestamp()

    def timestamp(self) -> DateTime:
        # Exposed as a method for easier testing and monkeypatching
        return pendulum.now()

    def load_challenge_req(self, req: bytes) -> None:
        self.challenge_data = handshake_challenge_serializer.loads(req)

        # API version matching
        supported_api_version = cast(
            Sequence[ApiVersion], self.challenge_data["supported_api_versions"]
        )
        self.backend_api_version, self.client_api_version = _settle_compatible_versions(
            supported_api_version, self.SUPPORTED_API_VERSIONS
        )

        # Parse and cast the challenge content
        backend_timestamp = cast(
            Optional[pendulum.DateTime], self.challenge_data.get("backend_timestamp")
        )
        ballpark_client_early_offset = cast(
            Optional[float], self.challenge_data.get("ballpark_client_early_offset")
        )
        ballpark_client_late_offset = cast(
            Optional[float], self.challenge_data.get("ballpark_client_late_offset")
        )

        # Those fields are missing with parsec API 2.3 and lower
        if (
            backend_timestamp is not None
            and ballpark_client_early_offset is not None
            and ballpark_client_late_offset is not None
        ):

            # Add `client_timestamp` to challenge data
            # so the dictionnary exposes the same fields as `TimestampOutOfBallparkRepSchema`
            self.challenge_data["client_timestamp"] = self.client_timestamp

            # The client is a bit less tolerant than the backend
            ballpark_client_early_offset *= BALLPARK_CLIENT_TOLERANCE
            ballpark_client_late_offset *= BALLPARK_CLIENT_TOLERANCE

            # Check whether our system clock is in sync with the backend
            if not timestamps_in_the_ballpark(
                client=self.client_timestamp,
                backend=backend_timestamp,
                ballpark_client_early_offset=ballpark_client_early_offset,
                ballpark_client_late_offset=ballpark_client_late_offset,
            ):
                raise HandshakeOutOfBallparkError(self.challenge_data)

    def process_result_req(self, req: bytes) -> None:
        data = handshake_result_serializer.loads(req)
        if data["result"] != "ok":
            if data["result"] == "bad_identity":
                raise HandshakeBadIdentity(data["help"])

            if data["result"] == "organization_expired":
                raise HandshakeOrganizationExpired(data["help"])

            elif data["result"] == "rvk_mismatch":
                raise HandshakeRVKMismatch(data["help"])

            elif data["result"] == "revoked_device":
                raise HandshakeRevokedDevice(data["help"])

            if data["result"] == "bad_admin_token":
                raise HandshakeBadAdministrationToken(data["help"])

            else:
                raise InvalidMessageError(
                    f"Bad `result` handshake: {data['result']} ({data['help']})"
                )


class AuthenticatedClientHandshake(BaseClientHandshake):
    SUPPORTED_API_VERSIONS = (API_V2_VERSION,)
    HANDSHAKE_TYPE: Union[HandshakeType, APIV1_HandshakeType] = HandshakeType.AUTHENTICATED
    HANDSHAKE_ANSWER_SERIALIZER = handshake_answer_serializer

    def __init__(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        user_signkey: SigningKey,
        root_verify_key: VerifyKey,
    ):
        super().__init__()
        self.organization_id = organization_id
        self.device_id = device_id
        self.user_signkey = user_signkey
        self.root_verify_key = root_verify_key

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        challenge = self.challenge_data["challenge"]

        assert isinstance(challenge, bytes)

        # TO-DO remove the else for the next release
        if self.backend_api_version >= (2, 5):
            # TO-DO Need to use "BaseSignedData" ?
            answer = self.user_signkey.sign(
                answer_serializer.dumps({"type": "signed_answer", "answer": challenge})
            )
        else:
            answer = self.user_signkey.sign(challenge)

        return self.HANDSHAKE_ANSWER_SERIALIZER.dumps(
            {
                "handshake": "answer",
                "type": self.HANDSHAKE_TYPE,
                "client_api_version": self.client_api_version,
                "organization_id": self.organization_id,
                "device_id": self.device_id,
                "rvk": self.root_verify_key,
                "answer": answer,
            }
        )


class InvitedClientHandshake(BaseClientHandshake):
    SUPPORTED_API_VERSIONS = (API_V2_VERSION,)

    def __init__(
        self,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    ):
        super().__init__()
        self.organization_id = organization_id
        self.invitation_type = invitation_type
        self.token = token

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": HandshakeType.INVITED,
                "client_api_version": self.client_api_version,
                "organization_id": self.organization_id,
                "invitation_type": self.invitation_type,
                "token": self.token,
            }
        )


class APIV1_AnonymousClientHandshake(BaseClientHandshake):
    SUPPORTED_API_VERSIONS = (API_V1_VERSION,)

    def __init__(
        self, organization_id: OrganizationID, root_verify_key: Optional[VerifyKey] = None
    ):
        super().__init__()
        self.organization_id = organization_id
        self.root_verify_key = root_verify_key

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        return apiv1_handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": APIV1_HandshakeType.ANONYMOUS,
                "client_api_version": self.client_api_version,
                "organization_id": self.organization_id,
                "rvk": self.root_verify_key,
            }
        )
