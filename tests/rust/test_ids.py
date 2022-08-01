# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from uuid import uuid4


@pytest.mark.rust
def test_device_name():
    from parsec.api.protocol.types import _RsDeviceName, DeviceName, _PyDeviceName

    assert DeviceName is _RsDeviceName

    py_dn = _PyDeviceName("my_device")
    rs_dn = DeviceName("my_device")

    assert str(py_dn) == str(rs_dn)
    assert repr(py_dn) == repr(rs_dn)
    assert hash(py_dn) == hash(rs_dn)
    assert DeviceName("device") == DeviceName("device")
    assert DeviceName("device1") != DeviceName("device2")
    assert isinstance(DeviceName.new(), DeviceName)

    with pytest.raises(ValueError) as excinfo:
        _PyDeviceName("Invalid Device Name")
    assert str(excinfo.value) == "Invalid data"

    with pytest.raises(ValueError) as excinfo:
        DeviceName("Invalid Device Name")
    assert str(excinfo.value) == "Invalid DeviceName"

    assert DeviceName(DeviceName("foo")) == DeviceName("foo")


@pytest.mark.rust
def test_device_id():
    from parsec.api.protocol.types import _RsDeviceID, DeviceID, _PyDeviceID, UserID, DeviceName

    assert DeviceID is _RsDeviceID

    py_di = _PyDeviceID("user@my_device")
    rs_di = DeviceID("user@my_device")

    assert str(py_di) == str(rs_di)
    assert repr(py_di) == repr(rs_di)
    assert hash(py_di) == hash(rs_di)
    assert isinstance(rs_di.user_id, UserID)
    assert isinstance(rs_di.device_name, DeviceName)
    assert str(rs_di.user_id) == str(py_di.user_id)
    assert str(rs_di.device_name) == str(py_di.device_name)
    assert DeviceID("user@device") == DeviceID("user@device")
    assert DeviceID("user1@device") != DeviceID("user2@device")

    with pytest.raises(ValueError) as excinfo:
        _PyDeviceID("Invalid Device ID")
    assert str(excinfo.value) == "Invalid data"

    with pytest.raises(ValueError) as excinfo:
        DeviceID("Invalid Device ID")
    assert str(excinfo.value) == "Invalid DeviceID"

    assert isinstance(DeviceID.new(), DeviceID)

    assert DeviceID(DeviceID("foo@bar")) == DeviceID("foo@bar")


@pytest.mark.rust
def test_organization_id():
    from parsec.api.protocol.types import OrganizationID, _PyOrganizationID, _RsOrganizationID

    assert OrganizationID is _RsOrganizationID

    py_oi = _PyOrganizationID("MyOrg")
    rs_oi = OrganizationID("MyOrg")

    assert str(py_oi) == str(rs_oi)
    assert repr(py_oi) == repr(rs_oi)
    assert hash(py_oi) == hash(rs_oi)
    assert OrganizationID("MyOrg") == OrganizationID("MyOrg")
    assert OrganizationID("MyOrg1") != OrganizationID("MyOrg2")

    with pytest.raises(ValueError) as excinfo:
        _PyOrganizationID("Invalid Organization ID")
    assert str(excinfo.value) == "Invalid data"

    with pytest.raises(ValueError) as excinfo:
        OrganizationID("Invalid Organization ID")
    assert str(excinfo.value) == "Invalid OrganizationID"

    assert OrganizationID(OrganizationID("foo")) == OrganizationID("foo")


