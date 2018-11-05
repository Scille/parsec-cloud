from parsec.trio_bonus import monkey_patch
from parsec._version import __version__


monkey_patch()


__all__ = ("__version__",)
