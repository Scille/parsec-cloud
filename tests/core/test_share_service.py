# import copy
# import json
# from os import path

# import gnupg
# import pytest

# from parsec.server import BaseServer
# from parsec.core import (CryptoService, FileService, GNUPGPubKeysService, IdentityService,
#                          MetaBlockService, MockedBackendAPIService, MockedBlockService,
#                          MockedCacheService, ShareService, UserManifestService)


# GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


# @pytest.fixture
# def backend_api_svc():
#     return MockedBackendAPIService()


# @pytest.fixture
# def crypto_svc():
#     service = CryptoService()
#     service.gnupg = gnupg.GPG(homedir=GNUPG_HOME + '/secret_env')
#     return service


# @pytest.fixture
# def user_manifest_svc():
#     return UserManifestService()


# @pytest.fixture(params=[ShareService, ])
# def share_svc(request, event_loop, backend_api_svc, crypto_svc, user_manifest_svc):
#     identity = '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'
#     service = request.param()
#     block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
#     identity_service = IdentityService()
#     server = BaseServer()
#     server.register_service(service)
#     server.register_service(block_service)
#     server.register_service(crypto_svc)
#     server.register_service(identity_service)
#     server.register_service(user_manifest_svc)
#     server.register_service(backend_api_svc)
#     server.register_service(FileService())
#     server.register_service(GNUPGPubKeysService())
#     server.register_service(MockedCacheService())
#     event_loop.run_until_complete(server.bootstrap_services())
#     event_loop.run_until_complete(identity_service.load_identity(identity=identity))
#     event_loop.run_until_complete(user_manifest_svc.load_user_manifest())
#     event_loop.run_until_complete(user_manifest_svc.create_file('/foo'))
#     yield service
#     event_loop.run_until_complete(server.teardown_services())


# @pytest.fixture
# def group(share_svc, event_loop, crypto_svc, name='foo_communiy'):
#     event_loop.run_until_complete(share_svc.group_create(name=name))
#     identities = [key['fingerprint'] for key in crypto_svc.gnupg.list_keys(secret=True)]
#     identities = identities[-2:]
#     result = event_loop.run_until_complete(share_svc.group_add_identities(name=name,
#                                                                           identities=identities))
#     result = event_loop.run_until_complete(share_svc.group_read(name=name))
#     result.update({'name': name})
#     return result


