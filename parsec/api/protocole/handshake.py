# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from secrets import token_bytes

from parsec.crypto import CryptoError
from parsec.serde import UnknownCheckedSchema, fields
from parsec.api.protocole.base import ProtocoleError, InvalidMessageError, serializer_factory


class HandshakeError(ProtocoleError):
    pass


class HandshakeFailedChallenge(HandshakeError):
    pass


class HandshakeBadIdentity(HandshakeError):
    pass


class HandshakeRVKMismatch(HandshakeError):
    pass


class HandshakeRevokedDevice(HandshakeError):
    pass


class HandshakeChallengeSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("challenge", required=True)
    challenge = fields.Bytes(required=True)


handshake_challenge_serializer = serializer_factory(HandshakeChallengeSchema)


class HandshakeAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    organization_id = fields.OrganizationID(required=True)
    device_id = fields.DeviceID(allow_none=True, missing=None)
    rvk = fields.VerifyKey(allow_none=True, missing=None)
    answer = fields.Bytes(allow_none=True, missing=None)


handshake_answer_serializer = serializer_factory(HandshakeAnswerSchema)


class HandshakeResultSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("result", required=True)
    result = fields.String(required=True)


handshake_result_serializer = serializer_factory(HandshakeResultSchema)


@attr.s
class ServerHandshake:
    challenge_size = attr.ib(default=48)
    challenge = attr.ib(default=None)
    answer = attr.ib(default=None)
    device_id = attr.ib(default=None)
    root_verify_key = attr.ib(default=None)
    state = attr.ib(default="stalled")

    def is_anonymous(self):
        return self.device_id is None

    def build_challenge_req(self) -> bytes:
        if not self.state == "stalled":
            raise HandshakeError("Invalid state.")

        self.challenge = token_bytes(self.challenge_size)
        self.state = "challenge"

        return handshake_challenge_serializer.dumps(
            {"handshake": "challenge", "challenge": self.challenge}
        )

    def process_answer_req(self, req: bytes):
        if not self.state == "challenge":
            raise HandshakeError("Invalid state.")

        data = handshake_answer_serializer.loads(req)

        defined_fields = [x for x in [data["device_id"], data["answer"]] if x]
        if len(defined_fields) not in (0, 2):
            raise InvalidMessageError("Field device_id and answer must be set together")

        self.answer = data["answer"] or b""
        self.organization_id = data["organization_id"]
        self.root_verify_key = data["rvk"]
        self.device_id = data["device_id"]
        self.state = "answer"

    def build_bad_format_result_req(self) -> bytes:
        if self.state not in ("answer", "challenge"):
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "bad_format"})

    def build_bad_identity_result_req(self) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "bad_identity"})

    def build_rvk_mismatch_result_req(self) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "rvk_mismatch"})

    def build_revoked_device_result_req(self) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps(
            {"handshake": "result", "result": "revoked_device"}
        )

    def build_result_req(self, verify_key=None) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        if verify_key:
            try:
                returned_challenge = verify_key.verify(self.answer)
                if returned_challenge != self.challenge:
                    raise HandshakeFailedChallenge("Invalid returned challenge")

            except CryptoError as exc:
                raise HandshakeFailedChallenge("Invalid answer signature") from exc

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "ok"})


@attr.s
class ClientHandshake:
    organization_id = attr.ib()
    device_id = attr.ib()
    user_signkey = attr.ib()
    root_verify_key = attr.ib()

    def process_challenge_req(self, req: bytes) -> bytes:
        data = handshake_challenge_serializer.loads(req)
        answer = self.user_signkey.sign(data["challenge"])
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "organization_id": self.organization_id,
                "device_id": self.device_id,
                "rvk": self.root_verify_key,
                "answer": answer,
            }
        )

    def process_result_req(self, req: bytes) -> bytes:
        data = handshake_result_serializer.loads(req)
        if data["result"] != "ok":
            if data["result"] == "bad_identity":
                raise HandshakeBadIdentity("Backend didn't recognized our identity")

            elif data["result"] == "rvk_mismatch":
                raise HandshakeRVKMismatch(
                    "Backend doesn't agree on the root verify key"
                    f" for organization `{self.organization_id}`"
                )

            elif data["result"] == "revoked_device":
                raise HandshakeRevokedDevice("Backend rejected revoked device")

            else:
                raise InvalidMessageError(f"Bad `result` handshake: {data['result']}")


@attr.s
class AnonymousClientHandshake:
    organization_id = attr.ib()
    root_verify_key = attr.ib(default=None)

    def process_challenge_req(self, req: bytes) -> bytes:
        handshake_challenge_serializer.loads(req)  # Sanity check
        return handshake_answer_serializer.dumps(
            {
                "handshake": "answer",
                "organization_id": self.organization_id,
                "rvk": self.root_verify_key,
            }
        )

    def process_result_req(self, req: bytes) -> bytes:
        data = handshake_result_serializer.loads(req)
        if data["result"] != "ok":
            if data["result"] == "bad_identity":
                raise HandshakeBadIdentity("Backend didn't recognized our identity")

            elif data["result"] == "rvk_mismatch":
                raise HandshakeRVKMismatch(
                    "Backend doesn't agree on the root verify key"
                    f" for organization `{self.organization_id}`"
                )

            else:
                raise InvalidMessageError(f"Bad `result` handshake: {data['result']}")
