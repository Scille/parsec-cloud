"""pytest-trio implementation."""
import contextlib
import inspect
import socket
from functools import partial

import pytest
import trio


def pytest_configure(config):
    """Inject documentation."""
    config.addinivalue_line("markers",
                            "trio: "
                            "mark the test as a coroutine, it will be "
                            "run using an asyncio event loop")


def _trio_test_runner_factory(item, clock):
    testfunc = item.function

    async def _bootstrap_fixture_and_run_test(**kwargs):
        kwargs = await _resolve_coroutine_fixtures_in(kwargs)
        await testfunc(**kwargs)

    def run_test_in_trio(**kwargs):
        trio._core.run(partial(_bootstrap_fixture_and_run_test, **kwargs), clock=clock)

    return run_test_in_trio


async def _resolve_coroutine_fixtures_in(deps):
    resolved_deps = {**deps}

    async def _resolve_and_update_deps(afunc, deps, entry):
        deps[entry] = await afunc()

    async with trio.open_nursery() as nursery:
        for depname, depval in resolved_deps.items():
            if isinstance(depval, CoroutineFixture):
                nursery.start_soon(
                    _resolve_and_update_deps, depval.resolve, resolved_deps, depname)
    return resolved_deps


class CoroutineFixture:
    """
    Represent a fixture that need to be run in a trio context to be resolved.
    Can be async function fixture or a syncronous fixture with async
    dependencies fixtures.
    """
    NOTSET = object()

    def __init__(self, fixturefunc, fixturedef, deps={}):
        self.fixturefunc = fixturefunc
        # Note fixturedef.func
        self.fixturedef = fixturedef
        self.deps = deps
        self._ret = self.NOTSET

    async def resolve(self):
        if self._ret is self.NOTSET:
            resolved_deps = await _resolve_coroutine_fixtures_in(self.deps)
            if inspect.iscoroutinefunction(self.fixturefunc):
                self._ret = await self.fixturefunc(**resolved_deps)
            else:
                self._ret = self.fixturefunc(**resolved_deps)
        return self._ret


def _install_coroutine_fixture_if_needed(fixturedef, request):
    deps = {dep: request.getfixturevalue(dep) for dep in fixturedef.argnames}
    corofix = None
    if not deps and inspect.iscoroutinefunction(fixturedef.func):
        # Top level async coroutine
        corofix = CoroutineFixture(fixturedef.func, fixturedef)
    elif any(dep for dep in deps.values() if isinstance(dep, CoroutineFixture)):
        # Fixture with coroutine fixture dependencies
        corofix = CoroutineFixture(fixturedef.func, fixturedef, deps)
    # The coroutine fixture must be evaluated from within the trio context
    # which is spawed in the function test's trio decorator.
    # The trick is to make pytest's fixture call return the CoroutineFixture
    # object which will be actully resolved just before we run the test.
    if corofix:
        fixturedef.func = lambda **kwargs: corofix


@pytest.hookimpl(tryfirst=True)
def pytest_fixture_setup(fixturedef, request):
    if 'trio' in request.keywords:
        _install_coroutine_fixture_if_needed(fixturedef, request)


@pytest.hookimpl(tryfirst=True)
def pytest_collection_modifyitems(session, config, items):
    # Retrieve test marked as `trio`
    for item in items:
        if 'trio' not in item.keywords:
            continue
        if not inspect.iscoroutinefunction(item.function):
            pytest.fail('test function `%r` is marked trio but is not async' % item)
        # Extract the clock fixture if provided
        clocks = [c for c in item.funcargs.values() if isinstance(c, trio.abc.Clock)]
        if not clocks:
            clock = None
        elif len(clocks) == 1:
            clock = clocks[0]
        else:
            raise pytest.fail("too many clocks spoil the broth!")
        item.obj = _trio_test_runner_factory(item, clock)


@pytest.hookimpl(tryfirst=True)
def pytest_exception_interact(node, call, report):
    if issubclass(call.excinfo.type, trio.MultiError):
        # TODO: not really elegant (pytest cannot output color with this hack)
        report.longrepr = ''.join(trio.format_exception(*call.excinfo._excinfo))


@pytest.fixture
def unused_tcp_port():
    """Find an unused localhost TCP port from 1024-65535 and return it."""
    with contextlib.closing(socket.socket()) as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


@pytest.fixture
def unused_tcp_port_factory():
    """A factory function, producing different unused TCP ports."""
    produced = set()

    def factory():
        """Return an unused port."""
        port = unused_tcp_port()

        while port in produced:
            port = unused_tcp_port()

        produced.add(port)

        return port
    return factory
