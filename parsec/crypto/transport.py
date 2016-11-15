import zmq


class SecurityTransportLayerError(Exception):
    pass


class SecurityTransportLayer:

    def __init__(self, addr='tcp://127.0.0.1:5001'):
        self._context = zmq.Context()
        self._socket = self._context.socket(zmq.REQ)
        self._socket.connect(addr)

    def __ll_com(self, cmd, params):
        self._socket.send_json({'cmd': cmd, 'params': params})
        ret = self._socket.recv_json()
        if not ret['ok']:
            raise SecurityTransportLayerError(ret.get('reason'))
        else:
            return ret['data']

    def encrypt(self, content):
        return self.__ll_com('encrypt', {'content': content}).encode()

    def decrypt(self, content):
        return self.__ll_com('decrypt', {'content': content}).encode()
