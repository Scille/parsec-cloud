import attr
from contextlib import contextmanager

from . import Effect, AsyncFunc, sync_perform, asyncio_perform, raise_, UnknownIntent
from .intents import base_dispatcher


async def asyncio_perform_sequence(seq, effect, fallback_dispatcher=None):
    assert isinstance(effect, (Effect, AsyncFunc))
    seq = list(seq)

    def fmt_log():
        next_item = ''
        if len(sequence.sequence) > 0:
            next_item = '\nNEXT EXPECTED: %s' % (sequence.sequence[0][0],)
        return '{{{\n%s%s\n}}}' % (
            '\n'.join('%s: %s' % x for x in log),
            next_item)

    def dispatcher(intent):
        p = sequence(intent)
        if p is not None:
            log.append(("sequence", intent))
            return p
        try:
            p = fallback_dispatcher(intent)
            log.append(("fallback", intent))
            return p
        except UnknownIntent:
            log.append(("NOT FOUND", intent))
            raise AssertionError(
                "Performer not found: %s! Log follows:\n%s" % (
                    intent, fmt_log()))

    if fallback_dispatcher is None:
        fallback_dispatcher = base_dispatcher
    sequence = SequenceDispatcher(seq)
    log = []
    with sequence.consume():
        return await asyncio_perform(dispatcher, effect)


def perform_sequence(seq, effect, fallback_dispatcher=None):
    assert isinstance(effect, Effect)
    seq = list(seq)

    def fmt_log():
        next_item = ''
        if len(sequence.sequence) > 0:
            next_item = '\nNEXT EXPECTED: %s' % (sequence.sequence[0][0],)
        return '{{{\n%s%s\n}}}' % (
            '\n'.join('%s: %s' % x for x in log),
            next_item)

    def dispatcher(intent):
        p = sequence(intent)
        if p is not None:
            log.append(("sequence", intent))
            return p
        try:
            p = fallback_dispatcher(intent)
            log.append(("fallback", intent))
            return p
        except UnknownIntent:
            log.append(("NOT FOUND", intent))
            raise AssertionError(
                "Performer not found: %s! Log follows:\n%s" % (
                    intent, fmt_log()))

    if fallback_dispatcher is None:
        fallback_dispatcher = base_dispatcher
    sequence = SequenceDispatcher(seq)
    log = []
    with sequence.consume():
        ret = sync_perform(dispatcher, effect)
        sequence.returned_value = ret
        return ret


NOT_RETURNED_YET = object()


@attr.s
class SequenceDispatcher:
    """
    A dispatcher which steps through a sequence of (intent, func) tuples and
    runs ``func`` to perform intents in strict sequence.

    This is the dispatcher used by :func:`perform_sequence`. In general that
    function should be used directly, instead of this dispatcher.

    It's important to use `with sequence.consume():` to ensure that all of the
    intents are performed. Otherwise, if your code has a bug that causes it to
    return before all effects are performed, your test may not fail.

    :obj:`None` is returned if the next intent in the sequence is not equal to
    the intent being performed, or if there are no more items left in the
    sequence (this is standard behavior for dispatchers that don't handle an
    intent). This lets this dispatcher be composed easily with others.

    :param list sequence: Sequence of (intent, fn).
    """
    sequence = attr.ib()
    returned_value = attr.ib(default=NOT_RETURNED_YET)

    def __call__(self, intent):
        if len(self.sequence) == 0:
            return
        exp_intent, func = self.sequence[0]
        if intent == exp_intent or (isinstance(exp_intent, IntentType) and
                                    isinstance(intent, exp_intent.type)):
            self.sequence = self.sequence[1:]
            return lambda i: func(i)

    def consumed(self):
        """Return True if all of the steps were performed."""
        return len(self.sequence) == 0

    @contextmanager
    def consume(self):
        """
        Return a context manager that can be used with the `with` syntax to
        ensure that all steps are performed by the end.
        """
        yield
        if not self.consumed():
            assert self.returned_value is not NOT_RETURNED_YET
            raise AssertionError(
                "Returned `%s`, but not all intents were performed: %s" % (
                    self.returned_value, [x[0] for x in self.sequence]))


@attr.s
class IntentType:
    type = attr.ib()


def noop(intent):
    """
    Return None. This is just a handy way to make your intent sequences (as
    used by :func:`perform_sequence`) more concise when the effects you're
    expecting in a test don't return a result (and are instead only performed
    for their side-effects)::

        seq = [
            (Prompt('Enter your name: '), lambda i: 'Chris')
            (Greet('Chris'), noop),
        ]

    """
    return None


def const(value):
    """
    Return function that takes an argument but always return given `value`.
    Useful when creating sequence used by :func:`perform_sequence`. For example,

    >>> dt = datetime(1970, 1, 1)
    >>> seq = [(Func(datetime.now), const(dt))]

    :param value: This will be returned when called by returned function
    :return: ``callable`` that takes an arg and always returns ``value``
    """
    return lambda intent: value


def conste(excp):
    """
    Like :func:`const` but takes and exception and returns function that raises
    the exception

    :param excp: Exception that will be raised
    :type: :obj:`Exception`
    :return: ``callable`` that will raise given exception
    """
    return lambda intent: raise_(excp)