# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pendulum
import uuid

from parsec.serde import packb, unpackb, UnknownCheckedSchema, Serializer


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


def test_repr_serializer():
    class MySchema(UnknownCheckedSchema):
        pass

    serializer = Serializer(MySchema)
    assert repr(serializer) == "Serializer(schema=MySchema)"
