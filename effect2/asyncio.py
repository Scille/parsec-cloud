import attr
import asyncio
from asyncio import Future
from inspect import iscoroutine

from . import TypeDispatcher, Effect
from .intents import Delay


async def perform_delay(intent):
    await asyncio.sleep(intent.delay)


base_asyncio_dispatcher = TypeDispatcher({
    Delay: perform_delay,
})



async def asyncio_perform(dispatcher, effect):
    if iscoroutine(effect):
        coro = effect
    elif isinstance(effect, Effect):
        intent = effect.intent
        coro = dispatcher(intent)(intent)
        if not iscoroutine(coro):
            return coro
    else:
        raise AssertionError('effect should be either an `Effect` or a'
                             ' coroutine awaiting `Effect`s or `asyncio.Future`s')
    return await _perform_coroutine(dispatcher, coro)


async def _perform_coroutine(dispatcher, coro):
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
                        res = await _perform_coroutine(dispatcher, sub_coro)
                except Exception as e:
                    exc = e
            elif isinstance(sub_effect, Future):
                sub_effect._asyncio_future_blocking = False  # Hackity hack
                try:
                    res = await sub_effect
                except Exception as e:
                    exc = e
            elif sub_effect is None:
                await asyncio.sleep(0)
            else:
                raise AssertionError('`asyncio_perform` can only await `Effect` '
                                     'or `asyncio.Future` (awaited: %s)' % sub_effect)
    except StopIteration as exc:
        return exc.value
