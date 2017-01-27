import zmq
from google.protobuf.message import DecodeError

from .abstract import BaseServer, BaseClient, BaseService
from .exceptions import BadMessageError


class ResRepServer(BaseServer):
    def __init__(self, service, addr='tcp://127.0.0.1:5000', **kwargs):
        super().__init__(**kwargs)
        self._stop = True
        assert isinstance(service, BaseService)
        self.service = service
        self._context = zmq.Context()
        self._socket = self._context.socket(zmq.REP)
        self._socket.bind(addr)

    def start(self):
        self._stop = False
        while not self._stop:
            msg = self._socket.recv()
            ret = self.service.dispatch_raw_msg(msg)
            self._socket.send(ret)

    def stop(self):
        self._stop = True


class ResRepClientMixin(BaseClient):
    def __init__(self, addr='tcp://127.0.0.1:5000', **kwargs):
        super().__init__(**kwargs)
        self._context = zmq.Context()
        self._socket = self._context.socket(zmq.REQ)
        self._socket.connect(addr)

    def _ll_communicate(self, **kwargs):
        request = self.request_cls(**kwargs)
        self._socket.send(request.SerializeToString())
        raw_response = self._socket.recv()
        try:
            response = self.response_cls()
            response.ParseFromString(raw_response)
            return response
        except DecodeError as exc:
            raise BadMessageError(exc)


class LocalClientMixin(BaseClient):
    """
    Don't go through zmq but instead directly send the protocol buffer
    to the service instance within the same process.
    """

    def __init__(self, service: BaseService, **kwargs):
        super().__init__(**kwargs)
        self.service = service

    def _ll_communicate(self, **kwargs):
        request = self.request_cls(**kwargs)
        return self.service.dispatch_msg(request)
