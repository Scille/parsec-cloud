import attr
from effect2 import ComposedDispatcher

from parsec.base import base_dispatcher
from parsec.backend.backend_api import register_backend_api
from parsec.backend.start_api import register_start_api


@attr.s
class BackendComponents:
    message = attr.ib()
    group = attr.ib()
    uservlob = attr.ib()
    vlob = attr.ib()
    pubkey = attr.ib()
    privkey = attr.ib()

    def get_dispatcher(self):
        return ComposedDispatcher([
            base_dispatcher,
            self.message.get_dispatcher(),
            self.group.get_dispatcher(),
            self.uservlob.get_dispatcher(),
            self.vlob.get_dispatcher(),
            self.pubkey.get_dispatcher(),
            self.privkey.get_dispatcher(),
        ])


def mocked_components_factory(app):
    from parsec.backend.pubkey import MockedPubKeyComponent
    from parsec.backend.privkey import MockedPrivKeyComponent
    from parsec.backend.vlob import MockedVlobComponent
    from parsec.backend.user_vlob import MockedUserVlobComponent
    from parsec.backend.message import InMemoryMessageComponent
    from parsec.backend.group import MockedGroupComponent
    return BackendComponents(
        message=InMemoryMessageComponent(),
        group=MockedGroupComponent(),
        uservlob=MockedUserVlobComponent(),
        vlob=MockedVlobComponent(),
        pubkey=MockedPubKeyComponent(),
        privkey=MockedPrivKeyComponent()
    )

def postgresql_components_factory(app, store):
    from parsec.backend import postgresql

    conn = postgresql.PostgreSQLConnection(store)

    async def on_startup(app):
        await conn.open_connection()
    app.on_startup.append(on_startup)

    async def on_shutdown(app):
        await conn.close_connection()
    app.on_shutdown.append(on_shutdown)

    return BackendComponents(
        message=postgresql.PostgreSQLMessageComponent(conn),
        group=postgresql.PostgreSQLGroupComponent(conn),
        uservlob=postgresql.PostgreSQLUserVlobComponent(conn),
        vlob=postgresql.PostgreSQLVlobComponent(conn),
        pubkey=postgresql.PostgreSQLPubKeyComponent(conn),
        privkey=postgresql.PostgreSQLPrivKeyComponent(conn)
    )


__all__ = ('register_backend_api', 'register_start_api',
           'postgresql_components_factory', 'mocked_components_factory')