# class TestShareServiceAPI:

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/unknown', 'identity': '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'}])
#     async def test_share_with_identity_and_file_not_found(self, share_svc, payload):
#         ret = await share_svc.dispatch_msg({'cmd': 'share_with_identity', **payload})
#         assert ret == {'status': 'not_found', 'label': 'File not found.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [{'path': '/foo', 'identity': 'unknown'}])
#     async def test_share_with_identity_and_identity_not_found(self, share_svc, payload):
#         ret = await share_svc.dispatch_msg({'cmd': 'share_with_identity', **payload})
#         assert ret == {'status': 'error', 'label': 'Encryption failure.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/foo', 'identity': '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'}])
#     async def test_share_with_identity(self,
#                                        backend_api_svc,
#                                        share_svc,
#                                        crypto_svc,
#                                        user_manifest_svc,
#                                        payload):
#         shared_vlob = await user_manifest_svc.get_properties(path=payload['path'])
#         ret = await share_svc.dispatch_msg({'cmd': 'share_with_identity', **payload})
#         assert ret == {'status': 'ok'}
#         messages = await backend_api_svc.message_get(payload['identity'])
#         encrypted_vlob = messages[-1]
#         received_vlob = await crypto_svc.asym_decrypt(encrypted_vlob)
#         received_vlob = json.loads(received_vlob.decode())
#         assert shared_vlob == received_vlob

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'share_with_identity', 'path': '/foo', 'group': 'foo'},
#         {'cmd': 'share_with_identity', 'path': '/foo', 'identity': 'foo', 'bad_field': 'foo'},
#         {'cmd': 'share_with_identity', 'path': '/foo', 'identity': ['foo']},
#         {'cmd': 'share_with_identity', 'path': '/foo', 'identity': 42},
#         {'cmd': 'share_with_identity', 'path': '/foo', 'identity': None},
#         {'cmd': 'share_with_identity', 'path': 42, 'identity': 'foo'},
#         {'cmd': 'share_with_identity', 'path': None, 'identity': 'foo'},
#         {}])
#     async def test_bad_msg_share_with_identity(self, share_svc, bad_msg):
#         ret = await share_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/unknown', 'group': '<group-here>'}])
#     async def test_share_with_group_and_file_not_found(self, share_svc, payload, group):
#         if payload.get('group') == '<group-here>':
#             payload['group'] = group['name']
#         ret = await share_svc.dispatch_msg({'cmd': 'share_with_group', **payload})
#         assert ret == {'status': 'not_found', 'label': 'File not found.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/foo', 'group': 'unknown'}])
#     async def test_share_with_group_and_identity_not_found(self, share_svc, payload, group):
#         ret = await share_svc.dispatch_msg({'cmd': 'share_with_group', **payload})
#         assert ret == {'status': 'not_found', 'label': 'Group not found.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/foo', 'group': '<group-here>'}])
#     async def test_share_with_group(self,
#                                     backend_api_svc,
#                                     share_svc,
#                                     crypto_svc,
#                                     user_manifest_svc,
#                                     payload,
#                                     group):
#         if payload.get('group') == '<group-here>':
#             payload['group'] = group['name']
#         shared_vlob = await user_manifest_svc.get_properties(path=payload['path'])
#         ret = await share_svc.dispatch_msg({'cmd': 'share_with_group', **payload})
#         assert ret == {'status': 'ok'}
#         for identity in group['users']:  # TODO try with more users?
#             messages = await backend_api_svc.message_get(identity)
#             encrypted_vlob = messages[-1]
#             received_vlob = await crypto_svc.asym_decrypt(encrypted_vlob)
#             received_vlob = json.loads(received_vlob.decode())
#             assert shared_vlob == received_vlob

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'share_with_group', 'path': '/foo', 'identity': 'foo'},
#         {'cmd': 'share_with_group', 'path': '/foo', 'group': 'foo', 'bad_field': 'foo'},
#         {'cmd': 'share_with_group', 'path': '/foo', 'group': ['foo']},
#         {'cmd': 'share_with_group', 'path': '/foo', 'group': 42},
#         {'cmd': 'share_with_group', 'path': '/foo', 'group': None},
#         {'cmd': 'share_with_group', 'path': 42, 'group': 'foo'},
#         {'cmd': 'share_with_group', 'path': None, 'group': 'foo'},
#         {}])
#     async def test_bad_msg_share_with_group(self, share_svc, bad_msg):
#         ret = await share_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.asyncio
#     async def test_share_stop(self, backend_api_svc, share_svc, user_manifest_svc):
#         shared_vlob = await user_manifest_svc.get_properties(path='/foo')
#         ret = await share_svc.dispatch_msg({'cmd': 'share_stop', 'path': '/foo'})
#         assert ret == {'status': 'ok'}
#         new_vlob = await user_manifest_svc.get_properties(path='/foo')
#         for property in shared_vlob.keys():
#             assert shared_vlob[property] != new_vlob[property]
#         ret = await share_svc.dispatch_msg({'cmd': 'share_stop', 'path': '/unknown'})
#         assert ret == {'status': 'not_found', 'label': 'File not found.'}

