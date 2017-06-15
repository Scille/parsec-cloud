import pytest

from parsec.core import IdentityService


JOHN_DOE_IDENTITY = 'John_Doe'
JOHN_DOE_PRIVATE_KEY = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICWgIBAAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO04KJSq1cH87KtmkqI3qewvXtW
qFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/mG/U9gURDi4aTTXT02RbHESBp
yMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49SLhqGNmNAnH2E3lxAgMBAAEC
gYBY2S0QFJG8AwCdfKKUK+t2q+UO6wscwdtqSk/grBg8MWXTb+8XjteRLy3gD9Eu
E1IpwPStjj7KYEnp2blAvOKY0E537d2a4NLrDbSi84q8kXqvv0UeGMC0ZB2r4C89
/6BTZii4mjIlg3iPtkbRdLfexjqmtELliPkHKJIIMH3fYQJBAKd/k1hhnoxEx4sq
GRKueAX7orR9iZHraoR9nlV69/0B23Na0Q9/mP2bLphhDS4bOyR8EXF3y6CjSVO4
LBDPOmUCQQCV5hr3RxGPuYi2n2VplocLK/6UuXWdrz+7GIqZdQhvhvYSKbqZ5tvK
Ue8TYK3Dn4K/B+a7wGTSx3soSY3RBqwdAkAv94jqtooBAXFjmRq1DuGwVO+zYIAV
GaXXa2H8eMqr2exOjKNyHMhjWB1v5dswaPv25tDX/caCqkBFiWiVJ8NBAkBnEnqo
Xe3tbh5btO7+08q4G+BKU9xUORURiaaELr1GMv8xLhBpkxy+2egS4v+Y7C3zPXOi
1oB9jz1YTnt9p6DhAkBy0qgscOzo4hAn062MAYWA6hZOTkvzRbRpnyTRctKwZPSC
+tnlGk8FAkuOm/oKabDOY1WZMkj5zWAXrW4oR3Q2
-----END RSA PRIVATE KEY-----
"""
JOHN_DOE_PUBLIC_KEY = b"""
-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO0
4KJSq1cH87KtmkqI3qewvXtWqFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/m
G/U9gURDi4aTTXT02RbHESBpyMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49
SLhqGNmNAnH2E3lxAgMBAAE=
-----END PUBLIC KEY-----
"""


@pytest.fixture
def identity_svc():
    return IdentityService()


class TestIdentityService:

    @pytest.mark.asyncio
    async def test_identity_load(self, identity_svc):
        await identity_svc.dispatch_msg({'cmd': 'identity_load',
                                         'id': JOHN_DOE_IDENTITY,
                                         'key': JOHN_DOE_PRIVATE_KEY})
        ret = await identity_svc.dispatch_msg({'cmd': 'identity_load',
                                               'id': JOHN_DOE_IDENTITY,
                                               'key': JOHN_DOE_PRIVATE_KEY})
        assert ret == {'status': 'identity_error', 'label': 'User already logged in.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'identity_load', 'id': JOHN_DOE_IDENTITY, 'key': JOHN_DOE_PRIVATE_KEY, 'bad_field': 'foo'},
        {'cmd': 'identity_load', 'id': JOHN_DOE_IDENTITY, 'key': 42},
        {'cmd': 'identity_load', 'id': 42, 'key': JOHN_DOE_PRIVATE_KEY},
        {}])
    async def test_bad_msg_identity_load(self, identity_svc, bad_msg):
        ret = await identity_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_identity_info(self, identity_svc):
        ret = await identity_svc.dispatch_msg({'cmd': 'identity_info'})
        assert {'status': 'identity_error', 'label': 'Identity not loaded.'} == ret
        ret = await identity_svc.dispatch_msg({'cmd': 'identity_load',
                                               'id': JOHN_DOE_IDENTITY,
                                               'key': JOHN_DOE_PRIVATE_KEY})
        ret = await identity_svc.dispatch_msg({'cmd': 'identity_info'})
        assert ret == {'id': 'John_Doe', 'status': 'ok'}

    @pytest.mark.asyncio
    async def test_identity_unload(self, identity_svc):
        await identity_svc.dispatch_msg({'cmd': 'identity_load',
                                         'id': JOHN_DOE_IDENTITY,
                                         'key': JOHN_DOE_PRIVATE_KEY})
        await identity_svc.dispatch_msg({'cmd': 'identity_unload'})
        ret = await identity_svc.dispatch_msg({'cmd': 'identity_info'})
        assert ret == {'status': 'identity_error', 'label': 'Identity not loaded.'}
        await identity_svc.dispatch_msg({'cmd': 'identity_unload'})
        assert ret == {'status': 'identity_error', 'label': 'Identity not loaded.'}
