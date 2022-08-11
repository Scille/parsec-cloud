# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from parsec.crypto import SecretKey, SigningKey, PrivateKey
from parsec.api.protocol import DeviceLabel, HumanHandle, DeviceID, UserProfile
from parsec.api.data import EntryID


@pytest.mark.rust
def test_sascode():
    from parsec.api.data.invite import _RsSASCode, SASCode, _PySASCode

    assert SASCode is _RsSASCode

    py_sas = _PySASCode.from_int(0)
    rs_sas = SASCode.from_int(0)

    assert py_sas.str == rs_sas.str
    assert repr(py_sas) == repr(rs_sas)
    assert _PySASCode("ABCD").str == SASCode("ABCD").str
    assert SASCode("ABCD") == SASCode("ABCD")
    assert SASCode("ABCD") != SASCode("BCDE")

    # Too long
    with pytest.raises(ValueError) as excinfo:
        SASCode("ABCDE")
    assert str(excinfo.value) == "Invalid SAS code"
    # Too short
    with pytest.raises(ValueError) as excinfo:
        SASCode("ABC")
    assert str(excinfo.value) == "Invalid SAS code"
    # Invalid characters
    with pytest.raises(ValueError) as excinfo:
        SASCode("ABC*")
    assert str(excinfo.value) == "Invalid SAS code"
    with pytest.raises(ValueError) as excinfo:
        SASCode("ABCI")
    assert str(excinfo.value) == "Invalid SAS code"
    with pytest.raises(ValueError) as excinfo:
        SASCode("ABCO")
    assert str(excinfo.value) == "Invalid SAS code"
    with pytest.raises(ValueError) as excinfo:
        SASCode("ABC0")
    assert str(excinfo.value) == "Invalid SAS code"

    # Too large int
    with pytest.raises(ValueError) as excinfo:
        SASCode.from_int(42424242)
    assert str(excinfo.value) == "Provided integer is too large"


@pytest.mark.rust
def test_generate_sas_codes():
    from parsec.api.data.invite import (
        _Rs_generate_sas_codes,
        generate_sas_codes,
        _Py_generate_sas_codes,
    )

    assert generate_sas_codes is _Rs_generate_sas_codes

    sk = SecretKey(b"a" * 32)

    py_claimer, py_greeter = _Py_generate_sas_codes(b"abcd", b"efgh", sk)
    rs_claimer, rs_greeter = generate_sas_codes(b"abcd", b"efgh", sk)

    assert py_claimer.str == rs_claimer.str
    assert py_greeter.str == rs_greeter.str


@pytest.mark.rust
@pytest.mark.parametrize("size", [1, 3, 10])
def test_generate_sas_code_candidates(size):
    from parsec.api.data.invite import (
        _Rs_generate_sas_code_candidates,
        generate_sas_code_candidates,
        _Py_generate_sas_code_candidates,
        SASCode,
    )

    assert generate_sas_code_candidates is _Rs_generate_sas_code_candidates

    sas = SASCode("ABCD")

    py_candidates = _Py_generate_sas_code_candidates(sas, size)
    rs_candidates = generate_sas_code_candidates(sas, size)

    assert len(py_candidates) == size
    assert len(rs_candidates) == size

    assert sas in py_candidates
    assert sas in rs_candidates


@pytest.mark.rust
def test_invite_user_data():
    from parsec.api.data.invite import _RsInviteUserData, InviteUserData, _PyInviteUserData

    assert InviteUserData is _RsInviteUserData

    dl = DeviceLabel("label")
    hh = HumanHandle("hubert.farnsworth@planetexpress.com", "Hubert Farnsworth")
    pk = PrivateKey.generate()
    sik = SigningKey.generate()
    sek = SecretKey.generate()

    py_iud = _PyInviteUserData(
        requested_device_label=dl,
        requested_human_handle=hh,
        public_key=pk.public_key,
        verify_key=sik.verify_key,
    )
    rs_iud = InviteUserData(
        requested_device_label=dl,
        requested_human_handle=hh,
        public_key=pk.public_key,
        verify_key=sik.verify_key,
    )

    assert rs_iud.requested_device_label.str == py_iud.requested_device_label.str
    assert str(rs_iud.requested_human_handle) == str(py_iud.requested_human_handle)
    rs_encrypted = rs_iud.dump_and_encrypt(key=sek)
    py_encrypted = py_iud.dump_and_encrypt(key=sek)

    # Decrypt Rust-encrypted with Rust
    rs_iud2 = InviteUserData.decrypt_and_load(rs_encrypted, sek)
    assert rs_iud.requested_device_label.str == rs_iud2.requested_device_label.str
    assert str(rs_iud.requested_human_handle) == str(rs_iud2.requested_human_handle)

    # Decrypt Python-encrypted with Python
    rs_iud3 = InviteUserData.decrypt_and_load(py_encrypted, sek)
    assert rs_iud.requested_device_label.str == rs_iud3.requested_device_label.str
    assert str(rs_iud.requested_human_handle) == str(rs_iud3.requested_human_handle)

    # Decrypt Rust-encrypted with Python
    py_iud2 = _PyInviteUserData.decrypt_and_load(rs_encrypted, sek)
    assert rs_iud.requested_device_label.str == py_iud2.requested_device_label.str
    assert str(rs_iud.requested_human_handle) == str(py_iud2.requested_human_handle)

    # With requested_human_handle and requested_device_label as None
    py_iud = _PyInviteUserData(
        requested_device_label=None,
        requested_human_handle=None,
        public_key=pk.public_key,
        verify_key=sik.verify_key,
    )
    rs_iud = InviteUserData(
        requested_device_label=None,
        requested_human_handle=None,
        public_key=pk.public_key,
        verify_key=sik.verify_key,
    )

    assert py_iud.requested_device_label is None
    assert rs_iud.requested_device_label is None
    assert py_iud.requested_human_handle is None
    assert rs_iud.requested_human_handle is None


