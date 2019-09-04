# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from secrets import token_bytes

from parsec import __api_version__
from parsec.crypto import CryptoError
from parsec.serde import UnknownCheckedSchema, OneOfSchema, fields
from parsec.api.protocol.base import ProtocolError, InvalidMessageError, serializer_factory
from parsec.api.protocol.types import OrganizationIDField, DeviceIDField


class HandshakeError(ProtocolError):
    pass


class HandshakeFailedChallenge(HandshakeError):
    pass


class HandshakeBadAdministrationToken(HandshakeError):
    pass


class HandshakeBadIdentity(HandshakeError):
    pass


class HandshakeRVKMismatch(HandshakeError):
    pass


class HandshakeRevokedDevice(HandshakeError):
    pass


class HandshakeAPIVersionError(HandshakeError):
    def __init__(self, peer_version):
        self.peer_version = peer_version
        self.message = (
            f"Incompatiblity between peer API version {peer_version} "
            f"and local API version {__api_version__}"
        )

    @classmethod
    def check_api_version(cls, peer_api_version):
        local_major, _ = __api_version__
        peer_major, _ = peer_api_version
        if local_major != peer_major:
            raise cls(peer_api_version)


class HandshakeChallengeSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("challenge", required=True)
    challenge = fields.Bytes(required=True)
    api_version = fields.ApiVersion(required=True)


handshake_challenge_serializer = serializer_factory(HandshakeChallengeSchema)


class HandshakeAuthenticatedAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("authenticated", required=True)
    api_version = fields.ApiVersion(required=True)
    organization_id = OrganizationIDField(required=True)
    device_id = DeviceIDField(required=True)
    rvk = fields.VerifyKey(required=True)
    answer = fields.Bytes(required=True)


class HandshakeAnonymousAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("anonymous", required=True)
    api_version = fields.ApiVersion(required=True)
    organization_id = OrganizationIDField(required=True)
    # Cannot provide rvk during organization bootstrap
    rvk = fields.VerifyKey(missing=None)


class HandshakeAdministrationAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("administration", required=True)
    api_version = fields.ApiVersion(required=True)
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


class HandshakeResultSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("result", required=True)
    result = fields.String(required=True)
    help = fields.String(missing=None)


handshake_result_serializer = serializer_factory(HandshakeResultSchema)


@attr.s
class ServerHandshake:
    challenge_size = attr.ib(default=48)
    challenge = attr.ib(default=None)
    answer_type = attr.ib(default=None)
    answer_data = attr.ib(default=None)
    state = attr.ib(default="stalled")

    @property
    def client_api_version(self):
        if self.answer_data is None:
            raise TypeError("The answer data is not available yet")
        return self.answer_data["api_version"]

    def is_anonymous(self):
        return self.device_id is None

    def build_challenge_req(self) -> bytes:
        if not self.state == "stalled":
            raise HandshakeError("Invalid state.")

        self.challenge = token_bytes(self.challenge_size)
        self.state = "challenge"

        return handshake_challenge_serializer.dumps(
            {"handshake": "challenge", "challenge": self.challenge, "api_version": __api_version__}
        )

    def process_answer_req(self, req: bytes):
        if not self.state == "challenge":
            raise HandshakeError("Invalid state.")

        data = handshake_answer_serializer.loads(req)

        data.pop("handshake")
        self.answer_type = data.pop("type")
        self.answer_data = data
        self.state = "answer"

        # API Version check
        # Is this necessary since the client is supposed to perform this test first?
        HandshakeAPIVersionError.check_api_version(self.client_api_version)

    def build_bad_format_result_req(self, help="Invalid params") -> bytes:
        if self.state not in ("answer", "challenge"):
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "bad_format", "help": help}
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

    def build_rvk_mismatch_result_req(self, help=None) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        help = help or (
            "Root verify key for organization `{self.organization_id}` "
            "differs between client and server"
        )

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


class BaseClientHandshake:
    challenge_data = attr.ib(default=None)

    @property
    def backend_api_version(self):
        if self.challenge_data is None:
            raise TypeError("The answer data is not available yet")
        return self.challenge_data["api_version"]

    def load_challenge_req(self, req: bytes):
        self.challenge_data = handshake_challenge_serializer.loads(req)
        HandshakeAPIVersionError.check_api_version(self.backend_api_version)

    def process_result_req(self, req: bytes) -> bytes:
        data = handshake_result_serializer.loads(req)
        if data["result"] != "ok":
            if data["result"] == "bad_identity":
                raise HandshakeBadIdentity(data["help"])

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

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        answer = self.user_signkey.sign(self.challenge_data["challenge"])
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "authenticated",
                "api_version": __api_version__,
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

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "anonymous",
                "api_version": __api_version__,
                "organization_id": self.organization_id,
                "rvk": self.root_verify_key,
            }
        )


@attr.s
class AdministrationClientHandshake(BaseClientHandshake):
    token = attr.ib()

    def process_challenge_req(self, req: bytes) -> bytes:
        self.load_challenge_req(req)
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "administration",
                "api_version": __api_version__,
                "token": self.token,
            }
        )
