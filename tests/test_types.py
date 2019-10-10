# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocol import DeviceID, UserID, DeviceName, OrganizationID
from parsec.crypto import SigningKey, PrivateKey, SecretKey, export_root_verify_key
from parsec.core.types import BackendAddr, BackendOrganizationAddr, BackendOrganizationBootstrapAddr


@pytest.mark.parametrize("raw", ["foo42", "FOO", "f", "f-o-o", "f_o_o", "x" * 32, "三国"])
def test_organization_id_user_id_and_device_name(raw):
    organization_id = OrganizationID(raw)
    assert organization_id == raw

    user_id = UserID(raw)
    assert user_id == raw

    device_name = DeviceName(raw)
    assert device_name == raw


@pytest.mark.parametrize("raw", ["x" * 33, "F~o", "f o"])
def test_bad_organization_id_user_id_and_device_name(raw):
    with pytest.raises(ValueError):
        OrganizationID(raw)
    with pytest.raises(ValueError):
        UserID(raw)
    with pytest.raises(ValueError):
        DeviceName(raw)


@pytest.mark.parametrize(
    "raw", ["ali-c_e@d-e_v", "ALICE@DEV", "a@x", "a" * 32 + "@" + "b" * 32, "关羽@三国"]
)
def test_device_id(raw):
    user_id, device_name = raw.split("@")
    device_id = DeviceID(raw)
    assert device_id == raw
    assert device_id.user_id == user_id
    assert device_id.device_name == device_name


@pytest.mark.parametrize(
    "raw", ["a", "a" * 33 + "@" + "x" * 32, "a" * 32 + "@" + "x" * 33, "a@@x", "a@1@x"]
)
def test_bad_device_id(raw):
    with pytest.raises(ValueError):
        DeviceID(raw)


@pytest.mark.parametrize(
    "url,expected",
    [
        ("parsec://foo", {"port": 443, "ssl": True}),
        ("parsec://foo?no_ssl=false", {"port": 443, "ssl": True}),
        ("parsec://foo?no_ssl=true", {"port": 80, "ssl": False}),
        ("parsec://foo:42", {"port": 42, "ssl": True}),
        ("parsec://foo:42?no_ssl=true", {"port": 42, "ssl": False}),
        ("parsec://foo:42?no_ssl=false", {"port": 42, "ssl": True}),
    ],
)
def test_backend_addr_good(url, expected):
    addr = BackendAddr(url)
    assert addr.scheme == "parsec"
    assert addr.hostname == "foo"
    assert addr.port == expected["port"]
    assert addr.use_ssl == expected["ssl"]


@pytest.mark.parametrize(
    "url",
    [
        "",
        "foo",
        "xx://foo:42",  # bad scheme
        "parsec://foo:42/dummy",  # path not allowed
        "parsec://foo:42?dummy",  # unknown param
        "parsec://foo:42?dummy=foo",  # unknown param
        "parsec://foo:42?no_ssl=true&dummy=foo",  # unknown param
        "parsec://foo:42?no_ssl",  # missing value for param
        "parsec://foo:42?no_ssl=",  # bad value for param
        "parsec://foo:42?no_ssl=nop",  # bad value for param
    ],
)
def test_backend_addr_bad_value(url):
    with pytest.raises(ValueError):
        BackendAddr(url)


@pytest.fixture(scope="session")
def verify_key():
    return SigningKey.generate().verify_key


@pytest.fixture(scope="session")
def exported_verify_key(verify_key):
    return export_root_verify_key(verify_key)


@pytest.mark.parametrize(
    "base_url,expected",
    [
        ("parsec://foo", {"port": 443, "ssl": True}),
        ("parsec://foo?no_ssl=false", {"port": 443, "ssl": True}),
        ("parsec://foo?no_ssl=true", {"port": 80, "ssl": False}),
        ("parsec://foo:42", {"port": 42, "ssl": True}),
        ("parsec://foo:42?no_ssl=true", {"port": 42, "ssl": False}),
        ("parsec://foo:42?no_ssl=false", {"port": 42, "ssl": True}),
    ],
)
def test_backend_organization_addr_good(base_url, expected, verify_key):
    addr = BackendOrganizationAddr.build(base_url, "org", verify_key)
    assert addr.scheme == "parsec"
    assert addr.hostname == "foo"
    assert addr.port == expected["port"]
    assert addr.use_ssl == expected["ssl"]
    assert addr.organization_id == "org"
    assert addr.root_verify_key == verify_key

    addr2 = BackendOrganizationAddr(str(addr))
    assert addr == addr2


