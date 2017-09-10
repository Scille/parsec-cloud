import attr

from . import sync_perform, asyncio_perform, raise_


def _build_dispatcher(to_perform_seq):
    logs = []

    def fmt_logs():
        return '\n'.join('%s: %s' % x for x in logs)

    def dispatcher(intent):
        try:
            exp_intent, performer = next(to_perform_seq)
        except StopIteration:
            logs.append(('NOT FOUND', intent))
            raise AssertionError("No more performer expected ! Log follows:\n%s" % fmt_logs())
        if exp_intent != intent and (not isinstance(exp_intent, IntentType) or
                                     not isinstance(intent, exp_intent.type)):
            logs.append(('NOT FOUND', intent))
            logs.append(('NEXT EXPECTED', exp_intent))
            raise AssertionError(
                "Performer not found: %s! Log follows:\n%s" % (
                    intent, fmt_logs()))
        logs.append(('sequence', intent))
        return performer

    return dispatcher


def _check_for_unused_seq(to_perform_seq, ret):
    unused_intents = [x for x, _ in to_perform_seq]
    if unused_intents:
        raise AssertionError(
            "Returned `%s`, but not all intents were performed: %s" % (ret, unused_intents))


def perform_sequence(seq, effect):
    to_perform_seq = iter(seq)
    dispatcher = _build_dispatcher(to_perform_seq)
    ret = sync_perform(dispatcher, effect)
    _check_for_unused_seq(to_perform_seq, ret)
    return ret


async def asyncio_perform_sequence(seq, effect):
    to_perform_seq = iter(seq)
    dispatcher = _build_dispatcher(to_perform_seq)
    ret = await asyncio_perform(dispatcher, effect)
    _check_for_unused_seq(to_perform_seq, ret)
    return ret


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