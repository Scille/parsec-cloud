# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Mapping
from hypercorn.config import Config as HyperConfig

import base64

from hypercorn.typing import ResponseSummary, HTTPScope

from parsec.backend.asgi.logger import ParsecLogger


def _create_http_scope(author: bytes = bytes(), add_author: bool = True) -> HTTPScope:
    scope = HTTPScope()
    scope["type"] = "http"
    scope["method"] = "GET"
    scope["query_string"] = b"/"
    scope["path"] = ""
    scope["scheme"] = ""
    scope["headers"] = [(b"Author", author)] if add_author else []

    return scope


def _create_empty_response() -> ResponseSummary:
    resp = ResponseSummary()
    resp["status"] = 200
    return resp


def _assert_anonymous_or_invalid(mapped: Mapping[str, str]):
    assert "author" in mapped
    assert mapped["author"] == "anonymous or invalid"


def test_base64_author():
    logger = ParsecLogger(HyperConfig())

    author_bytes = base64.b64encode(b"alice@work")
    mapped = logger.atoms(_create_http_scope(author_bytes), _create_empty_response(), 0.0)

    assert "author" in mapped
    assert mapped["author"] == "alice@work"


def test_bad_base64_author():
    import secrets

    logger = ParsecLogger(HyperConfig())

    # We use a sequence of 10 random bytes to simulate a bad base64 encoded author name
    author_bytes = secrets.token_bytes(10)
    mapped = logger.atoms(_create_http_scope(author_bytes), _create_empty_response(), 0.0)

    _assert_anonymous_or_invalid(mapped)

    assert "author" in mapped
    assert mapped["author"] == "anonymous or invalid"


def test_no_author_header():
    logger = ParsecLogger(HyperConfig())
    mapped = logger.atoms(_create_http_scope(add_author=False), _create_empty_response(), 0.0)

    _assert_anonymous_or_invalid(mapped)
