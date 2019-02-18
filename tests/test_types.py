# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.types import (
    DeviceID,
    UserID,
    BackendAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
)
from parsec.crypto import SigningKey, export_root_verify_key


def test_device_id():
    device_id = DeviceID("user_name@device_name")
    assert device_id == "user_name@device_name"
    assert device_id.user_id == "user_name"
    assert device_id.device_name == "device_name"


def test_user_id():
    user_id = UserID("user_name")
    assert user_id == "user_name"


def test_backend_addr_good():
    addr = BackendAddr("ws://foo:42")
    assert addr.scheme == "ws"
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
    addr = BackendOrganizationAddr.build("ws://foo:42", "org", verify_key)
    assert addr.scheme == "ws"
    assert addr.hostname == "foo"
    assert addr.port == 42
    assert addr.organization_id == "org"
    assert addr.root_verify_key == verify_key

    addr2 = BackendOrganizationAddr(str(addr))
    assert addr == addr2


@pytest.mark.parametrize(
    "url", ["ws://foo:42", "ws://foo:42/bad/org?rvk=<rvk>", "ws://foo:42/org?rvk=bad_rvk"]
)
def test_organization_addr_bad_value(url, exported_verify_key):
    url = url.replace("<rvk>", exported_verify_key)
    with pytest.raises(ValueError):
        BackendOrganizationAddr(url)


def test_organization_bootstrap_addr_good(verify_key):
    addr = BackendOrganizationBootstrapAddr.build("ws://foo:42", "org", "token-123")
    assert addr.scheme == "ws"
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
    "url", ["ws://foo:42", "ws://foo:42/bad/org?bootstrap-token=123", "ws://foo:42/org?"]
)
def test_organization_bootstrap_addr_bad_value(url):
    with pytest.raises(ValueError):
        BackendOrganizationBootstrapAddr(url)
