# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import logging
import re
from io import StringIO
from unittest.mock import ANY

import pytest
import sentry_sdk

from parsec._version import __version__ as parsec_version
from parsec.logging import (
    build_sentry_configuration,
    build_structlog_configuration,
    configure_stdlib_logger,
)


@pytest.fixture
def capsentry():
    events = []

    # This hub context isolates us from the "real" global one
    with sentry_sdk.Hub():

        def _collect_events(event):
            events.append(event)

        config = build_sentry_configuration(dsn="whatever", environment="ci")
        sentry_sdk.init(**config, transport=_collect_events)

        yield events


def remove_colors(log: str) -> str:
    # Much more readable to compare log without all those escapes sequences
    return re.sub(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]", "", log)


def pin_timestamp(log: str) -> str:
    # Let's fix the future once and for all Marty !
    return re.sub(
        r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(.[0-9]+)?Z",
        "2010-11-18T05:52:00.000Z",
        log,
    )


def pin_file_and_line(log: str) -> str:
    # Stacktrace in log contains absolute file path and line number which
    # don't play well with testing
    # This may breaks if your work dir contains very exotic stuff (e.g. \n, quotes)
    return re.sub(
        # Quotes escapes are present in JSON output but not in CONSOLE
        # Windows: wE uSE BaCKlaSH aS SepARaToR !
        r'File (\\?)".*\\?[/\\]tests\\?[/\\]test_logging.py(\\?)", line [0-9]+, ',
        r'File \1"<workdir>/tests/test_logging.py\2", line 42, ',
        log,
    )


def cook_log(log: str):
    return pin_file_and_line(pin_timestamp(remove_colors(log)))


def generate_raised_exception(exc_factory=lambda: RuntimeError("D'oh !")):
    try:
        raise exc_factory()
    except Exception as exc:
        return exc


def build_structlog_test_logger(**kwargs):
    config = build_structlog_configuration(**kwargs)
    return config["wrapper_class"](
        processors=config["processors"], logger=config["logger_factory"](), context={}
    )


def build_stdlib_test_logger(**kwargs):
    # Pytest overwrites root logger configuration with it `caplog` fixture,
    # so we create an isolated logger to run our test on
    logger = logging.Logger("my_test_logger")
    configure_stdlib_logger(logger, **kwargs)
    return logger


class LogStream:
    def __init__(self):
        self.stream = StringIO()
        self.offset = 0

    def read(self) -> str:
        buff = self.stream.getvalue()
        offset = self.offset
        self.offset = len(buff)
        return buff[offset:]


@pytest.mark.parametrize("familly", ("structlog", "stdlib"))
def test_level_filtering(familly):
    if familly == "structlog":
        logger_builder = build_structlog_test_logger
    else:
        assert familly == "stdlib"
        logger_builder = build_stdlib_test_logger

    out = LogStream()
    logger = logger_builder(log_level="WARNING", log_format="JSON", log_stream=out.stream)

    for level in ("debug", "info"):
        getattr(logger, level)("Donuts !")
        assert out.read() == ""

    logger.warning("Last donut")
    assert "Last donut" in out.read()


def test_stdout_output():
    # Test both stdlib and structlog to make sure their output won't be mixed

    out = LogStream()
    structlog_logger = build_structlog_test_logger(
        log_level="INFO", log_format="JSON", log_stream=out.stream
    )
    stdlib_logger = build_stdlib_test_logger(
        log_level="INFO", log_format="JSON", log_stream=out.stream
    )

    structlog_logger.info("this is a structlog log")
    stdlib_logger.info("this is a stdlib log")
    logs = cook_log(out.read())

    expected_structlog_log = '{"event": "this is a structlog log", "level": "info", "timestamp": "2010-11-18T05:52:00.000Z"}'
    expected_stdlib_log = '{"event": "this is a stdlib log", "level": "info", "timestamp": "2010-11-18T05:52:00.000Z"}'
    expected_logs = f"{expected_structlog_log}\n{expected_stdlib_log}\n"
    assert logs == expected_logs


def test_file_output(tmp_path):
    # Test both stdlib and structlog to make sure their output won't be mixed

    log_path = tmp_path / "my.log"
    with open(log_path, "w") as fd:
        structlog_logger = build_structlog_test_logger(
            log_level="INFO", log_format="JSON", log_stream=fd
        )
        stdlib_logger = build_stdlib_test_logger(log_level="INFO", log_format="JSON", log_stream=fd)

        structlog_logger.info("this is a structlog log")
        stdlib_logger.info("this is a stdlib log")
        logs = cook_log(log_path.read_text())

    expected_structlog_log = '{"event": "this is a structlog log", "level": "info", "timestamp": "2010-11-18T05:52:00.000Z"}'
    expected_stdlib_log = '{"event": "this is a stdlib log", "level": "info", "timestamp": "2010-11-18T05:52:00.000Z"}'
    expected_logs = f"{expected_structlog_log}\n{expected_stdlib_log}\n"
    assert logs == expected_logs


