# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os
import trio
import h11
from structlog import get_logger
from base64 import b64encode
from urllib.request import getproxies, proxy_bypass
from urllib.parse import urlsplit, SplitResult
from typing import Optional, List, Tuple
import pypac

from parsec.api.transport import USER_AGENT
from parsec.core.backend_connection.exceptions import BackendNotAvailable


logger = get_logger()


def cook_basic_auth_header(username: str, password: str) -> str:
    step1 = f"{username}:{password}".encode("utf8")
    step2 = b64encode(step1).decode("ascii")
    return f"Basic {step2}"


def _build_http_url(hostname: str, port: int, use_ssl: bool) -> str:
    if use_ssl:
        url = f"https://{hostname}"
        if port != 443:
            url += f":{port}"
    else:
        url = f"http://{hostname}"
        if port != 80:
            url += f":{port}"
    return url


def _get_proxy_from_pac(url: str, hostname: str) -> Optional[str]:
    """
    Returns:
        proxy url to use
        "" if no proxy should be used
        `None` if no config was found
    """
    # Hack to disable check on content-type (given server might not be well
    # configured, and `get_pac` silently fails on wrong content-type by returning None)
    # Also disable WPAD protocol to retrieve PAC from DNS given it produce annoying
    # WARNING logs
    get_pac_kwargs: dict = {"from_dns": False, "allowed_content_types": {""}}

    force_proxy_pac_url = os.environ.get("http_proxy_pac") or os.environ.get("HTTP_PROXY_PAC")
    if force_proxy_pac_url:
        get_pac_kwargs["url"] = force_proxy_pac_url
        logger.debug(
            "Retrieving .PAC proxy config url from `HTTP_PROXY_PAC` env var",
            pac_url=force_proxy_pac_url,
        )
    else:
        logger.debug("Retrieving .PAC proxy config url from system")

    try:
        pacfile = pypac.get_pac(**get_pac_kwargs)
    except Exception as exc:
        logger.warning("Error while retrieving .PAC proxy config", exc_info=exc)
        return None

    if not pacfile:
        return None

    try:
        proxies = pacfile.find_proxy_for_url(url, hostname)
        logger.debug("Found proxies info in .PAC proxy config", target_url=url, proxies=proxies)
        proxies = [p.strip() for p in proxies.split(";")]
        if len(proxies) > 1:
            logger.warning(
                "Found multiple proxies info in .PAC proxy config for target, only the first one is going to be used !",
                target_url=url,
                proxies=proxies,
            )
        # We don't handle multiple proxies so keep the first correct one and pray !
        for proxy in proxies:
            if proxy in ("DIRECT", ""):
                # PAC explicitly told us not to use a proxy
                return ""
            elif proxy:
                # Should be of style `PROXY 8.8.8.8:9999`
                proxy_type, proxy_netloc = proxy.strip().split()
                proxy_type = proxy_type.upper()
                if proxy_type in ("PROXY", "HTTP"):
                    return f"http://{proxy_netloc}"
                elif proxy_type == "HTTPS":
                    return f"https://{proxy_netloc}"
                else:
                    logger.warning(
                        "Unsupported proxy type requested by .PAC proxy config",
                        proxy=proxy,
                        target_url=url,
                    )

        else:
            return None

    except Exception as exc:
        logger.warning("Error while using .PAC proxy config", target_url=url, exc_info=exc)
        return None


def _get_proxy_from_environ_or_os_config(url: str, hostname: str) -> Optional[str]:
    """
    Returns:
        proxy url to use
        "" if no proxy should be used
        `None` if no config was found
    """
    # Retrieve proxies from environ vars then (depending on platform) os configuration
    if proxy_bypass(hostname):
        return ""  # no proxy should be used
    proxies = getproxies()
    proxy_type = "https" if url.startswith("https://") else "http"
    proxy_url = proxies.get(proxy_type)
    return proxy_url


def blocking_io_get_proxy(target_url: str, hostname: str) -> Optional[str]:
    """
    Returns: proxy url to use or `None`

    Unlike `_get_proxy_from_*`, "" value is never returned (`None` is used
    instead to represent the fact no proxy should be used)
    """
    # Proxy config is fetched from the following locations (in order):
    # - HTTP_PROXY_PAC environ variable
    # - HTTP_PROXY/HTTPS_PROXY/NO_PROXY environ variables
    # - Proxy PAC system configuration
    # - Proxy system configuration
    #
    # HTTP_PROXY_PAC is not a standard environ variable (but there anything as standard
    # in the http proxy crazy world ?), but it's a convenient way to test force
    # proxy pac config when debug is needed.

    # First, try to get proxy from the infamous PAC config system
    proxy_url_from_pac = _get_proxy_from_pac(target_url, hostname)
    if proxy_url_from_pac == "":
        logger.debug("Got .PAC config explicitly specifying direct access", target_url=target_url)
        return None
    elif proxy_url_from_pac is not None:
        logger.debug(
            "Got proxy from .PAC config",
            proxy_url=proxy_url_from_pac,
            target_url=target_url,
        )
        return proxy_url_from_pac

    # Fallback on direct proxy url config in environ variables & Windows registers table
    assert proxy_url_from_pac is None
    proxy_url_from_env = _get_proxy_from_environ_or_os_config(target_url, hostname)
    if proxy_url_from_env == "":
        logger.debug(
            "Got OS env config explicitly specifying direct access",
            target_url=target_url,
        )
        return None
    elif proxy_url_from_env is not None:
        logger.debug(
            "Got proxy from OS env config",
            proxy_url=proxy_url_from_env,
            target_url=target_url,
        )
        return proxy_url_from_env

    assert proxy_url_from_env is None
    logger.debug(
        "No proxy configuration found for target URL, using direct access",
        target_url=target_url,
    )
    return None


