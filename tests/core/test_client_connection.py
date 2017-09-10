import pytest
import attr
from unittest.mock import Mock
from effect2 import Effect, Constant, ComposedDispatcher

from parsec.core.client_connection import (
    on_connection_factory, EPushClientMsg, EClientSubscribeEvent, EClientUnsubscribeEvent)
from parsec.base import EEvent, EventComponent, base_dispatcher


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


@pytest.fixture
def dispatcher():
    return ComposedDispatcher(
        base_dispatcher,
        EventComponent().get_dispatcher()
    )


async def test_no_command(dispatcher):
    reader = MockedReader()
    writer = MockedWriter()
    perform_cmd = Mock()
    on_connection = on_connection_factory(perform_cmd, dispatcher)
    await on_connection(reader, writer)
    perform_cmd.assert_not_called()


async def test_simple(dispatcher):
    reader = MockedReader(b'foo\n')
    writer = MockedWriter()
    perform_cmd = Mock()
    perform_cmd.return_value = Effect(Constant(b'bar'))
    on_connection = on_connection_factory(perform_cmd, dispatcher)
    await on_connection(reader, writer)
    perform_cmd.assert_called_once_with(b'foo')
    assert writer.written == b'bar\n'


async def test_mix_cmds_and_pushed_msgs(dispatcher):
    reader = MockedReader(b'cmd1\ncmd2\ncmd3\n')
    writer = MockedWriter()

    async def perform_cmd(cmd):
        id = cmd[-1:]
        await Effect(EPushClientMsg(b'eventA' + id))
        await Effect(EPushClientMsg(b'eventB' + id))
        return b'cmd_resp' + id

    on_connection = on_connection_factory(perform_cmd, dispatcher)
    await on_connection(reader, writer)
    template = b'cmd_respX\neventAX\neventBX\n'
    expected_written = b''.join([template.replace(b'X', id) for id in (b'1', b'2', b'3')])
    assert writer.written == expected_written


async def test_events(dispatcher):
    reader = MockedReader(b'cmd\n')
    writer = MockedWriter()

    async def perform_cmd(cmd):
        await Effect(EClientSubscribeEvent('eventA', 'sender1'))
        await Effect(EClientSubscribeEvent('eventA', 'sender2'))
        await Effect(EClientSubscribeEvent('eventB', 'sender1'))
        await Effect(EEvent('eventC', 'sender1'))
        await Effect(EEvent('eventA', 'sender3'))
        await Effect(EEvent('eventA', 'sender1'))
        return b'cmd_resp'

    on_connection = on_connection_factory(perform_cmd, dispatcher)
    await on_connection(reader, writer)

    assert writer.written == b'cmd_resp\n{"event": "eventA", "sender": "sender1"}\n'


async def test_unsubscribe_events(dispatcher):
    reader = MockedReader(b'cmd\n')
    writer = MockedWriter()

    async def perform_cmd(cmd):
        await Effect(EClientSubscribeEvent('eventA', 'sender1'))
        await Effect(EClientSubscribeEvent('eventA', 'sender2'))
        await Effect(EClientSubscribeEvent('eventB', 'sender1'))
        await Effect(EClientUnsubscribeEvent('eventA', 'sender1'))
        await Effect(EEvent('eventA', 'sender1'))
        await Effect(EEvent('eventA', 'sender2'))
        await Effect(EEvent('eventB', 'sender1'))
        return b'cmd_resp'

    on_connection = on_connection_factory(perform_cmd, dispatcher)
    await on_connection(reader, writer)

    assert writer.written == (
        b'cmd_resp\n'
        b'{"event": "eventA", "sender": "sender2"}\n'
        b'{"event": "eventB", "sender": "sender1"}\n')
