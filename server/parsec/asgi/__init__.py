# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import os
from pathlib import Path
from typing import cast

import anyio
import uvicorn
from fastapi import FastAPI, HTTPException, Request, Response
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import RedirectResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from starlette.types import Receive, Scope, Send

from parsec._version import __version__ as parsec_version
from parsec.asgi.administration import administration_router
from parsec.asgi.redirect import redirect_router
from parsec.asgi.rpc import Backend, rpc_router
from parsec.templates import JINJA_ENV_CONFIG

type AsgiApp = FastAPI


templates = Jinja2Templates(
    directory=(Path(__file__) / "../../templates").resolve(), **JINJA_ENV_CONFIG
)

tags_metadata = [
    {
        "name": "administration",
        "description": "REST API for administration operations.",
    },
    {
        "name": "rpc",
        "description": "RPC Parsec API for business-logic operations.",
    },
    {
        "name": "testbed",
        "description": "API for testbed customization operations.",
    },
]

WEB_APP_BASE_URL = "/client/"


# This class is used to serve the static files of the web app (SPA)
# and redirect to the index.html if file not found on server.
# Useful when the user refresh the page or access a route directly.
class StaticFilesWithSPARedirect(StaticFiles):
    def lookup_path(self, path: str) -> tuple[str, os.stat_result | None]:
        match super().lookup_path(path):
            case (_, None):
                if path.startswith("assets/"):
                    raise HTTPException(status_code=404)
                else:
                    return super().lookup_path("index.html")
            case found:
                return found


def app_factory(
    backend: Backend,
    cors_allow_origins: list[str] = [],
    with_client_web_app: Path | None = None,
) -> AsgiApp:
    app = FastAPI(
        title="Parsec Server",
        version=parsec_version,
        openapi_tags=tags_metadata,
    )
    app.add_middleware(
        CORSMiddleware,
        allow_origins=cors_allow_origins,
        allow_methods=["OPTIONS", "GET", "POST", "PATCH"],
        allow_headers=["api-version", "authorization", "user-agent"],
    )
    app.state.backend = backend

    if with_client_web_app:

        def root(request: Request) -> Response:
            return RedirectResponse(url=WEB_APP_BASE_URL, status_code=301)

        app.mount(
            WEB_APP_BASE_URL, StaticFilesWithSPARedirect(directory=with_client_web_app, html=True)
        )
    else:

        def root(request: Request) -> Response:
            return templates.TemplateResponse(request=request, name="index.html")

    app.get("/")(root)

    app.mount("/static", StaticFiles(packages=[("parsec", "static")]))

    async def page_not_found(scope: Scope, receive: Receive, send: Send) -> None:
        request = Request(scope)
        response = templates.TemplateResponse(request=request, name="404.html", status_code=404)
        await response(scope, receive, send)

    app.router.default = page_not_found

    app.include_router(redirect_router)
    app.include_router(rpc_router)
    app.include_router(administration_router)

    return app


class Server(uvicorn.Server):
    """
    We are patching the unicorn server in order to be notified when the server should exit.

    This way we are able to communicate to the app that the SSE connections for the registered
    clients should get closed. This allows for the graceful shutdown of the server to properly
    complete, since this procedure does not cancel the ongoing tasks.

    Also note that the "force quit" feature of uvicorn (i.e sending another SIGINT while the
    server is in "should exit" mode) does not behave as expected in the sense that it won't
    cancel the ongoing tasks, as reported here:
    https://github.com/encode/uvicorn/discussions/2525#discussion-7603322

    This is why we also set a timeout for the graceful shutdown, so that the server won't
    hang indefinitely when shutting down while clients with SSE connections are still active.
    """

    _should_exit: bool

    @property
    def should_exit(self) -> bool:
        try:
            return self._should_exit
        except AttributeError:
            return False

    @should_exit.setter
    def should_exit(self, value: bool) -> None:
        self._should_exit = value
        if self._should_exit:
            app = cast(AsgiApp, self.config.app)
            backend = cast(Backend, app.state.backend)
            backend.events.stop()


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
        # Remove default log config to inherit instead the one we set in `parsec.logging`
        log_config=None,
        ssl_keyfile=ssl_keyfile,
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
        # Force a shutdown after 10 seconds, in case of a graceful shutdown failure
        # See the `Server` docstring for more information
        timeout_graceful_shutdown=10,
    )
    server = Server(config)

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
