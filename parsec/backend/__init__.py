import attr
from effect2 import ComposedDispatcher

from parsec.base import base_dispatcher
from parsec.backend.backend_api import register_backend_api
from parsec.backend.privkey import register_privkey_api


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


def mocked_components_factory():
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

def postgresql_components_factory(store):
    from parsec.backend import postgresql
    conn = loop.run_until_complete(postgresql.postgresql_connection_factory(store))
    return BackendComponents(
        message=postgresql.PostgreSQLMessageComponent(conn),
        group=postgresql.PostgreSQLGroupComponent(conn),
        uservlob=postgresql.PostgreSQLUserVlobComponent(conn),
        vlob=postgresql.PostgreSQLVlobComponent(conn),
        pubkey=postgresql.PostgreSQLPubKeyComponent(conn),
        privkey=postgresql.PostgreSQLPrivKeyComponent(conn)
    )

__all__ = ('register_backend_api', 'register_privkey_api',
           'postgresql_components_factory', 'mocked_components_factory')
