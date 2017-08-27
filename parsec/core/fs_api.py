from marshmallow import fields, validate
from effect2 import Effect, do

from parsec.core.fs import (
    EFSFileCreate, EFSFileRead, EFSFileWrite, EFSFileTruncate, EFSFolderCreate,
    EFSStat, EFSMove, EFSDelete
)
from parsec.tools import UnknownCheckedSchema


class PathOnlySchema(UnknownCheckedSchema):
    path = fields.String(required=True)


# class cmd_CREATE_GROUP_MANIFEST_Schema(UnknownCheckedSchema):
#     group = fields.String()


# class cmd_SHOW_dustbin_Schema(UnknownCheckedSchema):
#     path = fields.String(missing=None)


# class cmd_HISTORY_Schema(UnknownCheckedSchema):
#     first_version = fields.Integer(missing=1, validate=lambda n: n >= 1)
#     last_version = fields.Integer(missing=None, validate=lambda n: n >= 1)
#     summary = fields.Boolean(missing=False)


# class cmd_RESTORE_MANIFEST_Schema(UnknownCheckedSchema):
#     version = fields.Integer(missing=None, validate=lambda n: n >= 1)


class cmd_FILE_READ_Schema(UnknownCheckedSchema):
    path = fields.String(required=True)
    offset = fields.Int(missing=0, validate=validate.Range(min=0))
    size = fields.Int(missing=None, validate=validate.Range(min=0))


class cmd_FILE_WRITE_Schema(UnknownCheckedSchema):
    path = fields.String(required=True)
    offset = fields.Int(missing=0, validate=validate.Range(min=0))
    content = fields.Base64Bytes(required=True)


class cmd_FILE_TRUNCATE_Schema(UnknownCheckedSchema):
    path = fields.String(required=True)
    length = fields.Int(required=True, validate=validate.Range(min=0))


# class cmd_FILE_HISTORY_Schema(UnknownCheckedSchema):
#     path = fields.String(required=True)
#     first_version = fields.Int(missing=1, validate=validate.Range(min=1))
#     last_version = fields.Int(missing=None, validate=validate.Range(min=1))


# class cmd_FILE_RESTORE_Schema(UnknownCheckedSchema):
#     path = fields.String(required=True)
#     version = fields.Int(required=True, validate=validate.Range(min=1))


class cmd_MOVE_Schema(UnknownCheckedSchema):
    src = fields.String(required=True)
    dst = fields.String(required=True)


# class cmd_UNDELETE_Schema(UnknownCheckedSchema):
#     vlob = fields.String(required=True)


# @do
# def api_synchronize(msg):
#     UnknownCheckedSchema().load(msg)
#     yield Effect(EFSSynchronize())
#     return {'status': 'ok'}


# @do
# def api_group_create(msg):
#     msg = cmd_CREATE_GROUP_MANIFEST_Schema().load(msg)
#     yield Effect(EFSGroupCreate(**msg))
#     return {'status': 'ok'}


# @do
# def api_dustbin_show(msg):
#     msg = cmd_SHOW_dustbin_Schema().load(msg)
#     dustbin = yield Effect(EFSDustbinShow(**msg))
#     return {'status': 'ok', 'dustbin': dustbin}


# @do
# def api_manifest_history(msg):
#     msg = cmd_HISTORY_Schema().load(msg)
#     history = yield Effect(EFSManifestHistory(**msg))
#     history['status'] = 'ok'
#     return history


# @do
# def api_manifest_restore(msg):
#     msg = cmd_RESTORE_MANIFEST_Schema().load(msg)
#     yield Effect(EFSManifestRestore(**msg))
#     return {'status': 'ok'}


@do
def api_file_create(msg):
    msg = PathOnlySchema().load(msg)
    yield Effect(EFSFileCreate(**msg))
    return {'status': 'ok'}


@do
def api_file_read(msg):
    msg = cmd_FILE_READ_Schema().load(msg)
    content = yield Effect(EFSFileRead(**msg))
    return {'status': 'ok', 'content': content}


@do
def api_file_write(msg):
    msg = cmd_FILE_WRITE_Schema().load(msg)
    yield Effect(EFSFileWrite(**msg))
    return {'status': 'ok'}


@do
def api_file_truncate(msg):
    msg = cmd_FILE_TRUNCATE_Schema().load(msg)
    yield Effect(EFSFileTruncate(**msg))
    return {'status': 'ok'}


# @do
# def api_file_history(msg):
#     msg = cmd_FILE_HISTORY_Schema().load(msg)
#     history = yield Effect(EFSFileHistory(**msg))
#     history['status'] = 'ok'
#     return history


# @do
# def api_file_restore(msg):
#     msg = cmd_FILE_RESTORE_Schema().load(msg)
#     yield Effect(EFSFileRestore(**msg))
#     return {'status': 'ok'}


@do
def api_folder_create(msg):
    msg = PathOnlySchema().load(msg)
    yield Effect(EFSFolderCreate(**msg))
    return {'status': 'ok'}


@do
def api_stat(msg):
    msg = PathOnlySchema().load(msg)
    stat = yield Effect(EFSStat(**msg))
    stat['status'] = 'ok'
    return stat


@do
def api_move(msg):
    msg = cmd_MOVE_Schema().load(msg)
    yield Effect(EFSMove(**msg))
    return {'status': 'ok'}


@do
def api_delete(msg):
    msg = PathOnlySchema().load(msg)
    yield Effect(EFSDelete(**msg))
    return {'status': 'ok'}


# @do
# def api_undelete(msg):
#     msg = cmd_UNDELETE_Schema().load(msg)
#     yield Effect(EFSUndelete(**msg))
#     return {'status': 'ok'}
