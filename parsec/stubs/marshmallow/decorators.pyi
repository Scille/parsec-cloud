# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from typing import Callable, TypeVar

from typing_extensions import ParamSpec

P = ParamSpec("P")
R = TypeVar("R")

def post_load(
    fn: Callable[P, R], pass_many: bool = ..., pass_original: bool = ...
) -> Callable[P, R]: ...
