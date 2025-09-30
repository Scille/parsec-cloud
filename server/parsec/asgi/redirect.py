# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections import defaultdict
from typing import TYPE_CHECKING, Annotated, Any
from urllib.parse import SplitResult, parse_qs, quote_plus, urlencode, urlsplit, urlunsplit

from fastapi import APIRouter, Depends, HTTPException, Request
from fastapi.responses import RedirectResponse
from fastapi.templating import Jinja2Templates

if TYPE_CHECKING:
    from parsec.backend import Backend


def get_server_addr_split(request: Request) -> SplitResult | None:
    try:
        return request.app.state.server_addr_split
    except AttributeError:
        backend: Backend = request.app.state.backend
        if not backend.config.server_addr:
            request.app.state.server_addr_split = None
        else:
            request.app.state.server_addr_split = urlsplit(backend.config.server_addr.to_url())
    return request.app.state.server_addr_split


redirect_router = APIRouter()


@redirect_router.get("/redirect/{path:path}")
def redirect_parsec_url(
    path: str,
    request: Request,
    server_addr_split: Annotated[SplitResult | None, Depends(get_server_addr_split)],
) -> Any:
    if not server_addr_split:
        raise HTTPException(status_code=501, detail="Url redirection is not available")

    # Url may contains utf8 characters, so we have to encode it back to
    # the all ascii format compatible with HTTP. This cannot raises error
    # given path comes from `unquote_plus`.
    path = quote_plus(path, safe="/", encoding="utf8", errors="strict")

    # Build location url by merging provided path and query params with server addr
    # `no_ssl` param depends of server_addr, hence it cannot be overwritten !
    location_url_query_params = defaultdict(list)
    for k, v in request.query_params.multi_items():
        if k == "no_ssl":
            continue
        location_url_query_params[k].append(v)
    location_url_query_params.update(parse_qs(server_addr_split.query))
    # `doseq` stands for "do sequence", hence a key can have multiple values
    location_url_query = urlencode(query=location_url_query_params, doseq=True)
    location_url = urlunsplit(
        (server_addr_split.scheme, server_addr_split.netloc, path, location_url_query, None)
    )

    with_client_web_app: bool = request.app.state.with_client_web_app
    if with_client_web_app:
        templates: Jinja2Templates = request.app.state.templates
        return templates.TemplateResponse(
            context={
                # Used for direct 302 redirection when the user wants to open the native app
                "location_url": location_url,
                # Used to create an url pointing to the web client with `url_safe_location_url`
                # passed as URL param.
                "url_safe_location_url": quote_plus(
                    location_url, safe="/", encoding="utf8", errors="strict"
                ),
            },
            request=request,
            name="redirect.html",
            status_code=302,
        )  # TOOD: 301 ? 302 ?

    else:
        return RedirectResponse(url=location_url, status_code=302)