#     @pytest.mark.asyncio
#     async def test_share_stop_not_found(self, share_svc):
#         ret = await share_svc.dispatch_msg({'cmd': 'share_stop', 'path': '/unknown'})
#         assert ret == {'status': 'not_found', 'label': 'File not found.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'share_stop', 'path': '/foo', 'bad_field': 'foo'},
#         {'cmd': 'share_stop', 'path': 42},
#         {'cmd': 'share_stop', 'path': None},
#         {}])
#     async def test_bad_msg_share_stop_with_group(self, share_svc, bad_msg):
#         ret = await share_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     # TODO Remove tests bellow as they are duplicated in test_group_service?

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('group_payload', [
#         {'name': 'foo_community'},
#         {'name': 'Foo community'}])
#     async def test_group_create(self, share_svc, group_payload):
#         # Working
#         ret = await share_svc.dispatch_msg({'cmd': 'group_create', **group_payload})
#         assert ret == {'status': 'ok'}
#         # Already exist
#         ret = await share_svc.dispatch_msg({'cmd': 'group_create', **group_payload})
#         assert ret == {'status': 'already_exist', 'label': 'Group already exist.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'group_create', 'name': 'foo', 'bad_field': 'foo'},
#         {'cmd': 'group_create', 'name': 42},
#         {'cmd': 'group_create', 'name': None},
#         {}])
#     async def test_bad_msg_group_create(self, share_svc, bad_msg):
#         ret = await share_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.asyncio
#     async def test_group_read_not_found(self, share_svc, group):
#         ret = await share_svc.dispatch_msg({'cmd': 'group_read', 'name': 'unknown'})
#         assert ret == {'status': 'not_found', 'label': 'Group not found.'}

#     @pytest.mark.asyncio
#     async def test_group_read(self, share_svc, group):
#         ret = await share_svc.dispatch_msg({'cmd': 'group_read', 'name': group['name']})
#         assert ret == {'status': 'ok', 'admins': group['admins'], 'users': group['users']}

#     @pytest.mark.asyncio
#     async def test_group_add_identities_not_found(self, share_svc, group):
#         ret = await share_svc.dispatch_msg({'cmd': 'group_add_identities',
#                                             'name': 'unknown',
#                                             'identities': group['users']})
#         assert ret == {'status': 'not_found', 'label': 'Group not found.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('admin', [True, False])
#     @pytest.mark.parametrize('identities', [
#         [],
#         ['81DBCF6EB9C8B2965A65ACE5520D903047D69DC9', '3C3FA85FB9736362497EB23DC0485AC10E6274C7']])
#     async def test_group_add_identities(self,
#                                         backend_api_svc,
#                                         crypto_svc,
#                                         share_svc,
#                                         user_manifest_svc,
#                                         group,
#                                         identities,
#                                         admin):
#         origin_group = copy.deepcopy(group)
#         # Adding duplicates identities should not raise errors
#         for i in range(0, 2):
#             ret = await share_svc.dispatch_msg({'cmd': 'group_add_identities',
#                                                 'name': origin_group['name'],
#                                                 'identities': identities,
#                                                 'admin': admin})
#             ret = await share_svc.dispatch_msg({'cmd': 'group_read', 'name': origin_group['name']})
#             ret['users'] = sorted(ret['users'])
#             ret['admins'] = sorted(ret['admins'])
#             if admin:
#                 assert ret == {'status': 'ok',
#                                'admins': sorted(identities + origin_group['admins']),
#                                'users': sorted(origin_group['users'])}
#             else:
#                 assert ret == {'status': 'ok',
#                                'admins': sorted(origin_group['admins']),
#                                'users': sorted(identities + origin_group['users'])}
#             group_manifest = await user_manifest_svc.get_manifest(origin_group['name'])
#             shared_vlob = await group_manifest.get_vlob()
#             for identity in identities:  # TODO try with more users?
#                 messages = await backend_api_svc.message_get(identity)
#                 encrypted_vlob = messages[-1]
#                 received_vlob = await crypto_svc.asym_decrypt(encrypted_vlob)
#                 received_vlob = json.loads(received_vlob.decode())
#                 assert shared_vlob == received_vlob['vlob']

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': ['id'],
#          'bad_field': 'foo'},
#         {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': ['id'],
#          'admin': 42},
#         {'cmd': 'group_add_identities', 'name': 42, 'identities': ['id']},
#         {'cmd': 'group_add_identities', 'name': None, 'identities': ['id']},
#         {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': 'id'},
#         {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': 42},
#         {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': None},
#         {'cmd': 'group_add_identities', 'identities': ['id']},
#         {'cmd': 'group_add_identities', 'name': '<name-here>'},
#         {'cmd': 'group_add_identities'}, {}])
#     async def test_bad_group_add_identities(self, share_svc, bad_msg, group):
#         if bad_msg.get('name') == '<name-here>':
#             bad_msg['name'] = group['name']
#         ret = await share_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.asyncio
#     async def test_gorup_remove_identities_not_found(self, share_svc, group):
#         ret = await share_svc.dispatch_msg({'cmd': 'group_remove_identities',
#                                             'name': 'unknown',
#                                             'identities': group['users']})
#         assert ret == {'status': 'not_found', 'label': 'Group not found.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('admin', [True, False])
#     @pytest.mark.parametrize('identities', [
#         [],
#         ['81DBCF6EB9C8B2965A65ACE5520D903047D69DC9', '3C3FA85FB9736362497EB23DC0485AC10E6274C7']])
#     async def test_group_remove_identities(self,
#                                            backend_api_svc,
#                                            crypto_svc,
#                                            share_svc,
#                                            user_manifest_svc,
#                                            group,
#                                            identities,
#                                            admin):
#         origin_group = copy.deepcopy(group)
#         # Identities that will be removed
#         ret = await share_svc.dispatch_msg({'cmd': 'group_add_identities',
#                                             'name': origin_group['name'],
#                                             'identities': identities,
#                                             'admin': admin})
#         assert ret['status'] == 'ok'
#         # Removing non-existant identities should not raise errors
#         for i in range(0, 2):
#             initial_vlob = await user_manifest_svc.get_properties(group=origin_group['name'])
#             ret = await share_svc.dispatch_msg({'cmd': 'group_remove_identities',
#                                                 'name': origin_group['name'],
#                                                 'identities': identities,
#                                                 'admin': admin})
#             ret = await share_svc.dispatch_msg({'cmd': 'group_read', 'name': origin_group['name']})
#             ret['users'] = sorted(ret['users'])
#             ret['admins'] = sorted(ret['admins'])
#             if admin:
#                 assert ret == {'status': 'ok',
#                                'admins': sorted(origin_group['admins']),
#                                'users': sorted(origin_group['users'])}
#             else:
#                 assert ret == {'status': 'ok',
#                                'admins': sorted(origin_group['admins']),
#                                'users': sorted(origin_group['users'])}
#             new_vlob = await user_manifest_svc.get_properties(group=origin_group['name'])
#             assert initial_vlob != new_vlob
#             remaining_identities = origin_group['admins'] if admin else origin_group['users']
#             # Remaining identities should receive the new vlob
#             for identity in remaining_identities:  # TODO try with more users?
#                 messages = await backend_api_svc.message_get(identity)
#                 assert len(messages) == 2 + i
#                 encrypted_vlob = messages[-1]
#                 received_vlob = await crypto_svc.asym_decrypt(encrypted_vlob)
#                 received_vlob = json.loads(received_vlob.decode())
#                 assert received_vlob['vlob'] == new_vlob
#             # Removed identities should not receive the new vlob
#             for identity in identities:  # TODO try with more users?
#                 messages = await backend_api_svc.message_get(identity)
#                 assert len(messages) == 1

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': ['id'],
#          'bad_field': 'foo'},
#         {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': ['id'],
#          'admin': 42},
#         {'cmd': 'group_remove_identities', 'name': 42, 'identities': ['id']},
#         {'cmd': 'group_remove_identities', 'name': None, 'identities': ['id']},
#         {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': 'id'},
#         {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': 42},
#         {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': None},
#         {'cmd': 'group_remove_identities', 'identities': ['id']},
#         {'cmd': 'group_remove_identities', 'name': '<name-here>'},
#         {'cmd': 'group_remove_identities'}, {}])
#     async def test_bad_group_remove_identities(self, share_svc, bad_msg, group):
#         if bad_msg.get('name') == '<name-here>':
#             bad_msg['name'] = group['name']
#         ret = await share_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'
