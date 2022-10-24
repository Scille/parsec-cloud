from __future__ import annotations

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Dict, Callable, TypeVar
from .fields import Field

class ValidationError(Exception):
    @property
    def messages(self) -> list[str] | str | dict[str, str]: ...

class MarshalResult:
    def __init__(self, result_data, result_errors) -> None: ...
    @property
    def data(self): ...
    @property
    def errors(self): ...

class UnmarshalResult:
    def __init__(self, result_data, result_errors) -> None: ...
    @property
    def data(self): ...
    @property
    def errors(self): ...

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

class Schema:
    def dump(self, obj, many, update_fields: bool, **kwargs): ...
    def dumps(self, obj, many, update_fields: bool, *args, **kwargs): ...
    def load(self, data, many, partial): ...
    def loads(self, json_data, many, *args, **kwargs): ...
    @property
    def many(self): ...
    @property
    def strict(self): ...
    @property
    def partial(self): ...

    _declared_fields: Dict[str, Field]
