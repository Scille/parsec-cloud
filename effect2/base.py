import six
import attr


class UnknownIntent(Exception):
    pass


@attr.s
class Effect:
    intent = attr.ib()

    def __await__(self):
        return (yield self)


@attr.s
class TypeDispatcher:
    mapping = attr.ib()

    def __call__(self, intent):
        try:
            return self.mapping[type(intent)]
        except KeyError:
            raise UnknownIntent('No performer for intent `%s`' % intent)


class ComposedDispatcher(TypeDispatcher):
    def __init__(self, dispatchers, *args):
        self.mapping = {}
        if args:
            dispatchers = [dispatchers, *args]
        for d in dispatchers:
            self.mapping.update(d.mapping)


def raise_(exception):
    """Simple convenience function to allow raising exceptions from lambdas."""
    raise exception
