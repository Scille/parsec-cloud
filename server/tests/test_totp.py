# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime
from parsec.components.totp import compute_totp_one_time_password, generate_totp_secret, verify_totp


def test_compute_totp_one_time_password():
    # Officials test vectors for SHA1
    # see https://datatracker.ietf.org/doc/html/rfc6238#appendix-B
    secret = b"12345678901234567890"
    step = 30
    digits = 8
    for time, expected in [
        ("1970-01-01T00:00:59Z", "94287082"),
        ("2005-03-18T01:58:29Z", "07081804"),
        ("2005-03-18T01:58:31Z", "14050471"),
        ("2009-02-13T23:31:30Z", "89005924"),
        ("2033-05-18T03:33:20Z", "69279037"),
        ("2603-10-11T11:33:20Z", "65353130"),
    ]:
        got = compute_totp_one_time_password(
            now=DateTime.from_rfc3339(time), secret=secret, digits=digits, time_step_seconds=step
        )
        assert got == expected


def test_verify_totp():
    secret = b"12345678901234567890"
    step = 30
    now = DateTime.from_timestamp_seconds(step * 50000000)  # 2017-07-14T02:40:00Z
    one_time_password = "972579"

    # Current window
    assert verify_totp(now=now, secret=secret, one_time_password=one_time_password) is True
    # First second in the previous window
    assert (
        verify_totp(
            now=now.subtract(seconds=step), secret=secret, one_time_password=one_time_password
        )
        is True
    )
    assert (
        verify_totp(
            now=now.subtract(seconds=step + 1), secret=secret, one_time_password=one_time_password
        )
        is False
    )
    # Last second in the next window
    assert (
        verify_totp(
            now=now.add(seconds=(step * 2) - 1), secret=secret, one_time_password=one_time_password
        )
        is True
    )
    assert (
        verify_totp(
            now=now.add(seconds=step * 2), secret=secret, one_time_password=one_time_password
        )
        is False
    )


def test_generate_totp_secret():
    secret = generate_totp_secret()
    now = DateTime(2000, 1, 1)
    one_time_password = compute_totp_one_time_password(now=now, secret=secret)
    assert (
        verify_totp(now=now.add(seconds=30), secret=secret, one_time_password=one_time_password)
        is True
    )
