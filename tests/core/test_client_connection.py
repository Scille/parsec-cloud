import pytest
import attr
from unittest.mock import Mock
from effect import Effect, Constant
from effect.do import do

from parsec.core.client_connection import (
    on_connection_factory, EPushClientMsg, EClientSubscribeEvent)
from parsec.core.base import EEvent


@attr.s
class MockedReader:
    to_read = attr.ib(default=b'')
    async def read(self, size):
        curr_read = self.to_read[:size]
        self.to_read = self.to_read[size:]
        return curr_read


@attr.s
class MockedWriter:
    written = attr.ib(default=b'')
    def write(self, buff):
        self.written += buff


@pytest.mark.asyncio
async def test_no_command():
    reader = MockedReader()
    writer = MockedWriter()
    perform_cmd = Mock()
    on_connection = on_connection_factory(perform_cmd, Mock())
    await on_connection(reader, writer)
    perform_cmd.assert_not_called()


@pytest.mark.asyncio
async def test_simple():
    reader = MockedReader(b'foo\n')
    writer = MockedWriter()
    perform_cmd = Mock()
    perform_cmd.return_value = Effect(Constant(b'bar'))
    on_connection = on_connection_factory(perform_cmd)
    await on_connection(reader, writer)
    perform_cmd.assert_called_once_with(b'foo')
    assert writer.written == b'bar\n'


@pytest.mark.asyncio
async def test_mix_cmds_and_pushed_msgs():
    reader = MockedReader(b'cmd1\ncmd2\ncmd3\n')
    writer = MockedWriter()

    @do
    def perform_cmd(cmd):
        id = cmd[-1:]
        yield Effect(EPushClientMsg(b'eventA' + id))
        yield Effect(EPushClientMsg(b'eventB' + id))
        return b'cmd_resp' + id

    on_connection = on_connection_factory(perform_cmd)
    await on_connection(reader, writer)
    template = b'cmd_respX\neventAX\neventBX\n'
    expected_written = b''.join([template.replace(b'X', id) for id in (b'1', b'2', b'3')])
    assert writer.written == expected_written


@pytest.mark.asyncio
async def test_events():
    reader = MockedReader(b'cmd\n')
    writer = MockedWriter()

    @do
    def perform_cmd(cmd):
        yield Effect(EClientSubscribeEvent('eventA', 'sender1'))
        yield Effect(EClientSubscribeEvent('eventA', 'sender2'))
        yield Effect(EClientSubscribeEvent('eventB', 'sender1'))
        yield Effect(EEvent('eventC', 'sender1'))
        yield Effect(EEvent('eventA', 'sender3'))
        yield Effect(EEvent('eventA', 'sender1'))
        return b'cmd_resp'

    on_connection = on_connection_factory(perform_cmd)
    await on_connection(reader, writer)

    expected_possibilities = (b'cmd_resp\n{"sender": "sender1", "event": "eventA"}\n',
                              b'cmd_resp\n{"event": "eventA", "sender": "sender1"}\n')
    assert writer.written in expected_possibilities
