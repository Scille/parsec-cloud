# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import logging
import os
import re
import shutil
import sys
from pathlib import Path
from typing import Any, Callable, Generator

import hypothesis
import psutil
import pytest
import structlog
import trio
import trio_asyncio

from parsec._parsec import test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock
from parsec.backend.config import (
    MockedBlockStoreConfig,
    PostgreSQLBlockStoreConfig,
    RAID0BlockStoreConfig,
    RAID1BlockStoreConfig,
    RAID5BlockStoreConfig,
)
from parsec.core.mountpoint.manager import get_mountpoint_runner
from parsec.monitoring import TaskMonitoringInstrument

# TODO: needed ?
# Must be done before the module has any chance to be imported
pytest.register_assert_rewrite("tests.common.event_bus_spy")
from tests.common import (
    asyncio_reset_postgresql_testbed,
    bootstrap_postgresql_testbed,
    get_postgresql_url,
    get_side_effects_timeout,
    reset_postgresql_testbed,
)


# Pytest hooks


def pytest_addoption(parser):
    parser.addoption("--side-effects-timeout", default=get_side_effects_timeout(), type=float)
    parser.addoption("--hypothesis-max-examples", default=100, type=int)
    parser.addoption("--hypothesis-derandomize", action="store_true")
    parser.addoption(
        "--postgresql",
        action="store_true",
        help=(
            "Use PostgreSQL backend instead of default memory mock "
            "(use `PG_URL` env var to customize the database to use)"
        ),
    )
    parser.addoption("--runslow", action="store_true", help="Don't skip slow tests")
    parser.addoption("--runmountpoint", action="store_true", help="Don't skip FUSE/WinFSP tests")
    parser.addoption("--rungui", action="store_true", help="Don't skip GUI tests")
    parser.addoption("--rundiskfull", action="store_true", help="Don't skip the disk full tests")
    # TODO: remove me once client connection oxidation is done
    parser.addoption(
        "--enable-unstable-oxidized-client-connection",
        action="store_true",
        help="Use the unstable Rust client connection",
    )
    parser.addoption("--runrust", action="store_true", help="Don't skip rust tests")
    parser.addoption(
        "--run-postgresql-cluster",
        action="store_true",
        help=(
            "Instead of running the tests, only start a PostgreSQL cluster "
            "that could be use for other tests (through `PG_URL` env var) "
            "to avoid having to create a new cluster each time."
        ),
    )

    def _parse_slice_tests(value):
        try:
            nums, total = value.split("/")
            total = int(total)
            nums = [int(x) for x in nums.split(",")]
            if total >= 1 and all(1 <= x <= total for x in nums):
                return (nums, total)
        except ValueError:
            pass
        raise ValueError("--slice-tests option must be of the type `1/3`, `2/3`, `1,2/3` etc.")

    parser.addoption(
        "--slice-tests",
        default="1/1",
        help="Only run a portion of tests (useful starting tests in parallel)",
        type=_parse_slice_tests,
    )


def pytest_configure(config):
    # Configure structlog to redirect everything in logging
    structlog.configure(
        logger_factory=structlog.stdlib.LoggerFactory(),
        processors=[
            structlog.processors.StackInfoRenderer(),
            structlog.processors.format_exc_info,
            structlog.processors.TimeStamper(fmt="%Y-%m-%d %H:%M:%S"),
            structlog.stdlib.add_logger_name,
            structlog.stdlib.add_log_level,
            structlog.dev.ConsoleRenderer(),
        ],
    )
    # Lock configuration
    structlog.configure = lambda *args, **kwargs: None
    # Add helper to caplog
    _patch_caplog()
    if config.getoption("--run-postgresql-cluster"):
        pg_url = bootstrap_postgresql_testbed()
        capturemanager = config.pluginmanager.getplugin("capturemanager")
        if capturemanager:
            capturemanager.suspend(in_=True)
        print(f"usage: PG_URL={pg_url} py.test --postgresql tests")
        input("Press enter when you're done with...")
        pytest.exit("bye")
    elif config.getoption("--postgresql") and not _is_xdist_master(config):
        bootstrap_postgresql_testbed()
    # Configure custom side effets timeout
    if config.getoption("--side-effects-timeout"):
        import tests.common.trio_clock

        tests.common.trio_clock._set_side_effects_timeout(
            float(config.getoption("--side-effects-timeout"))
        )

    # TODO: remove me once client connection oxidation is done
    if config.getoption("--enable-unstable-oxidized-client-connection"):
        import parsec

        parsec.FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"] = True


