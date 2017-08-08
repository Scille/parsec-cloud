import pytest
import asyncio
from effect2.testing import asyncio_perform_sequence

from parsec.backend.privkey import (
    EPrivKeyGet, EPrivKeyAdd, MockedPrivKeyComponent)
from parsec.exceptions import PrivKeyHashCollision, PrivKeyNotFound

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


ALICE_PRIVATE_RSA = b"""
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
ALICE_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgGITzwWRxv+mTAwqQd9pmQ8qqUO0
4KJSq1cH87KtmkqI3qewvXtWqFsHP6ZNOT6wba7lrohJh1rDLU98GjorL4D/eX/m
G/U9gURDi4aTTXT02RbHESBpyMpeBUCzPTq1OgAk9OaawpV48vNkQifuT743hK49
SLhqGNmNAnH2E3lxAgMBAAE=
-----END PUBLIC KEY-----
"""
BOB_PRIVATE_RSA = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQDCqVQVdVhJqW9rrbObvDZ4ET6FoIyVn6ldWhOJaycMeFYBN3t+
cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S3rAV6JhDKDfDFrkzUVZmcc/g
HhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOKG889JN85nABKR9WkdwIDAQAB
AoGBAJFU3Dr9FgJA5rfMwpiV51CzByu61trqjgbtNkLVZhzwRr23z5Jxmd+yLHik
J6ia6sYvdUuHFLKQegGt/2xOjXn8UBpa725gLojHn2umtJDL7amTlBwiJfNXuZrF
BSKK9+xZnNDWMq1IuCqPeintbve+MNSc62JYuGGtXSz9L5f5AkEA/xBkUksBfEUl
65oEPgxvMKHNjLq48otRmCaG+i3MuQqTYQ+c8Z/l26yQL4OV2b36a8/tTaLhwhAZ
Ibtv05NKfQJBAMNgMbOsUWpY8A1Cec79Oj6RVe79E5ciZ4mW3lx5tjJRyNxwlQag
u+T6SwBIa6xMfLBQeizzxqXqxAyW/riQ6QMCQQCadUu7Re6tWZaAGTGufYsr8R/v
s/dh8ZpEwDgG8otCFzRul6zb6Y+huttJ2q55QIGQnka/N/7srSD6+23Zux1lAkBx
P30PzL6UimD7DqFUnev5AH1zPjbwz/x8AHt71wEJQebQAGIhqWHAZGS9ET14bg2I
ld172QI4glCJi6yyhyzJAkBzfmHZEE8FyLCz4z6b+Z2ghMds2Xz7RwgVqCIXt9Ku
P7Bq0eXXgyaBo+jpr3h4K7QnPh+PaHSlGqSfczZ6GIpx
-----END RSA PRIVATE KEY-----
"""
BOB_PUBLIC_RSA = b"""
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCqVQVdVhJqW9rrbObvDZ4ET6F
oIyVn6ldWhOJaycMeFYBN3t+cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S
3rAV6JhDKDfDFrkzUVZmcc/gHhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOK
G889JN85nABKR9WkdwIDAQAB
-----END PUBLIC KEY-----
"""


async def bootstrap_PostgreSQLPrivKeyComponent(request, loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    conn = module.PostgreSQLConnection(url)
    await conn.open_connection()

    def finalize():
        loop.run_until_complete(conn.close_connection())

    request.addfinalizer(finalize)
    return module.PostgreSQLPrivKeyComponent(conn)


@pytest.fixture(params=[MockedPrivKeyComponent, bootstrap_PostgreSQLPrivKeyComponent],
                ids=['mocked', 'postgresql'])
def component(request, loop):
    if asyncio.iscoroutinefunction(request.param):
        return loop.run_until_complete(request.param(request, loop))
    else:
        return request.param()


@pytest.fixture
def privkeys_loaded(component, loop):
    for intent in (EPrivKeyAdd("<Alice's hash>", b"<Alice's cipherkey>"),
                   EPrivKeyAdd("<Bob's hash>", b"<Bob's cipherkey>")):
        eff = component.perform_privkey_add(intent)
        loop.run_until_complete(asyncio_perform_sequence([], eff))


class TestPrivKeyComponent:
    async def test_privkey_get_ok(self, component, privkeys_loaded):
        intent = EPrivKeyGet("<Alice's hash>")
        eff = component.perform_privkey_get(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == b"<Alice's cipherkey>"

    async def test_privkey_get_missing(self, component, privkeys_loaded):
        intent = EPrivKeyGet('<dummy hash>')
        with pytest.raises(PrivKeyNotFound):
            eff = component.perform_privkey_get(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    async def test_privkey_add_ok(self, component):
        intent = EPrivKeyAdd("<Alice's hash>", b"<Alice's cipherkey>")
        eff = component.perform_privkey_add(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret is None
        # Make sure key is present
        intent = EPrivKeyGet("<Alice's hash>")
        eff = component.perform_privkey_get(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == b"<Alice's cipherkey>"

    async def test_privkey_add_duplicated(self, component, privkeys_loaded):
        intent = EPrivKeyAdd("<Alice's hash>", b"<Bob's cipherkey>")
        with pytest.raises(PrivKeyHashCollision):
            eff = component.perform_privkey_add(intent)
            sequence = [
            ]
            ret = await asyncio_perform_sequence(sequence, eff)
        # Make sure key hasn't changed
        intent = EPrivKeyGet("<Alice's hash>")
        eff = component.perform_privkey_get(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == b"<Alice's cipherkey>"
