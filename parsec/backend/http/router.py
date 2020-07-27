# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
import mimetypes
from urllib.parse import parse_qs, urlsplit, urlunsplit, urlencode
from wsgiref.handlers import format_date_time
from importlib import import_module
import importlib_resources
import jinja2

from parsec.backend.http.package_loader import PackageLoader
from parsec.core.types.backend_address import BackendAddr


_JINJA2_ENV = jinja2.Environment(loader=PackageLoader("parsec.backend.http.templates"))


def _get_method(target: str):
    """ Same as is_route but get 404 method if no other route found"""
    method = _is_route(target)
    if not method:
        method = get_404_method()
    return method


def _cook_target(target: bytes) -> str:
    target = target.decode("utf-8")
    return target


def get_method_and_execute(target: bytes, backend_addr: BackendAddr):
    """Entry point"""
    target = _cook_target(target)
    method = _get_method(target)
    return method(target=target, backend_addr=backend_addr)


def get_404_method():
    """Return the 404 method"""
    return _http_404


def _http_static(target: str, *arg, **kwarg):
    """Return static resources"""
    target_split = target.split("/")
    path = "parsec.backend.http"
    # get all path except file name
    index = 0
    while index < len(target_split) - 1:
        # prevent adding a dot when empty string
        if len(target_split[index]) > 0:
            path = path + "." + target_split[index]
        index = index + 1
    file_name = target_split[len(target_split) - 1]
    try:
        package = import_module(".static", "parsec.backend.http")
        if not importlib_resources.is_resource(package, file_name):
            return 404, {}, b""
        data = importlib_resources.read_binary(package, file_name)

        content_type, _ = mimetypes.guess_type(file_name)
        if content_type:
            return 200, _set_headers(data=data, content_type=content_type), data
        else:
            return 200, _set_headers(data=data), data
    except (ValueError, UnboundLocalError, TypeError, ModuleNotFoundError):
        return 404, {}, b""


def _http_404(*arg, **kwarg):
    """Return the 404 view"""
    status_code = 404
    data = _JINJA2_ENV.get_template("404.html").render()
    return status_code, _set_headers(data=data), data.encode("utf-8")


def _is_route(target: str):
    """Return the corresponding method if the url match a mapping route.
    if no route found, return None
    """
    match = None
    for regex, method in _MAPPING.items():
        match = re.match(regex, target)
        if match:
            return method
    return None


def _redirect_to_parsec(target: str, backend_addr, *arg, **kwarg):
    """Redirect the http invite request to a parsec url request"""
    if not backend_addr:
        return 501, {}, b"Url redirection is not available"
    backend_addr_split = urlsplit(backend_addr.to_url())
    target_split = urlsplit(target)
    # Merge params with backend priority in case no_ssl param is provided in both parts.
    query_params = parse_qs(target_split.query)
    # `no_ssl` param depends of backend_addr, hence it cannot be overwritten !
    query_params.pop("no_ssl", None)
    query_params.update(parse_qs(backend_addr_split.query))
    query = urlencode(query=query_params, doseq=True)
    path = target_split.path
    if path.startswith("/api/redirect"):
        path = path[len("/api/redirect") :]
    location_url = urlunsplit(
        (backend_addr_split.scheme, backend_addr_split.netloc, path, query, None)
    )
    headers = [("location", location_url)]
    return 302, _set_headers(headers=headers), b""


def _set_headers(data=b"", headers=(), content_type="text/html;charset=utf-8"):
    """Return headers after adding date, server infos and Content-Length"""
    return [
        *headers,
        ("Date", format_date_time(None).encode("ascii")),
        ("Server", "parsec"),
        # TODO use config.debug
        # ("Server", f"parsec/{parsec_version} {h11.PRODUCT_ID}"),
        ("Content-Length", str(len(data))),
        ("Content-type", content_type),
    ]


_MAPPING = {r"^/api/redirect(.*)$": _redirect_to_parsec, r"^/static/(.*)$": _http_static}
