import attr

from . import TypeDispatcher


@attr.s
class Delay:
    """
    An intent which represents a delay in time.

    When performed, the specified delay will pass and then the effect will
    result in None.

    :param float delay: The number of seconds to delay.
    """
    delay = attr.ib()


@attr.s
class Constant:
    """
    An intent that returns a pre-specified result when performed.

    :param result: The object which the Effect will result in.
    """
    result = attr.ib()


def perform_constant(intent):
    """Performer for :class:`Constant`."""
    return intent.result


@attr.s
class Error:
    """
    An intent that raises a pre-specified exception when performed.

    :param BaseException exception: Exception instance to raise.
    """
    exception = attr.ib()


def perform_error(intent):
    """Performer for :class:`Error`."""
    raise intent.exception


@attr.s(init=False)
class Func:
    """
    An intent that returns the result of the specified function.

    Note that Func is something of a cop-out. It doesn't follow the
    convention of an intent being transparent data that is easy to introspect,
    since it just wraps an opaque callable. This has two drawbacks:

    - it's harder to test, since the only thing you can do is call the
      function, instead of inspect its data.
    - it doesn't offer any ability for changing the way the effect is
      performed.

    If you use Func in your application code, know that you are giving
    up some ease of testing and flexibility. It's preferable to represent your
    intents as inert objects with public attributes of simple data. However,
    this is useful for integrating wih "legacy" side-effecting code in a quick
    way.

    :param func: The function to call when this intent is performed.
    :param args: Positional arguments to pass to the function.
    :param kwargs: Keyword arguments to pass to the function.
    """
    func = attr.ib()
    args = attr.ib()
    kwargs = attr.ib()

    def __init__(self, func, *args, **kwargs):
        self.func = func
        self.args = args
        self.kwargs = kwargs


def perform_func(intent):
    """Performer for :class:`Func`."""
    return intent.func(*intent.args, **intent.kwargs)


base_dispatcher = TypeDispatcher({
    Constant: perform_constant,
    Error: perform_error,
    Func: perform_func,
})
