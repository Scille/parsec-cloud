import abc
from typing import Any, BinaryIO

class BaseImage:
    @abc.abstractmethod
    def save(self, stream: BinaryIO, **kwargs: Any) -> None: ...
