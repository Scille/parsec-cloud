from typing import Callable, TypeVar

from typing_extensions import ParamSpec

P = ParamSpec("P")
R = TypeVar("R")

def post_load(
    fn: Callable[P, R], pass_many: bool = ..., pass_original: bool = ...
) -> Callable[P, R]: ...
