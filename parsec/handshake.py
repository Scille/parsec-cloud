import attr
import nacl.utils
from nacl.exceptions import BadSignatureError

from parsec import schema_fields as fields
from parsec.schema import UnknownCheckedSchema


class HandshakeError(Exception):
    pass


class HandshakeFormatError(HandshakeError):
    pass


class HandshakeBadIdentity(HandshakeError):
    pass


class HandshakeChallengeSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant('challenge', required=True)
    challenge = fields.Base64Bytes(required=True)


class HandshakeAnswerSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant('answer', required=True)
    identity = fields.String(required=True)
    answer = fields.Base64Bytes(required=True)


class HandshakeResultSchema(UnknownCheckedSchema):
    handshake = fields.CheckedConstant('result', required=True)
    result = fields.String(required=True)


@attr.s
class ServerHandshake:
    challenge_size = attr.ib(default=48)
    challenge = attr.ib(default=None)
    answer = attr.ib(default=None)
    state = attr.ib(default='stalled')

    def build_challenge_req(self):
        assert self.state == 'stalled'

        self.challenge = nacl.utils.random(self.challenge_size)
        data, _ = HandshakeChallengeSchema().dump({
            'handshake': 'challenge',
            'challenge': self.challenge
        })

        self.state = 'challenge'
        return data

    def process_answer_req(self, req):
        assert self.state == 'challenge'

        data, errors = HandshakeAnswerSchema().load(req)
        if errors:
            raise HandshakeFormatError('Invalid `answer` request %s: %s' % (req, errors))
        self.answer = data['answer']
        self.identity = data['identity']

        self.state = 'answer'

    def build_bad_format_result_req(self):
        assert self.state == 'answer'
        self.state = 'result'
        data, _ = HandshakeResultSchema().dump({
            'handshake': 'result',
            'result': 'bad_format',
        })
        return data

    def build_bad_identity_result_req(self):
        assert self.state == 'answer'
        self.state = 'result'
        data, _ = HandshakeResultSchema().dump({
            'handshake': 'result',
            'result': 'bad_identity',
        })
        return data

    def build_result_req(self, verify_key):
        assert self.state == 'answer'

        try:
            returned_challenge = verify_key.verify(self.answer)
            if returned_challenge != self.challenge:
                raise HandshakeFormatError('Invalid returned challenge')
        except BadSignatureError:
            raise HandshakeFormatError('Invalid answer signature')

        self.state = 'result'
        data, _ = HandshakeResultSchema().dump({
            'handshake': 'result',
            'result': 'ok',
        })
        return data


@attr.s
class ClientHandshake:
    user = attr.ib()

    def process_challenge_req(self, req):
        data, errors = HandshakeChallengeSchema().load(req)
        if errors:
            raise HandshakeFormatError('Invalid `challenge` request %s: %s' % (req, errors))
        answer = self.user.signkey.sign(data['challenge'])
        rep, _ = HandshakeAnswerSchema().dump({
            'handshake': 'answer',
            'identity': self.user.id,
            'answer': answer
        })
        return rep

    def process_result_req(self, req):
        data, errors = HandshakeResultSchema().load(req)
        if errors:
            raise HandshakeFormatError('Invalid `result` request %s: %s' % (req, errors))
        if data['result'] != 'ok':
            if data['result'] == 'bad_identity':
                raise HandshakeBadIdentity("Backend didn't recognized our identity")
            else:
                raise HandshakeFormatError('Bad result for result handshake: %s' % data['result'])
