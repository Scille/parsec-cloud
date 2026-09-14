# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import mimetypes
import os
import socket
from collections.abc import Sequence
from pathlib import Path
from typing import cast

import anyio
import uvicorn
from anyio.abc import TaskStatus
from fastapi import FastAPI, HTTPException, Request, Response
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import FileResponse, RedirectResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from starlette.datastructures import Headers
from starlette.staticfiles import PathLike
from starlette.types import ASGIApp, Message, Receive, Scope, Send

from parsec._version import __version__ as parsec_version
from parsec.asgi.administration import administration_router
from parsec.asgi.redirect import redirect_router
from parsec.asgi.rpc import Backend, rpc_router
from parsec.config import BackendConfig
from parsec.logging import get_logger

logger = get_logger()

type AsgiApp = FastAPI

# OpenAPI tags metadata for auto-generated documentation
openapi_tags = [
    {
        "name": "Organization",
        "description": "Administration API for **Organizations**. For more details, see "
        "[Server Administration Guide](https://docs.parsec.cloud/en/latest/hosting/administration/index.html).",
    },
    {
        "name": "Users",
        "description": "Administration API for **Users**.",
    },
    {
        "name": "Sequester",
        "description": "Administration API for **Sequester service**. For more details, see "
        "[Sequester service](https://docs.parsec.cloud/en/latest/hosting/sequester/index.html).",
    },
    {
        "name": "Stats",
        "description": "Administration API for **Statistics**.",
    },
]

WEB_APP_BASE_URL = "/client/"
WEB_APP_ASSETS_URL = "/client/assets/"
WEB_APP_CUSTOM_ASSETS_URL = "/client/custom/"
# Editics is a special case since it requires two origins in order to achieve
# iframe isolation on the client-side: one for Parsec client app and another
# for the editics document editor.
# This is achieved by configuring two domain names for Parsec server, and choosing
# which app can be served depending on the request's host info.
WEB_APP_EDITICS_URL = "/client/editics/"
# We use a very aggressive one-size-fits-all cache config here since all our
# static resources (including the optional web app) are designed to use
# cache-busting naming (i.e. having content hash in their name, e.g. `base-jFjh9D00.css`).
STATIC_ASSETS_CACHE_CONTROL = "max-age=31536000, public, immutable"
EDITICS_CONTENT_SECURITY_PROTOCOL_TEMPLATE = (
    "default-src 'self'"
    # x2t compiles its WASM module, OnlyOffice uses `new Function()`
    "; script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval' 'unsafe-inline'"
    "; style-src 'self' 'unsafe-inline'"
    "; img-src 'self' blob: data:"
    "; font-src 'self' blob: data:"
    "; connect-src 'self' blob: data:"
    "; worker-src 'self' blob:"
    "; frame-src 'self'"
    "; media-src 'self' blob: data:"
    "; object-src 'none'"
    "; base-uri 'self'"
    "; form-action 'none'"
    "; frame-ancestors 'self' {web_app_addr}"
)


def _try_brotli_response(full_path: PathLike, scope: Scope, status_code: int) -> Response | None:
    accept_encoding = Headers(scope=scope).get("accept-encoding", "")
    encodings = {e.split(";")[0].strip() for e in accept_encoding.split(",")}
    if "br" not in encodings:
        return None
    br_path = Path(str(full_path) + ".br")
    if not br_path.is_file():
        return None
    content_type, _ = mimetypes.guess_type(str(full_path))
    response = FileResponse(br_path, status_code=status_code, media_type=content_type)
    response.headers["Content-Encoding"] = "br"
    response.headers["Vary"] = "Accept-Encoding"
    return response


class StaticFilesWithCacheControl(StaticFiles):
    def file_response(
        self,
        full_path: PathLike,
        stat_result: os.stat_result,
        scope: Scope,
        status_code: int = 200,
    ) -> Response:
        if response := _try_brotli_response(full_path, scope, status_code):
            response.headers["Cache-Control"] = STATIC_ASSETS_CACHE_CONTROL
            return response
        response = super().file_response(full_path, stat_result, scope, status_code)
        response.headers["Cache-Control"] = STATIC_ASSETS_CACHE_CONTROL
        return response


