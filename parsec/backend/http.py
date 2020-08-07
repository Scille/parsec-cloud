# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
import attr
from typing import List, Dict, Tuple, Optional
import mimetypes
from urllib.parse import parse_qs, urlsplit, urlunsplit, urlencode
from wsgiref.handlers import format_date_time
import importlib_resources
import h11

from parsec.backend.config import BackendConfig
from parsec.backend import static as http_static_module
from parsec.backend.templates import get_template


@attr.s(slots=True, auto_attribs=True)
class HTTPRequest:
    method: str
    path: str
    query: Dict[str, List[str]]
    headers: Dict[bytes, bytes]

    async def get_data(self) -> bytes:
        # TODO: we don't need this yet ;-)
        raise NotImplementedError()

    @classmethod
    def from_h11_req(cls, h11_req: h11.Request) -> "HTTPRequest":
        # h11 make sur the headers and target are ISO-8859-1
        target_split = urlsplit(h11_req.target.decode("ISO-8859-1"))
        query_params = parse_qs(target_split.query)

        return cls(
            method=h11_req.method.decode(),
            path=target_split.path,
            query=query_params,
            headers=h11_req.headers,
        )


@attr.s(slots=True, auto_attribs=True)
class HTTPResponse:
    status_code: int
    headers: List[Tuple[bytes, bytes]]
    data: Optional[bytes]

    STATUS_CODE_TO_REASON = {
        200: b"OK",
        302: b"Found",
        400: b"Bad Request",
        404: b"Not Found",
        405: b"Method Not Allowed",
        501: b"Not Implemented",
    }

    @property
    def reason(self) -> bytes:
        return self.STATUS_CODE_TO_REASON.get(self.status_code, b"")

    @classmethod
    def build_html(
        cls, status_code: int, data: str, headers: Dict[str, str] = None
    ) -> "HTTPResponse":
        headers = headers or {}
        headers["content-type"] = "text/html;charset=utf-8"
        return cls.build(status_code=status_code, headers=headers, data=data.encode("utf-8"))

    @classmethod
    def build(
        cls, status_code: int, headers: Dict[str, str] = None, data: Optional[bytes] = None
    ) -> "HTTPResponse":
        headers = headers or {}
        if data:
            headers["content-Length"] = str(len(data))
        headers["date"] = format_date_time(None)

        def _encode(val):
            return val.encode("ISO-8859-1")

        return cls(status_code=status_code, headers=[(k, v) for k, v in headers.items()], data=data)


class HTTPComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    async def _http_404(self, req: HTTPRequest) -> HTTPResponse:
        data = get_template("404.html").render()
        return HTTPResponse.build_html(404, data=data)

    async def _http_root(self, req: HTTPRequest) -> HTTPResponse:
        data = get_template("index.html").render()
        return HTTPResponse.build_html(200, data=data)

    async def _http_redirect(self, req: HTTPRequest, path: str) -> HTTPResponse:
        if not self._config.backend_addr:
            return HTTPResponse.build(501, data=b"Url redirection is not available")
        backend_addr_split = urlsplit(self._config.backend_addr.to_url())

        # Build location url by merging provided path and query params with backend addr
        location_url_query_params = req.query.copy()
        # `no_ssl` param depends of backend_addr, hence it cannot be overwritten !
        location_url_query_params.pop("no_ssl", None)
        location_url_query_params.update(parse_qs(backend_addr_split.query))
        location_url_query = urlencode(query=location_url_query_params, doseq=True)
        location_url = urlunsplit(
            (backend_addr_split.scheme, backend_addr_split.netloc, path, location_url_query, None)
        )

        return HTTPResponse.build(302, headers={"location": location_url})

    async def _http_static(self, req: HTTPRequest, path: str) -> HTTPResponse:
        if path == "__init__.py":
            return HTTPResponse.build(404)

        try:
            # Note we don't support nested resources, this is fine for the moment
            # and it prevent us from malicious path containing `..`
            data = importlib_resources.read_binary(http_static_module, path)
        except (FileNotFoundError, ValueError):
            return HTTPResponse.build(404)

        headers = {}
        content_type, _ = mimetypes.guess_type(path)
        if content_type:
            headers = {"content-Type": content_type}
        return HTTPResponse.build(200, headers=headers, data=data)

    ROUTE_MAPPING = [
        (r"^/?$", _http_root),
        (r"^/redirect(?P<path>.*)$", _http_redirect),
        (r"^/static/(?P<path>.*)$", _http_static),
    ]

    async def handle_request(self, req: HTTPRequest) -> HTTPResponse:
        if req.method != "GET":
            return HTTPResponse.build(405)

        for path_pattern, route in self.ROUTE_MAPPING:
            match = re.match(path_pattern, req.path)
            if match:
                route_args = match.groupdict()
                break
        else:
            route = HTTPComponent._http_404
            route_args = {}

        return await route(self, req, **route_args)
