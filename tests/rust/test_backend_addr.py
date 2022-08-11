# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from collections import namedtuple
from uuid import uuid4
from parsec.api.protocol.invite import InvitationType

from parsec.crypto import SigningKey
from parsec.api.protocol import OrganizationID


def _check_equal_backend_addrs(rs, py):
    assert rs.to_url() == py.to_url()
    assert rs.hostname == py.hostname
    assert rs.port == py.port
    assert rs.netloc == py.netloc
    assert rs.use_ssl == py.use_ssl
    assert str(rs) == str(py)
    assert hash(rs) == hash(py)
    assert repr(rs) == repr(py)
    assert rs.to_http_domain_url() == py.to_http_domain_url().lower() + "/"


InitParams = namedtuple("InitParams", ["host", "port", "use_ssl"])


@pytest.mark.rust
@pytest.mark.parametrize(
    "params",
    [
        InitParams("parsec.cloud", None, True),
        InitParams("parsec.cloud", 1337, True),
        InitParams("parsec.cloud", None, False),
        InitParams("pArSeC.cLoUd", None, True),
        InitParams("127.0.0.1", 1337, True),
        InitParams("[::1]", 1337, True),
        InitParams("a", None, True),
    ],
)
def test_backend_addr_init(params):
    from parsec.core.types.backend_address import _PyBackendAddr, _RsBackendAddr, BackendAddr

    assert BackendAddr is _RsBackendAddr

    if not params.host:
        with pytest.raises(ValueError) as excinfo:
            _PyBackendAddr(params.host, params.port, params.use_ssl)
        assert str(excinfo.value) == "Hostname cannot be empty"
        with pytest.raises(ValueError) as excinfo:
            BackendAddr(params.host, params.port, params.use_ssl)
        assert str(excinfo.value) == "Hostname cannot be empty"

    py_ba = _PyBackendAddr(params.host, params.port, params.use_ssl)
    rs_ba = BackendAddr(params.host, params.port, params.use_ssl)
    _check_equal_backend_addrs(rs_ba, py_ba)

    py_ba = _PyBackendAddr(params.host, params.port, params.use_ssl)
    rs_ba = BackendAddr(params.host, params.port, params.use_ssl)

    _check_equal_backend_addrs(rs_ba, py_ba)

    py_ba = _PyBackendAddr(params.host, params.port, params.use_ssl)
    rs_ba = BackendAddr(params.host, params.port, params.use_ssl)
    _check_equal_backend_addrs(rs_ba, py_ba)


@pytest.mark.xfail(reason="Different behaviors between Rust and Python")
@pytest.mark.rust
def test_backend_addr_init_empty_host():
    from parsec.core.types.backend_address import _PyBackendAddr, _RsBackendAddr, BackendAddr

    assert BackendAddr is _RsBackendAddr

    py_ba = _PyBackendAddr("")
    rs_ba = BackendAddr("")
    _check_equal_backend_addrs(rs_ba, py_ba)


@pytest.mark.rust
def test_backend_addr_compare():
    from parsec.core.types.backend_address import BackendAddr, _RsBackendAddr

    assert BackendAddr is _RsBackendAddr

    assert BackendAddr.from_url("parsec://parsec.cloud") == BackendAddr.from_url(
        "parsec://parsec.cloud"
    )
    assert BackendAddr.from_url("parsec://parsec.cloud:1337") == BackendAddr.from_url(
        "parsec://parsec.cloud:1337"
    )
    assert BackendAddr.from_url("parsec://parse.cloud") != BackendAddr.from_url(
        "parsec://parsec.cloud"
    )
    assert BackendAddr.from_url("parsec://parsec.cloud:1338") != BackendAddr.from_url(
        "parsec://parsec.cloud:1337"
    )


