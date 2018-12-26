import attr
from secrets import token_bytes

from parsec.crypto import CryptoError
from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.protocole.base import ProtocoleError, Serializer


class HandshakeError(ProtocoleError):
    pass


class HandshakeFormatError(HandshakeError):
    pass


class HandshakeBadIdentity(HandshakeError):
    pass


class HandshakeRevokedDevice(HandshakeError):
    pass


class HandshakeChallengeSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("challenge", required=True)
    challenge = fields.Base64Bytes(required=True)


handshake_challenge_serializer = Serializer(HandshakeChallengeSchema)


class HandshakeAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("answer", required=True)
    identity = fields.DeviceID(allow_none=True, missing=None)
    answer = fields.Base64Bytes(allow_none=True, missing=None)


handshake_answer_serializer = Serializer(HandshakeAnswerSchema)


class HandshakeResultSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant("result", required=True)
    result = fields.String(required=True)


handshake_result_serializer = Serializer(HandshakeResultSchema)


@attr.s
class ServerHandshake:
    challenge_size = attr.ib(default=48)
    challenge = attr.ib(default=None)
    answer = attr.ib(default=None)
    identity = attr.ib(default=None)
    state = attr.ib(default="stalled")

    def is_anonymous(self):
        return self.identity is None

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

        self.answer = data["answer"] or b""
        self.identity = data["identity"]
        self.state = "answer"

    def build_bad_format_result_req(self) -> bytes:
        if not self.state in ("answer", "challenge"):
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "bad_format"})

    def build_bad_identity_result_req(self) -> bytes:
        if not self.state == "answer":
            raise HandshakeError("Invalid state.")

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "bad_identity"})

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
                    raise HandshakeFormatError("Invalid returned challenge")

            except CryptoError as exc:
                raise HandshakeFormatError("Invalid answer signature") from exc

        self.state = "result"
        return handshake_result_serializer.dumps({"handshake": "result", "result": "ok"})


@attr.s
class ClientHandshake:
    user_id = attr.ib()
    user_signkey = attr.ib()

    def process_challenge_req(self, req: bytes) -> bytes:
        data = handshake_challenge_serializer.loads(req)
        answer = self.user_signkey.sign(data["challenge"])
        return handshake_answer_serializer.dumps(
            {"handshake": "answer", "identity": self.user_id, "answer": answer}
        )

    def process_result_req(self, req: bytes) -> bytes:
        data = handshake_result_serializer.loads(req)
        if data["result"] != "ok":
            if data["result"] == "bad_identity":
                raise HandshakeBadIdentity("Backend didn't recognized our identity")
            if data["result"] == "revoked_device":
                raise HandshakeRevokedDevice("Backend rejected revoked device")

            else:
                raise HandshakeFormatError(f"Bad result for `result` handshake: {data['result']}")


@attr.s
class AnonymousClientHandshake:
    def process_challenge_req(self, req: bytes) -> bytes:
        handshake_challenge_serializer.loads(req)  # Sanity check
        return handshake_answer_serializer.dumps({"handshake": "answer"})

    def process_result_req(self, req: bytes) -> bytes:
        data = handshake_result_serializer.loads(req)
        if data["result"] != "ok":
            if data["result"] == "bad_identity":
                raise HandshakeBadIdentity("Backend didn't recognized our identity")

            else:
                raise HandshakeFormatError(f"Bad result for `result` handshake: {data['result']}")
