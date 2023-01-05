# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sys
from typing import Any, Callable, TypeVar

# Stop mypy type analysis here on non-win32 platforms
assert sys.platform == "win32"
from ctypes import FormatError, GetLastError, WinDLL
from ctypes.wintypes import BOOL, HANDLE, LPCWSTR, LPVOID

ERROR_ALREADY_EXISTS = 183

T = TypeVar("T")


def make_error(function: Any, function_name: str | None = None) -> WindowsError:
    code = GetLastError()
    description = FormatError(code).strip()
    if function_name is None:
        function_name = function.__name__
    exception = WindowsError(function_name)
    exception.winerror = code
    exception.strerror = description
    return exception


def check_null(result: T, function: Any, arguments: object, *args: object) -> T:
    if result is None:
        raise make_error(function)
    return result


def check_true(result: T, function: Any, arguments: object, *args: Any) -> T:
    if not result:
        raise make_error(function)
    return result


class Libraries:
    def __getattr__(self, name: str) -> WinDLL:
        library = WinDLL(name)
        self.__dict__[name] = library
        return library


dlls = Libraries()


def function_factory(
    function: Any,
    argument_types: list[Any] | None = None,
    return_type: Any | None = None,
    error_checking: Callable[..., T] | None = None,
) -> Any:
    if argument_types is not None:
        function.argtypes = argument_types
    function.restype = return_type
    if error_checking is not None:
        # Assign to this method is valid
        function.errcheck = error_checking
    return function


CreateMutex = function_factory(
    dlls.kernel32.CreateMutexW, [LPVOID, BOOL, LPCWSTR], HANDLE, check_null
)


CloseHandle = function_factory(dlls.kernel32.CloseHandle, [HANDLE], BOOL, check_true)