@pytest.mark.rust
@pytest.mark.parametrize(
    "url",
    [
        "",
        "http://parsec.cloud",
        "https://parsec.cloud",
        "parsec://:1337",
        "parsec://parsec.cloud",
        "parsec://parsec.cloud?no_ssl=true",
        "parsec://parsec.cloud?NO_SSL=true",
        "parsec://parsec.cloud?no_ssl=TRUE",
        "parsec://parsec.cloud:1337",
        "parsec://127.0.0.1:1337",
        "parsec://localhost",
        "PaRsEc://parsec.cloud",
        "parsec://parsec.cloud:1337?no_ssl=true",
        "parsec://a",
        "parsec://a.b/ContainsAPath",
        "http://a.b/redirect",
        "    parsec://parsec.cloud   ",
    ],
)
def test_backend_addr_from_url(url):
    from parsec.core.types.backend_address import _PyBackendAddr, _RsBackendAddr, BackendAddr

    assert BackendAddr is _RsBackendAddr

    if "ContainsAPath" in url:
        with pytest.raises(ValueError) as excinfo:
            _PyBackendAddr.from_url(url)
        assert str(excinfo.value) == "Cannot have path"
        with pytest.raises(ValueError) as excinfo:
            BackendAddr.from_url(url)
        assert str(excinfo.value) == "Cannot have path"

    elif url and not url.strip().lower().startswith("parsec://"):
        with pytest.raises(ValueError) as excinfo:
            _PyBackendAddr.from_url(url)
        assert str(excinfo.value) == "Must start with `parsec://`"
        with pytest.raises(ValueError) as excinfo:
            BackendAddr.from_url(url)
        assert str(excinfo.value) == "Must start with `parsec://`"

    elif not url or url == "parsec://:1337":
        with pytest.raises(ValueError) as excinfo:
            _PyBackendAddr.from_url(url)
        assert str(excinfo.value) in ["Missing mandatory hostname", "Must start with `parsec://`"]
        with pytest.raises(ValueError) as excinfo:
            BackendAddr.from_url(url)
        assert str(excinfo.value) == "Invalid URL"

    elif "/redirect" in url:
        _PyBackendAddr.from_url(url, allow_http_redirection=True).to_url() == BackendAddr.from_url(
            url, allow_http_redirection=True
        ).to_url()

    else:
        rs_ba = BackendAddr.from_url(url)
        py_ba = _PyBackendAddr.from_url(url)
        _check_equal_backend_addrs(rs_ba, py_ba)


@pytest.mark.rust
def test_backend_addr_from_url_invalid():
    from parsec.core.types.backend_address import _PyBackendAddr, _RsBackendAddr, BackendAddr

    assert BackendAddr is _RsBackendAddr

    with pytest.raises(ValueError):
        _PyBackendAddr.from_url("parsec://foo:42?dummy")

    BackendAddr.from_url("parsec://foo:42?dummy")


@pytest.mark.xfail(reason="Different behaviors between Rust and Python")
@pytest.mark.rust
def test_backend_addr_ipv6():
    from parsec.core.types.backend_address import _PyBackendAddr, _RsBackendAddr, BackendAddr

    assert BackendAddr is _RsBackendAddr

    rs_ba = BackendAddr.from_url("parsec://[0000:0000:0000:0000:0000:0000:0000:0001]:1337")
    py_ba = _PyBackendAddr.from_url("parsec://[0000:0000:0000:0000:0000:0000:0000:0001]:1337")
    _check_equal_backend_addrs(rs_ba, py_ba)

    rs_ba = BackendAddr.from_url("parsec://[::1]:1337")
    py_ba = _PyBackendAddr.from_url("parsec://[::1]:1337")
    _check_equal_backend_addrs(rs_ba, py_ba)

    rs_ba = BackendAddr("[0000:0000:0000:0000:0000:0000:0000:0001]", 1337, True)
    py_ba = _PyBackendAddr("[0000:0000:0000:0000:0000:0000:0000:0001]", 1337, True)
    _check_equal_backend_addrs(rs_ba, py_ba)


@pytest.mark.xfail(reason="Different behaviors between Rust and Python")
@pytest.mark.rust
def test_backend_addr_from_url_case_sensitive():
    from parsec.core.types.backend_address import _PyBackendAddr, _RsBackendAddr, BackendAddr

    assert BackendAddr is _RsBackendAddr

    rs_ba = BackendAddr.from_url("parsec://PaRsEc.ClOuD:1337")
    py_ba = _PyBackendAddr.from_url("parsec://PaRsEc.ClOuD:1337")
    _check_equal_backend_addrs(rs_ba, py_ba)


def _check_equal_backend_organization_addrs(rs, py):
    assert rs.to_url() == py.to_url()
    assert rs.hostname == py.hostname
    assert rs.port == py.port
    assert rs.netloc == py.netloc
    assert rs.use_ssl == py.use_ssl
    assert str(rs.organization_id) == str(py.organization_id)
    assert rs.root_verify_key == py.root_verify_key
    assert str(rs) == str(py)
    assert hash(rs) == hash(py)
    assert repr(rs) == repr(py)


