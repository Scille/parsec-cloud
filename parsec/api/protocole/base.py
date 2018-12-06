from abc import ABC
from parsec.schema import fields, UnknownCheckedSchema, OneOfSchema, post_load, ValidationError


class ProtocoleError(Exception, ABC):
    pass


# Expose `marshmallow.ValidationError` as a child of `ProtocoleError`
ProtocoleError.register(ValidationError)


class BaseReqSchema(UnknownCheckedSchema):
    cmd = fields.String(required=True)

    @post_load
    def _drop_cmd_field(self, item):
        if self.drop_cmd_field:
            item.pop("cmd")
        return item

    def __init__(self, drop_cmd_field=True, **kwargs):
        super().__init__(**kwargs)
        self.drop_cmd_field = drop_cmd_field


class BaseRepSchema(UnknownCheckedSchema):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.with_error_schema = RepWithErrorSchema()
        self.with_error_schema.type_schemas["ok"] = self

    status = fields.CheckedConstant("ok", required=True)


class ErrorRepSchema(BaseRepSchema):
    status = fields.String(required=True)
    reason = fields.String(allow_none=True)
    # TODO: should errors be better checked ?
    errors = fields.Dict(allow_none=True)


class RepWithErrorSchema(OneOfSchema):
    type_field = "status"
    type_field_remove = False
    fallback_type_schema = ErrorRepSchema
    type_schemas = {}  # Overridden by children


class CmdSerializer:
    def __repr__(self):
        return (
            f"{self.__class__.__name__}("
            f"{self.req_schema.__class__.__name__}, "
            f"{self.rep_noerror_schema.__class__.__name__})"
        )

    def __init__(self, req_schema_cls, rep_schema_cls):
        self.rep_noerror_schema = rep_schema_cls()

        class RepWithErrorSchema(OneOfSchema):
            type_field = "status"
            type_field_remove = False
            fallback_type_schema = ErrorRepSchema
            type_schemas = {"ok": self.rep_noerror_schema}

            def get_obj_type(self, obj):
                try:
                    return obj["status"]
                except (TypeError, KeyError):
                    return "ok"

        RepWithErrorSchema.__name__ = f"ErrorOr{rep_schema_cls.__name__}"

        self.req_schema = req_schema_cls(strict=True)
        self.rep_schema = RepWithErrorSchema(strict=True)

    def req_load(self, data):
        """
        Raises:
            ValidationError
        """
        return self.req_schema.load(data).data

    def req_dump(self, data):
        """
        Raises:
            ValidationError
        """
        return self.req_schema.dump(data).data

    def rep_load(self, data):
        """
        Raises:
            ValidationError
        """
        return self.rep_schema.load(data).data

    def rep_dump(self, data):
        """
        Raises:
            ValidationError
        """
        return self.rep_schema.dump(data).data


__all__ = ("ValidationError", "BaseReqSchema", "BaseRepSchema")
