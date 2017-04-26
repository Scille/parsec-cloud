import asyncio
import sys

from marshmallow import Schema, fields, validates_schema, ValidationError
from logbook import Logger, StreamHandler

from parsec.exceptions import BadMessageError


# TODO: useful ?
LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name})' \
             ' {record.level_name}: {record.channel}: {record.message}'
logger = Logger('Parsec')
StreamHandler(sys.stdout, format_string=LOG_FORMAT).push_application()


def event_handler(callback, sender):
    loop = asyncio.get_event_loop()
    loop.call_soon(asyncio.ensure_future, callback())


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
