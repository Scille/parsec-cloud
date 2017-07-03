import time

from . import ChainedIntent, TypeDispatcher, Effect
from .intents import Delay


base_sync_dispatcher = TypeDispatcher({
    Delay: lambda dispatcher, intent: time.sleep(intent.delay)
})


def sync_perform(dispatcher, effect):
    intent = effect.intent
    if isinstance(intent, ChainedIntent):
        try:
            sub_effect = next(intent.generator)
            while True:
                assert isinstance(sub_effect, Effect), (
                    '`ChainedIntent` generator must only yield `Effect` '
                    'objects (got %s)' % sub_effect)
                try:
                    ret = sync_perform(dispatcher, sub_effect)
                except Exception as exc:
                    sub_effect = intent.generator.throw(exc)
                else:
                    sub_effect = intent.generator.send(ret)
        except StopIteration as exc:
            return exc.value
    else:
        performer = dispatcher(intent)
        ret = performer(intent)
        if isinstance(ret, Effect):
            return sync_perform(dispatcher, ret)
        else:
            return ret

