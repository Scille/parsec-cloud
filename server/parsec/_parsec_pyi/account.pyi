# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

class PasswordAlgorithm:
    pass

class PasswordAlgorithmArgon2id(PasswordAlgorithm):
    def __init__(
        self,
        salt: bytes,
        opslimit: int,
        memlimit_kb: int,
        parallelism: int,
    ) -> None: ...
    @property
    def salt(self) -> bytes: ...
    @property
    def opslimit(self) -> int: ...
    @property
    def memlimit_kb(self) -> int: ...
    @property
    def parallelism(self) -> int: ...