@pytest.mark.rust
def test_backend_organization_addr_init():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationAddr,
        _RsBackendOrganizationAddr,
        BackendOrganizationAddr,
    )

    assert BackendOrganizationAddr is _RsBackendOrganizationAddr

    vk = SigningKey.generate().verify_key

    py_ba = _PyBackendOrganizationAddr(OrganizationID("MyOrg"), vk, hostname="parsec.cloud")
    rs_ba = BackendOrganizationAddr(OrganizationID("MyOrg"), vk, hostname="parsec.cloud")

    _check_equal_backend_organization_addrs(rs_ba, py_ba)


@pytest.mark.rust
def test_backend_organization_addr_from_url():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationAddr,
        _RsBackendOrganizationAddr,
        BackendOrganizationAddr,
    )

    assert BackendOrganizationAddr is _RsBackendOrganizationAddr

    RVK = "7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss"

    URL = f"parsec://parsec.cloud/MyOrg?rvk={RVK}"

    py_ba = _PyBackendOrganizationAddr.from_url(URL)
    rs_ba = BackendOrganizationAddr.from_url(URL)
    _check_equal_backend_organization_addrs(rs_ba, py_ba)

    # With spaces before and after
    py_ba = _PyBackendOrganizationAddr.from_url(" " + URL + " ")
    rs_ba = BackendOrganizationAddr.from_url(" " + URL + " ")
    _check_equal_backend_organization_addrs(rs_ba, py_ba)

    assert BackendOrganizationAddr.from_url(URL) == BackendOrganizationAddr.from_url(URL)
    # Different host
    assert BackendOrganizationAddr.from_url(
        f"parsec://parse.cloud/MyOrg?rvk={RVK}"
    ) != BackendOrganizationAddr.from_url(URL)
    # Different org
    assert BackendOrganizationAddr.from_url(
        f"parsec://parsec.cloud/MyOrg2?rvk={RVK}"
    ) != BackendOrganizationAddr.from_url(URL)

    URL = f"http://parsec.cloud/redirect/MyOrg?rvk={RVK}"
    _PyBackendOrganizationAddr.from_url(
        URL, allow_http_redirection=True
    ).to_url() == BackendOrganizationAddr.from_url(URL, allow_http_redirection=True).to_url()


@pytest.mark.rust
def test_backend_organization_addr_build():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationAddr,
        _RsBackendOrganizationAddr,
        BackendOrganizationAddr,
        BackendAddr,
    )

    assert BackendOrganizationAddr is _RsBackendOrganizationAddr

    BACKEND_ADDR = BackendAddr.from_url("parsec://parsec.cloud:1337")

    vk = SigningKey.generate().verify_key

    py_ba = _PyBackendOrganizationAddr.build(BACKEND_ADDR, OrganizationID("MyOrg"), vk)
    rs_ba = BackendOrganizationAddr.build(BACKEND_ADDR, OrganizationID("MyOrg"), vk)
    _check_equal_backend_organization_addrs(rs_ba, py_ba)


def _check_equal_backend_organization_bootstrap_addrs(rs, py):
    assert rs.to_url() == py.to_url()
    assert rs.hostname == py.hostname
    assert rs.port == py.port
    assert rs.netloc == py.netloc
    assert rs.use_ssl == py.use_ssl
    assert str(rs.organization_id) == str(py.organization_id)
    assert rs.token == py.token
    assert str(rs) == str(py)
    assert hash(rs) == hash(py)
    assert repr(rs) == repr(py)
    vk = SigningKey.generate().verify_key
    assert (
        rs.generate_organization_addr(root_verify_key=vk).to_url()
        == py.generate_organization_addr(root_verify_key=vk).to_url()
    )


@pytest.mark.rust
def test_backend_organization_bootstrap_addr_init():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationBootstrapAddr,
        _RsBackendOrganizationBootstrapAddr,
        BackendOrganizationBootstrapAddr,
    )

    assert BackendOrganizationBootstrapAddr is _RsBackendOrganizationBootstrapAddr

    token = "1234ABCD"

    py_ba = _PyBackendOrganizationBootstrapAddr(
        OrganizationID("MyOrg"), token=token, hostname="parsec.cloud"
    )
    rs_ba = BackendOrganizationBootstrapAddr(
        OrganizationID("MyOrg"), token=token, hostname="parsec.cloud"
    )
    _check_equal_backend_organization_bootstrap_addrs(rs_ba, py_ba)
    py_ba = _PyBackendOrganizationBootstrapAddr(
        OrganizationID("MyOrg"), token="", hostname="parsec.cloud"
    )
    rs_ba = BackendOrganizationBootstrapAddr(
        OrganizationID("MyOrg"), token="", hostname="parsec.cloud"
    )
    _check_equal_backend_organization_bootstrap_addrs(rs_ba, py_ba)


