# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import click
import pytest

from parsec._parsec import (
    DateTime,
)
from parsec.cli.utils import ParsecDateTimeClickType


def test_datetime_parsing():
    parser = ParsecDateTimeClickType()
    dt = DateTime(2000, 1, 1, 12, 30, 59)
    assert parser.convert(dt, None, None) is dt
    assert parser.convert("2000-01-01T12:30:59Z", None, None) == dt
    assert parser.convert("2000-01-01T12:30:59.123Z", None, None) == dt.add(microseconds=123000)
    assert parser.convert("2000-01-01T12:30:59.123000Z", None, None) == dt.add(microseconds=123000)
    assert parser.convert("2000-01-01T12:30:59.123456Z", None, None) == dt.add(microseconds=123456)
    assert parser.convert("2000-01-01", None, None) == DateTime(2000, 1, 1)

    with pytest.raises(click.exceptions.BadParameter) as exc:
        parser.convert("dummy", None, None)
    assert (
        str(exc.value)
        == "`dummy` must be a RFC3339 date/datetime (e.g. `2000-01-01` or `2000-01-01T00:00:00Z`)"
    )

    with pytest.raises(click.exceptions.BadParameter):
        parser.convert("2000-01-01Z", None, None)

    # Timezone naive is not allowed
    with pytest.raises(click.exceptions.BadParameter) as exc:
        parser.convert("2000-01-01T12:30:59.123456", None, None)
    with pytest.raises(click.exceptions.BadParameter):
        parser.convert("2000-01-01T12:30:59", None, None)
