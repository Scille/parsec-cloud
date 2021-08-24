# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import re
import attr
import json
from typing import Awaitable, Callable, List, Dict, Set, Optional, Pattern, Tuple, Union
import mimetypes
from urllib.parse import parse_qs, urlsplit, urlunsplit, urlencode
import importlib_resources
import h11

from parsec.serde import SerdeValidationError, SerdePackingError
from parsec.api.protocol import OrganizationID
from parsec.api.rest import (
    organization_config_req_serializer,
    organization_config_rep_serializer,
    organization_create_req_serializer,
    organization_create_rep_serializer,
    organization_update_req_serializer,
    organization_update_rep_serializer,
    organization_stats_req_serializer,
    organization_stats_rep_serializer,
)
from parsec.backend.config import BackendConfig
from parsec.backend import static as http_static_module
from parsec.backend.templates import get_template
from parsec.backend.organization import (
    BaseOrganizationComponent,
    OrganizationAlreadyExistsError,
    OrganizationNotFoundError,
    generate_bootstrap_token,
)

try:
    from typing import Protocol
except ImportError:
    from typing_extensions import Protocol  # type: ignore


class RequestHandler(Protocol):
    async def __call__(self, req: "HTTPRequest", **kwargs: str) -> "HTTPResponse":
        ...


@attr.s(slots=True, auto_attribs=True)
class HTTPRequest:
    method: str
    path: str
    query: Dict[str, List[str]]
    headers: Dict[bytes, bytes]
    _get_body: Callable[[int], Awaitable[bytes]]
    _body: Optional[bytes] = None

    async def get_body(self) -> bytes:
        if self._body is None:
            # h11 already sanitize content-length field
            content_length = int(self.headers.get(b"content-length", 0))
            self._body = await self._get_body(content_length)
        return self._body

    @classmethod
    def from_h11_req(
        cls, h11_req: h11.Request, get_body: Callable[[int], Awaitable[bytes]]
    ) -> "HTTPRequest":
        # h11 makes sure the headers and target are ISO-8859-1
        target_split = urlsplit(h11_req.target.decode("ISO-8859-1"))
        query_params = parse_qs(target_split.query)

        # Note h11 already does normalization on headers
        # (see https://h11.readthedocs.io/en/latest/api.html?highlight=request#headers-format)
        return cls(
            method=h11_req.method.decode(),
            path=target_split.path,
            query=query_params,
            headers=dict(h11_req.headers),
            get_body=get_body,
        )


@attr.s(slots=True, auto_attribs=True)
class HTTPResponse:
    status_code: int
    headers: Dict[bytes, bytes]
    data: Optional[bytes]

    @classmethod
    def build_html(
        cls, status_code: int, data: str, headers: Optional[Dict[bytes, bytes]] = None
    ) -> "HTTPResponse":
        headers = headers or {}
        headers[b"content-type"] = b"text/html;charset=utf-8"
        return cls.build(status_code=status_code, headers=headers, data=data.encode("utf-8"))

    @classmethod
    def build_rest(
        cls, status_code: int, data: Union[list, dict], headers: Optional[Dict[bytes, bytes]] = None
    ) -> "HTTPResponse":
        headers = headers or {}
        headers[b"content-type"] = b"application/json;charset=utf-8"
        return cls.build(
            status_code=status_code, headers=headers, data=json.dumps(data).encode("utf-8")
        )

    @classmethod
    def build(
        cls,
        status_code: int,
        headers: Optional[Dict[bytes, bytes]] = None,
        data: Optional[bytes] = None,
    ) -> "HTTPResponse":
        headers = headers or {}
        return cls(status_code=status_code, headers=headers, data=data)


