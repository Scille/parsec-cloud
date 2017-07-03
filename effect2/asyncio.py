import asyncio

from . import ChainedIntent, TypeDispatcher, Effect
from .intents import Delay


async def perform_delay(dispatcher, intent):
    await asyncio.sleep(intent.delay)


base_asyncio_dispatcher = TypeDispatcher({
    Delay: perform_delay
})


async def asyncio_perform(dispatcher, effect):
    intent = effect.intent
    if isinstance(intent, ChainedIntent):
        try:
            sub_effect = next(intent.generator)
            while True:
                assert isinstance(sub_effect, Effect), (
                    '`ChainedIntent` generator must only yield `Effect` '
                    'objects (got %s)' % sub_effect)
                try:
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
        else:
            return ret
