# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from typing import List, Tuple
from secrets import token_bytes

from parsec.crypto import CryptoError
from parsec.serde import BaseSchema, OneOfSchema, fields, validate
from parsec.api.protocol.base import ProtocolError, InvalidMessageError, serializer_factory
from parsec.api.protocol.types import OrganizationIDField, DeviceIDField
from parsec.api.version import ApiVersion, API_VERSION


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


class HandshakeAPIVersionError(HandshakeError):
    def __init__(self, backend_versions: ApiVersion, client_versions: ApiVersion):
        self.client_versions = client_versions
        self.backend_versions = backend_versions
        client_versions_str = "{" + ", ".join(map(str, client_versions)) + "}"
        backend_versions_str = "{" + ", ".join(map(str, backend_versions)) + "}"
        self.message = (
            f"No overlap between client API versions {client_versions_str} "
            f"and backend API versions {backend_versions_str}"
        )

    def __str__(self):
        return self.message


def _settle_compatible_versions(
    backend_versions: List[ApiVersion], client_versions: List[ApiVersion]
) -> Tuple[ApiVersion, ApiVersion]:
    # Try to use the newest version first
    for cv in reversed(sorted(client_versions)):
        # No need to compare `revision` because only `version` field breaks compatibility
        bv = next((bv for bv in backend_versions if bv.version == cv.version), None)
        if bv:
            return bv, cv
    raise HandshakeAPIVersionError(backend_versions, client_versions)


class ApiVersionField(fields.Tuple):
    def __init__(self, **kwargs):
        version = fields.Integer(required=True, validate=validate.Range(min=0))
        revision = fields.Integer(required=True, validate=validate.Range(min=0))
        super().__init__(version, revision, **kwargs)

    def _deserialize(self, *args, **kwargs):
        result = super()._deserialize(*args, **kwargs)
        return ApiVersion(*result)


class HandshakeChallengeSchema(BaseSchema):
    handshake = fields.CheckedConstant("challenge", required=True)
    challenge = fields.Bytes(required=True)
    supported_api_versions = fields.List(ApiVersionField(), required=True)


handshake_challenge_serializer = serializer_factory(HandshakeChallengeSchema)


