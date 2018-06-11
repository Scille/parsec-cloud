from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.group import BaseGroupComponent


class PGGroupComponent(BaseGroupComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def perform_group_create(self, name):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                group = await conn.execute("SELECT 1 FROM groups WHERE name = $1", name)

                if group:
                    raise AlreadyExistsError("Group Already exists.")

                await conn.execute("INSERT INTO groups (name) VALUES ($1)", name)

    async def perform_group_read(self, name):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                group = await conn.fetchrow("SELECT id, name FROM groups WHERE name = $1", name)

                if group is None:
                    raise NotFoundError("Group not found.")

                identities = await conn.fetch(
                    "SELECT name, admin FROM group_identities WHERE group_id = $1", group["id"]
                )

        group = Group(group["name"])

        for identity in identities:
            if identity["admin"]:
                group.admins |= identity["name"]

            else:
                group.users |= identity["name"]

        return group

    async def perform_group_add_identities(self, name, identities, admin):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                group = await conn.fetchrow("SELECT id, name FROM groups WHERE name = $1", name)

                if group is None:
                    raise NotFoundError("Group not found.")

                await conn.executemany(
                    """INSERT INTO group_identities (group_id, name, admin) VALUES ($1, $2, $3)
                    ON CONFLICT DO NOTHING
                    """,
                    [(group["id"], identity, admin) for identity in identities],
                )

    # TODO: send message to added user

    async def perform_group_remove_identities(self, name, identities, admin):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                group = await conn.fetchrow("SELECT id, name FROM groups WHERE name = $1", name)

                if group is None:
                    raise NotFoundError("Group not found.")

                await conn.executemany(
                    "DELETE FROM group_identities WHERE group_id = $1 AND name = $2 AND admin = $3",
                    [(group["id"], identity, admin) for identity in identities],
                )


# TODO: send message to removed user
