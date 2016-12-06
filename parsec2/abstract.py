from abc import ABCMeta, abstractmethod
from google.protobuf.message import Message as ProtoBufMsg


class BaseServer(metaclass=ABCMeta):
    @abstractmethod
    def start(self):
        pass

    @abstractmethod
    def stop(self):
        pass


class BaseClient(metaclass=ABCMeta):
    """
    Connect to a :class:`BaseServer`
    """

    @abstractmethod
    def _ll_communicate(self, **kwargs) -> ProtoBufMsg:
        pass

    @property
    @abstractmethod
    def request_cls(self) -> ProtoBufMsg:
        pass

    @property
    @abstractmethod
    def response_cls(self) -> ProtoBufMsg:
        pass


class BaseService(metaclass=ABCMeta):
    @abstractmethod
    def dispatch_msg(self, msg: ProtoBufMsg) -> ProtoBufMsg:
        pass

    @abstractmethod
    def dispatch_raw_msg(self, msg: bytes) -> bytes:
        pass
