from parsec.schema import (
    fields,
    ValidationError,
    UnknownCheckedSchema,
    OneOfSchema,
    InvalidCmd,
    post_load,
    abort,
)


class APIValidationError(Exception):
    pass


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

    def load(self, msg, **kwargs):
        # TODO: big hack to work around cmd_EVENT_SUBSCRIBE_Schema using OneOfSchema
        if kwargs:
            return super().load(msg)

        parsed_msg, errors = super().load(msg)
        if errors:
            raise InvalidCmd(errors)
        return parsed_msg

    # TODO: remove this and use the load instead
    def load_or_abort(self, msg):
        parsed_msg, errors = super().load(msg)
        if errors:
            raise abort(errors=errors)

        else:
            return parsed_msg


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


__all__ = ("APIValidationError", "BaseReqSchema", "BaseRepSchema")
