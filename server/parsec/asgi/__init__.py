# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path

import anyio
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


async def serve_parsec_asgi_app(
    app: AsgiApp,
    host: str,
    port: int,
    proxy_trusted_addresses: str | None,
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
        # Enable/Disable X-Forwarded-Proto, X-Forwarded-For to populate remote address info.
        # When enabled, is restricted to only trusting connecting IPs in forwarded-allow-ips.
        # See: https://www.uvicorn.org/settings/#http
        # Currently uvicorn only supports X-Forwarded-* headers (https://github.com/encode/uvicorn/issues/2237)
        # TODO: expose this setting to the user so it can be disabled.
        proxy_headers=True,
        # Comma separated list of IP Addresses, IP Networks, or literals (e.g. UNIX Socket path) to trust with proxy headers
        # Use "*" to trust all proxies. If not provided, the gunicorn/uvicorn `FORWARDED_ALLOW_IPS`
        # environment variable is used, defaulting to trusting only localhost if absent.
        forwarded_allow_ips=proxy_trusted_addresses,
        # Disable lifespan events, we don't need them for the moment
        # and they can cause CancelledError to bubble up in some cases
        lifespan="off",
        # TODO: configure access log format:
        # Timestamp is added by the log processor configured in `parsec.logging`,
        # here we configure peer address + req line + rep status + rep body size + time
        # (e.g. "GET 88.0.12.52:54160 /foo 1.1 404 823o 12343ms")
    )
    server = uvicorn.Server(config)

    async def server_task(task_status):
        # Protect server against cancellation
        with anyio.CancelScope(shield=True):
            task_status.started()
            await server.serve()
        tg.cancel_scope.cancel()

    async with anyio.create_task_group() as tg:
        await tg.start(server_task)
        try:
            await anyio.sleep_forever()
        finally:
            # Use should_exit to shutdown the server gracefully
            server.should_exit = True
