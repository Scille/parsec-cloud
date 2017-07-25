import attr
import asyncio
from functools import wraps

from . import ChainedIntent, TypeDispatcher, Effect
from .intents import Delay


def async_do(f):
    assert asyncio.iscoroutinefunction(f)

    @wraps(f)
    def wrapper(*args, **kwargs):
        coroutine = f(*args, **kwargs)
        return AsyncFunc(coroutine)

    return wrapper


@attr.s(init=False)
class AsyncFunc:
    """Use this to run async functions in a ``@do`` function."""
    coroutine = attr.ib()

    def __init__(self, coroutine):
        assert asyncio.iscoroutine(coroutine)
        self.coroutine = coroutine


async def perform_delay(dispatcher, intent):
    await asyncio.sleep(intent.delay)


base_asyncio_dispatcher = TypeDispatcher({
    Delay: perform_delay
})


async def asyncio_perform(dispatcher, effect):
    if isinstance(effect, AsyncFunc):
        return await effect.coroutine
    intent = effect.intent
    if isinstance(intent, ChainedIntent):
        try:
            sub_effect = next(intent.generator)
            while True:
                assert isinstance(sub_effect, (Effect, AsyncFunc)), (
                    '`ChainedIntent` generator must only yield `Effect` or '
                    '`AsyncFunc` objects (got %s)' % sub_effect)
                try:
                    if isinstance(sub_effect, AsyncFunc):
                        ret = await sub_effect.coroutine
                    else:
                        ret = await asyncio_perform(dispatcher, sub_effect)
                except Exception as exc:
                    sub_effect = intent.generator.throw(exc)
                else:
                    sub_effect = intent.generator.send(ret)
        except StopIteration as exc:
            return exc.value
    else:
        performer = dispatcher(intent)
        ret = performer(intent)
        if asyncio.iscoroutine(ret):
            ret = await ret
        if isinstance(ret, Effect):
            return await asyncio_perform(dispatcher, ret)
        elif isinstance(ret, AsyncFunc):
            return await ret.coroutine
        else:
            return ret
