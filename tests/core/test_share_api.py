# from base64 import encodebytes, decodebytes
# from copy import deepcopy
# import json
# from io import BytesIO
# import random

# from cryptography.hazmat.backends.openssl import backend as openssl
# from cryptography.hazmat.primitives import hashes
# from freezegun import freeze_time
# import pytest

# from parsec.core.buffers import BufferedBlock, BufferedUserVlob, BufferedVlob
# from parsec.core import (CoreService, IdentityService, MetaBlockService,
#                          MockedBackendAPIService, MockedBlockService)
# from parsec.core.manifest import GroupManifest, Manifest, UserManifest
# from parsec.crypto import AESCipher
# from parsec.exceptions import UserManifestError, UserManifestNotFound, FileNotFound, VlobNotFound
# from parsec.server import BaseServer


# JOHN_DOE_IDENTITY = 'John_Doe'
# JOHN_DOE_PRIVATE_KEY = b"""
# -----BEGIN RSA PRIVATE KEY-----
# MIICWgIBAAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO04KJSq1cH87KtmkqI3qewvXtW
# qFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/mG/U9gURDi4aTTXT02RbHESBp
# yMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49SLhqGNmNAnH2E3lxAgMBAAEC
# gYBY2S0QFJG8AwCdfKKUK+t2q+UO6wscwdtqSk/grBg8MWXTb+8XjteRLy3gD9Eu
# E1IpwPStjj7KYEnp2blAvOKY0E537d2a4NLrDbSi84q8kXqvv0UeGMC0ZB2r4C89
# /6BTZii4mjIlg3iPtkbRdLfexjqmtELliPkHKJIIMH3fYQJBAKd/k1hhnoxEx4sq
# GRKueAX7orR9iZHraoR9nlV69/0B23Na0Q9/mP2bLphhDS4bOyR8EXF3y6CjSVO4
# LBDPOmUCQQCV5hr3RxGPuYi2n2VplocLK/6UuXWdrz+7GIqZdQhvhvYSKbqZ5tvK
# Ue8TYK3Dn4K/B+a7wGTSx3soSY3RBqwdAkAv94jqtooBAXFjmRq1DuGwVO+zYIAV
# GaXXa2H8eMqr2exOjKNyHMhjWB1v5dswaPv25tDX/caCqkBFiWiVJ8NBAkBnEnqo
# Xe3tbh5btO7+08q4G+BKU9xUORURiaaELr1GMv8xLhBpkxy+2egS4v+Y7C3zPXOi
# 1oB9jz1YTnt9p6DhAkBy0qgscOzo4hAn062MAYWA6hZOTkvzRbRpnyTRctKwZPSC
# +tnlGk8FAkuOm/oKabDOY1WZMkj5zWAXrW4oR3Q2
# -----END RSA PRIVATE KEY-----
# """
# JOHN_DOE_PUBLIC_KEY = b"""
# -----BEGIN PUBLIC KEY-----
# MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO0
# 4KJSq1cH87KtmkqI3qewvXtWqFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/m
# G/U9gURDi4aTTXT02RbHESBpyMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49
# SLhqGNmNAnH2E3lxAgMBAAE=
# -----END PUBLIC KEY-----
# """


# @pytest.fixture
# def backend_svc():
#     return MockedBackendAPIService()


# @pytest.fixture
# def core_svc(event_loop, backend_svc, identity_svc):
#     service = CoreService()
#     block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
#     server = BaseServer()
#     server.register_service(service)
#     server.register_service(identity_svc)
#     server.register_service(block_service)
#     server.register_service(MockedBackendAPIService())
#     event_loop.run_until_complete(server.bootstrap_services())
#     event_loop.run_until_complete(service.load_user_manifest())
#     event_loop.run_until_complete(service.group_create('foo_community'))
#     yield service
#     event_loop.run_until_complete(server.teardown_services())


# @pytest.fixture
# def identity_svc(event_loop):
#     identity = JOHN_DOE_IDENTITY
#     identity_key = BytesIO(JOHN_DOE_PRIVATE_KEY)
#     service = IdentityService()
#     event_loop.run_until_complete(service.load(identity, identity_key.read()))
#     return service


# @pytest.fixture
# def group(core_svc, event_loop, name='foo_communiy'):
#     event_loop.run_until_complete(core_svc.group_create(name=name))
#     identities = [key['fingerprint'] for key in crypto_svc.gnupg.list_keys(secret=True)]
#     identities = identities[-2:]
#     result = event_loop.run_until_complete(core_svc.group_add_identities(name=name,
#                                                                          identities=identities))
#     result = event_loop.run_until_complete(core_svc.group_read(name=name))
#     result.update({'name': name})
#     return result


