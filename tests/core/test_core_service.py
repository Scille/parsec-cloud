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
#                          MockedBackendAPIService, MockedBlockService, SynchronizerService)
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
#     server.register_service(SynchronizerService())
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


# class TestCoreService:

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_load_user_manifest(self, core_svc, identity_svc):
#         await core_svc.file_create('/test')
#         await core_svc.stat('/test')
#         ret = await core_svc.dispatch_msg({'cmd': 'user_manifest_load'})
#         assert ret == {'status': 'ok'}
#         await core_svc.stat('/test')
#         identity = '3C3FA85FB9736362497EB23DC0485AC10E6274C7'
#         manifest = await core_svc.get_manifest()
#         old_identity = manifest.id
#         assert old_identity != identity
#         await identity_svc.load_identity(identity)
#         ret = await core_svc.dispatch_msg({'cmd': 'user_manifest_load'})
#         assert ret == {'status': 'ok'}
#         manifest = await core_svc.get_manifest()
#         assert manifest.id == identity
#         with pytest.raises(UserManifestNotFound):
#             await core_svc.stat('/test')
#         await identity_svc.load_identity(old_identity)
#         ret = await core_svc.dispatch_msg({'cmd': 'user_manifest_load'})
#         assert ret == {'status': 'ok'}
#         await core_svc.stat('/test')

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     async def test_get_manifest(self, core_svc):
#         manifest = await core_svc.get_manifest()
#         assert manifest.id == core_svc.identity.id
#         group_manifest = await core_svc.get_manifest('foo_community')
#         assert group_manifest.id is not None
#         with pytest.raises(UserManifestNotFound):
#             await core_svc.get_manifest('unknown')
#         with pytest.raises(UserManifestNotFound):
#             core_svc.user_manifest = None  # TODO too intrusive
#             await core_svc.get_manifest()

#     @pytest.mark.xfail
#     @pytest.mark.asyncio
#     @pytest.mark.parametrize('group', [None, 'foo_community'])
#     async def test_get_properties(self, core_svc, group):
#         foo_vlob = await core_svc.file_create('/foo', group=group)
#         bar_vlob = await core_svc.file_create('/bar', group=group)
#         await core_svc.file_delete('/bar', group)
#         # Lookup group
#         group_manifest = await core_svc.get_manifest(group='foo_community')
#         group_vlob = await core_svc.get_properties(group='foo_community')
#         assert await group_manifest.get_vlob() == group_vlob
#         # Lookup foo by path
#         vlob = await core_svc.get_properties(path='/foo', dustbin=False, group=group)
#         assert vlob == foo_vlob
#         with pytest.raises(UserManifestNotFound):
#             await core_svc.get_properties(path='/foo', dustbin=True, group=group)
#         # Lookup bar by path
#         vlob = await core_svc.get_properties(path='/bar', dustbin=True, group=group)
#         vlob = deepcopy(vlob)  # TODO use deepcopy?
#         del vlob['removed_date']
#         del vlob['path']
#         assert vlob == bar_vlob
#         with pytest.raises(UserManifestNotFound):
#             await core_svc.get_properties(path='/bar', dustbin=False, group=group)
#         # Lookup foo by id
#         vlob = await core_svc.get_properties(id=foo_vlob['id'], dustbin=False, group=group)
#         assert vlob == foo_vlob
#         with pytest.raises(UserManifestNotFound):
#             await core_svc.get_properties(id=foo_vlob['id'], dustbin=True, group=group)
#         # Lookup bar by id
#         vlob = await core_svc.get_properties(id=bar_vlob['id'], dustbin=True, group=group)
#         vlob = deepcopy(vlob)  # TODO use deepcopy?
#         del vlob['removed_date']
#         del vlob['path']
#         assert vlob == bar_vlob
#         with pytest.raises(UserManifestNotFound):
#             await core_svc.get_properties(id=bar_vlob['id'], dustbin=False, group=group)
