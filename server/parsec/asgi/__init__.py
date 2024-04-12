# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path

import uvicorn
from fastapi import FastAPI, Request
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from starlette.types import Receive, Scope, Send

from parsec._version import __version__ as parsec_version
from parsec.asgi.administration import administration_router
from parsec.asgi.redirect import redirect_router
from parsec.asgi.rpc import rpc_router
from parsec.templates import JINJA_ENV_CONFIG

type AsgiApp = FastAPI


templates = Jinja2Templates(
    directory=(Path(__file__) / "../../templates").resolve(), **JINJA_ENV_CONFIG
)


def app_factory() -> AsgiApp:
    app = FastAPI(
        title="Parsec Server",
        version=parsec_version,
        # Disable auto-generated docs for the moment as it is broken (due to custom types validator for events)
        docs_url=None,
        redoc_url=None,
        openapi_url=None,
    )

    # Must be overwritten before the app is started !
    app.state.backend = None

    app.mount("/static", StaticFiles(packages=[("parsec", "static")]))

    @app.get("/")
    # pyright isn't able to see that root is used by FastAPI (cf: https://github.com/microsoft/pylance-release/issues/3622)
    def root(request: Request):  # pyright: ignore[reportUnusedFunction]
        return templates.TemplateResponse("index.html", {"request": request})

    async def page_not_found(scope: Scope, receive: Receive, send: Send) -> None:
        request = Request(scope)
        response = templates.TemplateResponse("404.html", {"request": request}, status_code=404)
        await response(scope, receive, send)

    app.router.default = page_not_found

    app.include_router(redirect_router)
    app.include_router(rpc_router)
    app.include_router(administration_router)

    return app


app: AsgiApp = app_factory()


# TODO: implement forward_proto_enforce_https
#     # Do https redirection if incoming request doesn't follow forward proto rules
#     if backend.config.forward_proto_enforce_https:
#         header_key, header_expected_value = backend.config.forward_proto_enforce_https

#         @app.before_request
#         def redirect_unsecure() -> ResponseReturnValue | None:
#             header_value = request.headers.get(header_key)
#             # If redirection header match and protocol match, then no need for a redirection.
#             if header_value is not None and header_value != header_expected_value:
#                 if request.url.startswith("http://"):
#                     return quart_redirect(request.url.replace("http://", "https://", 1), code=301)
#             return None


async def serve_parsec_asgi_app(
    app: AsgiApp,
    host: str,
    port: int,
    ssl_certfile: Path | None = None,
    ssl_keyfile: Path | None = None,
    workers: int | None = None,
) -> None:
    # `app.state.backend` must be overwritten by caller !
    assert app.state.backend is not None
    assert not parsec_version.startswith("v")
    if app.debug:
        # ex: parsec/3.0.1+dev1
        server_header = f"parsec/{parsec_version}"
    else:
        v_major, _ = parsec_version.split(".", 1)
        # ex: parsec/3
        server_header = f"parsec/{v_major}"
    # Note: Uvicorn comes with default values for incoming data size to
    # avoid DoS abuse, so just trust them on that ;-)
    config = uvicorn.Config(
        app=app,
        host=host,
        port=port,
        server_header=False,
        headers=[("Server", server_header)],
        log_level="info",
        # TODO: uvicorn's typing is broken here
        ssl_keyfile=ssl_keyfile,  # type: ignore
        ssl_certfile=ssl_certfile,
        workers=workers,
        # TODO: configure access log format:
        # Timestamp is added by the log processor configured in `parsec.logging`,
        # here we configure peer address + req line + rep status + rep body size + time
        # (e.g. "GET 88.0.12.52:54160 /foo 1.1 404 823o 12343ms")
    )
    server = uvicorn.Server(config)
    await server.serve()