@pytest.mark.parametrize(
    "url",
    [
        "",
        "foo",
        "xx://foo:42/org?rvk=<rvk>",  # bad scheme
        "parsec://foo:42/org?rvk=<rvk>&dummy",  # unknown param
        "parsec://foo:42/org?rvk=<rvk>&dummy=foo",  # unknown param
        "parsec://foo:42/org",  # missing mandatory rvk param
        "parsec://foo:42/org?rvk=",  # missing value for param
        "parsec://foo:42/org?rvk=nop",  # bad value for param
        "parsec://foo:42?rvk=<rvk>",  # missing org name
        "parsec://foo:42/?rvk=<rvk>",  # missing org name
        "parsec://foo:42/bad/org?rvk=<rvk>",  # bad org name
        "parsec://foo:42/~org?rvk=<rvk>",  # bad org name
    ],
)
def test_backend_organization_addr_bad_value(url, exported_verify_key):
    url = url.replace("<rvk>", exported_verify_key)
    with pytest.raises(ValueError):
        BackendOrganizationAddr(url)


@pytest.mark.parametrize(
    "base_url,expected",
    [
        ("parsec://foo", {"port": 443, "ssl": True}),
        ("parsec://foo?no_ssl=false", {"port": 443, "ssl": True}),
        ("parsec://foo?no_ssl=true", {"port": 80, "ssl": False}),
        ("parsec://foo:42", {"port": 42, "ssl": True}),
        ("parsec://foo:42?no_ssl=true", {"port": 42, "ssl": False}),
        ("parsec://foo:42?no_ssl=false", {"port": 42, "ssl": True}),
    ],
)
def test_backend_organization_bootstrap_addr_good(base_url, expected, verify_key):
    addr = BackendOrganizationBootstrapAddr.build(base_url, "org", "token-123")
    assert addr.scheme == "parsec"
    assert addr.hostname == "foo"
    assert addr.port == expected["port"]
    assert addr.use_ssl == expected["ssl"]
    assert addr.organization_id == "org"
    assert addr.bootstrap_token == "token-123"

    addr2 = BackendOrganizationBootstrapAddr(str(addr))
    assert addr == addr2

    org_addr = addr.generate_organization_addr(verify_key)
    assert isinstance(org_addr, BackendOrganizationAddr)
    assert org_addr.root_verify_key == verify_key
    assert org_addr.scheme == addr.scheme
    assert org_addr.hostname == addr.hostname
    assert org_addr.port == addr.port
    assert org_addr.use_ssl == addr.use_ssl
    assert org_addr.organization_id == addr.organization_id


@pytest.mark.parametrize(
    "url",
    [
        "",
        "foo",
        "xx://foo:42/org?bootstrap-token=123",  # bad scheme
        "parsec://foo:42/org?bootstrap-token=123&dummy",  # unknown param
        "parsec://foo:42/org?bootstrap-token=123&dummy=foo",  # unknown param
        "parsec://foo:42/org?",  # missing mandatory bootstrap-token param
        "parsec://foo:42/org?bootstrap-token=",  # missing value for param
        "parsec://foo:42?bootstrap-token=123",  # missing org name
        "parsec://foo:42/?bootstrap-token=123",  # missing org name
        "parsec://foo:42/bad/org?bootstrap-token=123",  # bad org name
        "parsec://foo:42/~org?bootstrap-token=123",  # bad org name
    ],
)
def test_backend_organization_bootstrap_addr_bad_value(url):
    with pytest.raises(ValueError):
        BackendOrganizationBootstrapAddr(url)


@pytest.mark.parametrize("key_type", (SigningKey, PrivateKey, SecretKey))
def test_keys_dont_leak_on_repr(key_type):
    key = key_type.generate()
    assert repr(key).startswith(f"<{key_type.__module__}.{key_type.__qualname__} object at ")
