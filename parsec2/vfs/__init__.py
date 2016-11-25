from ..base import BaseConponentClient
from .vfs_pb2 import (
    Request, RequestWithContent, Response, ReadFileResponse, ListDirResponse)


def bind(input_type, output_type=None):
    def wrapper(func):

        @wraps(func)
        def wrap(input: input_type, **kwargs):
            if args:
            if output == None:
                assert isinstance(ret, output_type)

        return wrap

    return wrapper


class BaseClient:

    def __init__(self, addr='tcp://127.0.0.1:5000'):
        self._context = zmq.Context()
        self._socket = self._context.socket(zmq.REQ)
        self._socket.connect(addr)

    def _ll_send_request(type, request):
        root_request = RootRequest(type=type, body=request.SerializeToString())
        return Response(self._ll_communicate(root_request.SerializeToString()))

    def _ll_communicate(self, payload):
        self._socket.send(payload)
        return self._socket.recv()


class BaseServer:
    def __init__(self, addr='tcp://127.0.0.1:5000'):
        self._context = zmq.Context()
        self._socket = self._context.socket(zmq.REP)
        self._socket.bind(addr)
        self._run = False

    def start_server(self, payload):
        self._run = True
        while self._run:
            msg = socket.recv_json()

            cmd = msg.get('cmd')
            params = msg.get('params')
            try:
                print('==>', cmd, params)
                data = vfs.cmd_dispach(cmd, params)
            except ParsecVFSException as exc:
                ret = {'ok': False, 'reason': str(exc)}
            else:
                ret = {'ok': True}
                if data is not None:
                    ret['data'] = data
            print('<==', ret)
            socket.send_json(ret)

        self._socket.send(payload)
        return self._socket.recv()



class VFSClient(BaseClient):

    def create_file(request: RequestWithContent) -> Response:
        return Response(**self.__ll_communicate(RequestType.CREATE_FILE, request.SerializeToString()))

    def read_file(request: Request) -> ReadFileResponse:
        return ReadFileResponse(**self.__ll_communicate(RequestType.READ_FILE, request.SerializeToString()))

    def write_file(request: RequestWithContent) -> Response:
        return Response(**self.__ll_communicate(RequestType.WRITE_FILE, request.SerializeToString()))

    def delete_file(request: Request) -> Response:
        return Response(**self.__ll_communicate(RequestType.DELETE_FILE, request.SerializeToString()))

    def stat(request: Request) -> Response:
        return Response(**self.__ll_communicate(RequestType.STAT, request.SerializeToString()))

    def list_dir(request: Request) -> ListDirResponse:
        return ListDirResponse(**self.__ll_communicate(RequestType.LIST_DIR, request.SerializeToString()))

    def make_dir(request: Request) -> Response:
        return Response(**self.__ll_communicate(RequestType.MAKE_DIR, request.SerializeToString()))

    def remove_dir(request: Request) -> Response:
        return Response(**self.__ll_communicate(RequestType.REMOVE_DIR, request.SerializeToString()))


class VFSServer:


def client_factory(name, interface_cls):
    nmspc = {}
    for key, method in interface_cls.__dict__:

        @wraps(method)
        def wrap(self, request):
            return self.__ll_communicate(request)

        nmspc[key] = wrap

    return type(name, (BaseClient, ), nmspc)

class MetaBaseInterface(type):

    def __new__(cls, name, bases, nmspc):
        methods = {}
        for key, value in nmspc.items():
            if isinstance(value, method):
                methods[key] =
        nmspc.update(methods)
        # If user has passed parent documents as implementation, we need
        # to retrieve the original templates
        cooked_bases = []
        for base in bases:
            if issubclass(base, Implementation):
                base = base.opts.template
            cooked_bases.append(base)
        return type.__new__(cls, name, tuple(cooked_bases), nmspc)


class BaseInterface(metaclass=MetaBaseInterface):
    pass