@pytest.mark.rust
def test_human_handle():
    from parsec.api.protocol.types import HumanHandle, _PyHumanHandle, _RsHumanHandle

    assert HumanHandle is _RsHumanHandle

    py_hh = _PyHumanHandle("a@b.c", "User Name")
    rs_hh = HumanHandle("a@b.c", "User Name")

    assert str(py_hh) == str(rs_hh)
    assert repr(py_hh) == repr(rs_hh)
    assert py_hh.label == rs_hh.label
    assert py_hh.email == rs_hh.email
    assert hash(py_hh) == hash(rs_hh)
    assert HumanHandle("a@b.c", "User") == HumanHandle("a@b.c", "User")
    assert HumanHandle("a@b.c", "User") != HumanHandle("e@e.f", "User")

    with pytest.raises(ValueError) as excinfo:
        _PyHumanHandle("a" * 300, "User Name")
    assert str(excinfo.value) == "Invalid email address"

    with pytest.raises(ValueError) as excinfo:
        HumanHandle("a" * 300, "User Name")
    assert str(excinfo.value) == "Invalid email address"

    with pytest.raises(ValueError) as excinfo:
        _PyHumanHandle("a@b.c", "a" * 255)
    assert str(excinfo.value) == "Invalid label"

    with pytest.raises(ValueError) as excinfo:
        HumanHandle("a@b.c", "a" * 255)
    assert str(excinfo.value) == "Invalid label"


@pytest.mark.rust
def test_user_id():
    from parsec.api.protocol.types import UserID, _PyUserID, _RsUserID, _PyDeviceName, DeviceName

    assert UserID is _RsUserID

    py_ui = _PyUserID("UserId")
    rs_ui = UserID("UserId")

    assert str(py_ui) == str(rs_ui)
    assert repr(py_ui) == repr(rs_ui)
    assert hash(py_ui) == hash(rs_ui)
    assert UserID("User") == UserID("User")
    assert UserID("User1") != UserID("User2")

    # Create from DeviceName
    py_di = py_ui.to_device_id(_PyDeviceName("DeviceName"))
    rs_di = rs_ui.to_device_id(DeviceName("DeviceName"))

    assert str(py_di) == str(rs_di)
    assert repr(py_di) == repr(rs_di)

    with pytest.raises(ValueError) as excinfo:
        _PyUserID("Invalid User Id")
    assert str(excinfo.value) == "Invalid data"

    with pytest.raises(ValueError) as excinfo:
        UserID("Invalid User Id")
    assert str(excinfo.value) == "Invalid UserID"

    assert UserID(UserID("foo")) == UserID("foo")


@pytest.mark.rust
def test_device_label():
    from parsec.api.protocol.types import DeviceLabel, _PyDeviceLabel, _RsDeviceLabel

    assert DeviceLabel is _RsDeviceLabel

    py_dl = _PyDeviceLabel("a" * 255)
    rs_dl = DeviceLabel("a" * 255)

    assert str(py_dl) == str(rs_dl)
    assert py_dl.str == rs_dl.str
    assert hash(py_dl) == hash(rs_dl)
    assert repr(py_dl) == repr(rs_dl)
    assert DeviceLabel("abcdef") == DeviceLabel("abcdef")
    assert DeviceLabel("abcdef") != DeviceLabel("ghijkl")

    with pytest.raises(ValueError) as excinfo:
        _PyDeviceLabel("a" * 256)
    assert str(excinfo.value) == "Invalid data"

    with pytest.raises(ValueError) as excinfo:
        DeviceLabel("a" * 256)
    assert str(excinfo.value) == "Invalid DeviceLabel"

    assert DeviceLabel(DeviceLabel("foo")) == DeviceLabel("foo")


@pytest.mark.rust
def test_entry_id():
    from parsec.api.data.entry import EntryID, _RsEntryID, _PyEntryID

    assert EntryID is _RsEntryID

    ID = uuid4()

    py_ei = _PyEntryID(ID)
    rs_ei = EntryID(ID)

    assert str(py_ei) == str(rs_ei)
    assert repr(py_ei) == repr(rs_ei)
    assert py_ei.uuid == rs_ei.uuid
    assert py_ei.hex == rs_ei.hex
    assert py_ei.bytes == rs_ei.bytes

    assert str(EntryID.from_hex(str(ID))) == str(_PyEntryID.from_hex(str(ID)))
    assert str(EntryID.from_bytes(ID.bytes)) == str(_PyEntryID.from_bytes(ID.bytes))

    with pytest.raises(ValueError) as excinfo:
        _PyEntryID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"

    with pytest.raises(ValueError) as excinfo:
        EntryID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"


