from marshmallow import Schema, fields, validates_schema, ValidationError

from parsec.exceptions import BadMessageError


class UnknownCheckedSchema(Schema):

    """
    ModelSchema with check for unknown field
    """

    @validates_schema(pass_original=True)
    def check_unknown_fields(self, data, original_data):
        for key in original_data:
            if key not in self.fields or self.fields[key].dump_only:
                raise ValidationError('Unknown field name {}'.format(key))


class BaseCmdSchema(UnknownCheckedSchema):
    cmd = fields.String(required=True)

    def load(self, msg):
        parsed_msg, errors = super().load(msg)
        if errors:
            raise BadMessageError(errors)
        return parsed_msg
