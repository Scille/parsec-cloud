import pendulum
import uuid

from parsec.serde import packb, unpackb


def test_pack_datetime():
    data = {"date": pendulum.now()}
    packed = packb(data)
    unpacked = unpackb(packed)
    assert unpacked == data
    assert isinstance(unpacked["date"], pendulum.Pendulum)


def test_pack_uuid():
    data = {"uuid": uuid.uuid4()}
    packed = packb(data)
    unpacked = unpackb(packed)
    assert unpacked == data
    assert isinstance(unpacked["uuid"], uuid.UUID)
