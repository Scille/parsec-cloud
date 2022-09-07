# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Type, Optional, List, Tuple
from pathlib import Path
import logging
import trio
from quart import (
    render_template,
    ResponseReturnValue,
    request,
    redirect as quart_redirect,
    ctx,
)
from quart_trio import QuartTrio
from hypercorn.config import Config as HyperConfig
from hypercorn.trio import serve

from parsec import __version__ as parsec_version
from parsec.backend.app import BackendApp
from parsec.backend.asgi.logger import ParsecLogger
from parsec.backend.config import BackendConfig
from parsec.backend.asgi.administration import administration_bp
from parsec.backend.asgi.redirect import redirect_bp
from parsec.backend.asgi.rpc import rpc_bp
from parsec.backend.asgi.ws import ws_bp
from parsec.backend.templates import JINJA_ENV_CONFIG


# Max size for HTTP body, 1Mo seems plenty given our API never upload big chunk of data
# (biggest request should be the `block_create` command with typically ~512Ko of data)
MAX_CONTENT_LENGTH = 1 * 1024**2


class BackendQuartTrio(QuartTrio):
    """A QuartTrio app that ensures that the backend is available in `g` global object."""

    backend: BackendApp

    def app_context(self) -> ctx.AppContext:
        app_context = super().app_context()
        app_context.g.backend = self.backend
        return app_context


def app_factory(
    backend: BackendApp, app_cls: Type[BackendQuartTrio] = BackendQuartTrio
) -> BackendQuartTrio:
    app = app_cls(
        __name__,
        static_folder="../static",
        static_url_path="/static",
        template_folder="templates",
    )
    app.config["MAX_CONTENT_LENGTH"] = MAX_CONTENT_LENGTH
    app.jinja_options = JINJA_ENV_CONFIG  # Overload config
    app.backend = backend
    app.register_blueprint(administration_bp)
    app.register_blueprint(redirect_bp)
    app.register_blueprint(rpc_bp)
    app.register_blueprint(ws_bp)

    # Do https redirection if incoming request doesn't follow forward proto rules
    if backend.config.forward_proto_enforce_https:
        header_key, header_expected_value = backend.config.forward_proto_enforce_https

        @app.before_request
        def redirect_unsecure() -> Optional[ResponseReturnValue]:
            header_value = request.headers.get(header_key)
            # If redirection header match and protocol match, then no need for a redirection.
            if header_value is not None and header_value != header_expected_value:
                if request.url.startswith("http://"):
                    return quart_redirect(request.url.replace("http://", "https://", 1), code=301)
            return None

    @app.route("/", methods=["GET"])
    async def root():
        return await render_template("index.html")

    @app.errorhandler(404)
    async def page_not_found(e):
        return await render_template("404.html"), 404

    return app


def _patch_server_header(backend_config: BackendConfig, hyper_config: HyperConfig) -> None:
    # By default, Hypercorn uses `hypercorn-h11` as `server` header.
    # This cannot be customized but only disabled, so we have to rely on function
    # patching to provide our own custom server header.

    # Our shinny custom server header !
    if backend_config.debug:
        server_header_value = f"parsec/{parsec_version}"
    else:
        server_header_value = "parsec"
    server_header_tuple = (b"server", server_header_value.encode("ascii"))

    # Disable `response_headers()` from adding Hypercorn default server header...
    hyper_config.include_server_header = False

    # ...then patch `response_headers()` to add our custom server header
    vanilla_response_headers = hyper_config.response_headers

    def response_headers_with_parsec_server_header(
        protocol: str,
    ) -> List[Tuple[bytes, bytes]]:
        headers = vanilla_response_headers(protocol)
        headers.append(server_header_tuple)
        return headers

    hyper_config.response_headers = response_headers_with_parsec_server_header  # type: ignore[assignment]


async def serve_backend_with_asgi(
    backend: BackendApp,
    host: str,
    port: int,
    ssl_certfile: Optional[Path] = None,
    ssl_keyfile: Optional[Path] = None,
    task_status=trio.TASK_STATUS_IGNORED,
) -> None:
    app = app_factory(backend)
    # Note: Hypercorn comes with default values for incoming data size to
    # avoid DoS abuse, so just trust them on that ;-)
    hyper_config = HyperConfig.from_mapping(
        {
            "bind": [f"{host}:{port}"],
            "accesslog": logging.getLogger("hypercorn.access"),
            # Timestamp is added by the log processor configured in `parsec.logging`,
            # here we configure peer address + req line + rep status + rep body size + time
            # (e.g. "GET 88.0.12.52:54160 /foo 1.1 404 823o 12343ms")
            "logger_class": ParsecLogger,
            "access_log_format": "%(h)s %(r)s %(s)s %(b)so %(D)sus (author: %(author)s)",
            "errorlog": logging.getLogger("hypercorn.error"),
            "certfile": str(ssl_certfile) if ssl_certfile else None,
            "keyfile": str(ssl_keyfile) if ssl_certfile else None,
        }
    )
    _patch_server_header(backend.config, hyper_config=hyper_config)

    await serve(app, hyper_config, task_status=task_status)

    # `hypercorn.serve` catches KeyboardInterrupt and returns, so re-raise
    # the keyboard interrupt to continue shutdown
    raise KeyboardInterrupt


__all__ = ("app_factory", "serve_backend_with_asgi")
