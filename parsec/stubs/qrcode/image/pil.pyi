from typing import Any, BinaryIO
import qrcode.image.base

class PilImage(qrcode.image.base.BaseImage):
    def save(self, stream: BinaryIO, **kwargs: Any) -> None: ...
