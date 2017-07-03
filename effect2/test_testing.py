import attr
import asyncio
import pytest

from . import Effect, asyncio_perform, sync_perform, TypeDispatcher, ChainedIntent, do
from .testing import perform_sequence, const, conste, noop

@attr.s
class EDo:
    arg = attr.ib()

@do
def do_2_things():
    return [(yield Effect(EDo(0))), (yield Effect(EDo(1)))]


def test_perform_sequence():
    effect = do_2_things()
    sequence = [
        (EDo(0), lambda intent: intent.arg),
        (EDo(1), lambda _: 'one'),
    ]
    ret = perform_sequence(sequence, effect)
    assert ret == [0, 'one']


def test_use_const_noop_shortcuts():
    effect = do_2_things()
    sequence = [
        (EDo(0), const(0)),
        (EDo(1), noop),
    ]
    ret = perform_sequence(sequence, effect)
    assert ret == [0, None]


def test_use_conste_shortcut():
    class MyExc(Exception):
        pass

    effect = do_2_things()
    sequence = [
        (EDo(0), noop),
        (EDo(1), conste(MyExc('foo'))),
    ]
    with pytest.raises(MyExc) as exc:
        perform_sequence(sequence, effect)
    assert exc.value.args == ('foo', )


def test_perform_sequence_too_many_effects():
    effect = do_2_things()
    sequence = [
        (EDo(0), const(0)),
    ]
    with pytest.raises(AssertionError):
        perform_sequence(sequence, effect)


def test_bad_perform_sequence_too_few_effects():
    effect = do_2_things()
    sequence = [
        (EDo(0), const(0)),
        (EDo(1), const('one')),
        (EDo(1), const('one')),
    ]
    with pytest.raises(AssertionError):
        perform_sequence(sequence, effect)


def test_bad_perform_sequence_wrong_effect():
    effect = do_2_things()
    sequence = [
        (EDo(0), const(0)),
        (EDo(0), const('one')),
    ]
    with pytest.raises(AssertionError):
        perform_sequence(sequence, effect)