@pytest.mark.rust
def test_backend_organization_bootstrap_addr_from_url():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationBootstrapAddr,
        _RsBackendOrganizationBootstrapAddr,
        BackendOrganizationBootstrapAddr,
        BackendActionAddr,
        _PyBackendActionAddr,
        _RsBackendActionAddr,
    )

    assert BackendOrganizationBootstrapAddr is _RsBackendOrganizationBootstrapAddr
    assert BackendActionAddr is _RsBackendActionAddr

    URL = "parsec://parsec.cloud/MyOrg?action=bootstrap_organization&token=1234ABCD"

    py_ba = _PyBackendActionAddr.from_url(URL)
    rs_ba = BackendActionAddr.from_url(URL)
    assert isinstance(py_ba, _PyBackendOrganizationBootstrapAddr)
    assert isinstance(rs_ba, BackendOrganizationBootstrapAddr)
    _check_equal_backend_organization_bootstrap_addrs(rs_ba, py_ba)

    # With spaces before and after
    py_ba = _PyBackendActionAddr.from_url(" " + URL + " ")
    rs_ba = BackendActionAddr.from_url(" " + URL + " ")
    _check_equal_backend_organization_bootstrap_addrs(rs_ba, py_ba)

    REDIRECT_URL = "http://parsec.cloud/redirect/MyOrg?action=bootstrap_organization&token=1234ABCD"
    py_ba = _PyBackendActionAddr.from_url(REDIRECT_URL, allow_http_redirection=True)
    rs_ba = BackendActionAddr.from_url(REDIRECT_URL, allow_http_redirection=True)
    assert isinstance(py_ba, _PyBackendOrganizationBootstrapAddr)
    assert isinstance(rs_ba, BackendOrganizationBootstrapAddr)

    with pytest.raises(ValueError) as excinfo:
        BackendOrganizationBootstrapAddr.from_url(
            "parsec://parsec.cloud?action=bootstrap_organization&token=123"
        )
    assert str(excinfo.value) == "Path doesn't form a valid organization id"
    with pytest.raises(ValueError) as excinfo:
        _PyBackendActionAddr.from_url(
            "parsec://parsec.cloud?action=bootstrap_organization&token=123"
        )
    assert str(excinfo.value) == "Invalid URL format"

    assert BackendActionAddr.from_url(URL) == BackendActionAddr.from_url(URL)
    # Different host
    assert BackendActionAddr.from_url(
        "parsec://parse.cloud/MyOrg?action=bootstrap_organization&token=1234ABCD"
    ) != BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg?action=bootstrap_organization&token=1234ABCD"
    )
    # Different org
    assert BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg2?action=bootstrap_organization&token=1234ABCD"
    ) != BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg?action=bootstrap_organization&token=1234ABCD"
    )
    # Different token
    assert BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg?action=bootstrap_organization&token=1234ABCE"
    ) != BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg?action=bootstrap_organization&token=1234ABCD"
    )

    # Missing action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url("parsec://parsec.cloud/MyOrg?token=1234ABCD")
    assert str(excinfo.value) == "Invalid URL format"

    # Invalid action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url(
            "parsec://parsec.cloud/MyOrg?action=do_something&token=1234ABCD"
        )
    assert str(excinfo.value) == "Invalid URL format"

    # Missing token
    rs_ba = BackendActionAddr.from_url("parsec://parsec.cloud/MyOrg?action=bootstrap_organization")
    assert rs_ba.token == ""


@pytest.mark.rust
def test_backend_organization_bootstrap_addr_build():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationBootstrapAddr,
        _RsBackendOrganizationBootstrapAddr,
        BackendOrganizationBootstrapAddr,
        BackendAddr,
    )

    assert BackendOrganizationBootstrapAddr is _RsBackendOrganizationBootstrapAddr

    BACKEND_ADDR = BackendAddr.from_url("parsec://parsec.cloud:1337")

    token = "1234ABCD"

    py_ba = _PyBackendOrganizationBootstrapAddr.build(
        BACKEND_ADDR, OrganizationID("MyOrg"), token=token
    )
    rs_ba = BackendOrganizationBootstrapAddr.build(
        BACKEND_ADDR, OrganizationID("MyOrg"), token=token
    )
    _check_equal_backend_organization_bootstrap_addrs(rs_ba, py_ba)


