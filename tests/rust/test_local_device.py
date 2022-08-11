# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.api.data.entry import EntryID
from parsec.api.protocol.types import DeviceID, OrganizationID, UserProfile
from parsec.core.types.backend_address import BackendAddr, BackendOrganizationAddr
import pytest

from parsec.crypto import PrivateKey, SecretKey, SigningKey


@pytest.mark.rust
def test_local_device():
    from parsec.core.types.local_device import _RsLocalDevice, LocalDevice, _PyLocalDevice

    assert LocalDevice is _RsLocalDevice

    def _assert_local_device_eq(py, rs):
        assert isinstance(py, _PyLocalDevice)
        assert isinstance(rs, _RsLocalDevice)

        assert py.organization_addr == rs.organization_addr
        assert py.device_id == rs.device_id
        assert py.device_label == rs.device_label
        assert py.human_handle == rs.human_handle
        assert py.signing_key == rs.signing_key
        assert py.private_key == rs.private_key
        assert py.profile == rs.profile
        assert py.user_manifest_id == rs.user_manifest_id
        assert py.user_manifest_key == rs.user_manifest_key
        assert py.local_symkey == rs.local_symkey

        assert py.is_admin == rs.is_admin
        assert py.is_outsider == rs.is_outsider
        assert py.slug == rs.slug
        assert py.slughash == rs.slughash
        assert py.root_verify_key == rs.root_verify_key
        assert py.organization_id == rs.organization_id
        assert py.device_name == rs.device_name
        assert py.user_id == rs.user_id
        assert py.verify_key == rs.verify_key
        assert py.public_key == rs.public_key
        assert py.user_display == rs.user_display
        assert py.short_user_display == rs.short_user_display
        assert py.device_display == rs.device_display

    signing_key = SigningKey.generate()
    kwargs = {
        "organization_addr": BackendOrganizationAddr.build(
            BackendAddr.from_url("parsec://foo"),
            organization_id=OrganizationID("org"),
            root_verify_key=signing_key.verify_key,
        ),
        "device_id": DeviceID.new(),
        "device_label": None,
        "human_handle": None,
        "signing_key": signing_key,
        "private_key": PrivateKey.generate(),
        "profile": UserProfile.ADMIN,
        "user_manifest_id": EntryID.new(),
        "user_manifest_key": SecretKey.generate(),
        "local_symkey": SecretKey.generate(),
    }

    py_ba = _PyLocalDevice(**kwargs)
    rs_ba = LocalDevice(**kwargs)
    _assert_local_device_eq(py_ba, rs_ba)