async def maybe_connect_through_proxy(
    hostname: str, port: int, use_ssl: bool
) -> Optional[trio.abc.Stream]:
    target_url = _build_http_url(hostname, port, use_ssl)

    proxy_url = await trio.to_thread.run_sync(blocking_io_get_proxy, target_url, hostname)
    if not proxy_url:
        return None

    logger.debug("Using proxy to connect", proxy_url=proxy_url, target_url=target_url)

    # A proxy has been retrieved, parse it url and handle potential auth

    try:
        proxy = urlsplit(proxy_url)
        # Typing helper, as result could be SplitResultBytes if we have provided bytes instead of str
        assert isinstance(proxy, SplitResult)
    except ValueError as exc:
        logger.warning(
            "Invalid proxy url, switching to no proxy",
            proxy_url=proxy_url,
            target_url=target_url,
            exc_info=exc,
        )
        # Invalid url
        return None
    if proxy.port:
        proxy_port = proxy.port
    else:
        proxy_port = 443 if proxy.scheme == "https" else 80
    if not proxy.hostname:
        logger.warning(
            "Missing required hostname in proxy url, switching to no proxy",
            proxy_url=proxy_url,
            target_url=target_url,
        )
        return None

    proxy_headers: List[Tuple[str, str]] = []
    if proxy.username is not None and proxy.password is not None:
        logger.debug("Using `Proxy-Authorization` header with proxy", username=proxy.username)
        proxy_headers.append(
            (
                "Proxy-Authorization",
                cook_basic_auth_header(proxy.username, proxy.password),
            )
        )

    # Connect to the proxy

    stream: trio.abc.Stream
    try:
        stream = await trio.open_tcp_stream(proxy.hostname, proxy_port)

    except OSError as exc:
        logger.warning(
            "Impossible to connect to proxy",
            proxy_hostname=proxy.hostname,
            proxy_port=proxy_port,
            target_url=target_url,
            exc_info=exc,
        )
        raise BackendNotAvailable(exc) from exc

    if proxy.scheme == "https":
        logger.debug("Using TLS to secure proxy connection", hostname=proxy.hostname)
        from parsec.core.backend_connection.transport import _upgrade_stream_to_ssl

        stream = _upgrade_stream_to_ssl(stream, proxy.hostname)

    # Ask the proxy to connect the actual host

    conn = h11.Connection(our_role=h11.CLIENT)

    async def send(event):
        data = conn.send(event)
        await stream.send_all(data)

    async def next_event():
        while True:
            event = conn.next_event()
            if event is h11.NEED_DATA:
                # Note there is no need to handle 100 continue here given we are client
                # (see https://h11.readthedocs.io/en/v0.10.0/api.htm1l#flow-control)
                data = await stream.receive_some(2048)
                conn.receive_data(data)
                continue
            return event

    host = f"{hostname}:{port}"
    try:
        req = h11.Request(
            method="CONNECT",
            target=host,
            headers=[
                # According to RFC7230 (https://datatracker.ietf.org/doc/html/rfc7230#section-5.4)
                # Client must provide Host header, but the proxy must replace it
                # with the host information of the request-target. So in theory
                # we could set any dummy value for the Host header here !
                ("Host", host),
                # User-Agent is not a mandatory header, however http proxy are
                # half-broken shenanigans and get suspicious if it is missing
                ("User-Agent", USER_AGENT),
                *proxy_headers,
            ],
        )
        logger.debug("Sending CONNECT to proxy", proxy_url=proxy_url, req=req)
        await send(req)
        answer = await next_event()
        logger.debug("Receiving CONNECT answer from proxy", proxy_url=proxy_url, answer=answer)
        if not isinstance(answer, h11.Response) or not 200 <= answer.status_code < 300:
            logger.warning(
                "Bad answer from proxy to CONNECT request",
                proxy_url=proxy_url,
                target_host=host,
                target_url=target_url,
                answer_status=answer.status_code,
            )
            raise BackendNotAvailable("Bad answer from proxy")
        # Successful CONNECT should reset the connection's statemachine
        answer = await next_event()
        while not isinstance(answer, h11.PAUSED):
            logger.debug(
                "Receiving additional answer to our CONNECT from proxy",
                proxy_url=proxy_url,
                answer=answer,
            )
            answer = await next_event()

    except trio.BrokenResourceError as exc:
        logger.warning(
            "Proxy has unexpectedly closed the connection",
            proxy_url=proxy_url,
            target_host=host,
            target_url=target_url,
            exc_info=exc,
        )
        raise BackendNotAvailable("Proxy has unexpectedly closed the connection") from exc

    return stream
