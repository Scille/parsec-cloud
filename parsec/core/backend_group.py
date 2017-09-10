import attr
from effect2 import Effect

from parsec.exceptions import exception_from_status
from parsec.core.backend import BackendCmd


@attr.s
class EBackendGroupCreate:
    name = attr.ib()


@attr.s
class EBackendGroupRead:
    name = attr.ib()


@attr.s
class EBackendGroupAddIdentities:
    name = attr.ib()
    identities = attr.ib()
    admin = attr.ib(default=False)


@attr.s
class EBackendGroupRemoveIdentities:
    name = attr.ib()
    identities = attr.ib()
    admin = attr.ib(default=False)


@attr.s
class Group:
    name = attr.ib()
    admins = attr.ib()
    users = attr.ib()


async def perform_group_create(intent):
    msg = {'name': intent.name}
    ret = await Effect(BackendCmd('group_create', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])


async def perform_group_read(intent):
    msg = {'name': intent.name}
    ret = await Effect(BackendCmd('group_read', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return Group(intent.name, ret['admins'], ret['users'])


async def perform_group_add_identities(intent):
    msg = {'name': intent.name, 'identities': intent.identities, 'admin': intent.admin}
    ret = await Effect(BackendCmd('group_add_identities', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])


async def perform_group_remove_identities(intent):
    msg = {'name': intent.name, 'identities': intent.identities, 'admin': intent.admin}
    ret = await Effect(BackendCmd('group_remove_identities', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
