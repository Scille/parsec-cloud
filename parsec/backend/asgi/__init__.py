# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from quart import render_template, Response, request, redirect as quart_redirect
from quart_trio import QuartTrio

from parsec import __version__ as parsec_version
from parsec.backend.app import BackendApp
from parsec.backend.asgi.administration import administration_bp
from parsec.backend.asgi.redirect import redirect_bp
from parsec.backend.asgi.rpc import rpc_bp
from parsec.backend.asgi.ws import ws_bp
from parsec.backend.templates import JINJA_ENV_CONFIG


def app_factory(backend: BackendApp) -> QuartTrio:
    app = QuartTrio(
        __name__, static_folder="../static", static_url_path="/static", template_folder="templates"
    )
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
        # TODO: Won't be needed once we drop the legacy server code
        header_key = header_key.decode("utf8")
        header_expected_value = header_expected_value.decode("utf8")

        @app.before_request
        def redirect_unsecure() -> Response:
            header_value = request.headers.get(header_key)
            # If redirection header match and protocol match, then no need for a redirection.
            if header_value is not None and header_value != header_expected_value:
                if request.url.startswith("http://"):
                    return quart_redirect(request.url.replace("http://", "https://", 1), code=301)

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
