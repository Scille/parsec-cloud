from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.group import BaseGroupComponent, Group


class MemoryGroupComponent(BaseGroupComponent):

    def __init__(self, *args):
        super().__init__(*args)
        self._groups = {}

    async def perform_group_create(self, name):
        if name in self._groups:
            raise AlreadyExistsError("Group Already exists.")

        self._groups[name] = Group(name)

    async def perform_group_read(self, name):
        try:
            return self._groups[name]

        except KeyError:
            raise NotFoundError("Group not found.")

    async def perform_group_add_identities(self, name, identities, admin):
        try:
            group = self._groups[name]
        except KeyError:
            raise NotFoundError("Group not found.")

        if admin:
            group.admins |= identities
        else:
            group.users |= identities

    # TODO: send message to added user

    async def perform_group_remove_identities(self, name, identities, admin):
        try:
            group = self._groups[name]
        except KeyError:
            raise NotFoundError("Group not found.")

        if admin:
            group.admins -= identities
        else:
            group.users -= identities


# TODO: send message to removed user
