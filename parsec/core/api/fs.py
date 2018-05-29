from parsec.core.app import Core, ClientContext

from parsec.core.fs import FSInvalidPath
from parsec.utils import to_jsonb64
from parsec.schema import BaseCmdSchema, fields, validate

# TODO: catch BackendNotAvailable in api functions


class PathOnlySchema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_CREATE_GROUP_MANIFEST_Schema(BaseCmdSchema):
    group = fields.String()


class cmd_SHOW_dustbin_Schema(BaseCmdSchema):
    path = fields.String(missing=None)


class cmd_HISTORY_Schema(BaseCmdSchema):
    first_version = fields.Integer(missing=1, validate=lambda n: n >= 1)
    last_version = fields.Integer(missing=None, validate=lambda n: n >= 1)
    summary = fields.Boolean(missing=False)


class cmd_RESTORE_MANIFEST_Schema(BaseCmdSchema):
    version = fields.Integer(missing=None, validate=lambda n: n >= 1)


class cmd_FILE_READ_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    offset = fields.Integer(missing=0, validate=validate.Range(min=0))
    size = fields.Integer(missing=None, validate=validate.Range(min=0))


class cmd_FILE_WRITE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    offset = fields.Integer(missing=0, validate=validate.Range(min=0))
    content = fields.Base64Bytes(required=True, validate=validate.Length(min=0))


class cmd_FILE_TRUNCATE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    length = fields.Integer(required=True, validate=validate.Range(min=0))


class cmd_FILE_HISTORY_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    first_version = fields.Integer(missing=1, validate=validate.Range(min=1))
    last_version = fields.Integer(missing=None, validate=validate.Range(min=1))


class cmd_FILE_RESTORE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=1))


class cmd_MOVE_Schema(BaseCmdSchema):
    src = fields.String(required=True)
    dst = fields.String(required=True)


class cmd_UNDELETE_Schema(BaseCmdSchema):
    vlob = fields.String(required=True)


def _normalize_path(path):
    return "/" + "/".join([x for x in path.split("/") if x])


async def file_create(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    try:
        await core.fs.file_create(req["path"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}


async def file_read(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_FILE_READ_Schema().load_or_abort(req)
    try:
        content = await core.fs.file_read(req["path"], req["size"], req["offset"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok", "content": to_jsonb64(content)}


async def file_write(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_FILE_WRITE_Schema().load_or_abort(req)
    try:
        await core.fs.file_write(req["path"], req["content"], req["offset"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}


async def file_truncate(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_FILE_TRUNCATE_Schema().load_or_abort(req)
    try:
        await core.fs.file_truncate(req["path"], req["length"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}


async def stat(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    try:
        stat = await core.fs.file_truncate(req["path"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok", **stat}


async def folder_create(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    try:
        await core.fs.folder_create(req["path"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}


async def move(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_MOVE_Schema().load_or_abort(req)
    try:
        await core.fs.move(req["src"], req["dst"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}


async def delete(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    if req["path"] == "/":
        return {"status": "invalid_path", "reason": "Cannot remove `/` root folder"}

    req = PathOnlySchema().load_or_abort(req)
    try:
        await core.fs.delete(req["path"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}


async def flush(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    try:
        await core.fs.file_flush(req["path"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}


async def synchronize(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    try:
        await core.fs.sync(req["path"])
    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    return {"status": "ok"}
