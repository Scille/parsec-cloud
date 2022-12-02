# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
from secrets import token_bytes
from typing import Dict, Sequence, TypedDict, Union, cast

from parsec._parsec import DateTime
from parsec.api.protocol.base import (
    InvalidMessageError,
    ProtocolError,
    serializer_factory,
    settle_compatible_versions,
)
from parsec.api.protocol.invite import (
    InvitationToken,
    InvitationTokenField,
    InvitationType,
    InvitationTypeField,
)
from parsec.api.protocol.types import DeviceID, DeviceIDField, OrganizationID, OrganizationIDField
from parsec.api.version import API_V1_VERSION, API_V2_VERSION, API_V3_VERSION, ApiVersion
from parsec.crypto import CryptoError, SigningKey, VerifyKey
from parsec.serde import BaseSchema, OneOfSchema, fields, post_load, validate
from parsec.utils import (
    BALLPARK_CLIENT_EARLY_OFFSET,
    BALLPARK_CLIENT_LATE_OFFSET,
    BALLPARK_CLIENT_TOLERANCE,
    timestamps_in_the_ballpark,
)


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


class HandshakeType(Enum):
    AUTHENTICATED = "AUTHENTICATED"
    INVITED = "INVITED"
    NOT_INITIALIZED = "NOT_INITIALIZED"


HandshakeTypeField = fields.enum_field_factory(HandshakeType)


class APIV1_HandshakeType(Enum):
    ANONYMOUS = "anonymous"


APIV1_HandshakeTypeField = fields.enum_field_factory(APIV1_HandshakeType)


class ApiVersionField(fields.Tuple):
    def __init__(self, **kwargs: object):
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        revision = fields.Integer(required=True, validate=validate.Range(min=0))
        super().__init__(version, revision, **kwargs)

    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> ApiVersion:
        result = super()._deserialize(value, attr, obj)
        return ApiVersion(*result)


class ChallengeData(TypedDict):
    challenge: bytes
    supported_api_versions: Sequence[ApiVersion]
    client_timestamp: DateTime | None
    backend_timestamp: DateTime | None
    ballpark_client_early_offset: float | None
    ballpark_client_late_offset: float | None


class HandshakeChallengeSchema(BaseSchema):
    handshake = fields.CheckedConstant("challenge", required=True)
    challenge = fields.Bytes(required=True)
    supported_api_versions = fields.List(ApiVersionField(), required=True)
    # Those fields have been added to API version 2.4 (Parsec 2.7.0)
    # They are provided to the client in order to allow them to detect whether
    # their system clock is out of sync and let them close the connection.
    # They will be missing for older backend so they cannot be strictly required.
    # TODO: This backward compatibility should be removed once Parsec < 2.4 support is dropped
    ballpark_client_early_offset = fields.Float(required=False, allow_none=False)
    ballpark_client_late_offset = fields.Float(required=False, allow_none=False)
    backend_timestamp = fields.DateTime(required=False, allow_none=False)

    @post_load
    def make_obj(self, data: Dict[str, object]) -> ChallengeData:
        # Cannot use `missing=None` with `allow_none=False`
        data.setdefault("client_timestamp", None)
        data.setdefault("backend_timestamp", None)
        data.setdefault("ballpark_client_early_offset", None)
        data.setdefault("ballpark_client_late_offset", None)
        return cast(ChallengeData, data)


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
    SUPPORTED_API_VERSIONS = (API_V1_VERSION, API_V2_VERSION, API_V3_VERSION)
    CHALLENGE_SIZE = 48

    def __init__(self) -> None:
        # Challenge
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

        self.challenge = token_bytes(self.CHALLENGE_SIZE)
        self.state = "challenge"

        return handshake_challenge_serializer.dumps(
            {
                "handshake": "challenge",
                "challenge": self.challenge,
                "supported_api_versions": self.SUPPORTED_API_VERSIONS,
                "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
                "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
                "backend_timestamp": DateTime.now(),
            }
        )

    def process_answer_req(self, req: bytes) -> None:
        if not self.state == "challenge":
            raise HandshakeError("Invalid state.")

        # Retrieve the version used by client first
        data = handshake_answer_version_only_serializer.loads(req)
        client_api_version = data["client_api_version"]

        # API version matching
        self.backend_api_version, self.client_api_version = settle_compatible_versions(
            self.SUPPORTED_API_VERSIONS, [client_api_version]
        )

        # Use the correct serializer
        # `settle_compatible_versions` is called before,
        # so we already settled on a version from `self.SUPPORTED_API_VERSIONS`
        if client_api_version.version == 1:
            serializer = apiv1_handshake_answer_serializer
        else:
            serializer = handshake_answer_serializer

        # Now we know how to deserialize the rest of the data
        data = serializer.loads(req)

        data.pop("handshake")
        self.answer_type = data.pop("type")
        self.answer_data = data
        self.state = "answer"

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
        an attacker could use it to brute force information.
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

    def build_rvk_mismatch_result_req(self, help: str | None = None) -> bytes:
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

    def build_result_req(self, verify_key: VerifyKey | None = None) -> bytes:
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

                # Provides compatibility with API version 2.4 and below
                # TODO: Remove once API v2.x is deprecated
                if (2, 0) <= self.client_api_version < (2, 5):
                    returned_challenge = verify_key.verify(answer)

                # Used in API v2.5+ and API v3.x
                else:
                    returned_challenge = answer_serializer.loads(verify_key.verify(answer))[
                        "answer"
                    ]

                if returned_challenge != self.challenge:
                    raise HandshakeFailedChallenge("Invalid returned challenge")

            except CryptoError as exc:
                raise HandshakeFailedChallenge("Invalid answer signature") from exc

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "ok"})


