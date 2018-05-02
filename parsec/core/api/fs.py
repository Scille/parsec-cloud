from parsec.networking import ClientContext
from parsec.core.app import Core
from parsec.core.fs import BaseFolderEntry, BaseFileEntry
from parsec.core.backend_connection import BackendNotAvailable
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
    dirpath, filename = req["path"].rsplit("/", 1)
    parent = await core.fs.fetch_path(dirpath or "/")
    if not isinstance(parent, BaseFolderEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a directory" % parent.path}

    new_file = await parent.create_file(filename)
    await new_file.flush()
    await parent.flush()
    return {"status": "ok"}


async def file_read(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_FILE_READ_Schema().load_or_abort(req)
    file = await core.fs.fetch_path(req["path"])
    if not isinstance(file, BaseFileEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a file" % file.path}

    content = await file.read(req["size"], req["offset"])
    return {"status": "ok", "content": to_jsonb64(content)}


async def file_write(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_FILE_WRITE_Schema().load_or_abort(req)
    file = await core.fs.fetch_path(req["path"])
    if not isinstance(file, BaseFileEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a file" % file.path}

    await file.write(req["content"], req["offset"])
    return {"status": "ok"}


async def file_truncate(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_FILE_TRUNCATE_Schema().load_or_abort(req)
    file = await core.fs.fetch_path(req["path"])
    if not isinstance(file, BaseFileEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a file" % file.path}

    await file.truncate(req["length"])
    return {"status": "ok"}


async def stat(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    obj = await core.fs.fetch_path(req["path"])
    if isinstance(obj, BaseFolderEntry):
        return {
            "status": "ok",
            "type": "folder",
            "created": obj.created.isoformat(),
            "updated": obj.updated.isoformat(),
            "base_version": obj.base_version,
            "is_placeholder": obj.is_placeholder,
            "need_sync": obj.need_sync,
            "need_flush": obj.need_flush,
            "children": list(sorted(obj.keys())),
        }

    else:
        return {
            "status": "ok",
            "type": "file",
            "created": obj.created.isoformat(),
            "updated": obj.updated.isoformat(),
            "base_version": obj.base_version,
            "is_placeholder": obj.is_placeholder,
            "need_sync": obj.need_sync,
            "need_flush": obj.need_flush,
            "size": obj.size,
        }


async def folder_create(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    dirpath, name = req["path"].rsplit("/", 1)
    parent = await core.fs.fetch_path(dirpath or "/")
    if not isinstance(parent, BaseFolderEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a directory" % parent.path}

    child = await parent.create_folder(name)
    await child.flush()
    await parent.flush()
    return {"status": "ok"}


async def move(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_MOVE_Schema().load_or_abort(req)
    if req["src"] == "/":
        return {"status": "invalid_path", "reason": "Cannot move `/` root folder"}

    if req["dst"] == "/":
        return {"status": "invalid_path", "reason": "Path `/` already exists"}

    srcdirpath, srcfilename = req["src"].rsplit("/", 1)
    dstdirpath, dstfilename = req["dst"].rsplit("/", 1)

    src = _normalize_path(req["src"])
    dst = _normalize_path(req["dst"])
    if src == dst:
        return {"status": "invalid_path", "reason": "Cannot move `%s` to itself" % src}

    if dst.startswith(src + "/"):
        return {
            "status": "invalid_path", "reason": "Cannot move `%s` to a subdirectory of itself" % src
        }

    srcparent = await core.fs.fetch_path(srcdirpath or "/")
    dstparent = await core.fs.fetch_path(dstdirpath or "/")

    if not isinstance(srcparent, BaseFolderEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a directory" % srcparent.path}

    if not isinstance(dstparent, BaseFolderEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a directory" % dstparent.path}

    if srcfilename not in srcparent:
        return {"status": "invalid_path", "reason": "Path `%s` doesn't exists" % req["src"]}

    if dstfilename in dstparent:
        return {"status": "invalid_path", "reason": "Path `%s` already exists" % req["dst"]}

    obj = await srcparent.delete_child(srcfilename)
    await dstparent.insert_child(dstfilename, obj)

    await dstparent.flush()
    if srcparent != dstparent:
        await srcparent.flush()
    return {"status": "ok"}


async def delete(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    if req["path"] == "/":
        return {"status": "invalid_path", "reason": "Cannot remove `/` root folder"}

    req = PathOnlySchema().load_or_abort(req)
    dirpath, name = req["path"].rsplit("/", 1)
    parent = await core.fs.fetch_path(dirpath or "/")
    if not isinstance(parent, BaseFolderEntry):
        return {"status": "invalid_path", "reason": "Path `%s` is not a directory" % parent.path}

    await parent.delete_child(name)
    await parent.flush()
    return {"status": "ok"}


async def flush(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    obj = await core.fs.fetch_path(req["path"])
    await obj.flush()
    return {"status": "ok"}


async def synchronize(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.fs:
        return {"status": "login_required", "reason": "Login required"}

    req = PathOnlySchema().load_or_abort(req)
    to_sync_target = await core.fs.fetch_path(req["path"])
    # If the path to the target contains placeholders, we must synchronize
    # them here
    to_sync = [to_sync_target]
    curr_path = req["path"]
    while to_sync[-1].is_placeholder:
        curr_path, _ = curr_path.rsplit("/", 1)
        if not curr_path:
            curr_path = "/"
        to_sync.append(await core.fs.fetch_path(curr_path))
    await to_sync_target.sync(recursive=True)
    to_sync_parents = to_sync[1:]
    # TODO: If parent contains placeholders than what compose the path to
    # the target, there will be synchronized as empty files/folders.
    # It would be better (and faster) to skip them entirely.
    for to_sync_parent in to_sync_parents:
        await to_sync_parent.sync(ignore_placeholders=True)
    return {"status": "ok"}