def _check_equal_backend_organization_file_link_addrs(rs, py):
    assert rs.to_url() == py.to_url()
    assert rs.hostname == py.hostname
    assert rs.port == py.port
    assert rs.netloc == py.netloc
    assert rs.use_ssl == py.use_ssl
    assert str(rs.organization_id) == str(py.organization_id)
    assert str(rs.workspace_id) == str(py.workspace_id)
    assert rs.encrypted_path == py.encrypted_path
    assert str(rs) == str(py)
    assert hash(rs) == hash(py)
    assert repr(rs) == repr(py)


@pytest.mark.rust
def test_backend_organization_file_link_addr_init():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationFileLinkAddr,
        _RsBackendOrganizationFileLinkAddr,
        BackendOrganizationFileLinkAddr,
    )
    from parsec.api.data import EntryID

    assert BackendOrganizationFileLinkAddr is _RsBackendOrganizationFileLinkAddr

    WORKSPACE_ID = uuid4()

    py_ba = _PyBackendOrganizationFileLinkAddr(
        OrganizationID("MyOrg"),
        workspace_id=EntryID(WORKSPACE_ID),
        encrypted_path=b"/",
        hostname="parsec.cloud",
    )
    rs_ba = BackendOrganizationFileLinkAddr(
        OrganizationID("MyOrg"),
        workspace_id=EntryID(WORKSPACE_ID),
        encrypted_path=b"/",
        hostname="parsec.cloud",
    )
    _check_equal_backend_organization_file_link_addrs(rs_ba, py_ba)


@pytest.mark.rust
def test_backend_organization_file_link_addr_from_url():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationFileLinkAddr,
        _RsBackendOrganizationFileLinkAddr,
        BackendOrganizationFileLinkAddr,
        BackendActionAddr,
        _PyBackendActionAddr,
        _RsBackendActionAddr,
        binary_urlsafe_encode,
    )

    assert BackendOrganizationFileLinkAddr is _RsBackendOrganizationFileLinkAddr
    assert BackendActionAddr is _RsBackendActionAddr

    WORKSPACE_ID = str(uuid4()).replace("-", "")
    PATH = binary_urlsafe_encode("%2F".encode())

    URL = f"parsec://parsec.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}"

    py_ba = _PyBackendActionAddr.from_url(URL)
    rs_ba = BackendActionAddr.from_url(URL)
    assert isinstance(py_ba, _PyBackendOrganizationFileLinkAddr)
    assert isinstance(rs_ba, BackendOrganizationFileLinkAddr)
    _check_equal_backend_organization_file_link_addrs(rs_ba, py_ba)

    # With spaces before and after
    py_ba = _PyBackendActionAddr.from_url(" " + URL + " ")
    rs_ba = BackendActionAddr.from_url(" " + URL + " ")
    _check_equal_backend_organization_file_link_addrs(rs_ba, py_ba)

    assert BackendActionAddr.from_url(URL) == BackendActionAddr.from_url(URL)
    # Different host
    assert BackendActionAddr.from_url(
        f"parsec://parse.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}"
    ) != BackendActionAddr.from_url(
        f"parsec://parsec.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}"
    )
    # Different workspace_id
    assert BackendActionAddr.from_url(
        f"parsec://parsec.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}"
    ) != BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}".format(
            WORKSPACE_ID=str(uuid4()).replace("-", ""), PATH=PATH
        )
    )
    # Different path
    assert BackendActionAddr.from_url(
        f"parsec://parsec.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}"
    ) != BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}".format(
            WORKSPACE_ID=WORKSPACE_ID, PATH=binary_urlsafe_encode("%2Fa".encode())
        )
    )

    # Missing action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url(
            f"parsec://parsec.cloud/MyOrg?workspace_id={WORKSPACE_ID}&path={PATH}"
        )
    assert str(excinfo.value) == "Invalid URL format"

    # Invalid action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url(
            f"parsec://parsec.cloud/MyOrg?action=do_something&workspace_id={WORKSPACE_ID}&path={PATH}"
        )
    assert str(excinfo.value) == "Invalid URL format"

    # Missing workspace_id
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url(
            "parsec://parsec.cloud/MyOrg?action=file_link&path={PATH}"
        )
    assert str(excinfo.value) == "Invalid URL format"

    # Missing path
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url(
            f"parsec://parsec.cloud/MyOrg?action=file_link&workspace_id={WORKSPACE_ID}"
        )
    assert str(excinfo.value) == "Invalid URL format"


