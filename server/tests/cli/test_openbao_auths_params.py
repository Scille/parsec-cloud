# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import json
import re
from typing import Any

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
    auths = _parse_openbao_auths(json.dumps([]))
    assert auths == []


def test_parse_ok() -> None:
    auths = _parse_openbao_auths(
        json.dumps(
            [
                {
                    "id": "PRO_CONNECT",
                    "mount_path": "auth/pro_connect",
                    "name": "ProConnect",
                    "help_link": "https://pro_connect.parsec.invalid/help",
                    "icon": "http://pro_connect.parsec.invalid/icon.svg",
                },
                {
                    "id": "HEXAGONE",
                    "mount_path": "auth/hexagone",
                    "name": "Hexagone",
                },
            ]
        )
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
        json.dumps(
            [{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": None, "icon": None}]
        )
    )
    assert auths == [
        OpenBaoAuthConfig(id="A", mount_path="auth/a", name="A", help_link=None, icon=None)
    ]


def test_parse_not_json() -> None:
    with pytest.raises(BadParameter):
        _parse_openbao_auths("foo")


@pytest.mark.parametrize(
    "param",
    [
        pytest.param({"id": "A", "mount_path": "auth/a", "name": "A"}, id="not_a_list"),
        pytest.param(["A"], id="item_not_an_object"),
        pytest.param(
            [{"id": "A", "mount_path": "auth/a", "name": "A", "helpLink": "https://a.invalid"}],
            id="unknown_key",
        ),
        pytest.param([{"mount_path": "auth/a", "name": "A"}], id="missing_id"),
        pytest.param([{"id": "A", "name": "A"}], id="missing_mount_path"),
        pytest.param([{"id": "A", "mount_path": "auth/a"}], id="missing_name"),
        pytest.param([{"id": 42, "mount_path": "auth/a", "name": "A"}], id="id_not_a_string"),
        pytest.param([{"id": "", "mount_path": "auth/a", "name": "A"}], id="empty_id"),
        pytest.param([{"id": "A", "mount_path": "", "name": "A"}], id="empty_mount_path"),
        pytest.param([{"id": "A", "mount_path": "auth/a", "name": ""}], id="empty_name"),
        pytest.param(
            [{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": 42}],
            id="help_link_not_a_string",
        ),
        pytest.param(
            [{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": ""}],
            id="empty_help_link",
        ),
        pytest.param(
            [{"id": "A", "mount_path": "auth/a", "name": "A", "help_link": "ftp://a.invalid"}],
            id="help_link_not_http",
        ),
        pytest.param(
            [{"id": "A", "mount_path": "auth/a", "name": "A", "icon": 42}],
            id="icon_not_a_string",
        ),
        pytest.param(
            [{"id": "A", "mount_path": "auth/a", "name": "A", "icon": "a.svg"}],
            id="icon_not_http",
        ),
        pytest.param(
            [
                {"id": "A", "mount_path": "auth/a", "name": "A"},
                {"id": "A", "mount_path": "auth/b", "name": "B"},
            ],
            id="duplicated_id",
        ),
    ],
)
def test_parse_bad(param: Any) -> None:
    with pytest.raises(BadParameter):
        _parse_openbao_auths(json.dumps(param))


def test_parse_unknown_keys_message() -> None:
    # Pydantic reports one error per unknown key
    with pytest.raises(BadParameter) as exc:
        _parse_openbao_auths(
            json.dumps([{"id": "A", "mount_path": "auth/a", "name": "A", "foo": 1, "bar": 2}])
        )
    assert "[0].foo: Extra inputs are not permitted" in exc.value.message
    assert "[0].bar: Extra inputs are not permitted" in exc.value.message


def test_parse_missing_key_message() -> None:
    with pytest.raises(BadParameter, match=re.escape("[0].name: Field required")):
        _parse_openbao_auths(json.dumps([{"id": "A", "mount_path": "auth/a"}]))


def test_parse_duplicated_id_message() -> None:
    with pytest.raises(BadParameter, match="Duplicate id"):
        _parse_openbao_auths(
            json.dumps(
                [
                    {"id": "A", "mount_path": "auth/a", "name": "A"},
                    {"id": "A", "mount_path": "auth/b", "name": "B"},
                ]
            )
        )
