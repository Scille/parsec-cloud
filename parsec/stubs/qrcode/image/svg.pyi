from typing import Any, BinaryIO
import qrcode.image.base

class SvgFragmentImage(qrcode.image.base.BaseImage):
    def save(self, stream: BinaryIO, **kwargs: Any) -> None: ...

class SvgImage(SvgFragmentImage):
    pass

class SvgPathImage(SvgImage):
    pass
