# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, Any
from urllib.parse import parse_qs, quote_plus, urlencode, urlsplit, urlunsplit

from quart import Blueprint, abort, g, redirect, request

if TYPE_CHECKING:
    from parsec._parsec import BackendAddr
    from parsec.backend.app import BackendApp


redirect_bp = Blueprint("redirect", __name__)


@redirect_bp.route("/redirect/<path:path>", methods=["GET"])
def redirect_parsec_url(path: str) -> Any:  # type: ignore[misc]
    backend: BackendApp = g.backend
    backend_addr: BackendAddr | None = backend.config.backend_addr
    if not backend_addr:
        abort(501, description="Url redirection is not available")

    backend_addr: "BackendAddr"
    backend_addr_split = urlsplit(backend_addr.to_url())

    # Url may contains utf8 characters, so we have to encode it back to
    # the all ascii format compatible with HTTP. This cannot raises error
    # given path comes from `unquote_plus`.
    path = quote_plus(path, safe="/", encoding="utf8", errors="strict")

    # Build location url by merging provided path and query params with backend addr
    # `no_ssl` param depends of backend_addr, hence it cannot be overwritten !
    location_url_query_params = {k: vs for k, vs in request.args.lists() if k != "no_ssl"}
    location_url_query_params.update(parse_qs(backend_addr_split.query))
    # `doseq` stands for "do sequence", hence a key can have multiple values
    location_url_query = urlencode(query=location_url_query_params, doseq=True)
    location_url = urlunsplit(
        (backend_addr_split.scheme, backend_addr_split.netloc, path, location_url_query, None)
    )

    return redirect(location_url)
