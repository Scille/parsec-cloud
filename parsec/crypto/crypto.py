from json import loads, dumps, JSONDecodeError
from parsec.crypto.aes import AESCipher, AESCipherError
from parsec.crypto.rsa import RSACipher, RSACipherError
import base64


def _content_wrap(content):
    return base64.encodebytes(content).decode()


def _content_unwrap(wrapped_content):
    return base64.decodebytes(wrapped_content.encode())


class CryptoEngineError(Exception):
    pass


# TODO : Make this class able to load other crypto drivers than RSA (DSA, custom ones, etc.)
class CryptoEngine:

    def __init__(self, key_file=None):
        if key_file:
            self.load_key(key_file)

    def generate_key(self, key_size, file=None, passphrase=None):
        self.key = RSACipher.generate_key()
        with open(file, 'w') as f:
            f.write(self.key.exportKey(passphrase=passphrase))

    def load_key(self, file, passphrase=None):
        with open(file, 'r') as f:
            self.key = RSACipher.load_key(key=f.read(), passphrase=passphrase)

    def encrypt(self, content):
        signature = RSACipher.sign(self.key, content)
        aes_key, enc = AESCipher.encrypt(content)
        encrypted_key = RSACipher.encrypt(self.key, aes_key)
        encrypted_data = {'content': _content_wrap(enc),
                          'headers': {'signature': signature,
                                      'encryption_key': _content_wrap(encrypted_key)}
                          }
        return dumps(encrypted_data)

    def decrypt(self, content):
        # Get key legth
        try:
            encrypted = loads(content)
            encrypted['content'] = _content_unwrap(encrypted['content'])
            encrypted['headers']['encryption_key'] = _content_unwrap(
                encrypted['headers']['encryption_key'])
        except (KeyError, JSONDecodeError):
            raise CryptoEngineError('Cannot parse encrypted content')
        try:
            aes_key = RSACipher.decrypt(self.key, encrypted['headers']['encryption_key'])
            dec = AESCipher.decrypt(encrypted['content'], aes_key)
        except RSACipherError:
            raise CryptoEngineError('Cannot decrypt AES key')
        except AESCipherError:
            raise CryptoEngineError('Cannot Decrypt file content')
        # check signature, we have only one key yet.
        if not RSACipher.verify(self.key, dec, encrypted['headers']['signature']):
            raise CryptoEngineError('Invalid signature')
        return dec

    def cmd_dispach(self, cmd, params):
        cmd = cmd.lower()
        if hasattr(self, cmd) and cmd in ('encrypt', 'decrypt'):
            try:
                ret = getattr(self, cmd)(**params)
            except TypeError as e:
                raise CryptoEngineError('Bad params for cmd `%s`' % cmd)
        else:
            raise CryptoEngineError('Unknown cmd `%s`' % cmd)

        return ret


def main(addr='tcp://127.0.0.1:5001'):
    import zmq
    from os.path import expanduser
    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind(addr)

    crypto_engine = CryptoEngine(expanduser("~/.parsec/rsa_key"))
    while True:
        msg = socket.recv_json()
        cmd = msg.get('cmd')
        params = msg.get('params')
        try:
            print('==>', cmd, params)
            data = crypto_engine.cmd_dispach(cmd, params)
        except CryptoEngineError as exc:
            ret = {'ok': False, 'reason': str(exc)}
        else:
            ret = {'ok': True}
            if data is not None:
                ret['data'] = data
        print('<==', ret)
        socket.send_json(ret)

if __name__ == "__main__":
    main()
