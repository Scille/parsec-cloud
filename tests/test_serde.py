# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import pendulum
import uuid
from collections import namedtuple

from parsec.serde import (
    packb,
    unpackb,
    BaseSchema,
    OneOfSchema,
    MsgpackSerializer,
    fields,
    SerdeError,
)


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
    class MySchema(BaseSchema):
        pass

    serializer = MsgpackSerializer(MySchema)
    assert repr(serializer) == "MsgpackSerializer(schema=MySchema)"


def test_serializer_loads_bad_data():
    class BirdSchema(BaseSchema):
        flying = fields.Boolean(required=True)

    serializer = MsgpackSerializer(BirdSchema)
    for raw in (packb(0), packb([]), packb({}), b"dummy"):
        with pytest.raises(SerdeError):
            serializer.loads(raw)


def test_oneof_schema():
    class BirdSchema(BaseSchema):
        flying = fields.Boolean()

    class FishSchema(BaseSchema):
        swimming = fields.Int()

    class AnimalSchema(OneOfSchema):
        type_field = "type"
        type_schemas = {"bird": BirdSchema(), "fish": FishSchema()}

    schema = AnimalSchema()

    res, errors = schema.load({"type": "fish", "swimming": True})
    assert not errors
    assert res == {"swimming": True}

    res, errors = schema.load({"type": "dummy", "swimming": True})
    assert errors == {"type": ["Unsupported value: dummy"]}

    # Schema ignore unknown fields
    res, errors = schema.load(
        [{"type": "fish", "swimming": True}, {"type": "bird", "swimming": True}], many=True
    )
    assert res == [{"swimming": True}, {}]
    assert not errors

    bird_cls = namedtuple("bird", "flying,ignore_me")
    bird = bird_cls(True, "whatever")
    fish_cls = namedtuple("fish", "swimming")
    fish = fish_cls(True)

    res, errors = schema.dump(bird)
    assert res == {"type": "bird", "flying": True}
    assert not errors

    res, errors = schema.dump([bird, fish], many=True)
    assert res == [{"type": "bird", "flying": True}, {"type": "fish", "swimming": True}]
    assert not errors
