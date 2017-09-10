import time
from inspect import iscoroutine

from . import TypeDispatcher, Effect
from .intents import Delay


base_sync_dispatcher = TypeDispatcher({
    Delay: lambda dispatcher, intent: time.sleep(intent.delay)
})



def sync_perform(dispatcher, effect):
    if iscoroutine(effect):
        coro = effect
    elif isinstance(effect, Effect):
        intent = effect.intent
        coro = dispatcher(intent)(intent)
        if not iscoroutine(coro):
            return coro
    else:
        raise AssertionError('effect should be either an `Effect` or a'
                             ' coroutine awaiting `Effect`s')
    return _perform_coroutine(dispatcher, coro)


def _perform_coroutine(dispatcher, coro):
    try:
        exc = res = None
        while True:
            if exc:
                sub_effect = coro.throw(exc)
                exc = None
            else:
                sub_effect = coro.send(res)
            if isinstance(sub_effect, Effect):
                intent = sub_effect.intent
                try:
                    sub_coro = dispatcher(intent)(intent)
                    if not iscoroutine(sub_coro):
                        res = sub_coro
                    else:
                        res = _perform_coroutine(dispatcher, sub_coro)
                except Exception as e:
                    exc = e
            else:
                raise AssertionError('`sync_perform` can only await `Effect`'
                                     ' (awaited: %s)' % sub_effect)
    except StopIteration as exc:
        return exc.value
