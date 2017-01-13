from abc import ABCMeta, abstractmethod
from google.protobuf.message import Message as ProtoBufMsg


class BaseServer(metaclass=ABCMeta):

    @abstractmethod
    def start(self):
        pass  # pragma: no cover

    @abstractmethod
    def stop(self):
        pass  # pragma: no cover


class BaseClient(metaclass=ABCMeta):

    """
    Connect to a :class:`BaseServer`
    """

    @abstractmethod
    def _ll_communicate(self, **kwargs) -> ProtoBufMsg:
        pass  # pragma: no cover

    @property
    @abstractmethod
    def request_cls(self) -> ProtoBufMsg:
        pass  # pragma: no cover

    @property
    @abstractmethod
    def response_cls(self) -> ProtoBufMsg:
        pass  # pragma: no cover


class BaseService(metaclass=ABCMeta):

    @abstractmethod
    def dispatch_msg(self, msg: ProtoBufMsg) -> ProtoBufMsg:
        pass  # pragma: no cover

    @abstractmethod
    def dispatch_raw_msg(self, msg: bytes) -> bytes:
        pass  # pragma: no cover