def _is_xdist_master(config):
    return config.getoption("dist") != "no" and not os.environ.get("PYTEST_XDIST_WORKER")


def _patch_caplog():
    from _pytest.logging import LogCaptureFixture

    def _remove_colors(msg):
        return re.sub(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]", "", str(msg))

    def _find(self, log):
        __tracebackhide__ = True
        matches_msgs = []
        matches_records = []
        for record in self.records:
            monochrome_msg = _remove_colors(record.msg)
            if log in monochrome_msg:
                matches_msgs.append(monochrome_msg)
                matches_records.append(record)
        return matches_msgs, matches_records

    def _register_asserted_records(self, *records):
        try:
            asserted_records = self.asserted_records
        except AttributeError:
            asserted_records = set()
            setattr(self, "asserted_records", asserted_records)
        asserted_records.update(records)

    def _assert_occurred(self, log):
        __tracebackhide__ = True
        matches_msgs, matches_records = _find(self, log)
        assert matches_msgs
        _register_asserted_records(self, *matches_records)
        return matches_msgs

    def _assert_occurred_once(self, log):
        __tracebackhide__ = True
        matches_msgs, matches_records = _find(self, log)
        assert len(matches_msgs) == 1
        _register_asserted_records(self, matches_records[0])
        return matches_msgs[0]

    def _assert_not_occurred(self, log):
        __tracebackhide__ = True
        matches_msgs, matches_records = _find(self, log)
        assert not matches_msgs

    LogCaptureFixture.assert_occurred = _assert_occurred
    LogCaptureFixture.assert_occurred_once = _assert_occurred_once
    LogCaptureFixture.assert_not_occurred = _assert_not_occurred


def pytest_runtest_setup(item):
    if item.get_closest_marker("slow") and not item.config.getoption("--runslow"):
        pytest.skip("need --runslow option to run")
    if item.get_closest_marker("win32") and sys.platform != "win32":
        pytest.skip("test specific to win32")
    if item.get_closest_marker("linux") and sys.platform != "linux":
        pytest.skip("test specific to linux")
    if item.get_closest_marker("mountpoint"):
        if not item.config.getoption("--runmountpoint"):
            pytest.skip("need --runmountpoint option to run")
        elif not get_mountpoint_runner():
            pytest.skip("FUSE/WinFSP not available")
    if item.get_closest_marker("diskfull"):
        if not item.config.getoption("--rundiskfull"):
            pytest.skip("need --rundiskfull option to run")
    if item.get_closest_marker("gui"):
        if not item.config.getoption("--rungui"):
            pytest.skip("need --rungui option to run")
    if item.get_closest_marker("postgresql"):
        if not item.config.getoption("--postgresql"):
            pytest.skip("need --postgresql option to run")
    if item.get_closest_marker("rust") and not item.config.getoption("--runrust"):
        pytest.skip("need --runrust option to run")


