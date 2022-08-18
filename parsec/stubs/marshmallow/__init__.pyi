# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Dict, Tuple, Callable, TypeVar
from .fields import Field

class ValidationError(Exception): ...

MarshalResult: Tuple[dict, dict]
UnmarshalResult: Tuple[dict, dict]
T = TypeVar("T")
post_load: Callable[[T], T]
pre_load: Callable[[T], T]
pre_dump: Callable[[T], T]
post_dump: Callable[[T], T]

class MarshmallowSchema:
    def __init__(
        self,
        only=(),
        exclude=(),
        prefix="",
        strict=None,
        many=False,
        context=None,
        load_only=(),
        dump_only=(),
        partial=False,
    ): ...
    def dump(self, obj): ...
    def dumps(self, obj, *args, **kwargs): ...
    def load(self, data): ...
    def loads(self, data): ...

    _declared_fields: Dict[str, Field]
