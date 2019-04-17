# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from secrets import token_bytes

from parsec.crypto import CryptoError
from parsec.serde import UnknownCheckedSchema, OneOfSchema, fields
from parsec.api.protocole.base import ProtocoleError, InvalidMessageError, serializer_factory
from parsec import __api_version__


class HandshakeError(ProtocoleError):
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


class HandshakeAPIVersionError(Exception):
    def __init__(self, expected_version=None):
        self.expected_version = expected_version
        if expected_version is not None:
            self.message = f"Bad API version : expected version {expected_version}"
        else:
            self.message = "Bad API version"


class HandshakeChallengeSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("challenge", required=True)
    challenge = fields.Bytes(required=True)
    api_version = fields.SemVer(required=True)


handshake_challenge_serializer = serializer_factory(HandshakeChallengeSchema)


class HandshakeAuthenticatedAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("authenticated", required=True)
    organization_id = fields.OrganizationID(required=True)
    device_id = fields.DeviceID(required=True)
    rvk = fields.VerifyKey(required=True)
    answer = fields.Bytes(required=True)


class HandshakeAnonymousAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("anonymous", required=True)
    organization_id = fields.OrganizationID(required=True)
    # Cannot provide rvk during organization bootstrap
    rvk = fields.VerifyKey(missing=None)


class HandshakeAdministrationAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    type = fields.CheckedConstant("administration", required=True)
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
    help = fields.String(allow_none=True, missing=None)


handshake_result_serializer = serializer_factory(HandshakeResultSchema)


@attr.s
class ServerHandshake:
    challenge_size = attr.ib(default=48)
    challenge = attr.ib(default=None)
    answer_type = attr.ib(default=None)
    answer_data = attr.ib(default=None)
    state = attr.ib(default="stalled")

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

    def check_api_version(self, data):
        remote_version = tuple(map(int, data["api_version"].split(".")))
        local_version = tuple(map(int, __api_version__.split(".")))
        if remote_version[0] != local_version[0]:
            raise HandshakeAPIVersionError(data["api_version"])
        if remote_version[1] > local_version[1]:
            raise HandshakeAPIVersionError(data["api_version"])


@attr.s
class AuthenticatedClientHandshake(BaseClientHandshake):
    organization_id = attr.ib()
    device_id = attr.ib()
    user_signkey = attr.ib()
    root_verify_key = attr.ib()

    def process_challenge_req(self, req: bytes) -> bytes:
        data = handshake_challenge_serializer.loads(req)
        self.check_api_version(data)
        answer = self.user_signkey.sign(data["challenge"])
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "authenticated",
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
        data = handshake_challenge_serializer.loads(req)  # Sanity check
        self.check_api_version(data)
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "type": "anonymous",
                "organization_id": self.organization_id,
                "rvk": self.root_verify_key,
            }
        )


@attr.s
class AdministrationClientHandshake(BaseClientHandshake):
    token = attr.ib()

    def process_challenge_req(self, req: bytes) -> bytes:
        data = handshake_challenge_serializer.loads(req)  # Sanity check
        self.check_api_version(data)
        return handshake_answer_serializer.dumps(
            {"handshake": "answer", "type": "administration", "token": self.token}
        )
