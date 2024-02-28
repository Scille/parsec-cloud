# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from enum import Enum, auto
from typing import Final


# Unset singleton used as default value in function parameter when `None`
# can be a valid value.
# We implement this as an enum to satisfy type checker (see
# https://github.com/python/typing/issues/689#issuecomment-561425237)
class UnsetType(Enum):
    Unset = auto()


Unset: Final = UnsetType.Unset


class BadOutcome:
    pass


class BadOutcomeEnum(BadOutcome, Enum):
    pass
