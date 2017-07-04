import attr
import asyncio
import pytest

from . import Effect, asyncio_perform, sync_perform, TypeDispatcher, ChainedIntent, do
from .testing import conste


def test_do():
    @attr.s
    class EDo:
        arg = attr.ib()

    @do
    def do_func(a, b):
        done_a = yield Effect(EDo(a))
        done_b = yield Effect(EDo(b))
        return [done_a, done_b]

    effect = do_func(1, 2)
    assert isinstance(effect, Effect)
    assert isinstance(effect.intent, ChainedIntent)
    dispatcher = TypeDispatcher({
        EDo: lambda intent: 'done: %s' % intent.arg
    })
    ret = sync_perform(dispatcher, effect)
    assert ret == ['done: 1', 'done: 2']


class TestSynPerform:

    def test_base(self):
        @attr.s
        class EDoSomething:
            arg = attr.ib()

        dispatcher = TypeDispatcher({
            EDoSomething: lambda intent: 'bar'
        })
        effect = Effect(EDoSomething('foo'))
        ret = sync_perform(dispatcher, effect)
        assert ret == 'bar'

    def test_chained_intent(self):
        @attr.s
        class ENumToString:
            num = attr.ib()

        def collect_intent_results():
            intent_results = []
            for i in range(5):
                res = yield Effect(ENumToString(i))
                intent_results.append(res)
            return ''.join(intent_results)

        effect = Effect(ChainedIntent(collect_intent_results()))
        dispatcher = TypeDispatcher({
            ENumToString: lambda intent: str(intent.num)
        })
        ret = sync_perform(dispatcher, effect)
        assert ret == '01234'

    def test_bad_chained_intent_dont_yield_effect(self):
        class NotAnEffect:
            pass

        def bad_yielder():
            yield NotAnEffect()

        effect = Effect(ChainedIntent(bad_yielder()))
        dispatcher = TypeDispatcher({
        })
        with pytest.raises(AssertionError) as exc:
            sync_perform(dispatcher, effect)
        assert exc.value.args[0].startswith(
            '`ChainedIntent` generator must only yield `Effect` objects (got')

    @pytest.mark.asyncio
    async def test_chained_intent_raise_exception(self):
        class DoError(Exception):
            pass

        class EDo:
            pass

        def perform_do(intent):
            raise DoError()

        def yielder():
            yield Effect(EDo())

        effect = Effect(ChainedIntent(yielder()))
        dispatcher = TypeDispatcher({
            EDo: perform_do
        })
        with pytest.raises(DoError):
            sync_perform(dispatcher, effect)

    @pytest.mark.asyncio
    async def test_chained_intent_raise_exception_and_catch_it(self):
        class DoError(Exception):
            pass

        class EBadDo:
            pass

        class EGoodDo:
            pass

        def perform_bad_do(intent):
            raise DoError()

        def yielder():
            try:
                yield Effect(EBadDo())
            except DoError:
                return (yield Effect(EGoodDo()))
            else:
                raise AssertionError("Didn't get notified of the excetion")

        effect = Effect(ChainedIntent(yielder()))
        dispatcher = TypeDispatcher({
            EBadDo: perform_bad_do,
            EGoodDo: lambda intent: 'ok'
        })
        ret = sync_perform(dispatcher, effect)
        assert ret == 'ok'

    def test_performer_return_effect(self):
        class EDoA:
            pass

        class EDoB:
            pass

        dispatcher = TypeDispatcher({
            EDoA: lambda intent: Effect(EDoB()),
            EDoB: lambda intent: 'bar'
        })
        effect = Effect(EDoA())
        ret = sync_perform(dispatcher, effect)
        assert ret == 'bar'


class TestAsynIOPerform:

    @pytest.mark.asyncio
    async def test_base(self):
        @attr.s
        class EDoSomething:
            arg = attr.ib()

        dispatcher = TypeDispatcher({
            EDoSomething: lambda intent: 'bar'
        })
        effect = Effect(EDoSomething('foo'))
        ret = await asyncio_perform(dispatcher, effect)
        assert ret == 'bar'

    @pytest.mark.asyncio
    async def test_chained_intent(self):
        @attr.s
        class ENumToString:
            num = attr.ib()

        def collect_intent_results():
            intent_results = []
            for i in range(5):
                res = yield Effect(ENumToString(i))
                intent_results.append(res)
            return ''.join(intent_results)

        effect = Effect(ChainedIntent(collect_intent_results()))
        dispatcher = TypeDispatcher({
            ENumToString: lambda intent: str(intent.num)
        })
        ret = await asyncio_perform(dispatcher, effect)
        assert ret == '01234'

    @pytest.mark.asyncio
    async def test_bad_chained_intent_dont_yield_effect(self):
        class NotAnEffect:
            pass

        def bad_yielder():
            yield NotAnEffect()

        effect = Effect(ChainedIntent(bad_yielder()))
        dispatcher = TypeDispatcher({
        })
        with pytest.raises(AssertionError) as exc:
            await asyncio_perform(dispatcher, effect)
        assert exc.value.args[0].startswith(
            '`ChainedIntent` generator must only yield `Effect` objects (got')

    @pytest.mark.asyncio
    async def test_chained_intent_raise_exception(self):
        class DoError(Exception):
            pass

        class EDo:
            pass

        def perform_do(intent):
            raise DoError()

        def yielder():
            yield Effect(EDo())

        effect = Effect(ChainedIntent(yielder()))
        dispatcher = TypeDispatcher({
            EDo: perform_do
        })
        with pytest.raises(DoError):
            await asyncio_perform(dispatcher, effect)

    @pytest.mark.asyncio
    async def test_chained_intent_raise_exception_and_catch_it(self):
        class DoError(Exception):
            pass

        class EBadDo:
            pass

        class EGoodDo:
            pass

        def perform_bad_do(intent):
            raise DoError()

        def yielder():
            try:
                yield Effect(EBadDo())
            except DoError:
                return (yield Effect(EGoodDo()))
            else:
                raise AssertionError("Didn't get notified of the excetion")

        effect = Effect(ChainedIntent(yielder()))
        dispatcher = TypeDispatcher({
            EBadDo: perform_bad_do,
            EGoodDo: lambda intent: 'ok'
        })
        ret = await asyncio_perform(dispatcher, effect)
        assert ret == 'ok'

    @pytest.mark.asyncio
    async def test_performer_return_effect(self):
        class EDoA:
            pass

        class EDoB:
            pass

        dispatcher = TypeDispatcher({
            EDoA: lambda intent: Effect(EDoB()),
            EDoB: lambda intent: 'bar'
        })
        effect = Effect(EDoA())
        ret = await asyncio_perform(dispatcher, effect)
        assert ret == 'bar'

    @pytest.mark.asyncio
    async def test_asyncio_performer(self):
        class EDoA:
            pass

        async def performer(intent):
            await asyncio.sleep(0)
            return 'bar'

        dispatcher = TypeDispatcher({
            EDoA: performer,
        })
        effect = Effect(EDoA())
        ret = await asyncio_perform(dispatcher, effect)
        assert ret == 'bar'

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_asyncio_performer_await_effect(self):
        class EDoA:
            pass

        class EDoB:
            pass

        async def performer(intent):
            await asyncio.sleep(0)
            return await Effect(EDoB)

        dispatcher = TypeDispatcher({
            EDoA: performer,
            EDoB: lambda intent: 'bar'
        })
        effect = Effect(EDoA())
        ret = await asyncio_perform(dispatcher, effect)
        assert ret == 'bar'