# This class is used to serve the static files of the web app (SPA)
# and redirect to the index.html if file not found on server.
# Useful when the user refresh the page or access a route directly.
# Note we don't use cache-control for this resource as `index.html` is the
# entry point from where the other resources are loaded (i.e. it is the
# one providing the cache-busting info!).
class StaticFilesWithSPARedirect(StaticFiles):
    def lookup_path(self, path: str) -> tuple[str, os.stat_result | None]:
        match super().lookup_path(path):
            case (_, None):
                return super().lookup_path("index.html")
            case found:
                return found


class EditicsHostIsolationMiddleware:
    """Ensure the editics pages are only served from the editics domain.

    The server can be reached from two different domains:
    - the main server domain (e.g. `app.parsec.cloud`), which must never serve
      the `editics/` pages (they are meant to be isolated in an iframe)
    - the editics domain (e.g. `editics.app.parsec.cloud`), which must only
      serve the `editics/` pages (in particular the main web app entry points
      such as `index.html` or `assets/` must not be reachable from it)

    This middleware only wraps the `/client/*` mounts (the other routes, such
    as the RPC ones queried by the editics pages, are shared by both domains).
    """

    def __init__(self, app: ASGIApp, mount_path: str, config: BackendConfig) -> None:
        self.app = app
        self.editics_netloc = (
            config.editics_server_addr.netloc.encode("latin-1")
            if config.editics_server_addr
            else None
        )
        self.is_editics_path = mount_path.startswith(WEB_APP_EDITICS_URL)
        if self.is_editics_path:
            web_app_addr = (
                "https://" if config.server_addr.use_ssl else "http://"
            ) + config.server_addr.netloc
            self.editics_content_security_policy = (
                EDITICS_CONTENT_SECURITY_PROTOCOL_TEMPLATE.format(web_app_addr=web_app_addr)
            )

    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        if scope["type"] != "http":
            await self.app(scope, receive, send)
            return

        is_editics_host = (
            next((v for k, v in scope["headers"] if k == b"host"), b"") == self.editics_netloc
        )
        if is_editics_host:
            if self.is_editics_path:

                async def _send_with_csp(message: Message) -> None:
                    if message["type"] == "http.response.start":
                        message["headers"].append(
                            (b"content-security-policy", self.editics_content_security_policy)
                        )
                    await send(message)

                await self.app(scope, receive, _send_with_csp)

            else:
                raise HTTPException(403)

        else:
            if self.is_editics_path:
                raise HTTPException(403)
            else:
                await self.app(scope, receive, send)


