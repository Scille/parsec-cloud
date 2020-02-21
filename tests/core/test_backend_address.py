# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.types import (
    BackendAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationClaimUserAddr,
    BackendOrganizationClaimDeviceAddr,
    BackendOrganizationFileLinkAddr,
)


ORG = "MyOrg"
RVK = "P25GRG3XPSZKBEKXYQFBOLERWQNEDY3AO43MVNZCLPXPKN63JRYQssss"
TOKEN = "1234ABCD"
DOMAIN = "parsec.cloud.com"
USER_ID = "John"
DEVICE_ID = "John%40Dev42"
PATH = "%2Fdir%2Ffile"
WORKSPACE_ID = "2d4ded12-7406-4608-833b-7f57f01156e2"
DEFAULT_ARGS = {
    "ORG": ORG,
    "RVK": RVK,
    "TOKEN": TOKEN,
    "DOMAIN": DOMAIN,
    "USER_ID": USER_ID,
    "DEVICE_ID": DEVICE_ID,
    "PATH": PATH,
    "WORKSPACE_ID": WORKSPACE_ID,
}


class AddrTestbed:
    def __init__(self, name, cls, template):
        self.name = name
        self.cls = cls
        self.template = template

    def __repr__(self):
        return self.name

    @property
    def url(self):
        return self.generate_url()

    def generate_url(self, **kwargs):
        return self.template.format(**{**DEFAULT_ARGS, **kwargs})


BackendAddrTestbed = AddrTestbed("backend_addr", BackendAddr, "parsec://{DOMAIN}")
BackendOrganizationAddrTestbed = AddrTestbed(
    "org_addr", BackendOrganizationAddr, "parsec://{DOMAIN}/{ORG}?rvk={RVK}"
)
BackendOrganizationBootstrapAddrTestbed = AddrTestbed(
    "org_bootstrap_addr",
    BackendOrganizationBootstrapAddr,
    "parsec://{DOMAIN}/{ORG}?action=bootstrap_organization&token={TOKEN}",
)
BackendOrganizationClaimUserAddrTestbed = AddrTestbed(
    "org_claim_user_addr",
    BackendOrganizationClaimUserAddr,
    "parsec://{DOMAIN}/{ORG}?action=claim_user&user_id={USER_ID}&token={TOKEN}&rvk={RVK}",
)
BackendOrganizationClaimDeviceAddrTestbed = AddrTestbed(
    "org_claim_device_addr",
    BackendOrganizationClaimDeviceAddr,
    "parsec://{DOMAIN}/{ORG}?action=claim_device&device_id={DEVICE_ID}&token={TOKEN}&rvk={RVK}",
)
BackendOrganizationClaimUserAddrNoTokenTestbed = AddrTestbed(
    "org_claim_user_addr_no_token",
    BackendOrganizationClaimUserAddr,
    "parsec://{DOMAIN}/{ORG}?action=claim_user&user_id={USER_ID}&rvk={RVK}",
)
BackendOrganizationClaimDeviceAddrNoTokenTestbed = AddrTestbed(
    "org_claim_device_addr_no_token",
    BackendOrganizationClaimDeviceAddr,
    "parsec://{DOMAIN}/{ORG}?action=claim_device&device_id={DEVICE_ID}&rvk={RVK}",
)
BackendOrganizationFileLinkAddrTestbed = AddrTestbed(
    "org_file_link_addr",
    BackendOrganizationFileLinkAddr,
    "parsec://{DOMAIN}/{ORG}?action=file_link&workspace_id={WORKSPACE_ID}&path={PATH}&rvk={RVK}",
)


@pytest.fixture(
    params=[
        BackendAddrTestbed,
        BackendOrganizationAddrTestbed,
        BackendOrganizationBootstrapAddrTestbed,
        BackendOrganizationClaimUserAddrTestbed,
        BackendOrganizationClaimDeviceAddrTestbed,
        BackendOrganizationClaimUserAddrNoTokenTestbed,
        BackendOrganizationClaimDeviceAddrNoTokenTestbed,
        BackendOrganizationFileLinkAddrTestbed,
    ]
)
def addr_testbed(request):
    return request.param