@pytest.mark.parametrize("log_format", ("CONSOLE", "JSON"))
def test_structlog_log_format(log_format):
    out = LogStream()
    logger = build_structlog_test_logger(
        log_level="INFO", log_format=log_format, log_stream=out.stream
    )

    # Test base format

    logger.warning("It is over 9000 !", power=9000, target="Goku")
    log = cook_log(out.read())
    if log_format == "CONSOLE":
        expected_log = "2010-11-18T05:52:00.000Z [warning  ] It is over 9000 !              power=9000 target=Goku\n"
        assert log == expected_log
    else:
        assert log_format == "JSON"
        expected_log = '{"power": 9000, "target": "Goku", "event": "It is over 9000 !", "level": "warning", "timestamp": "2010-11-18T05:52:00.000Z"}\n'
        assert log == expected_log

    # Test format with stacktrace

    logger.warning(
        "It is over 9000 !", power=9000, target="Goku", exc_info=generate_raised_exception()
    )
    log = cook_log(out.read())
    if log_format == "CONSOLE":
        expected_log = (
            # fmt: off
            '2010-11-18T05:52:00.000Z [warning  ] It is over 9000 !              power=9000 target=Goku\n'
            'Traceback (most recent call last):\n'
            '  File "<workdir>/tests/test_logging.py", line 42, in generate_raised_exception\n'
            '    raise exc_factory()\n'
            'RuntimeError: D\'oh !\n'
            # fmt: on
        )
        assert log == expected_log
    else:
        assert log_format == "JSON"
        expected_log = (
            # fmt: off
            '{"power": 9000, '
            '"target": "Goku", '
            '"event": "It is over 9000 !", '
            '"level": "warning", '
            '"timestamp": "2010-11-18T05:52:00.000Z", '
            '"exception": "Traceback (most recent call last):\\n'
            '  File \\"<workdir>/tests/test_logging.py\\", line 42, in generate_raised_exception\\n'
            '    raise exc_factory()\\n'
            'RuntimeError: D\'oh !"}\n'
            # fmt: on
        )
        assert log == expected_log


@pytest.mark.parametrize("log_format", ("JSON", "CONSOLE"))
def test_stdlib_log_format(log_format):
    out = LogStream()
    logger = build_stdlib_test_logger(
        log_level="INFO", log_format=log_format, log_stream=out.stream
    )

    # Test base format

    logger.warning("It is over %s !", 9000)
    log = cook_log(out.read())
    if log_format == "CONSOLE":
        expected_log = "2010-11-18T05:52:00.000Z [warning  ] It is over 9000 !\n"
        assert log == expected_log
    else:
        assert log_format == "JSON"
        expected_log = '{"event": "It is over 9000 !", "level": "warning", "timestamp": "2010-11-18T05:52:00.000Z"}\n'
        assert log == expected_log

    # Test format with stacktrace

    logger.warning("It is over %s !", 9000, exc_info=generate_raised_exception())
    log = cook_log(out.read())
    if log_format == "CONSOLE":
        expected_log = (
            # fmt: off
            '2010-11-18T05:52:00.000Z [warning  ] It is over 9000 !              \n'
            'Traceback (most recent call last):\n'
            '  File "<workdir>/tests/test_logging.py", line 42, in generate_raised_exception\n'
            '    raise exc_factory()\n'
            'RuntimeError: D\'oh !\n'
            # fmt: on
        )
        assert log == expected_log
    else:
        assert log_format == "JSON"
        expected_log = (
            # fmt: off
            '{"event": "It is over 9000 !", '
            '"level": "warning", '
            '"timestamp": "2010-11-18T05:52:00.000Z", '
            '"exception": "Traceback (most recent call last):\\n'
            '  File \\"<workdir>/tests/test_logging.py\\", line 42, in generate_raised_exception\\n'
            '    raise exc_factory()\\n'
            'RuntimeError: D\'oh !"}\n'
            # fmt: on
        )
        assert log == expected_log