@pytest.mark.rust
def test_backend_organization_file_link_addr_build():
    from parsec.core.types.backend_address import (
        _PyBackendOrganizationFileLinkAddr,
        _RsBackendOrganizationFileLinkAddr,
        BackendOrganizationFileLinkAddr,
        BackendActionAddr,
        _RsBackendActionAddr,
        binary_urlsafe_encode,
        BackendOrganizationAddr,
    )

    from parsec.api.data import EntryID

    assert BackendOrganizationFileLinkAddr is _RsBackendOrganizationFileLinkAddr
    assert BackendActionAddr is _RsBackendActionAddr

    WORKSPACE_ID = uuid4()
    PATH = binary_urlsafe_encode(b"%2F").encode()
    BACKEND_ADDR = BackendOrganizationAddr.from_url(
        "parsec://parsec.cloud/MyOrg?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss"
    )

    py_ba = _PyBackendOrganizationFileLinkAddr.build(
        BACKEND_ADDR, EntryID(WORKSPACE_ID), encrypted_path=PATH
    )
    rs_ba = BackendOrganizationFileLinkAddr.build(
        BACKEND_ADDR, EntryID(WORKSPACE_ID), encrypted_path=PATH
    )
    _check_equal_backend_organization_file_link_addrs(rs_ba, py_ba)


def _check_equal_backend_invitation_addrs(rs, py):
    assert rs.to_url() == py.to_url()
    assert rs.hostname == py.hostname
    assert rs.port == py.port
    assert rs.netloc == py.netloc
    assert rs.use_ssl == py.use_ssl
    assert str(rs.organization_id) == str(py.organization_id)
    assert str(rs.invitation_type) == str(py.invitation_type)
    assert rs.token == py.token
    assert str(rs) == str(py)
    assert hash(rs) == hash(py)
    assert repr(rs) == repr(py)
    vk = SigningKey.generate().verify_key
    assert (
        rs.generate_organization_addr(root_verify_key=vk).to_url()
        == py.generate_organization_addr(root_verify_key=vk).to_url()
    )
    assert rs.to_http_redirection_url() == py.to_http_redirection_url()


@pytest.mark.rust
def test_backend_invitation_addr_init():
    from parsec.core.types.backend_address import (
        _PyBackendInvitationAddr,
        BackendInvitationAddr,
        _RsBackendInvitationAddr,
    )
    from parsec.api.protocol import InvitationToken

    assert _RsBackendInvitationAddr is BackendInvitationAddr

    TOKEN = InvitationToken(uuid4())

    py_ba = _PyBackendInvitationAddr(
        OrganizationID("MyOrg"),
        invitation_type=InvitationType.USER,
        token=TOKEN,
        hostname="parsec.cloud",
    )
    rs_ba = BackendInvitationAddr(
        OrganizationID("MyOrg"),
        invitation_type=InvitationType.USER,
        token=TOKEN,
        hostname="parsec.cloud",
    )

    _check_equal_backend_invitation_addrs(rs_ba, py_ba)