# class TestShareServiceAPI:

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/unknown', 'identity': '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'}])
#     async def test_share_with_identity_and_file_not_found(self, core_svc, payload):
#         ret = await core_svc.dispatch_msg({'cmd': 'share_with_identity', **payload})
#         assert ret == {'status': 'file_not_found', 'label': 'File not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [{'path': '/foo', 'identity': 'unknown'}])
#     async def test_share_with_identity_and_identity_not_found(self, core_svc, payload):
#         ret = await core_svc.dispatch_msg({'cmd': 'share_with_identity', **payload})
#         assert ret == {'status': 'error', 'label': 'Encryption failure.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/foo', 'identity': '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'}])
#     async def test_share_with_identity(self,
#                                        core_svc,
#                                        payload):
#         shared_vlob = await core_svc.get_properties(path=payload['path'])
#         ret = await core_svc.dispatch_msg({'cmd': 'share_with_identity', **payload})
#         assert ret == {'status': 'ok'}
#         messages = await backend_svc.message_get(payload['identity'])
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
#     async def test_bad_msg_share_with_identity(self, core_svc, bad_msg):
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/unknown', 'group': '<group-here>'}])
#     async def test_share_with_group_and_file_not_found(self, core_svc, payload, group):
#         if payload.get('group') == '<group-here>':
#             payload['group'] = group['name']
#         ret = await core_svc.dispatch_msg({'cmd': 'share_with_group', **payload})
#         assert ret == {'status': 'user_manifest_not_found', 'label': 'File not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/foo', 'group': 'unknown'}])
#     async def test_share_with_group_and_identity_not_found(self, core_svc, payload, group):
#         ret = await core_svc.dispatch_msg({'cmd': 'share_with_group', **payload})
#         assert ret == {'status': 'user_manifest_not_found', 'label': 'Group not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('payload', [
#         {'path': '/foo', 'group': '<group-here>'}])
#     async def test_share_with_group(self,
#                                     backend_svc,
#                                     core_svc,
#                                     payload,
#                                     group):
#         if payload.get('group') == '<group-here>':
#             payload['group'] = group['name']
#         shared_vlob = await core_svc.get_properties(path=payload['path'])
#         ret = await core_svc.dispatch_msg({'cmd': 'share_with_group', **payload})
#         assert ret == {'status': 'ok'}
#         for identity in group['users']:  # TODO try with more users?
#             messages = await backend_svc.message_get(identity)
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
#     async def test_bad_msg_share_with_group(self, core_svc, bad_msg):
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_share_stop(self, backend_svc, core_svc):
#         shared_vlob = await core_svc.get_properties(path='/foo')
#         ret = await core_svc.dispatch_msg({'cmd': 'share_stop', 'path': '/foo'})
#         assert ret == {'status': 'ok'}
#         new_vlob = await core_svc.get_properties(path='/foo')
#         for property in shared_vlob.keys():
#             assert shared_vlob[property] != new_vlob[property]
#         ret = await core_svc.dispatch_msg({'cmd': 'share_stop', 'path': '/unknown'})
#         assert ret == {'status': 'user_manifest_not_found', 'label': 'File not found.'}