@pytest.mark.parametrize("familly", ("structlog", "stdlib"))
def test_exception_handling(familly):
    out = LogStream()
    if familly == "structlog":
        logger_builder = build_structlog_test_logger
    else:
        assert familly == "stdlib"
        logger_builder = build_stdlib_test_logger
    logger = logger_builder(log_level="WARNING", log_format="JSON", log_stream=out.stream)

    # Ignored exception

    logger.info("Donuts !")
    assert out.read() == ""

    # Test exception from a regular context

    logger.exception("No more donuts", exc_info=generate_raised_exception())
    log = cook_log(out.read())

    expected_log = (
        # fmt: off
        '{"event": "No more donuts", '
        '"level": "error", '
        '"timestamp": "2010-11-18T05:52:00.000Z", '
        '"exception": "Traceback (most recent call last):\\n'
        '  File \\"<workdir>/tests/test_logging.py\\", line 42, in generate_raised_exception\\n'
        '    raise exc_factory()\\n'
        'RuntimeError: D\'oh !"}\n'
        # fmt: on
    )
    assert log == expected_log

    # Test exception from within another exception context

    try:
        raise RuntimeError("Ignored error")
    except Exception:
        logger.exception("No more donuts", exc_info=generate_raised_exception())
    log = cook_log(out.read())
    assert log == expected_log

    # Test implicit exception catching

    try:
        raise RuntimeError("Don't forget me !")
    except Exception:
        logger.exception("No more donuts")
    log = cook_log(out.read())

    expected_log = (
        # fmt: off
        '{"event": "No more donuts", '
        '"level": "error", '
        '"timestamp": "2010-11-18T05:52:00.000Z", '
        '"exception": "Traceback (most recent call last):\\n'
        '  File \\"<workdir>/tests/test_logging.py\\", line 42, in test_exception_handling'
        '\\n    raise RuntimeError(\\"Don\'t forget me !\\")\\n'
        'RuntimeError: Don\'t forget me !"}\n'
        # fmt: on
    )
    assert log == expected_log

    # Test implicit exception catching from non-exception level

    try:
        raise RuntimeError("Don't forget me !")
    except Exception:
        logger.error("No more donuts", exc_info=True)
    log = cook_log(out.read())
    assert log == expected_log


def test_sentry_structlog_integration(capsentry):
    out = LogStream()
    # Set log_level to DEBUG to make sure it is not filtered before reaching Sentry
    logger = build_structlog_test_logger(
        log_level="DEBUG", log_format="JSON", log_stream=out.stream
    )

    # Debug level is ignored by Sentry

    logger.debug("Just ignore me, please.", name="chameleon")
    assert capsentry == []

    # Info&Warning are kept as breadcrumbs

    logger.info("Keep me, I'm info !", name="Donkey")
    logger.warning("Keep me, I'm warning !", name="Rhino")
    assert capsentry == []
    breadcrumbs = [
        {
            "type": "log",
            "level": "info",
            "category": "parsec.structlog",
            "message": "Keep me, I'm info !",
            "timestamp": ANY,
            "data": {"name": "Donkey"},
        },
        {
            "type": "log",
            "level": "warning",
            "category": "parsec.structlog",
            "message": "Keep me, I'm warning !",
            "timestamp": ANY,
            "data": {"name": "Rhino"},
        },
    ]

    # Error triggers Sentry

    logger.error("Can't ignore me now !", name="Tyrannosaurus")
    assert len(capsentry) == 1
    report = capsentry[0]
    assert report["level"] == "error"
    assert report["logger"] == "parsec.structlog"
    assert report["message"] == "Can't ignore me now !"
    assert report["extra"] == {"name": "Tyrannosaurus", "sys.argv": ANY}
    assert report["breadcrumbs"] == {"values": breadcrumbs}
    assert report["release"] == parsec_version
    assert not report.get("exception")
    # Last error is also part of the breadcrumbs
    breadcrumbs.append(
        {
            "type": "log",
            "level": "error",
            "category": "parsec.structlog",
            "message": "Can't ignore me now !",
            "timestamp": ANY,
            "data": {"name": "Tyrannosaurus", "sys.argv": ANY},
        }
    )

    # Another Sentry trigger, this time with exception info

    capsentry.clear()
    try:
        raise ValueError("Ignored error")
    except Exception:
        logger.exception(
            "No more donuts", name="Cookie Monster", exc_info=generate_raised_exception()
        )
    assert len(capsentry) == 1
    report = capsentry[0]
    assert report["level"] == "error"
    assert report["logger"] == "parsec.structlog"
    assert report["message"] == "No more donuts"
    assert report["extra"] == {"name": "Cookie Monster", "sys.argv": ANY}
    assert report["breadcrumbs"] == {"values": breadcrumbs}
    assert report["release"] == parsec_version
    assert report["exception"] == {
        "values": [
            {
                "type": "ValueError",
                "value": "Ignored error",
                "module": None,
                "mechanism": {"type": "structlog", "handled": True},
                "stacktrace": {"frames": [ANY]},
            },
            {
                "type": "RuntimeError",
                "value": "D'oh !",
                "module": None,
                "mechanism": {"type": "structlog", "handled": True},
                "stacktrace": {"frames": [ANY]},
            },
        ]
    }
    # Last error is also part of the breadcrumbs
    breadcrumbs.append(
        {
            "type": "log",
            "level": "error",
            "category": "parsec.structlog",
            "message": "No more donuts",
            "timestamp": ANY,
            "data": {"name": "Cookie Monster", "sys.argv": ANY},
        }
    )

    # Finaly Sentry trigger, this time with implicit exception info

    capsentry.clear()
    try:
        raise ValueError("Don't forget me !")
    except Exception:
        logger.exception("No more donuts", name="Homer J. Simpson")
    assert len(capsentry) == 1
    report = capsentry[0]
    assert report["level"] == "error"
    assert report["logger"] == "parsec.structlog"
    assert report["message"] == "No more donuts"
    assert report["extra"] == {"name": "Homer J. Simpson", "sys.argv": ANY}
    assert report["breadcrumbs"] == {"values": breadcrumbs}
    assert report["release"] == parsec_version
    assert report["exception"] == {
        "values": [
            {
                "type": "ValueError",
                "value": "Don't forget me !",
                "module": None,
                "mechanism": {"type": "structlog", "handled": True},
                "stacktrace": {"frames": [ANY]},
            }
        ]
    }