@pytest.mark.rust
def test_backend_invitation_addr_from_url():
    from parsec.core.types.backend_address import (
        _PyBackendInvitationAddr,
        BackendInvitationAddr,
        _RsBackendInvitationAddr,
        BackendActionAddr,
        _PyBackendActionAddr,
    )

    assert _RsBackendInvitationAddr is BackendInvitationAddr

    TOKEN = uuid4()

    # Claim user
    URL = f"parsec://parsec.cloud/MyOrg?action=claim_user&token={TOKEN}"

    py_ba = _PyBackendInvitationAddr.from_url(URL)
    rs_ba = BackendInvitationAddr.from_url(URL)
    _check_equal_backend_invitation_addrs(rs_ba, py_ba)

    # With spaces before and after
    py_ba = _PyBackendInvitationAddr.from_url(" " + URL + " ")
    rs_ba = BackendInvitationAddr.from_url(" " + URL + " ")
    _check_equal_backend_invitation_addrs(rs_ba, py_ba)

    py_ba = _PyBackendActionAddr.from_url(URL)
    rs_ba = BackendActionAddr.from_url(URL)
    assert isinstance(py_ba, _PyBackendInvitationAddr)
    assert isinstance(rs_ba, BackendInvitationAddr)
    _check_equal_backend_invitation_addrs(rs_ba, py_ba)

    # Claim device
    URL = f"parsec://parsec.cloud/MyOrg?action=claim_device&token={TOKEN}"
    py_ba = _PyBackendInvitationAddr.from_url(URL)
    rs_ba = BackendInvitationAddr.from_url(URL)
    _check_equal_backend_invitation_addrs(rs_ba, py_ba)

    py_ba = _PyBackendActionAddr.from_url(URL)
    rs_ba = BackendActionAddr.from_url(URL)
    assert isinstance(py_ba, _PyBackendInvitationAddr)
    assert isinstance(rs_ba, BackendInvitationAddr)
    _check_equal_backend_invitation_addrs(rs_ba, py_ba)

    assert BackendActionAddr.from_url(URL) == BackendActionAddr.from_url(URL)
    # Different host
    assert BackendActionAddr.from_url(
        f"parsec://par.cloud/MyOrg?action=claim_user&token={TOKEN}"
    ) != BackendActionAddr.from_url(f"parsec://parsec.cloud/MyOrg?action=claim_user&token={TOKEN}")
    # Different action
    assert BackendActionAddr.from_url(
        f"parsec://parsec.cloud/MyOrg?action=claim_device&token={TOKEN}"
    ) != BackendActionAddr.from_url(f"parsec://parsec.cloud/MyOrg?action=claim_user&token={TOKEN}")
    # Different token
    assert BackendActionAddr.from_url(
        f"parsec://parsec.cloud/MyOrg?action=claim_user&token={TOKEN}"
    ) != BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg?action=claim_user&token={TOKEN}".format(TOKEN=uuid4())
    )

    # Missing action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendInvitationAddr.from_url(f"parsec://parsec.cloud/MyOrg?token={TOKEN}")
    assert str(excinfo.value) == "Missing mandatory `action` param"

    # Invalid action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendInvitationAddr.from_url(
            f"parsec://parsec.cloud/MyOrg?action=do_something&token={TOKEN}"
        )
    assert str(excinfo.value) == "Expected `action=claim_user` or `action=claim_device` param value"

    # Missing token
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url("parsec://parsec.cloud/MyOrg?action=claim_user")
    assert str(excinfo.value) == "Invalid URL format"

    # Invalid token
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url(f"parsec://parsec.cloud/MyOrg?action=claim_user&token=a")
    assert str(excinfo.value) == "Invalid URL format"


@pytest.mark.rust
def test_backend_invitation_addr_build():
    from parsec.core.types.backend_address import (
        _PyBackendInvitationAddr,
        BackendInvitationAddr,
        _RsBackendInvitationAddr,
        BackendAddr,
    )
    from parsec.api.protocol import InvitationToken

    assert _RsBackendInvitationAddr is BackendInvitationAddr

    INVITATION_TOKEN = InvitationToken(uuid4())
    BACKEND_ADDR = BackendAddr.from_url("parsec://parsec.cloud/")

    py_ba = _PyBackendInvitationAddr.build(
        BACKEND_ADDR,
        organization_id=OrganizationID("MyOrg"),
        invitation_type=InvitationType.USER,
        token=INVITATION_TOKEN,
    )
    rs_ba = BackendInvitationAddr.build(
        BACKEND_ADDR,
        organization_id=OrganizationID("MyOrg"),
        invitation_type=InvitationType.USER,
        token=INVITATION_TOKEN,
    )
    _check_equal_backend_invitation_addrs(rs_ba, py_ba)


def _check_equal_backend_pki_enrollment_addrs(rs, py):
    assert rs.to_url() == py.to_url()
    assert rs.hostname == py.hostname
    assert rs.port == py.port
    assert rs.netloc == py.netloc
    assert rs.use_ssl == py.use_ssl
    assert rs.to_http_domain_url() == py.to_http_domain_url() + "/"
    assert rs.to_http_domain_url("/path") == py.to_http_domain_url("/path")
    assert str(rs.organization_id) == str(py.organization_id)
    assert str(rs) == str(py)
    assert hash(rs) == hash(py)
    assert repr(rs) == repr(py)
    vk = SigningKey.generate().verify_key
    assert (
        rs.generate_organization_addr(root_verify_key=vk).to_url()
        == py.generate_organization_addr(root_verify_key=vk).to_url()
    )