#     @pytest.mark.asyncio
#     async def test_share_stop_not_found(self, core_svc):
#         ret = await core_svc.dispatch_msg({'cmd': 'share_stop', 'path': '/unknown'})
#         assert ret == {'status': 'user_manifest_not_found', 'label': 'File not found.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'share_stop', 'path': '/foo', 'bad_field': 'foo'},
#         {'cmd': 'share_stop', 'path': 42},
#         {'cmd': 'share_stop', 'path': None},
#         {}])
#     async def test_bad_msg_share_stop_with_group(self, core_svc, bad_msg):
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     # TODO Remove tests bellow as they are duplicated in test_group_service?

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('group_payload', [
#         {'name': 'foo_community'},
#         {'name': 'Foo community'}])
#     async def test_group_create(self, core_svc, group_payload):
#         # Working
#         ret = await core_svc.dispatch_msg({'cmd': 'group_create', **group_payload})
#         assert ret == {'status': 'ok'}
#         # Already exist
#         ret = await core_svc.dispatch_msg({'cmd': 'group_create', **group_payload})
#         assert ret == {'status': 'already_exist', 'label': 'Group already exist.'}

#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('bad_msg', [
#         {'cmd': 'group_create', 'name': 'foo', 'bad_field': 'foo'},
#         {'cmd': 'group_create', 'name': 42},
#         {'cmd': 'group_create', 'name': None},
#         {}])
#     async def test_bad_msg_group_create(self, core_svc, bad_msg):
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_group_read_not_found(self, core_svc, group):
#         ret = await core_svc.dispatch_msg({'cmd': 'group_read', 'name': 'unknown'})
#         assert ret == {'status': 'user_manifest_not_found', 'label': 'Group not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_group_read(self, core_svc, group):
#         ret = await core_svc.dispatch_msg({'cmd': 'group_read', 'name': group['name']})
#         assert ret == {'status': 'ok', 'admins': group['admins'], 'users': group['users']}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_group_add_identities_not_found(self, core_svc, group):
#         ret = await core_svc.dispatch_msg({'cmd': 'group_add_identities',
#                                             'name': 'unknown',
#                                             'identities': group['users']})
#         assert ret == {'status': 'user_manifest_not_found', 'label': 'Group not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('admin', [True, False])
#     @pytest.mark.parametrize('identities', [
#         [],
#         ['81DBCF6EB9C8B2965A65ACE5520D903047D69DC9', '3C3FA85FB9736362497EB23DC0485AC10E6274C7']])
#     async def test_group_add_identities(self,
#                                         backend_svc,
#                                         core_svc,
#                                         group,
#                                         identities,
#                                         admin):
#         origin_group = deepcopy(group)
#         # Adding duplicates identities should not raise errors
#         for i in range(0, 2):
#             ret = await core_svc.dispatch_msg({'cmd': 'group_add_identities',
#                                                 'name': origin_group['name'],
#                                                 'identities': identities,
#                                                 'admin': admin})
#             ret = await core_svc.dispatch_msg({'cmd': 'group_read', 'name': origin_group['name']})
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
#             group_manifest = await core_svc.get_manifest(origin_group['name'])
#             shared_vlob = await group_manifest.get_vlob()
#             for identity in identities:  # TODO try with more users?
#                 messages = await backend_svc.message_get(identity)
#                 encrypted_vlob = messages[-1]
#                 received_vlob = await crypto_svc.asym_decrypt(encrypted_vlob)
#                 received_vlob = json.loads(received_vlob.decode())
#                 assert shared_vlob == received_vlob['vlob']

#     @pytest.mark.xfail
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
#     async def test_bad_group_add_identities(self, core_svc, bad_msg, group):
#         if bad_msg.get('name') == '<name-here>':
#             bad_msg['name'] = group['name']
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_gorup_remove_identities_not_found(self, core_svc, group):
#         ret = await core_svc.dispatch_msg({'cmd': 'group_remove_identities',
#                                             'name': 'unknown',
#                                             'identities': group['users']})
#         assert ret == {'status': 'user_manifest_not_found', 'label': 'Group not found.'}

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('admin', [True, False])
#     @pytest.mark.parametrize('identities', [
#         [],
#         ['81DBCF6EB9C8B2965A65ACE5520D903047D69DC9', '3C3FA85FB9736362497EB23DC0485AC10E6274C7']])
#     async def test_group_remove_identities(self,
#                                            backend_svc,
#                                            core_svc,
#                                            group,
#                                            identities,
#                                            admin):
#         origin_group = deepcopy(group)
#         # Identities that will be removed
#         ret = await core_svc.dispatch_msg({'cmd': 'group_add_identities',
#                                             'name': origin_group['name'],
#                                             'identities': identities,
#                                             'admin': admin})
#         assert ret['status'] == 'ok'
#         # Removing non-existant identities should not raise errors
#         for i in range(0, 2):
#             initial_vlob = await core_svc.get_properties(group=origin_group['name'])
#             ret = await core_svc.dispatch_msg({'cmd': 'group_remove_identities',
#                                                 'name': origin_group['name'],
#                                                 'identities': identities,
#                                                 'admin': admin})
#             ret = await core_svc.dispatch_msg({'cmd': 'group_read', 'name': origin_group['name']})
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
#             new_vlob = await core_svc.get_properties(group=origin_group['name'])
#             assert initial_vlob != new_vlob
#             remaining_identities = origin_group['admins'] if admin else origin_group['users']
#             # Remaining identities should receive the new vlob
#             for identity in remaining_identities:  # TODO try with more users?
#                 messages = await backend_svc.message_get(identity)
#                 assert len(messages) == 2 + i
#                 encrypted_vlob = messages[-1]
#                 received_vlob = await crypto_svc.asym_decrypt(encrypted_vlob)
#                 received_vlob = json.loads(received_vlob.decode())
#                 assert received_vlob['vlob'] == new_vlob
#             # Removed identities should not receive the new vlob
#             for identity in identities:  # TODO try with more users?
#                 messages = await backend_svc.message_get(identity)
#                 assert len(messages) == 1

#     @pytest.mark.xfail
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
#     async def test_bad_group_remove_identities(self, core_svc, bad_msg, group):
#         if bad_msg.get('name') == '<name-here>':
#             bad_msg['name'] = group['name']
#         ret = await core_svc.dispatch_msg(bad_msg)
#         assert ret['status'] == 'bad_msg'
