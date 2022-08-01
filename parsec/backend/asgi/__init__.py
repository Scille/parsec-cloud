# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Type, Optional
from quart import (
    render_template,
    Response,
    ResponseReturnValue,
    request,
    redirect as quart_redirect,
    ctx,
)
from quart_trio import QuartTrio

from parsec import __version__ as parsec_version
from parsec.backend.app import BackendApp
from parsec.backend.asgi.administration import administration_bp
from parsec.backend.asgi.redirect import redirect_bp
from parsec.backend.asgi.rpc import rpc_bp
from parsec.backend.asgi.ws import ws_bp
from parsec.backend.templates import JINJA_ENV_CONFIG


# Max size for HTTP body, 1Mo seems plenty given our API never upload big chunk of data
# (biggest request should be the `block_create` command with typically ~512Ko of data)
MAX_CONTENT_LENGTH = 1 * 1024 ** 2


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
        __name__, static_folder="../static", static_url_path="/static", template_folder="templates"
    )
    app.config["MAX_CONTENT_LENGTH"] = MAX_CONTENT_LENGTH
    app.jinja_options = JINJA_ENV_CONFIG  # Overload config
    app.backend = backend
    app.register_blueprint(administration_bp)
    app.register_blueprint(redirect_bp)
    app.register_blueprint(rpc_bp)
    app.register_blueprint(ws_bp)

    if backend.config.debug:
        server_header = f"parsec/{parsec_version}"
    else:
        server_header = "parsec"

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

    @app.after_request
    def add_server_header(response: Response) -> Response:
        response.headers["server"] = server_header
        return response

    @app.route("/", methods=["GET"])
    async def root():
        return await render_template("index.html")

    @app.errorhandler(404)
    async def page_not_found(e):
        return await render_template("404.html"), 404

    return app


__all__ = ("app_factory",)