@pytest.mark.rust
def test_backend_pki_enrollment_addr_init():
    from parsec.core.types.backend_address import (
        _PyBackendPkiEnrollmentAddr,
        _RsBackendPkiEnrollmentAddr,
        BackendPkiEnrollmentAddr,
    )

    assert BackendPkiEnrollmentAddr is _RsBackendPkiEnrollmentAddr

    py_ba = _PyBackendPkiEnrollmentAddr(OrganizationID("MyOrg"), hostname="parsec.cloud")
    rs_ba = BackendPkiEnrollmentAddr(OrganizationID("MyOrg"), hostname="parsec.cloud")
    _check_equal_backend_pki_enrollment_addrs(rs_ba, py_ba)


@pytest.mark.rust
def test_backend_pki_enrollment_addr_from_url():
    from parsec.core.types.backend_address import (
        _PyBackendPkiEnrollmentAddr,
        _RsBackendPkiEnrollmentAddr,
        BackendPkiEnrollmentAddr,
        BackendActionAddr,
        _PyBackendActionAddr,
        _RsBackendActionAddr,
    )

    assert BackendPkiEnrollmentAddr is _RsBackendPkiEnrollmentAddr
    assert BackendActionAddr is _RsBackendActionAddr

    URL = "parsec://parsec.cloud/MyOrg?action=pki_enrollment"

    py_ba = _PyBackendActionAddr.from_url(URL)
    rs_ba = BackendActionAddr.from_url(URL)
    assert isinstance(py_ba, _PyBackendPkiEnrollmentAddr)
    assert isinstance(rs_ba, BackendPkiEnrollmentAddr)
    _check_equal_backend_pki_enrollment_addrs(rs_ba, py_ba)

    # With spaces before and after
    py_ba = _PyBackendActionAddr.from_url(" " + URL + " ")
    rs_ba = BackendActionAddr.from_url(" " + URL + " ")
    _check_equal_backend_pki_enrollment_addrs(rs_ba, py_ba)

    REDIRECT_URL = "http://parsec.cloud/redirect/MyOrg?action=pki_enrollment"
    py_ba = _PyBackendActionAddr.from_url(REDIRECT_URL, allow_http_redirection=True)
    rs_ba = BackendActionAddr.from_url(REDIRECT_URL, allow_http_redirection=True)
    assert isinstance(py_ba, _PyBackendPkiEnrollmentAddr)
    assert isinstance(rs_ba, BackendPkiEnrollmentAddr)

    with pytest.raises(ValueError) as excinfo:
        BackendPkiEnrollmentAddr.from_url("parsec://parsec.cloud?action=pki_enrollment")
    assert str(excinfo.value) == "Path doesn't form a valid organization id"
    with pytest.raises(ValueError) as excinfo:
        _PyBackendPkiEnrollmentAddr.from_url("parsec://parsec.cloud?action=pki_enrollment")
    assert str(excinfo.value) == "Invalid OrganizationID"

    assert BackendActionAddr.from_url(URL) == BackendActionAddr.from_url(URL)
    # Different host
    assert BackendActionAddr.from_url(
        "parsec://parse.cloud/MyOrg?action=pki_enrollment"
    ) != BackendActionAddr.from_url("parsec://parsec.cloud/MyOrg?action=pki_enrollment")
    # Different org
    assert BackendActionAddr.from_url(
        "parsec://parsec.cloud/MyOrg2?action=pki_enrollment"
    ) != BackendActionAddr.from_url("parsec://parsec.cloud/MyOrg?action=pki_enrollment")

    # Missing action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url("parsec://parsec.cloud/MyOrg")
    assert str(excinfo.value) == "Invalid URL format"

    # Invalid action
    with pytest.raises(ValueError) as excinfo:
        rs_ba = BackendActionAddr.from_url("parsec://parsec.cloud/MyOrg?action=do_something")
    assert str(excinfo.value) == "Invalid URL format"


@pytest.mark.rust
def test_backend_pki_enrollment_addr_build():
    from parsec.core.types.backend_address import (
        _PyBackendPkiEnrollmentAddr,
        _RsBackendPkiEnrollmentAddr,
        BackendPkiEnrollmentAddr,
        BackendAddr,
    )

    assert BackendPkiEnrollmentAddr is _RsBackendPkiEnrollmentAddr

    BACKEND_ADDR = BackendAddr.from_url("parsec://parsec.cloud:1337")

    py_ba = _PyBackendPkiEnrollmentAddr.build(BACKEND_ADDR, OrganizationID("MyOrg"))
    rs_ba = BackendPkiEnrollmentAddr.build(BACKEND_ADDR, OrganizationID("MyOrg"))
    _check_equal_backend_pki_enrollment_addrs(rs_ba, py_ba)
