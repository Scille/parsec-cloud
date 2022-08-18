# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

missing_: object

class Field:
    default: object

    def __init__(
        self,
        default=missing_,
        attribute=None,
        load_from=None,
        dump_to=None,
        error=None,
        validate=None,
        required=False,
        allow_none=None,
        load_only=False,
        dump_only=False,
        missing=missing_,
        error_messages=None,
        **metadata
    ): ...
    def serialize(self, attr, obj, accessor=None): ...
    def deserialize(self, value): ...
    def _serialize(self, value, attr, obj): ...
    def _deserialize(self, value, attr, ob): ...
    def __deepcopy__(self, memo): ...

class Int(Field): ...
class Float(Field): ...
class String(Field): ...
class List(Field): ...
class Dict(Field): ...
class Nested(Field): ...
class Integer(Field): ...
class Boolean(Field): ...
class Email(Field): ...
