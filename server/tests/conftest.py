# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import logging
import os
import re
from collections.abc import Generator
from typing import Literal

import pytest
import structlog

from parsec.config import (
    BaseBlockStoreConfig,
    BaseDatabaseConfig,
    MockedBlockStoreConfig,
    MockedDatabaseConfig,
    PostgreSQLBlockStoreConfig,
    PostgreSQLDatabaseConfig,
)
from parsec.logging import _make_filtering_bound_logger

# Must be done before the module has any chance to be imported
pytest.register_assert_rewrite("tests.common.event_bus_spy")
pytest.register_assert_rewrite("tests.common.invite")
from tests.common import (
    LogCaptureFixture,
    bootstrap_postgresql_testbed,
    get_postgresql_url,
)
from tests.common.patch_httpx import patch_httpx_stream_support

# TODO: Currently httpx ASGI transport only parse the response once it has been totally
# sent, which doesn't play well with streamed response (i.e. SSE in our case)
# (see https://github.com/encode/httpx/issues/2186)
patch_httpx_stream_support()


# Pytest hooks


def pytest_addoption(parser: pytest.Parser) -> None:
    parser.addoption(
        "--postgresql",
        action="store_true",
        help=(
            "Use PostgreSQL backend instead of default memory mock "
            "(use `PG_URL` env var to customize the database to use)"
        ),
    )
    parser.addoption(
        "--run-postgresql-cluster",
        action="store_true",
        help=(
            "Instead of running the tests, only start a PostgreSQL cluster "
            "that could be use for other tests (through `PG_URL` env var) "
            "to avoid having to create a new cluster each time."
        ),
    )

    def _parse_slice_tests(value: str) -> tuple[list[int], int]:
        try:
            raw_nums, raw_total = value.split("/")
            total = int(raw_total)
            nums = [int(x) for x in raw_nums.split(",")]
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


def get_log_level_from_cli(config: pytest.Config) -> int:
    match config.getoption("log_cli_level"):
        case None:
            return logging.INFO
        case raw:
            assert isinstance(raw, str)
            return getattr(logging, raw.upper())


def pytest_configure(config: pytest.Config) -> None:
    # Configure structlog to redirect everything in logging
    structlog.configure(
        logger_factory=structlog.stdlib.LoggerFactory(),
        wrapper_class=_make_filtering_bound_logger(get_log_level_from_cli(config)),
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
            capturemanager.suspend(in_=True)  # type: ignore
        print(f"usage: PG_URL={pg_url} py.test --postgresql tests")
        input("Press enter when you're done with...")
        pytest.exit("bye")
    elif config.getoption("--postgresql") and not _is_xdist_master(config):
        bootstrap_postgresql_testbed()


def _is_xdist_master(config: pytest.Config) -> bool:
    return config.getoption("dist") != "no" and not os.environ.get("PYTEST_XDIST_WORKER")


def _patch_caplog() -> None:
    from pytest import LogCaptureFixture

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
        matches_msgs, _ = _find(self, log)
        assert not matches_msgs

    LogCaptureFixture.assert_occurred = _assert_occurred  # type: ignore
    LogCaptureFixture.assert_occurred_once = _assert_occurred_once  # type: ignore
    LogCaptureFixture.assert_not_occurred = _assert_not_occurred  # type: ignore


def pytest_runtest_setup(item: pytest.Item) -> None:
    if item.get_closest_marker("postgresql"):
        if not item.config.getoption("--postgresql"):
            pytest.skip("need --postgresql option to run")


def pytest_collection_modifyitems(config: pytest.Config, items: list[pytest.Item]) -> None:
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
def no_logs_gte_error(caplog: LogCaptureFixture) -> Generator[None, None, None]:
    yield

    # The test should use `caplog.assert_occurred_once` to indicate a log was expected,
    # otherwise we consider error logs as *actual* errors.
    asserted_records: set[logging.LogRecord] | Literal[True] = getattr(
        caplog, "asserted_records", set()
    )

    if asserted_records is True:
        return

    errors = [
        record
        for record in caplog.get_records("call")
        if record.levelno >= logging.ERROR and record not in asserted_records
    ]

    assert not errors


@pytest.fixture
def disable_no_logs_gte_error_check(caplog: LogCaptureFixture) -> None:
    caplog.asserted_records = True  # type: ignore


# Other main fixtures


@pytest.fixture
def postgresql_url(request: pytest.FixtureRequest) -> str:
    if not request.node.get_closest_marker("postgresql"):
        raise RuntimeError(
            "`postgresql_url` can only be used in tests decorated with `@pytest.mark.postgresql`"
        )
    url = get_postgresql_url()
    assert url is not None
    return url


@pytest.fixture
def with_postgresql(request: pytest.FixtureRequest) -> bool:
    return bool(request.config.getoption("--postgresql"))


@pytest.fixture
def skip_if_postgresql(with_postgresql: bool) -> None:
    """Test is not expected to pass"""
    if with_postgresql:
        pytest.skip("Test is not compatible with PostgreSQL backend")


@pytest.fixture
def xfail_if_postgresql(with_postgresql: bool) -> None:
    """Test is expected to pass in the future"""
    if with_postgresql:
        pytest.xfail("TODO: Test is not compatible with PostgreSQL backend yet")


@pytest.fixture
async def db_config(request: pytest.FixtureRequest) -> BaseDatabaseConfig:
    if request.config.getoption("--postgresql"):
        url = get_postgresql_url()
        assert url is not None
        return PostgreSQLDatabaseConfig(
            url=url,
            min_connections=1,
            max_connections=10,
        )

    elif request.node.get_closest_marker("postgresql"):
        pytest.skip("`Test is postgresql-only")

    else:
        return MockedDatabaseConfig()


@pytest.fixture
def blockstore_config(db_config: BaseDatabaseConfig) -> BaseBlockStoreConfig:
    # TODO: allow to test against swift ?
    if db_config.is_mocked():
        return MockedBlockStoreConfig()
    else:
        return PostgreSQLBlockStoreConfig()


# Finally other fixtures


from tests.common import *  # noqa: F403, republishing fixtures
