# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import trio
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    Bundle,
    initialize,
    rule,
    invariant,
    run_state_machine_as_test,
    TrioAsyncioRuleBasedStateMachine,
    multiple,
)
from unittest.mock import ANY

from parsec._parsec import (
    RealmUpdateRolesRepOk,
    RealmUpdateRolesRepAlreadyGranted,
    RealmUpdateRolesRepNotAllowed,
    RealmUpdateRolesRepInvalidData,
    RealmGetRoleCertificatesRepOk,
)

from parsec.backend.asgi import app_factory
from parsec.api.data import RealmRoleCertificate
from parsec.api.protocol import RealmRole

from tests.common import call_with_control
from tests.backend.common import realm_get_role_certificates, realm_update_roles


@pytest.mark.slow
def test_shuffle_roles(
    hypothesis_settings,
    reset_testbed,
    backend_factory,
    backend_data_binder_factory,
    backend_authenticated_ws_factory,
    local_device_factory,
    realm_factory,
    coolorg,
    next_timestamp,
):
    class ShuffleRoles(TrioAsyncioRuleBasedStateMachine):
        realm_role_strategy = st.one_of(st.just(x) for x in RealmRole.values())
        User = Bundle("user")

        async def start_backend(self):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory(populated=False) as backend:
                    await started_cb(backend=backend)

            return await self.get_root_nursery().start(call_with_control, _backend_controlled_cb)

        @property
        def backend_asgi_app(self):
            return self._backend_asgi_app

        @initialize(target=User)
        async def init(self):
            await reset_testbed()
            self.backend_controller = await self.start_backend()
            self._backend_asgi_app = app_factory(self.backend_controller.backend)
            self.org = coolorg
            device = local_device_factory(org=self.org)

            # Create organization and first user
            self.backend_data_binder = backend_data_binder_factory(self.backend_asgi_app.backend)
            await self.backend_data_binder.bind_organization(self.org, device)

            # Create realm
            self.realm_id = await realm_factory(self.backend_asgi_app.backend, device)
            self.current_roles = {device.user_id: RealmRole.OWNER}
            self.certifs = [ANY]

            self.wss = {}
            return device

        async def get_ws(self, device):
            try:
                return self.wss[device.user_id]
            except KeyError:
                pass

            async def _start_ws(device, *, task_status=trio.TASK_STATUS_IGNORED):
                async with backend_authenticated_ws_factory(self.backend_asgi_app, device) as ws:
                    task_status.started(ws)
                    await trio.sleep_forever()

            ws = await self.get_root_nursery().start(_start_ws, device)
            self.wss[device.user_id] = ws
            return ws

        @rule(target=User, author=User, role=realm_role_strategy)
        async def give_role_to_new_user(self, author, role):
            # Create new user/device
            new_device = local_device_factory(org=self.org)
            await self.backend_data_binder.bind_device(new_device)
            self.current_roles[new_device.user_id] = None
            # Assign role
            author_ws = await self.get_ws(author)
            if await self._give_role(author_ws, author, new_device, role):
                return new_device
            else:
                return multiple()

        @rule(author=User, recipient=User, role=realm_role_strategy)
        async def change_role_for_existing_user(self, author, recipient, role):
            author_ws = await self.get_ws(author)
            await self._give_role(author_ws, author, recipient, role)

        async def _give_role(self, author_ws, author, recipient, role):
            author_ws = await self.get_ws(author)

            certif = RealmRoleCertificate(
                author=author.device_id,
                timestamp=next_timestamp(),
                realm_id=self.realm_id,
                user_id=recipient.user_id,
                role=role,
            ).dump_and_sign(author.signing_key)
            rep = await realm_update_roles(author_ws, certif, check_rep=False)
            if author.user_id == recipient.user_id:
                # The reason is no longer generated
                assert isinstance(rep, RealmUpdateRolesRepInvalidData)

            else:
                owner_only = (RealmRole.OWNER,)
                owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)
                existing_recipient_role = self.current_roles[recipient.user_id]
                if existing_recipient_role in owner_or_manager or role in owner_or_manager:
                    allowed_roles = owner_only
                else:
                    allowed_roles = owner_or_manager

                if self.current_roles[author.user_id] in allowed_roles:
                    # print(f"+ {author.user_id} -{role.value}-> {recipient.user_id}")
                    if existing_recipient_role != role:
                        assert isinstance(rep, RealmUpdateRolesRepOk)
                        self.current_roles[recipient.user_id] = role
                        self.certifs.append(certif)
                    else:
                        assert isinstance(rep, RealmUpdateRolesRepAlreadyGranted)
                else:
                    # print(f"- {author.user_id} -{role.value}-> {recipient.user_id}")
                    assert isinstance(rep, RealmUpdateRolesRepNotAllowed)

            return isinstance(rep, RealmUpdateRolesRepOk)

        @rule(author=User)
        async def get_role_certificates(self, author):
            ws = await self.get_ws(author)
            rep = await realm_get_role_certificates(ws, self.realm_id)
            if self.current_roles.get(author.user_id) is not None:
                try:
                    assert rep == RealmGetRoleCertificatesRepOk(certificates=self.certifs)
                except TypeError:
                    assert isinstance(rep, RealmGetRoleCertificatesRepOk)
                except:
                    assert False
            else:
                assert rep == {}

        @invariant()
        async def check_current_roles(self):
            try:
                backend = self.backend_agi_app.backend
            except AttributeError:
                return
            roles = await backend.realm.get_current_roles(self.org.organization_id, self.realm_id)
            assert roles == {k: v for k, v in self.current_roles.items() if v is not None}

    run_state_machine_as_test(ShuffleRoles, settings=hypothesis_settings)
