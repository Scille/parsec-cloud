# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

import pendulum

from parsec.api.protocol import (
    UserID,
    DeviceID,
    UserProfile,
    HumanHandle,
    DeviceLabel,
    RealmID,
    RealmRole,
)
from parsec.crypto import PrivateKey, SigningKey


@pytest.mark.rust
def test_user_certificate():
    from parsec.api.data.certif import (
        _RsUserCertificateContent,
        UserCertificateContent,
        _PyUserCertificateContent,
    )

    assert UserCertificateContent is _RsUserCertificateContent

    def _assert_user_certificate_eq(py, rs):
        assert py.is_admin == rs.is_admin
        assert py.author == rs.author
        assert py.timestamp == rs.timestamp
        assert py.user_id == rs.user_id
        assert py.human_handle == rs.human_handle
        assert py.public_key == rs.public_key
        assert py.profile == rs.profile

    kwargs = {
        "author": DeviceID.new(),
        "timestamp": pendulum.now(),
        "user_id": UserID("bob"),
        "human_handle": HumanHandle("bob@example.com", "Boby McBobFace"),
        "public_key": PrivateKey.generate().public_key,
        "profile": UserProfile.ADMIN,
    }

    py_uc = _PyUserCertificateContent(**kwargs)
    rs_uc = UserCertificateContent(**kwargs)
    _assert_user_certificate_eq(py_uc, rs_uc)

    kwargs = {
        "author": DeviceID.new(),
        "timestamp": pendulum.now(),
        "user_id": UserID("alice"),
        "human_handle": None,
        "public_key": PrivateKey.generate().public_key,
        "profile": UserProfile.STANDARD,
    }

    py_uc = py_uc.evolve(**kwargs)
    rs_uc = rs_uc.evolve(**kwargs)
    _assert_user_certificate_eq(py_uc, rs_uc)

    sign_key = SigningKey.generate()
    py_data = py_uc.dump_and_sign(sign_key)
    rs_data = rs_uc.dump_and_sign(sign_key)

    py_uc = _PyUserCertificateContent.verify_and_load(
        rs_data,
        sign_key.verify_key,
        expected_author=py_uc.author,
        expected_user=py_uc.user_id,
        expected_human_handle=py_uc.human_handle,
    )
    rs_uc = UserCertificateContent.verify_and_load(
        py_data,
        sign_key.verify_key,
        expected_author=rs_uc.author,
        expected_user=rs_uc.user_id,
        expected_human_handle=rs_uc.human_handle,
    )
    _assert_user_certificate_eq(py_uc, rs_uc)

    py_uc = _PyUserCertificateContent.unsecure_load(rs_data)
    rs_uc = UserCertificateContent.unsecure_load(py_data)
    _assert_user_certificate_eq(py_uc, rs_uc)


@pytest.mark.rust
def test_revoked_user_certificate():
    from parsec.api.data.certif import (
        _RsRevokedUserCertificateContent,
        RevokedUserCertificateContent,
        _PyRevokedUserCertificateContent,
    )

    assert RevokedUserCertificateContent is _RsRevokedUserCertificateContent

    def _assert_revoked_user_certificate_eq(py, rs):
        assert py.author == rs.author
        assert py.timestamp == rs.timestamp
        assert py.user_id == rs.user_id

    kwargs = {"author": DeviceID.new(), "timestamp": pendulum.now(), "user_id": UserID("bob")}

    py_ruc = _PyRevokedUserCertificateContent(**kwargs)
    rs_ruc = RevokedUserCertificateContent(**kwargs)
    _assert_revoked_user_certificate_eq(py_ruc, rs_ruc)

    kwargs = {"author": DeviceID.new(), "timestamp": pendulum.now(), "user_id": UserID("alice")}

    py_ruc = py_ruc.evolve(**kwargs)
    rs_ruc = rs_ruc.evolve(**kwargs)
    _assert_revoked_user_certificate_eq(py_ruc, rs_ruc)

    sign_key = SigningKey.generate()
    py_data = py_ruc.dump_and_sign(sign_key)
    rs_data = rs_ruc.dump_and_sign(sign_key)

    py_ruc = _PyRevokedUserCertificateContent.verify_and_load(
        rs_data, sign_key.verify_key, expected_author=py_ruc.author, expected_user=py_ruc.user_id
    )
    rs_ruc = RevokedUserCertificateContent.verify_and_load(
        py_data, sign_key.verify_key, expected_author=rs_ruc.author, expected_user=rs_ruc.user_id
    )
    _assert_revoked_user_certificate_eq(py_ruc, rs_ruc)

    py_ruc = _PyRevokedUserCertificateContent.unsecure_load(rs_data)
    rs_ruc = RevokedUserCertificateContent.unsecure_load(py_data)
    _assert_revoked_user_certificate_eq(py_ruc, rs_ruc)


