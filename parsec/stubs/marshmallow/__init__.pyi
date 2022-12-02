# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from typing import Any, Callable, Dict, Generic, Iterable, TypeVar

from .fields import Field

class ValidationError(Exception):
    @property
    def messages(self) -> list[str] | str | dict[str, str]: ...

class MarshalResult(Generic[T]):
    def __init__(self, result_data: T, result_errors: Iterable[Any]) -> None: ...
    @property
    def data(self) -> T: ...
    @property
    def errors(self) -> list[str]: ...

class UnmarshalResult(Generic[T]):
    def __init__(self, result_data: T, result_errors: Iterable[Any]) -> None: ...
    @property
    def data(self) -> T: ...
    @property
    def errors(self) -> list[str]: ...

T = TypeVar("T")
post_load: Callable[[T], T]
pre_load: Callable[[T], T]
pre_dump: Callable[[T], T]
post_dump: Callable[[T], T]

class MarshmallowSchema(Generic[T]):
    def __init__(
        self,
        only: tuple[object, ...] = (),
        exclude: object = (),
        prefix: str = "",
        strict: object | None = None,
        many: bool = False,
        context: object | None = None,
        load_only: Iterable[T] = (),
        dump_only: Iterable[T] = (),
        partial: bool = False,
    ): ...
    def dump(self, obj: T) -> T: ...
    def dumps(self, obj: T, *args: object, **kwargs: object) -> T: ...
    def load(self, data: bytes) -> T: ...
    def loads(self, data: bytes) -> T: ...

    _declared_fields: Dict[str, Field[Any]]

class Schema:
    def dump(
        self, obj: Any, many: bool, update_fields: bool, **kwargs: object
    ) -> MarshalResult[Any]: ...
    def dumps(
        self, obj: object, many: bool, update_fields: bool, *args: object, **kwargs: object
    ) -> MarshalResult[Any]: ...
    def load(
        self, data: object, many: bool, partial: bool | tuple[object, ...]
    ) -> MarshalResult[Any]: ...
    def loads(
        self, json_data: str, many: bool, *args: object, **kwargs: object
    ) -> MarshalResult[Any]: ...
    @property
    def many(self) -> bool: ...
    @property
    def strict(self) -> bool: ...
    @property
    def partial(self) -> bool: ...

    _declared_fields: Dict[str, Field[Any]]