@pytest.mark.rust
def test_realm_id():
    from parsec.api.protocol.realm import RealmID, _RsRealmID, _PyRealmID

    assert RealmID is _RsRealmID

    ID = uuid4()

    py_ei = _PyRealmID(ID)
    rs_ei = RealmID(ID)

    assert str(py_ei) == str(rs_ei)
    assert repr(py_ei) == repr(rs_ei)
    assert py_ei.uuid == rs_ei.uuid
    assert py_ei.hex == rs_ei.hex
    assert py_ei.bytes == rs_ei.bytes

    assert str(RealmID.from_hex(str(ID))) == str(_PyRealmID.from_hex(str(ID)))
    assert str(RealmID.from_bytes(ID.bytes)) == str(_PyRealmID.from_bytes(ID.bytes))

    with pytest.raises(ValueError) as excinfo:
        _PyRealmID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"

    with pytest.raises(ValueError) as excinfo:
        RealmID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"


@pytest.mark.rust
def test_block_id():
    from parsec.api.protocol.block import BlockID, _RsBlockID, _PyBlockID

    assert BlockID is _RsBlockID

    ID = uuid4()

    py_ei = _PyBlockID(ID)
    rs_ei = BlockID(ID)

    assert str(py_ei) == str(rs_ei)
    assert repr(py_ei) == repr(rs_ei)
    assert py_ei.uuid == rs_ei.uuid
    assert py_ei.hex == rs_ei.hex
    assert py_ei.bytes == rs_ei.bytes

    assert str(BlockID.from_hex(str(ID))) == str(_PyBlockID.from_hex(str(ID)))
    assert str(BlockID.from_bytes(ID.bytes)) == str(_PyBlockID.from_bytes(ID.bytes))

    with pytest.raises(ValueError) as excinfo:
        _PyBlockID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"

    with pytest.raises(ValueError) as excinfo:
        BlockID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"


@pytest.mark.rust
def test_vblob_id():
    from parsec.api.protocol.vlob import VlobID, _RsVlobID, _PyVlobID

    assert VlobID is _RsVlobID

    ID = uuid4()

    py_ei = _PyVlobID(ID)
    rs_ei = VlobID(ID)

    assert str(py_ei) == str(rs_ei)
    assert repr(py_ei) == repr(rs_ei)
    assert py_ei.uuid == rs_ei.uuid
    assert py_ei.hex == rs_ei.hex
    assert py_ei.bytes == rs_ei.bytes

    assert str(VlobID.from_hex(str(ID))) == str(_PyVlobID.from_hex(str(ID)))
    assert str(VlobID.from_bytes(ID.bytes)) == str(_PyVlobID.from_bytes(ID.bytes))

    with pytest.raises(ValueError) as excinfo:
        _PyVlobID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"

    with pytest.raises(ValueError) as excinfo:
        VlobID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"


@pytest.mark.rust
def test_chunk_id():
    from parsec.core.types.manifest import ChunkID, _RsChunkID, _PyChunkID

    assert ChunkID is _RsChunkID

    ID = uuid4()

    py_ei = _PyChunkID(ID)
    rs_ei = ChunkID(ID)

    assert str(py_ei) == str(rs_ei)
    assert repr(py_ei) == repr(rs_ei)
    assert py_ei.uuid == rs_ei.uuid
    assert py_ei.hex == rs_ei.hex
    assert py_ei.bytes == rs_ei.bytes

    assert str(ChunkID.from_hex(str(ID))) == str(_PyChunkID.from_hex(str(ID)))
    assert str(ChunkID.from_bytes(ID.bytes)) == str(_PyChunkID.from_bytes(ID.bytes))

    with pytest.raises(ValueError) as excinfo:
        _PyChunkID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"

    with pytest.raises(ValueError) as excinfo:
        ChunkID(str(uuid4()))
    assert str(excinfo.value) == "Not a UUID"

    d = {rs_ei: None}
    assert rs_ei in d