@pytest.mark.rust
def test_invite_user_confirmation():
    from parsec.api.data.invite import (
        _RsInviteUserConfirmation,
        InviteUserConfirmation,
        _PyInviteUserConfirmation,
    )

    assert InviteUserConfirmation is _RsInviteUserConfirmation

    di = DeviceID("a@b")
    dl = DeviceLabel("label")
    hh = HumanHandle("hubert.farnsworth@planetexpress.com", "Hubert Farnsworth")
    profile = UserProfile.STANDARD
    sk = SigningKey.generate()
    vk = sk.verify_key
    sek = SecretKey.generate()

    py_iuc = _PyInviteUserConfirmation(
        device_id=di, device_label=dl, human_handle=hh, profile=profile, root_verify_key=vk
    )
    rs_iuc = InviteUserConfirmation(
        device_id=di, device_label=dl, human_handle=hh, profile=profile, root_verify_key=vk
    )

    assert rs_iuc.device_label.str == py_iuc.device_label.str
    assert str(rs_iuc.human_handle) == str(py_iuc.human_handle)
    assert rs_iuc.device_id.str == py_iuc.device_id.str
    assert rs_iuc.profile == py_iuc.profile
    rs_encrypted = rs_iuc.dump_and_encrypt(key=sek)
    py_encrypted = py_iuc.dump_and_encrypt(key=sek)

    # Decrypt Rust-encrypted with Rust
    rs_iuc2 = InviteUserConfirmation.decrypt_and_load(rs_encrypted, sek)
    assert rs_iuc.device_label.str == rs_iuc2.device_label.str
    assert str(rs_iuc.human_handle) == str(rs_iuc2.human_handle)
    assert rs_iuc.device_id.str == rs_iuc2.device_id.str
    assert rs_iuc.profile == rs_iuc2.profile

    # Decrypt Python-encrypted with Python
    rs_iuc3 = InviteUserConfirmation.decrypt_and_load(py_encrypted, sek)
    assert rs_iuc.device_label.str == rs_iuc3.device_label.str
    assert str(rs_iuc.human_handle) == str(rs_iuc3.human_handle)
    assert rs_iuc.device_id.str == rs_iuc3.device_id.str
    assert rs_iuc.profile == rs_iuc3.profile

    # Decrypt Rust-encrypted with Python
    py_iuc2 = _PyInviteUserConfirmation.decrypt_and_load(rs_encrypted, sek)
    assert rs_iuc.device_label.str == py_iuc2.device_label.str
    assert str(rs_iuc.human_handle) == str(py_iuc2.human_handle)
    assert rs_iuc.device_id.str == py_iuc2.device_id.str
    assert rs_iuc.profile == py_iuc2.profile

    # With human_handle and device_label as None
    py_iuc = _PyInviteUserConfirmation(
        device_id=di, device_label=None, human_handle=None, profile=profile, root_verify_key=vk
    )
    rs_iuc = InviteUserConfirmation(
        device_id=di, device_label=None, human_handle=None, profile=profile, root_verify_key=vk
    )

    assert py_iuc.device_label is None
    assert rs_iuc.device_label is None
    assert py_iuc.human_handle is None
    assert rs_iuc.human_handle is None


@pytest.mark.rust
def test_invite_device_data():
    from parsec.api.data.invite import _RsInviteDeviceData, InviteDeviceData, _PyInviteDeviceData

    assert InviteDeviceData is _RsInviteDeviceData

    dl = DeviceLabel("label")
    sk = SigningKey.generate()
    vk = sk.verify_key
    sek = SecretKey.generate()

    py_idd = _PyInviteDeviceData(requested_device_label=dl, verify_key=vk)
    rs_idd = InviteDeviceData(requested_device_label=dl, verify_key=vk)

    assert rs_idd.requested_device_label.str == py_idd.requested_device_label.str

    rs_encrypted = rs_idd.dump_and_encrypt(key=sek)
    py_encrypted = py_idd.dump_and_encrypt(key=sek)

    # Decrypt Rust-encrypted with Rust
    rs_idd2 = InviteDeviceData.decrypt_and_load(rs_encrypted, sek)
    assert rs_idd.requested_device_label.str == rs_idd2.requested_device_label.str

    # Decrypt Python-encrypted with Python
    rs_idd3 = InviteDeviceData.decrypt_and_load(py_encrypted, sek)
    assert rs_idd.requested_device_label.str == rs_idd3.requested_device_label.str

    # Decrypt Rust-encrypted with Python
    py_idd2 = _PyInviteDeviceData.decrypt_and_load(rs_encrypted, sek)
    assert rs_idd.requested_device_label.str == py_idd2.requested_device_label.str

    # With requested_human_handle and requested_device_label as None
    py_idd = _PyInviteDeviceData(requested_device_label=None, verify_key=vk)
    rs_idd = InviteDeviceData(requested_device_label=None, verify_key=vk)

    assert py_idd.requested_device_label is None
    assert rs_idd.requested_device_label is None


