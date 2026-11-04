# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from email.message import Message

import anyio
import pytest


class LetterBox:
    def __init__(self):
        self._send_email, self._recv_email = anyio.create_memory_object_stream(10)
        self.emails = []

    async def get_next(self) -> tuple[str, Message]:
        return await self._recv_email.receive()

    def count(self) -> int:
        return len(self.emails)

    def _push(self, to_addr: str, message: Message) -> None:
        email = (to_addr, message)
        self._send_email.send_nowait(email)
        self.emails.append(email)


@pytest.fixture
def email_invite_letterbox(monkeypatch: pytest.MonkeyPatch) -> LetterBox:
    letterbox = LetterBox()

    async def _mocked_send_email(email_config, to_addr: str, message: Message) -> None:
        letterbox._push(to_addr, message)

    monkeypatch.setattr("parsec.components.invite.send_email", _mocked_send_email)
    return letterbox


@pytest.fixture
def email_account_letterbox(monkeypatch: pytest.MonkeyPatch) -> LetterBox:
    letterbox = LetterBox()

    async def _mocked_send_email(email_config, to_addr: str, message: Message) -> None:
        letterbox._push(to_addr, message)

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    return letterbox


@pytest.fixture
def email_totp_reset_letterbox(monkeypatch: pytest.MonkeyPatch) -> LetterBox:
    letterbox = LetterBox()

    async def _mocked_send_email(email_config, to_addr: str, message: Message) -> None:
        letterbox._push(to_addr, message)

    monkeypatch.setattr("parsec.components.totp.send_email", _mocked_send_email)
    return letterbox
