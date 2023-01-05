# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
from typing import Any, Dict, Iterable, Type, TypeVar, Union

from marshmallow import MarshalResult, UnmarshalResult, ValidationError
from marshmallow import Schema as MarshmallowSchema

try:
    import toastedmarshmallow

    class BaseSchema(MarshmallowSchema):
        class Meta:
            jit = toastedmarshmallow.Jit

except ImportError:
    BaseSchema = MarshmallowSchema  # type: ignore[misc, assignment]


T = TypeVar("T", bound=Iterable[Any])


# Shamelessly taken from marshmallow-oneofschema (
# https://github.com/maximkulkin/marshmallow-oneofschema - MIT licensed)
# This is needed because marshmallow-oneofschema depends of marshmallow which
# cannot be installed along with toastedmarshmallow
class OneOfSchema(BaseSchema):
    """
    This is a special kind of schema that actually multiplexes other schemas
    based on object type. When serializing values, it uses get_obj_type() method
    to get object type name. Then it uses `type_schemas` name-to-BaseSchema mapping
    to get schema for that particular object type, serializes object using that
    schema and adds an extra "type" field with name of object type.
    Deserialization is reverse.

    Example:

        class Foo(object):
            def __init__(self, foo):
                self.foo = foo

        class Bar(object):
            def __init__(self, bar):
                self.bar = bar

        class FooSchema(marshmallow.BaseSchema):
            foo = marshmallow.fields.String(required=True)

            @marshmallow.post_load
            def make_foo(self, data):
                return Foo(**data)

        class BarSchema(marshmallow.BaseSchema):
            bar = marshmallow.fields.Integer(required=True)

            @marshmallow.post_load
            def make_bar(self, data):
                return Bar(**data)

        class MyUberSchema(marshmallow.OneOfSchema):
            type_schemas = {
                'foo': FooSchema,
                'bar': BarSchema,
            }

            def get_obj_type(self, obj):
                if isinstance(obj, Foo):
                    return 'foo'
                elif isinstance(obj, Bar):
                    return 'bar'
                else:
                    raise Exception('Unknown object type: %s' % repr(obj))

        MyUberSchema().dump([Foo(foo='hello'), Bar(bar=123)], many=True).data
        # => [{'type': 'foo', 'foo': 'hello'}, {'type': 'bar', 'bar': 123}]

    You can control type field name added to serialized object representation by
    setting `type_field` class property.
    """

    type_field: str = "type"
    type_schemas: Dict[Union[Enum, str], Union[BaseSchema, Type[BaseSchema]]] = {}
    fallback_type_schema: Type[BaseSchema] | None = None
    _instantiated_schemas: Dict[Union[Enum, str], BaseSchema] | None = None
    _instantiated_fallback_schema: BaseSchema | None = None

    def _build_schemas(self) -> None:
        enum_types = {type(k) for k in self.type_schemas.keys()}
        if len(enum_types) != 1 or not issubclass(enum_types.pop(), Enum):
            raise ValueError("type_schemas key can only be one Enum")
        self._instantiated_schemas = {}
        for k, v in self.type_schemas.items():
            schema_instance = v if isinstance(v, BaseSchema) else v()
            if self.type_field not in schema_instance.fields:  # type: ignore[attr-defined]
                raise ValueError(f"{schema_instance} needs to define '{self.type_field}' field")
            self._instantiated_schemas[k] = schema_instance
            if isinstance(k, Enum):
                self._instantiated_schemas[k.value] = schema_instance

        if self.fallback_type_schema:
            self._instantiated_fallback_schema = self.fallback_type_schema()

    def _get_schema(self, type: Union[Enum, str]) -> BaseSchema | None:
        if self._instantiated_schemas is None:
            self._build_schemas()
        assert self._instantiated_schemas is not None
        return self._instantiated_schemas.get(type, self._instantiated_fallback_schema)

    def get_obj_type(self, obj: dict[str, object]) -> Any:
        """Returns name of object schema"""
        raise NotImplementedError()

    def dump(
        self, obj: T, many: Any = None, update_fields: bool = True, **kwargs: Any
    ) -> MarshalResult[T]:
        many = self.many if many is None else bool(many)
        if not many:
            result = self._dump(obj, update_fields, **kwargs)
        else:
            result_data = []
            result_errors = {}

            for idx, o in enumerate(obj):
                result = self._dump(o, update_fields, **kwargs)
                result_data.append(result.data)
                if result.errors:
                    result_errors[idx] = result.errors

            result = MarshalResult(result_data, result_errors)

        if result.errors and self.strict:
            raise ValidationError(result.errors)

        return result

    def _dump(self, obj: Any, update_fields: bool = True, **kwargs: Any) -> MarshalResult[Any]:
        obj_type = self.get_obj_type(obj)
        if not obj_type:
            return MarshalResult(
                None, {"_schema": "Unknown object class: %s" % obj.__class__.__name__}
            )

        schema = self._get_schema(obj_type)
        if not schema:
            return MarshalResult(None, {"_schema": "Unsupported object type: %s" % obj_type})

        result = schema.dump(obj, many=False, update_fields=update_fields, **kwargs)
        return result

    def load(  # type: ignore[override]
        self, data: Dict[Any, Any], many: Any | None = None, partial: Any | None = None
    ) -> MarshalResult[Any]:
        many = self.many if many is None else bool(many)
        if partial is None:
            partial = self.partial

        if not many:
            result = self._load(data, partial=partial)
        else:
            result_data = []
            result_errors = {}

            for idx, item in enumerate(data):
                result = self._load(item, partial=partial)
                result_data.append(result.data)
                if result.errors:
                    result_errors[idx] = result.errors

            result = UnmarshalResult(result_data, result_errors)
        if result.errors and self.strict:
            raise ValidationError(result.errors)

        return result

    def _load(self, data: dict[Any, Any], partial: Any = None) -> Any:
        if not isinstance(data, dict):
            return UnmarshalResult({}, {"_schema": "Invalid data type: %s" % data})

        data = dict(data)

        data_type = data.get(self.type_field)

        if not data_type:
            return UnmarshalResult({}, {self.type_field: ["Missing data for required field."]})

        try:
            schema = self._get_schema(data_type)
        except TypeError:
            # data_type could be unhashable
            return UnmarshalResult({}, {self.type_field: ["Invalid value: %s" % data_type]})

        if not schema:
            return UnmarshalResult({}, {self.type_field: ["Unsupported value: %s" % data_type]})

        return schema.load(data, many=False, partial=partial)

    def validate(self, data: dict[Any, Any], many: bool | None = None, partial: Any = None) -> Any:
        try:
            return self.load(data, many=many, partial=partial).errors
        except ValidationError as ve:
            return ve.messages


class OneOfSchemaLegacy(OneOfSchema):
    _instantiated_schemas: Dict[Enum | str, BaseSchema] | None = None
    _instantiated_fallback_schema: BaseSchema | None = None

    def _get_schema(self, type: Enum | str) -> BaseSchema | None:
        if self._instantiated_schemas is None:
            self._instantiated_schemas = {
                k: v if isinstance(v, BaseSchema) else v() for k, v in self.type_schemas.items()
            }

            if self.fallback_type_schema:
                self._instantiated_fallback_schema = self.fallback_type_schema()

        return self._instantiated_schemas.get(type, self._instantiated_fallback_schema)