def test_sentry_stdlib_integration(capsentry):
    out = LogStream()
    # Set log_level to DEBUG to make sure it is not filtered before reaching Sentry
    logger = build_stdlib_test_logger(log_level="DEBUG", log_format="JSON", log_stream=out.stream)

    # Debug level is ignored by Sentry

    logger.debug("Just ignore me, please.")
    assert capsentry == []

    # Info&Warning are kept as breadcrumbs

    logger.info("Keep me, I'm info !")
    logger.warning("Keep me, I'm warning !")
    assert capsentry == []
    breadcrumbs = [
        {
            "type": "log",
            "level": "info",
            "category": "my_test_logger",
            "message": "Keep me, I'm info !",
            "timestamp": ANY,
            "data": {},
        },
        {
            "type": "log",
            "level": "warning",
            "category": "my_test_logger",
            "message": "Keep me, I'm warning !",
            "timestamp": ANY,
            "data": {},
        },
    ]

    # Error triggers Sentry

    logger.error("Can't ignore me now !")
    assert len(capsentry) == 1
    report = capsentry[0]
    assert report["level"] == "error"
    assert report["logger"] == "my_test_logger"
    assert report["logentry"] == {"message": "Can't ignore me now !", "params": []}
    assert report["extra"] == {"sys.argv": ANY}
    assert report["breadcrumbs"] == {"values": breadcrumbs}
    assert report["release"] == parsec_version
    assert not report.get("exception")
    # Last error is also part of the breadcrumbs
    breadcrumbs.append(
        {
            "type": "log",
            "level": "error",
            "category": "my_test_logger",
            "message": "Can't ignore me now !",
            "timestamp": ANY,
            "data": {},
        }
    )

    # Another Sentry trigger, this time with exception info

    capsentry.clear()
    try:
        raise ValueError("Ignored error")
    except Exception:
        logger.exception("No more donuts", exc_info=generate_raised_exception())
    assert len(capsentry) == 1
    report = capsentry[0]
    assert report["level"] == "error"
    assert report["logger"] == "my_test_logger"
    assert report["logentry"] == {"message": "No more donuts", "params": []}
    assert report["extra"] == {"sys.argv": ANY}
    assert report["breadcrumbs"] == {"values": breadcrumbs}
    assert report["release"] == parsec_version
    assert report["exception"] == {
        "values": [
            {
                "type": "ValueError",
                "value": "Ignored error",
                "module": None,
                "mechanism": {"type": "logging", "handled": True},
                "stacktrace": {"frames": [ANY]},
            },
            {
                "type": "RuntimeError",
                "value": "D'oh !",
                "module": None,
                "mechanism": {"type": "logging", "handled": True},
                "stacktrace": {"frames": [ANY]},
            },
        ]
    }
    # Last error is also part of the breadcrumbs
    breadcrumbs.append(
        {
            "type": "log",
            "level": "error",
            "category": "my_test_logger",
            "message": "No more donuts",
            "timestamp": ANY,
            "data": {},
        }
    )

    # Finaly Sentry trigger, this time with implicit exception info

    capsentry.clear()
    try:
        raise ValueError("Don't forget me !")
    except Exception:
        logger.exception("No more donuts")
    assert len(capsentry) == 1
    report = capsentry[0]
    assert report["level"] == "error"
    assert report["logger"] == "my_test_logger"
    assert report["logentry"] == {"message": "No more donuts", "params": []}
    assert report["extra"] == {"sys.argv": ANY}
    assert report["breadcrumbs"] == {"values": breadcrumbs}
    assert report["release"] == parsec_version
    assert report["exception"] == {
        "values": [
            {
                "type": "ValueError",
                "value": "Don't forget me !",
                "module": None,
                "mechanism": {"type": "logging", "handled": True},
                "stacktrace": {"frames": [ANY]},
            }
        ]
    }
