from blinker import Namespace, ANY
import contextvars


_signals_namespace = contextvars.ContextVar("signals_namespace")


class SignalsContext:
    def __init__(self):
        self.signals_namespace = Namespace()
        self._token = None

    def push(self):
        self._token = _signals_namespace.set(self.signals_namespace)

    def pop(self):
        ns = _signals_namespace.get()
        if ns is not self.signals_namespace:
            raise RuntimeError("Invalid order in stack")
        _signals_namespace.reset(self._token)

    def __enter__(self):
        self.push()
        return self

    def __exit__(self, *exc):
        self.pop()
        return False


def get_signal(name):
    return _signals_namespace.get().signal(name)


__all__ = ("ANY", "SignalsContext", "get_signal")
