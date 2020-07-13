# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
import mimetypes
from urllib.parse import urlparse
from parsec._version import __version__ as parsec_version
from wsgiref.handlers import format_date_time
from parsec.core.types import BackendAddr
from importlib import import_module
import importlib_resources

from parsec.backend.http import PackageLoader

import jinja2
import h11

_JINJA2_ENV = jinja2.Environment(loader=PackageLoader("parsec.backend.http.templates"))


def _create_parsec_url(backend_addr):
    """Return parsec url from backend_addr if exist, otherwise create one"""
    if not backend_addr:
        backend_addr = BackendAddr(hostname="localhost", port=6888).to_url()
    else:
        backend_addr = backend_addr

    return backend_addr.to_url()


def get_method(url: bytes):
    """ Same as is_route but get 404 method if no other route found"""
    method = is_route(url)
    if not method:
        method = get_404_method()
    return method


def get_method_and_execute(url: bytes, backend_addr):
    method = get_method(url)
    return method(url, backend_addr=backend_addr)


def get_404_method():
    """Return the 404 method"""
    return _http_404


def _http_static(url: bytes, *arg, **kwarg):
    """Return static resources"""
    url = url.decode("utf-8")
    url_split = url.split("/")
    path = "parsec.backend.http"
    # get all path except file name
    index = 0
    while index < len(url_split) - 1:
        # prevent adding a dot when empty string
        if len(url_split[index]) > 0:
            path = path + "." + url_split[index]
        index = index + 1
    file_name = url_split[len(url_split) - 1]
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
    except (ValueError, RuntimeError, TypeError, NameError, Exception):
        return 404, {}, b""


def _http_404(*arg, **kwarg):
    """Return the 404 view"""
    status_code = 404
    data = _JINJA2_ENV.get_template("404.html").render()
    return status_code, _set_headers(data=data), data.encode("utf-8")


def is_route(url: bytes):
    """Return the corresponding method if the url match a mapping route.
    if no route found, return None
    """
    match = None
    for regex, method in _MAPPING.items():
        match = re.match(regex, url)
        if match:
            return method
    return None


def _redirect_to_parsec(url: bytes, backend_addr, *arg, **kwarg):
    """Redirect the http invite request to a parsec url request"""
    location = _create_parsec_url(backend_addr=backend_addr) + "/"
    parsed_url = urlparse(url)
    query_string = parsed_url.query.decode("utf-8")
    location = location + query_string
    headers = [("location", location)]
    return 302, _set_headers(headers=headers), b""


def _set_headers(data=b"", headers=(), content_type="text/html;charset=utf-8"):
    """Return headers after adding date, server infos and Content-Length"""
    return [
        *headers,
        ("Date", format_date_time(None).encode("ascii")),
        ("Server", f"parsec/{parsec_version} {h11.PRODUCT_ID}"),
        ("Content-Length", str(len(data))),
        ("Content-type", content_type),
    ]


_MAPPING = {rb"^/api/redirect(.*)$": _redirect_to_parsec, rb"^/static/(.*)$": _http_static}
