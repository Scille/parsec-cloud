from marshmallow import Schema, ValidationError, validates_schema, post_load
from marshmallow import validate  # noqa: republishing
from marshmallow_oneofschema import OneOfSchema  # noqa: republishing

from parsec import schema_fields as fields  # noqa: republishing
from parsec.utils import abort


class _UnknownCheckedSchema(Schema):

    """
    ModelSchema with check for unknown field
    """

    @validates_schema(pass_original=True)
    def check_unknown_fields(self, data, original_data):
        for key in original_data:
            if key not in self.fields or self.fields[key].dump_only:
                raise ValidationError("Unknown field name {}".format(key))


class InvalidCmd(Exception):
    pass


class _BaseCmdSchema(_UnknownCheckedSchema):
    cmd = fields.String(required=True)

    @post_load
    def _drop_cmd_field(self, item):
        if self.drop_cmd_field:
            item.pop("cmd")
        return item

    def __init__(self, drop_cmd_field=True, **kwargs):
        super().__init__(**kwargs)
        self.drop_cmd_field = drop_cmd_field

    def load(self, msg):
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


BaseCmdSchema = _BaseCmdSchema()
UnknownCheckedSchema = _UnknownCheckedSchema()