class BaseClientHandshake:
    SUPPORTED_API_VERSIONS: Sequence[ApiVersion]  # Overwritten by subclasses

    def __init__(self) -> None:
        self.challenge_data: ChallengeData
        self.backend_api_version: ApiVersion
        self.client_api_version: ApiVersion
        self.client_timestamp = self.timestamp()

    def timestamp(self) -> DateTime:
        # Exposed as a method for easier testing and monkeypatching
        return DateTime.now()

    def load_challenge_req(self, req: bytes) -> None:
        self.challenge_data = cast(ChallengeData, handshake_challenge_serializer.loads(req))

        # API version matching
        supported_api_version = self.challenge_data["supported_api_versions"]
        self.backend_api_version, self.client_api_version = settle_compatible_versions(
            supported_api_version, self.SUPPORTED_API_VERSIONS
        )

        # Parse and cast the challenge content
        backend_timestamp = self.challenge_data["backend_timestamp"]
        ballpark_client_early_offset = self.challenge_data["ballpark_client_early_offset"]
        ballpark_client_late_offset = self.challenge_data["ballpark_client_late_offset"]

        # Those fields are missing with parsec API 2.3 and lower
        if (
            backend_timestamp is not None
            and ballpark_client_early_offset is not None
            and ballpark_client_late_offset is not None
        ):

            # Add `client_timestamp` to challenge data
            # so the dictionary exposes the same fields as `TimestampOutOfBallparkRepSchema`
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
    SUPPORTED_API_VERSIONS = (API_V2_VERSION, API_V3_VERSION)
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

        # Provides compatibility with API version 2.4 and below
        # TODO: Remove once API v2.x is deprecated
        if (2, 0) <= self.backend_api_version < (2, 5):
            answer = self.user_signkey.sign(challenge)

        # Used in API v2.5+ and API v3.x
        else:
            answer = self.user_signkey.sign(
                answer_serializer.dumps({"type": "signed_answer", "answer": challenge})
            )

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
    SUPPORTED_API_VERSIONS = (API_V2_VERSION, API_V3_VERSION)

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
        self,
        organization_id: OrganizationID,
        root_verify_key: VerifyKey | None = None,
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
