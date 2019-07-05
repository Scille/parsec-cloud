# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.types import (
    DeviceID,
    UserID,
    BackendAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
)
from parsec.crypto import SigningKey, PrivateKey, SecretKey, export_root_verify_key


def test_device_id():
    device_id = DeviceID("user_id@device_name")
    assert device_id == "user_id@device_name"
    assert device_id.user_id == "user_id"
    assert device_id.device_name == "device_name"


def test_user_id():
    user_id = UserID("user_id")
    assert user_id == "user_id"


def test_backend_addr_good():
    addr = BackendAddr("parsec://foo:42")
    assert addr.scheme == "parsec"
    assert addr.hostname == "foo"
    assert addr.port == 42


@pytest.mark.parametrize("url", ["", "foo", "xx://foo:42"])
def test_backend_addr_bad_value(url):
    with pytest.raises(ValueError):
        BackendAddr(url)


@pytest.fixture(scope="session")
def verify_key():
    return SigningKey.generate().verify_key


@pytest.fixture(scope="session")
def exported_verify_key(verify_key):
    return export_root_verify_key(verify_key)


def test_organization_addr_good(verify_key):
    addr = BackendOrganizationAddr.build("parsec://foo:42", "org", verify_key)
    assert addr.scheme == "parsec"
    assert addr.hostname == "foo"
    assert addr.port == 42
    assert addr.organization_id == "org"
    assert addr.root_verify_key == verify_key

    addr2 = BackendOrganizationAddr(str(addr))
    assert addr == addr2


@pytest.mark.parametrize(
    "url",
    ["parsec://foo:42", "parsec://foo:42/bad/org?rvk=<rvk>", "parsec://foo:42/org?rvk=bad_rvk"],
)
def test_organization_addr_bad_value(url, exported_verify_key):
    url = url.replace("<rvk>", exported_verify_key)
    with pytest.raises(ValueError):
        BackendOrganizationAddr(url)


def test_organization_bootstrap_addr_good(verify_key):
    addr = BackendOrganizationBootstrapAddr.build("parsec://foo:42", "org", "token-123")
    assert addr.scheme == "parsec"
    assert addr.hostname == "foo"
    assert addr.port == 42
    assert addr.organization_id == "org"
    assert addr.bootstrap_token == "token-123"

    addr2 = BackendOrganizationBootstrapAddr(str(addr))
    assert addr == addr2

    org_addr = addr.generate_organization_addr(verify_key)
    assert isinstance(org_addr, BackendOrganizationAddr)
    assert org_addr.root_verify_key == verify_key


@pytest.mark.parametrize(
    "url",
    ["parsec://foo:42", "parsec://foo:42/bad/org?bootstrap-token=123", "parsec://foo:42/org?"],
)
def test_organization_bootstrap_addr_bad_value(url):
    with pytest.raises(ValueError):
        BackendOrganizationBootstrapAddr(url)


@pytest.mark.parametrize("key_type", (SigningKey, PrivateKey, SecretKey))
def test_keys_dont_leak_on_repr(key_type):
    key = key_type.generate()
    assert repr(key).startswith(f"<{key_type.__module__}.{key_type.__qualname__} object at ")