@pytest.mark.rust
def test_device_certificate():
    from parsec.api.data.certif import (
        _RsDeviceCertificateContent,
        DeviceCertificateContent,
        _PyDeviceCertificateContent,
    )

    assert DeviceCertificateContent is _RsDeviceCertificateContent

    def _assert_device_certificate_eq(py, rs):
        assert py.author == rs.author
        assert py.timestamp == rs.timestamp
        assert py.device_id == rs.device_id
        assert py.device_label == rs.device_label
        assert py.verify_key == rs.verify_key

    kwargs = {
        "author": DeviceID.new(),
        "timestamp": pendulum.now(),
        "device_id": DeviceID("bob@dev1"),
        "device_label": DeviceLabel("dev machine"),
        "verify_key": SigningKey.generate().verify_key,
    }

    py_dc = _PyDeviceCertificateContent(**kwargs)
    rs_dc = DeviceCertificateContent(**kwargs)
    _assert_device_certificate_eq(py_dc, rs_dc)

    kwargs = {
        "author": DeviceID.new(),
        "timestamp": pendulum.now(),
        "device_id": DeviceID("alice@dev1"),
        "device_label": None,
        "verify_key": SigningKey.generate().verify_key,
    }

    py_dc = py_dc.evolve(**kwargs)
    rs_dc = rs_dc.evolve(**kwargs)
    _assert_device_certificate_eq(py_dc, rs_dc)

    sign_key = SigningKey.generate()
    py_data = py_dc.dump_and_sign(sign_key)
    rs_data = rs_dc.dump_and_sign(sign_key)

    py_dc = _PyDeviceCertificateContent.verify_and_load(
        rs_data, sign_key.verify_key, expected_author=py_dc.author, expected_device=py_dc.device_id
    )
    rs_dc = DeviceCertificateContent.verify_and_load(
        py_data, sign_key.verify_key, expected_author=rs_dc.author, expected_device=rs_dc.device_id
    )
    _assert_device_certificate_eq(py_dc, rs_dc)

    py_dc = _PyDeviceCertificateContent.unsecure_load(rs_data)
    rs_dc = DeviceCertificateContent.unsecure_load(py_data)
    _assert_device_certificate_eq(py_dc, rs_dc)


@pytest.mark.rust
def test_realm_role_certificate():
    from parsec.api.data.certif import (
        _RsRealmRoleCertificateContent,
        RealmRoleCertificateContent,
        _PyRealmRoleCertificateContent,
    )

    assert RealmRoleCertificateContent is _RsRealmRoleCertificateContent

    def _assert_realm_role_certificate_eq(py, rs):
        assert py.author == rs.author
        assert py.timestamp == rs.timestamp
        assert py.realm_id == rs.realm_id
        assert py.user_id == rs.user_id
        assert py.role == rs.role

    kwargs = {
        "author": DeviceID.new(),
        "timestamp": pendulum.now(),
        "realm_id": RealmID.new(),
        "user_id": UserID("bob"),
        "role": RealmRole.OWNER,
    }

    py_rrc = _PyRealmRoleCertificateContent(**kwargs)
    rs_rrc = RealmRoleCertificateContent(**kwargs)
    _assert_realm_role_certificate_eq(py_rrc, rs_rrc)

    kwargs = {
        "author": DeviceID.new(),
        "timestamp": pendulum.now(),
        "realm_id": RealmID.new(),
        "user_id": UserID("alice"),
        "role": RealmRole.CONTRIBUTOR,
    }

    py_rrc = py_rrc.evolve(**kwargs)
    rs_rrc = rs_rrc.evolve(**kwargs)
    _assert_realm_role_certificate_eq(py_rrc, rs_rrc)

    sign_key = SigningKey.generate()
    py_data = py_rrc.dump_and_sign(sign_key)
    rs_data = rs_rrc.dump_and_sign(sign_key)

    py_rrc = _PyRealmRoleCertificateContent.verify_and_load(
        rs_data,
        sign_key.verify_key,
        expected_author=py_rrc.author,
        expected_realm=py_rrc.realm_id,
        expected_user=py_rrc.user_id,
        expected_role=py_rrc.role,
    )
    rs_rrc = RealmRoleCertificateContent.verify_and_load(
        py_data,
        sign_key.verify_key,
        expected_author=rs_rrc.author,
        expected_realm=rs_rrc.realm_id,
        expected_user=rs_rrc.user_id,
        expected_role=rs_rrc.role,
    )
    _assert_realm_role_certificate_eq(py_rrc, rs_rrc)

    py_rrc = _PyRealmRoleCertificateContent.unsecure_load(rs_data)
    rs_rrc = RealmRoleCertificateContent.unsecure_load(py_data)
    _assert_realm_role_certificate_eq(py_rrc, rs_rrc)
