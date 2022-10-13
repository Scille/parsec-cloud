# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Optional
import base64
from hypercorn.config import Config as HyperConfig
from hypercorn.typing import ResponseSummary, HTTPScope

from parsec.backend.asgi.logger import ParsecLogger


def _create_http_scope(author: Optional[bytes] = None) -> HTTPScope:
    scope = HTTPScope()
    scope["type"] = "http"
    scope["method"] = "GET"
    scope["query_string"] = b"/"
    scope["path"] = ""
    scope["scheme"] = ""
    scope["headers"] = [(b"Author", author)] if author is not None else []

    return scope


def _create_empty_response() -> ResponseSummary:
    resp = ResponseSummary()
    resp["status"] = 200
    return resp


def test_base64_author():
    logger = ParsecLogger(HyperConfig())

    author_bytes = base64.b64encode(b"alice@work")
    mapped = logger.atoms(_create_http_scope(author_bytes), _create_empty_response(), 0.0)

    assert "author" in mapped
    assert mapped["author"] == "alice@work"


def test_bad_base64_author():
    logger = ParsecLogger(HyperConfig())

    # Invalid base64 sequence
    author_bytes = b"<dummy>"
    mapped = logger.atoms(_create_http_scope(author_bytes), _create_empty_response(), 0.0)

    assert "author" not in mapped


def test_no_author_header():
    logger = ParsecLogger(HyperConfig())
    mapped = logger.atoms(_create_http_scope(), _create_empty_response(), 0.0)

    assert "author" not in mapped
