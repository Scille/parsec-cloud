import os
from .trio_bonus import monkey_patch


monkey_patch()

os.environ["MARSHMALLOW_SCHEMA_DEFAULT_JIT"] = "toastedmarshmallow.Jit"


__version__ = "0.5.0"