class HTTPComponent:
    def __init__(self, config: BackendConfig):
        self._config = config
        self._organization_component: BaseOrganizationComponent
        self.route_mapping: List[Tuple[Pattern[str], Set[str], RequestHandler]] = [
            (re.compile(r"^/?$"), {"GET"}, self._http_root),
            (re.compile(r"^/redirect/(?P<path>.*)$"), {"GET"}, self._http_redirect),
            (re.compile(r"^/static/(?P<path>.*)$"), {"GET"}, self._http_static),
            (
                re.compile(r"^/administration/organizations$"),
                {"POST"},
                self._http_api_organization_create,
            ),
            (
                re.compile(r"^/administration/organizations/(?P<organization_id>[^/]*)$"),
                {"GET", "PATCH"},
                self._http_api_organization_config,
            ),
            (
                re.compile(r"^/administration/organizations/(?P<organization_id>[^/]*)/stats$"),
                {"GET", "PATCH"},
                self._http_api_organization_stats,
            ),
        ]

    def register_components(self, **other_components):
        self._organization_component = other_components["organization"]

    async def _http_404(self, req: HTTPRequest, **kwargs: str) -> HTTPResponse:
        data = get_template("404.html").render()
        return HTTPResponse.build_html(404, data=data)

    async def _http_root(self, req: HTTPRequest, **kwargs: str) -> HTTPResponse:
        data = get_template("index.html").render()
        return HTTPResponse.build_html(200, data=data)

    async def _http_redirect(self, req: HTTPRequest, **kwargs: str) -> HTTPResponse:
        path = kwargs["path"]
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

        return HTTPResponse.build(302, headers={b"location": location_url.encode("ascii")})

    async def _http_static(self, req: HTTPRequest, **kwargs: str) -> HTTPResponse:
        path = kwargs["path"]
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
            headers[b"content-Type"] = content_type.encode("ascii")
        return HTTPResponse.build(200, headers=headers, data=data)

    async def handle_request(self, req: HTTPRequest) -> HTTPResponse:
        # Loop over patterns
        for pattern, allowed_methods, handler in self.route_mapping:
            # Match against the path
            match = pattern.match(req.path)
            # Run the request handler
            if match:
                if req.method not in allowed_methods:
                    return HTTPResponse.build(405)
                route_args = match.groupdict()
                break
        else:
            handler = self._http_404
            route_args = {}

        return await handler(req, **route_args)

    # Administration API

    async def _http_api_organization_create(self, req: HTTPRequest, **kwargs: str) -> HTTPResponse:
        error_rep, data = await self._api_check_auth_and_load_body(
            req, organization_create_req_serializer
        )
        if error_rep:
            return error_rep

        organization_id = data.pop("organization_id")
        bootstrap_token = generate_bootstrap_token()
        try:
            await self._organization_component.create(
                id=organization_id, bootstrap_token=bootstrap_token, **data
            )
        except OrganizationAlreadyExistsError:
            return HTTPResponse.build_rest(400, {"error": "already_exists"})

        return HTTPResponse.build_rest(
            200, organization_create_rep_serializer.dump({"bootstrap_token": bootstrap_token})
        )

    async def _http_api_organization_config(self, req: HTTPRequest, **kwargs: str) -> HTTPResponse:
        try:
            organization_id = OrganizationID(kwargs["organization_id"])
        except ValueError:
            return HTTPResponse.build_rest(400, {"error": "bad_data"})

        if req.method == "GET":
            error_rep, _ = await self._api_check_auth_and_load_body(
                req, organization_config_req_serializer
            )
            if error_rep:
                return error_rep

            try:
                organization = await self._organization_component.get(id=organization_id)
            except OrganizationNotFoundError:
                return HTTPResponse.build_rest(404, {"error": "not_found"})

            return HTTPResponse.build_rest(
                200,
                organization_config_rep_serializer.dump(
                    {
                        "is_bootstrapped": organization.is_bootstrapped(),
                        "is_expired": organization.is_expired,
                        "user_profile_outsider_allowed": organization.user_profile_outsider_allowed,
                        "active_users_limit": organization.active_users_limit,
                    }
                ),
            )

        else:
            assert req.method == "PATCH"
            error_rep, data = await self._api_check_auth_and_load_body(
                req, organization_update_req_serializer
            )
            if error_rep:
                return error_rep

            try:
                await self._organization_component.update(id=organization_id, **data)
            except OrganizationNotFoundError:
                return HTTPResponse.build_rest(404, {"error": "not_found"})

            return HTTPResponse.build_rest(200, organization_update_rep_serializer.dump({}))

    async def _http_api_organization_stats(self, req: HTTPRequest, **kwargs: str) -> HTTPResponse:
        try:
            organization_id = OrganizationID(kwargs["organization_id"])
        except ValueError:
            return HTTPResponse.build_rest(400, {"error": "bad_data"})

        error_rep, _ = await self._api_check_auth_and_load_body(
            req, organization_stats_req_serializer
        )
        if error_rep:
            return error_rep

        try:
            stats = await self._organization_component.stats(id=organization_id)
        except OrganizationNotFoundError:
            return HTTPResponse.build_rest(404, {"error": "not_found"})

        return HTTPResponse.build_rest(
            200,
            organization_stats_rep_serializer.dump(
                {
                    "status": "ok",
                    "realms": stats.realms,
                    "data_size": stats.data_size,
                    "metadata_size": stats.metadata_size,
                    "users": stats.users,
                    "active_users": stats.active_users,
                    "users_per_profile_detail": stats.users_per_profile_detail,
                }
            ),
        )

    async def _api_check_auth_and_load_body(self, req: HTTPRequest, req_serializer):
        # Check authentication
        authorization = req.headers.get(b"authorization", b"")
        if authorization != f"Bearer {self._config.administration_token}".encode("ascii"):
            return HTTPResponse.build_rest(403, {"error": "not_allowed"}), None

        # Deserialize body
        if req.method == "GET":
            return None, None

        body = await req.get_body()
        try:
            data = req_serializer.loads(body)
        except SerdeValidationError as exc:
            return HTTPResponse.build_rest(400, {"error": "bad_data", "reason": exc.errors}), None
        except SerdePackingError:
            breakpoint()

            return (
                HTTPResponse.build_rest(400, {"error": "bad_data", "reason": "Invalid JSON"}),
                None,
            )

        return None, data