@pytest.fixture(
    params=[
        BackendOrganizationAddrTestbed,
        BackendOrganizationBootstrapAddrTestbed,
        BackendOrganizationClaimUserAddrTestbed,
        BackendOrganizationClaimDeviceAddrTestbed,
        BackendOrganizationClaimUserAddrNoTokenTestbed,
        BackendOrganizationClaimDeviceAddrNoTokenTestbed,
        BackendOrganizationFileLinkAddrTestbed,
    ]
)
def addr_with_org_testbed(request):
    return request.param


@pytest.fixture(
    params=[
        BackendOrganizationBootstrapAddrTestbed,
        BackendOrganizationClaimUserAddrTestbed,
        BackendOrganizationClaimDeviceAddrTestbed,
    ]
)
def addr_with_token_testbed(request):
    return request.param


@pytest.fixture
def addr_file_link_testbed(request):
    return BackendOrganizationFileLinkAddrTestbed


def test_good_addr(addr_testbed):
    addr = addr_testbed.cls.from_url(addr_testbed.url)
    url2 = addr.to_url()
    assert url2 == addr_testbed.url


def test_good_addr_with_port(addr_testbed):
    url = addr_testbed.generate_url(DOMAIN="example.com:4242")
    addr = addr_testbed.cls.from_url(url)
    url2 = addr.to_url()
    assert url2 == url


@pytest.mark.parametrize("bad_port", ["NaN", "999999"])
def test_addr_with_bad_port(addr_testbed, bad_port):
    url = addr_testbed.generate_url(DOMAIN=f"example.com:{bad_port}")
    with pytest.raises(ValueError):
        addr_testbed.cls.from_url(url)


def test_good_addr_with_unknown_field(addr_testbed):
    url = addr_testbed.url
    url_with_unknown_field = url + ("&" if "?" in url else "?") + "unknown_field=ok"
    addr = addr_testbed.cls.from_url(url_with_unknown_field)
    url2 = addr.to_url()
    assert url2 == url


def test_good_addr_with_unicode_org_name(addr_with_org_testbed):
    orgname = "康熙帝"
    orgname_percent_quoted = "%E5%BA%B7%E7%86%99%E5%B8%9D"
    url = addr_with_org_testbed.generate_url(ORG=orgname_percent_quoted)
    addr = addr_with_org_testbed.cls.from_url(url)
    assert addr.organization_id == orgname
    url2 = addr.to_url()
    assert url2 == url


def test_addr_with_bad_percent_encoded_org_name(addr_with_org_testbed):
    bad_percent_quoted = "%E5%BA%B7%E7"  # Not a valid utf8 string
    url = addr_with_org_testbed.generate_url(ORG=bad_percent_quoted)
    with pytest.raises(ValueError):
        addr_with_org_testbed.cls.from_url(url)


def test_good_addr_with_unicode_token(addr_with_token_testbed):
    token = "康熙帝"
    token_percent_quoted = "%E5%BA%B7%E7%86%99%E5%B8%9D"
    url = addr_with_token_testbed.generate_url(TOKEN=token_percent_quoted)
    addr = addr_with_token_testbed.cls.from_url(url)
    assert addr.token == token
    url2 = addr.to_url()
    assert url2 == url


def test_addr_with_bad_percent_encoded_token(addr_with_token_testbed):
    bad_percent_quoted = "%E5%BA%B7%E7"  # Not a valid utf8 string
    url = addr_with_token_testbed.generate_url(TOKEN=bad_percent_quoted)
    with pytest.raises(ValueError):
        addr_with_token_testbed.cls.from_url(url)


@pytest.mark.parametrize("invalid_workspace", [None, "4def"])
def test_file_link_addr_invalid_workspace(addr_file_link_testbed, invalid_workspace):
    url = addr_file_link_testbed.generate_url(WORKSPACE_ID=invalid_workspace)
    with pytest.raises(ValueError):
        addr_file_link_testbed.cls.from_url(url)


@pytest.mark.parametrize("invalid_path", [None, "dir/path"])
def test_file_link_addr_invalid_path(addr_file_link_testbed, invalid_path):
    url = addr_file_link_testbed.generate_url(PATH=invalid_path)
    with pytest.raises(ValueError):
        addr_file_link_testbed.cls.from_url(url)