class HandshakeAuthenticatedAnswerSchema(BaseSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("authenticated", required=True)
    client_api_version = ApiVersionField(required=True)
    organization_id = OrganizationIDField(required=True)
    device_id = DeviceIDField(required=True)
    rvk = fields.VerifyKey(required=True)
    answer = fields.Bytes(required=True)


class HandshakeAnonymousAnswerSchema(BaseSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("anonymous", required=True)
    client_api_version = ApiVersionField(required=True)
    organization_id = OrganizationIDField(required=True)
    # Cannot provide rvk during organization bootstrap
    rvk = fields.VerifyKey(missing=None)


class HandshakeAdministrationAnswerSchema(BaseSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("administration", required=True)
    client_api_version = ApiVersionField(required=True)
    token = fields.String(required=True)


class HandshakeAnswerSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "authenticated": HandshakeAuthenticatedAnswerSchema(),
        "anonymous": HandshakeAnonymousAnswerSchema(),
        "administration": HandshakeAdministrationAnswerSchema(),
    }

    def get_obj_type(self, obj):
        return obj["type"]


handshake_answer_serializer = serializer_factory(HandshakeAnswerSchema)


class HandshakeResultSchema(BaseSchema):
    handshake = fields.CheckedConstant("result", required=True)
    result = fields.String(required=True)
    help = fields.String(missing=None)


handshake_result_serializer = serializer_factory(HandshakeResultSchema)


@attr.s
class ServerHandshake:
    # Class attribute
    supported_api_versions = frozenset([API_VERSION])

    # Challenge
    challenge_size = attr.ib(default=48)
    challenge = attr.ib(default=None)
    answer_type = attr.ib(default=None)
    answer_data = attr.ib(default=None)

    # API version
    client_api_version = attr.ib(default=None)
    backend_api_version = attr.ib(default=None)

    # State
    state = attr.ib(default="stalled")

    def is_anonymous(self):
        return self.device_id is None

    def build_challenge_req(self) -> bytes:
        if not self.state == "stalled":
            raise HandshakeError("Invalid state.")

        self.challenge = token_bytes(self.challenge_size)
        self.state = "challenge"

        return handshake_challenge_serializer.dumps(
            {
                "handshake": "challenge",
                "challenge": self.challenge,
                "supported_api_versions": self.supported_api_versions,
            }
        )

    def process_answer_req(self, req: bytes):
        if not self.state == "challenge":
            raise HandshakeError("Invalid state.")

        data = handshake_answer_serializer.loads(req)

        data.pop("handshake")
        self.answer_type = data.pop("type")
        self.answer_data = data
        self.state = "answer"

        # API version matching
        client_api_versions = frozenset([self.answer_data["client_api_version"]])
        self.backend_api_version, self.client_api_version = _settle_compatible_versions(
            self.supported_api_versions, client_api_versions
        )

    def build_bad_protocol_result_req(self, help="Invalid params") -> bytes:
        if self.state not in ("answer", "challenge"):
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "bad_protocol", "help": help}
        )

    def build_bad_administration_token_result_req(
        self, help="Invalid administration token"
    ) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "bad_admin_token", "help": help}
        )

    def build_bad_identity_result_req(self, help="Unknown Organization or Device") -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "bad_identity", "help": help}
        )

    def build_organization_expired_result_req(self, help="Trial organization has expired") -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "organization_expired", "help": help}
        )

    def build_rvk_mismatch_result_req(self, help=None) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        help = help or ("Root verify key for organization differs between client and server")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "rvk_mismatch", "help": help}
        )

    def build_revoked_device_result_req(self, help="Device has been revoked") -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "revoked_device", "help": help}
        )

    def build_result_req(self, verify_key=None) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        if self.answer_type == "authenticated":
            if not verify_key:
                raise HandshakeError(
                    "`verify_key` param must be provided for authenticated handshake"
                )

            try:
                returned_challenge = verify_key.verify(self.answer_data["answer"])
                if returned_challenge != self.challenge:
                    raise HandshakeFailedChallenge("Invalid returned challenge")

            except CryptoError as exc:
                raise HandshakeFailedChallenge("Invalid answer signature") from exc

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "ok"})


@attr.s
class BaseClientHandshake:
    # Class attribute
    supported_api_versions = frozenset([API_VERSION])

    # Challenge
    challenge_data = attr.ib(default=None)

    # API version
    backend_api_version = attr.ib(default=None)
    client_api_version = attr.ib(default=None)

    def load_challenge_req(self, req: bytes):
        self.challenge_data = handshake_challenge_serializer.loads(req)

        # API version matching
        backend_api_versions = frozenset(self.challenge_data["supported_api_versions"])
        self.backend_api_version, self.client_api_version = _settle_compatible_versions(
            backend_api_versions, self.supported_api_versions
        )

    def process_result_req(self, req: bytes) -> bytes:
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


@attr.s
class AuthenticatedClientHandshake(BaseClientHandshake):
    organization_id = attr.ib()
    device_id = attr.ib()
    user_signkey = attr.ib()
    root_verify_key = attr.ib()

    challenge_data = attr.ib(default=None)
    backend_api_version = attr.ib(default=None)
    client_api_version = attr.ib(default=None)

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        answer = self.user_signkey.sign(self.challenge_data["challenge"])
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "authenticated",
                "client_api_version": self.client_api_version,
                "organization_id": self.organization_id,
                "device_id": self.device_id,
                "rvk": self.root_verify_key,
                "answer": answer,
            }
        )


@attr.s
class AnonymousClientHandshake(BaseClientHandshake):
    organization_id = attr.ib()
    root_verify_key = attr.ib(default=None)

    challenge_data = attr.ib(default=None)
    backend_api_version = attr.ib(default=None)
    client_api_version = attr.ib(default=None)

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "anonymous",
                "client_api_version": self.client_api_version,
                "organization_id": self.organization_id,
                "rvk": self.root_verify_key,
            }
        )


@attr.s
class AdministrationClientHandshake(BaseClientHandshake):
    token = attr.ib()

    challenge_data = attr.ib(default=None)
    backend_api_version = attr.ib(default=None)
    client_api_version = attr.ib(default=None)

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "administration",
                "client_api_version": self.client_api_version,
                "token": self.token,
            }
        )
