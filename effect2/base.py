import six
import attr
import inspect
from functools import wraps


class UnknownIntent(Exception):
    pass


@attr.s
class ChainedIntent:
    generator = attr.ib()


@attr.s
class Effect:
    intent = attr.ib()


def do(f):

    @wraps(f)
    def wrapper(*args, **kwargs):
        gen = f(*args, **kwargs)
        if not inspect.isgenerator(gen):
            res = gen
            def generator_no_yield():
                return res
                yield
            gen = generator_no_yield()
        return Effect(ChainedIntent(gen))

    return wrapper


@attr.s
class TypeDispatcher:
    mapping = attr.ib()

    def __call__(self, intent):
        try:
            return self.mapping[type(intent)]
        except KeyError:
            raise UnknownIntent('No performer for intent `%s`' % intent)


class ComposedDispatcher(TypeDispatcher):
    def __init__(self, dispatchers):
        self.mapping = {}
        for d in dispatchers:
            self.mapping.update(d.mapping)


def raise_(exception):
    """Simple convenience function to allow raising exceptions from lambdas."""
    raise exception
