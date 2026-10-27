# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import pytest
from click import BadParameter

from parsec.cli.run import _parse_openbao_auths
from parsec.config import (
    OpenBaoAuthConfig,
)


def test_parse_none() -> None:
    auths = _parse_openbao_auths(None)
    assert auths is None


def test_parse_empty() -> None:
    auths = _parse_openbao_auths("[]")
    assert auths == []


def test_parse_ok() -> None:
    auths = _parse_openbao_auths(
        """[
            {
                "id": "PRO_CONNECT",
                "mount_path": "auth/pro_connect",
                "name": "ProConnect",
                "help_link": "https://pro_connect.parsec.invalid/help",
                "icon": "http://pro_connect.parsec.invalid/icon.svg"
            },
            {
                "id": "HEXAGONE",
                "mount_path": "auth/hexagone",
                "name": "Hexagone"
            }
        ]"""
    )
    assert auths == [
        OpenBaoAuthConfig(
            id="PRO_CONNECT",
            mount_path="auth/pro_connect",
            name="ProConnect",
            help_link="https://pro_connect.parsec.invalid/help",
            icon="http://pro_connect.parsec.invalid/icon.svg",
        ),
        OpenBaoAuthConfig(
            id="HEXAGONE",
            mount_path="auth/hexagone",
            name="Hexagone",
            help_link=None,
            icon=None,
        ),
    ]


def test_parse_explicit_null_optional_fields() -> None:
    auths = _parse_openbao_auths(
        '[{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": null, "icon": null}]'
    )
    assert auths == [
        OpenBaoAuthConfig(id="A", mount_path="auth/a", name="A", help_link=None, icon=None)
    ]


@pytest.mark.parametrize(
    "param",
    [
        "foo",  # Not JSON
        '{"id": "A", "mount_path": "auth/a", "name": "A"}',  # Not a list
        '["A"]',  # Item not an object
        '[{"id": "A", "mount_path": "auth/a", "name": "A", "helpLink": "https://a.invalid"}]',  # Unknown key
        '[{"mount_path": "auth/a", "name": "A"}]',  # Missing `id`
        '[{"id": "A", "name": "A"}]',  # Missing `mount_path`
        '[{"id": "A", "mount_path": "auth/a"}]',  # Missing `name`
        '[{"id": 42, "mount_path": "auth/a", "name": "A"}]',  # `id` not a string
        '[{"id": "", "mount_path": "auth/a", "name": "A"}]',  # Empty `id`
        '[{"id": "A", "mount_path": "", "name": "A"}]',  # Empty `mount_path`
        '[{"id": "A", "mount_path": "auth/a", "name": ""}]',  # Empty `name`
        '[{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": 42}]',  # `help_link` not a string
        '[{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": ""}]',  # Empty `help_link`
        '[{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": "ftp://a.invalid"}]',  # `help_link` not http(s)
        '[{"id": "A", "mount_path": "auth/a", "name": "A", "icon": 42}]',  # `icon` not a string
        '[{"id": "A", "mount_path": "auth/a", "name": "A", "icon": "a.svg"}]',  # `icon` not http(s)
        '[{"id": "A", "mount_path": "auth/a", "name": "A"}, {"id": "A", "mount_path": "auth/b", "name": "B"}]',  # Duplicated `id`
    ],
)
def test_parse_bad(param: str) -> None:
    with pytest.raises(BadParameter):
        _parse_openbao_auths(param)


def test_parse_unknown_keys_message() -> None:
    # Unknown keys are sorted to have a deterministic error message
    with pytest.raises(BadParameter, match="Unknown keys: bar, foo"):
        _parse_openbao_auths(
            '[{"id": "A", "mount_path": "auth/a", "name": "A", "foo": 1, "bar": 2}]'
        )


def test_parse_missing_key_message() -> None:
    with pytest.raises(BadParameter, match="Missing key name"):
        _parse_openbao_auths('[{"id": "A", "mount_path": "auth/a"}]')