def pytest_collection_modifyitems(config, items):
    for item in items:
        if "trio" in item.keywords:
            item.fixturenames.append("task_monitoring")
        if "gui" in item.keywords and "trio" in item.keywords:
            import qtrio

            item.add_marker(pytest.mark.trio(run=qtrio.run))

    # Divide tests into slices of equal size
    slices_to_run, total_slices = config.getoption("--slice-tests")
    if total_slices > 1:
        # Reorder tests to be deterministic given they will be ran across multiples instances
        # Note this must be done as an in-place update to have it taken into account
        # And finally note we order by hash instead of by name, this is to improve
        # dispatching across jobs, the idea being a slow test is likely to be surrounded
        # by other slow tests.
        from zlib import adler32

        items.sort(key=lambda x: adler32(repr(x.location).encode("utf8")))

        per_slice_count = (len(items) // total_slices) + 1

        skip = pytest.mark.skip(reason="Test not in the requested slice range")
        for cur_slice in range(1, total_slices + 1):
            if cur_slice not in slices_to_run:
                for item in items[(cur_slice - 1) * per_slice_count : cur_slice * per_slice_count]:
                    item.user_properties.append(("test_out_of_slice", True))
                    item.add_marker(skip)


# Autouse fixtures


@pytest.fixture(autouse=True)
def no_logs_gte_error(caplog):
    yield

    # TODO: Concurrency bug in Hypercorn when the server is teardown while a
    # client websocket is currently disconnecting
    # see: https://github.com/Scille/parsec-cloud/issues/2716
    def skip_hypercorn_buggy_log(record):
        try:
            _, exc, _ = record.exc_info
        except (ValueError, TypeError):
            exc = None

        if record.name == "asyncio" and isinstance(exc, ConnectionError):
            return True

        if record.name != "hypercorn.error":
            return True

        if record.exc_text.endswith(
            "wsproto.utilities.LocalProtocolError: Connection cannot be closed in state ConnectionState.CLOSED"
        ):
            return False

        if record.exc_text.endswith(
            "trio.BusyResourceError: another task is currently sending data on this SocketStream"
        ):
            return False

        if record.exc_text.endswith(
            "wsproto.utilities.LocalProtocolError: Event CloseConnection(code=1000, reason=None) cannot be sent in state ConnectionState.CLOSED."
        ):
            return False

        return True

    # The test should use `caplog.assert_occurred_once` to indicate a log was expected,
    # otherwise we consider error logs as *actual* errors.
    asserted_records = getattr(caplog, "asserted_records", set())
    errors = [
        record
        for record in caplog.get_records("call")
        if record.levelno >= logging.ERROR
        and record not in asserted_records
        and skip_hypercorn_buggy_log(record)
    ]

    assert not errors


@pytest.fixture(scope="session")
def hypothesis_settings(request):
    return hypothesis.settings(
        max_examples=request.config.getoption("--hypothesis-max-examples"),
        derandomize=request.config.getoption("--hypothesis-derandomize"),
        deadline=None,
    )


# Other main fixtures


@pytest.fixture
async def nursery():
    # A word about the nursery fixture:
    # The whole point of trio is to be able to build a graph of coroutines to
    # simplify teardown. Using a single top level nursery kind of mitigate this
    # given unrelated coroutines will end up there and be closed all together.
    # Worst, among those coroutine it could exists a relationship that will be lost
    # in a more or less subtle way (typically using a factory fixture that use the
    # default nursery behind the scene).
    # Bonus points occur if using trio-asyncio that creates yet another hidden
    # layer of relationship that could end up in cryptic dead lock hardened enough
    # to survive ^C.
    # Finally if your still no convinced, factory fixtures not depending on async
    # fixtures (like nursery is) can be used inside the Hypothesis tests.
    # I know you love Hypothesis. Checkmate. You won't use this fixture ;-)
    raise RuntimeError("Bad kitty ! Bad !!!")


@pytest.fixture
def postgresql_url(request):
    if not request.node.get_closest_marker("postgresql"):
        raise RuntimeError(
            "`postgresql_url` can only be used in tests decorated with `@pytest.mark.postgresql`"
        )
    return get_postgresql_url()


@pytest.fixture
async def asyncio_loop(request):
    # asyncio loop is only needed for triopg
    if not request.config.getoption("--postgresql"):
        yield None

    else:
        # When a ^C happens, trio send a Cancelled exception to each running
        # coroutine. We must protect this one to avoid deadlock if it is cancelled
        # before another coroutine that uses trio-asyncio.
        with trio.CancelScope(shield=True):
            async with trio_asyncio.open_loop() as loop:
                yield loop


@pytest.fixture
async def task_monitoring():
    trio.lowlevel.add_instrument(TaskMonitoringInstrument())


@pytest.fixture(scope="session")
def monitor():
    from tests.monitor import Monitor

    return Monitor()


@pytest.fixture()
def backend_store(request):
    if request.config.getoption("--postgresql"):
        reset_postgresql_testbed()
        return get_postgresql_url()

    elif request.node.get_closest_marker("postgresql"):
        pytest.skip("`Test is postgresql-only")

    else:
        return "MOCKED"


@pytest.fixture
def blockstore(backend_store, fixtures_customization):
    # TODO: allow to test against swift ?
    if backend_store.startswith("postgresql://"):
        config = PostgreSQLBlockStoreConfig()
    else:
        config = MockedBlockStoreConfig()

    raid = fixtures_customization.get("blockstore_mode", "NO_RAID").upper()
    if raid == "RAID0":
        config = RAID0BlockStoreConfig(blockstores=[config, MockedBlockStoreConfig()])
    elif raid == "RAID1":
        config = RAID1BlockStoreConfig(blockstores=[config, MockedBlockStoreConfig()])
    elif raid == "RAID1_PARTIAL_CREATE_OK":
        config = RAID1BlockStoreConfig(
            blockstores=[config, MockedBlockStoreConfig()], partial_create_ok=True
        )
    elif raid == "RAID5":
        config = RAID5BlockStoreConfig(
            blockstores=[config, MockedBlockStoreConfig(), MockedBlockStoreConfig()]
        )
    elif raid == "RAID5_PARTIAL_CREATE_OK":
        config = RAID5BlockStoreConfig(
            blockstores=[config, MockedBlockStoreConfig(), MockedBlockStoreConfig()],
            partial_create_ok=True,
        )
    else:
        assert raid == "NO_RAID"

    return config


@pytest.fixture(autouse=True)
def persistent_mockup(
    fixtures_customization: dict[str, Any]
) -> Generator[None, None, Callable[[], None] | None]:
    if fixtures_customization.get("real_data_storage", False) or fixtures_customization.get(
        "alternate_workspace_storage", False
    ):
        test_toggle_local_db_in_memory_mock(False)
        yield None

    else:
        test_toggle_local_db_in_memory_mock(True)
        # No database should be in use while we clear the mock. Hence it's
        # better to do the clear as part of the init given autouse fixtures
        # run first.
        test_clear_local_db_in_memory_mock()

        yield test_clear_local_db_in_memory_mock


# `persistent_mockup` is autouse, hence a test only have it explicitly has dependency to be
# able to use it clear function. Here we just re-expose this function with a better name ;-)
@pytest.fixture
def clear_persistent_mockup(persistent_mockup):
    return persistent_mockup or (lambda: None)


@pytest.fixture
def data_base_dir(tmp_path: Path) -> Path:
    return tmp_path / "data"


@pytest.fixture
def clear_database_dir(data_base_dir: Path) -> Callable[[bool], None]:
    db_dir = data_base_dir
    proc = psutil.Process()

    def _clear_database_dir():
        still_opened = [
            opened for opened in proc.open_files() if Path(opened.path).is_relative_to(db_dir)
        ]
        if still_opened:
            raise RuntimeError(f"Database dir contains still opened files: {still_opened}")

        try:
            shutil.rmtree(db_dir)
        except FileNotFoundError:
            pass

    return _clear_database_dir


@pytest.fixture
def reset_testbed(
    request,
    caplog,
    clear_persistent_mockup: Callable[[], None],
    clear_database_dir: Callable[[bool], None],
):
    async def _reset_testbed(keep_logs=False):
        if request.config.getoption("--postgresql"):
            await trio_asyncio.aio_as_trio(asyncio_reset_postgresql_testbed)
        clear_database_dir()
        clear_persistent_mockup()
        if not keep_logs:
            caplog.clear()

    return _reset_testbed


# Finally other fixtures


from tests.common import *  # noqa, republishing fixtures