def app_factory(
    backend: Backend,
    cors_allow_origins: list[str] = [],
    with_client_web_app: Path | None = None,
) -> AsgiApp:
    app = FastAPI(
        title="Parsec Server",
        version=parsec_version,
        openapi_tags=openapi_tags,
    )
    app.add_middleware(
        CORSMiddleware,
        allow_origins=cors_allow_origins,
        allow_methods=["OPTIONS", "GET", "POST", "PATCH"],
        # last-event-id is used for SEE synchronization when an interruption occurs.
        allow_headers=["api-version", "authorization", "user-agent", "last-event-id"],
        # Needed to be able to retrieve and show server version in the web version
        expose_headers=["server"],
    )
    logger.debug(f"Allowed origins: {cors_allow_origins}")
    app.state.backend = backend
    app.state.with_client_web_app = with_client_web_app
    templates = Jinja2Templates(env=backend.config.jinja_env)

    if with_client_web_app:
        # Note we must declare `/client/` route *after* `/client/.../` narrower routes
        # (e.g. `/client/custom/`) given the router tries each route according to
        # their order of declaration!
        # (See `Route priority` in https://starlette.dev/routing/)

        # The `/client/editics` is *only* available from the editics domain (and
        # hence never available if `backend.config.editics_server_addr` is `None`).
        # And, conversely, other `/client/*` routes are not allowed from the editics domain.
        def _with_editics_isolation(app: ASGIApp, mount_path: str) -> ASGIApp:
            return EditicsHostIsolationMiddleware(app, mount_path=mount_path, config=backend.config)

        app.mount(
            WEB_APP_ASSETS_URL,
            _with_editics_isolation(
                StaticFilesWithCacheControl(directory=with_client_web_app / "assets"),
                WEB_APP_ASSETS_URL,
            ),
        )

        custom_assets_dir = with_client_web_app / "custom"
        if custom_assets_dir.is_dir():
            custom_assets_app = StaticFilesWithCacheControl(
                directory=with_client_web_app / "custom"
            )
        else:
            # No customization, we still need a specific handler to ensure we return 404,
            # otherwise the SPA redirect (with status 200) will be triggered which
            # may trick the client into thinking a customization is present!
            class NoCustomDirAlways404:
                async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
                    raise HTTPException(status_code=404)

            custom_assets_app = NoCustomDirAlways404()
        app.mount(
            WEB_APP_CUSTOM_ASSETS_URL,
            _with_editics_isolation(custom_assets_app, WEB_APP_CUSTOM_ASSETS_URL),
        )

        editics_dir = with_client_web_app / "editics"
        if editics_dir.is_dir():
            # All editics resources (including the html entrypoints, e.g.
            # `offline-abc123456.html`) have cache-busting names
            app.mount(
                WEB_APP_EDITICS_URL,
                _with_editics_isolation(
                    StaticFilesWithCacheControl(directory=editics_dir), WEB_APP_EDITICS_URL
                ),
            )

        app.mount(
            WEB_APP_BASE_URL,
            _with_editics_isolation(
                StaticFilesWithSPARedirect(directory=with_client_web_app, html=True),
                WEB_APP_BASE_URL,
            ),
        )

        def root(request: Request) -> Response:
            return RedirectResponse(url=WEB_APP_BASE_URL, status_code=301)

    else:

        def root(request: Request) -> Response:
            return templates.TemplateResponse(request=request, name="index.html")

    app.get("/", include_in_schema=False)(root)

    app.mount("/static", StaticFilesWithCacheControl(packages=[("parsec", "static")]))

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

    def __init__(self, config: uvicorn.Config) -> None:
        super().__init__(config)
        self._should_exit = False

    @property
    def should_exit(self) -> bool:
        return self._should_exit

    @should_exit.setter
    def should_exit(self, value: bool) -> None:
        self._should_exit = value
        if self._should_exit:
            app = cast(AsgiApp, self.config.app)
            backend = cast(Backend, app.state.backend)
            backend.events.stop()

    def _log_started_message(self, listeners: Sequence[socket.SocketType]) -> None:
        config = self.config

        if config.fd is not None:  # pragma: py-win32
            sock = listeners[0]
            server_url = sock.getsockname()
            logger.info(f"Uvicorn running on socket {server_url} (Press CTRL+C to quit)")

        elif config.uds is not None:  # pragma: py-win32
            server_url = config.uds
            logger.info(f"Uvicorn running on unix socket {server_url} (Press CTRL+C to quit)")

        else:
            addr_format = "{schema}://{host}:{port}"
            host = config.host
            if ":" in host:
                # It's an IPv6 address.
                addr_format = "{schema}://[{host}]:{port}"

            port = config.port
            if port == 0:
                port = listeners[0].getsockname()[1]

            protocol_name = "https" if config.ssl else "http"
            server_url = addr_format.format(schema=protocol_name, host=host, port=port)
            logger.info(f"Uvicorn running on {server_url} (Press CTRL+C to quit)")

        if isinstance(config.app, FastAPI) and config.app.openapi_url:
            logger.info(
                "OpenAPI URLS",
                redoc=f"{server_url}{config.app.redoc_url}",
                openapi_json=f"{server_url}{config.app.openapi_url}",
                docs=f"{server_url}{config.app.docs_url}",
            )


async def serve_parsec_asgi_app(
    app: AsgiApp,
    host: str,
    port: int,
    proxy_trusted_addresses: str | None,
    ssl_ciphers: list[str],
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

    logger.debug("Using the following TLS ciphers suite", ciphers=ssl_ciphers)

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
        # openssl expects the list to be colon separated.
        ssl_ciphers=":".join(ssl_ciphers),
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

    async def server_task(task_status: TaskStatus):
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