@pytest.mark.rust
def test_invite_device_confirmation():
    from parsec.api.data.invite import (
        _RsInviteDeviceConfirmation,
        InviteDeviceConfirmation,
        _PyInviteDeviceConfirmation,
    )

    assert InviteDeviceConfirmation is _RsInviteDeviceConfirmation

    di = DeviceID("a@b")
    dl = DeviceLabel("label")
    hh = HumanHandle("hubert.farnsworth@planetexpress.com", "Hubert Farnsworth")
    profile = UserProfile.STANDARD
    pk = PrivateKey.generate()
    umi = EntryID.new()
    umk = SecretKey.generate()
    sk = SigningKey.generate()
    vk = sk.verify_key
    sek = SecretKey.generate()

    py_idc = _PyInviteDeviceConfirmation(
        device_id=di,
        device_label=dl,
        human_handle=hh,
        profile=profile,
        private_key=pk,
        user_manifest_id=umi,
        user_manifest_key=umk,
        root_verify_key=vk,
    )
    rs_idc = InviteDeviceConfirmation(
        device_id=di,
        device_label=dl,
        human_handle=hh,
        profile=profile,
        private_key=pk,
        user_manifest_id=umi,
        user_manifest_key=umk,
        root_verify_key=vk,
    )

    assert rs_idc.device_label.str == py_idc.device_label.str
    assert str(rs_idc.human_handle) == str(py_idc.human_handle)
    assert rs_idc.device_id.str == py_idc.device_id.str
    assert rs_idc.profile == py_idc.profile
    assert rs_idc.user_manifest_id.hex == py_idc.user_manifest_id.hex

    rs_encrypted = rs_idc.dump_and_encrypt(key=sek)
    py_encrypted = py_idc.dump_and_encrypt(key=sek)

    # Decrypt Rust-encrypted with Rust
    rs_idc2 = InviteDeviceConfirmation.decrypt_and_load(rs_encrypted, sek)
    assert rs_idc.device_label.str == rs_idc2.device_label.str
    assert str(rs_idc.human_handle) == str(rs_idc2.human_handle)
    assert rs_idc.device_id.str == rs_idc2.device_id.str
    assert rs_idc.profile == rs_idc2.profile
    assert rs_idc.user_manifest_id.hex == rs_idc2.user_manifest_id.hex

    # Decrypt Python-encrypted with Python
    rs_idc3 = InviteDeviceConfirmation.decrypt_and_load(py_encrypted, sek)
    assert rs_idc.device_label.str == rs_idc3.device_label.str
    assert str(rs_idc.human_handle) == str(rs_idc3.human_handle)
    assert rs_idc.device_id.str == rs_idc3.device_id.str
    assert rs_idc.profile == rs_idc3.profile
    assert rs_idc.user_manifest_id.hex == rs_idc3.user_manifest_id.hex

    # Decrypt Rust-encrypted with Python
    py_idc2 = _PyInviteDeviceConfirmation.decrypt_and_load(rs_encrypted, sek)
    assert rs_idc.device_label.str == py_idc2.device_label.str
    assert str(rs_idc.human_handle) == str(py_idc2.human_handle)
    assert rs_idc.device_id.str == py_idc2.device_id.str
    assert rs_idc.profile == py_idc2.profile
    assert rs_idc.user_manifest_id.hex == rs_idc2.user_manifest_id.hex

    # With human_handle and device_label as None
    py_idc = _PyInviteDeviceConfirmation(
        device_id=di,
        device_label=None,
        human_handle=None,
        profile=profile,
        private_key=pk,
        user_manifest_id=umi,
        user_manifest_key=umk,
        root_verify_key=vk,
    )
    rs_idc = InviteDeviceConfirmation(
        device_id=di,
        device_label=None,
        human_handle=None,
        profile=profile,
        private_key=pk,
        user_manifest_id=umi,
        user_manifest_key=umk,
        root_verify_key=vk,
    )

    assert py_idc.device_label is None
    assert rs_idc.device_label is None
    assert py_idc.human_handle is None
    assert rs_idc.human_handle is None

    # No device_label or human_handle
    # with pytest.raises(ValueError):
    #     InviteDeviceConfirmation(
    #         device_id=di,
    #         profile=profile,
    #         private_key=pk,
    #         user_manifest_id=umi,
    #         user_manifest_key=umk,
    #         root_verify_key=vk,
    #     )
