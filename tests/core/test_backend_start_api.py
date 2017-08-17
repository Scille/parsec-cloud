import pytest
from unittest.mock import patch
from effect2.testing import asyncio_perform_sequence, const, noop

from parsec.tools import to_jsonb64
from parsec.core.backend import BackendCmd
from parsec.core.backend_start_api import (
    EBackendCipherKeyGet, EBackendCipherKeyAdd, StartAPIComponent,
    EBackendIdentityRegister
)
from parsec.exceptions import PrivKeyHashCollision, PrivKeyNotFound, BackendIdentityRegisterError

from tests.common import AsyncMock


async def test_perform_cipherkey_get():
    component = StartAPIComponent('http://foo/bar')
    with patch('parsec.core.backend_start_api.aiohttp.ClientSession.get',
               new_callable=AsyncMock) as mock_get, \
         patch('parsec.core.backend_start_api.decrypt_with_password') as mock_decrypt_with_password:
        mock_get.return_value.aenter.status = 200
        mock_get.return_value.aenter.text.set_asyncret(b"<alice's cipherkey>")
        mock_decrypt_with_password.return_value = b"<alice's key>"

        ret = await component.perform_cipherkey_get(EBackendCipherKeyGet(
            'alice@test.com', 'P@ssw0rd.'))
        assert ret == b"<alice's key>"
        mock_get.assert_called_once_with(
            'http://foo/bar/cipherkey/uPwfLWMmP5BOfDXOx_O9D7cN0z3Kzvakd_iiGChmvYo='
        )
        mock_decrypt_with_password.assert_called_once_with(
            b'P@ssw0rd.', b'alice@test.com', b"<alice's cipherkey>")


async def test_perform_cipherkey_get_unknown_id():
    component = StartAPIComponent('http://foo/bar')
    with patch('parsec.core.backend_start_api.aiohttp.ClientSession.get',
               new_callable=AsyncMock) as mock_get:
        mock_get.return_value.aenter.status = 404
        mock_get.return_value.aenter.text.set_asyncret(b'')

        with pytest.raises(PrivKeyNotFound):
            await component.perform_cipherkey_get(EBackendCipherKeyGet(
                'alice@test.com', 'P@ssw0rd.'))


async def test_perform_cipherkey_add():
    component = StartAPIComponent('http://foo/bar')
    with patch('parsec.core.backend_start_api.aiohttp.ClientSession.post',
               new_callable=AsyncMock) as mock_post, \
         patch('parsec.core.backend_start_api.encrypt_with_password') as mock_encrypt_with_password:
        mock_post.return_value.aenter.status = 200
        mock_post.return_value.aenter.text.set_asyncret(b'')
        mock_encrypt_with_password.return_value = b"<alice's cipherkey>"

        ret = await component.perform_cipherkey_add(EBackendCipherKeyAdd(
            'alice@test.com', 'P@ssw0rd.', b"<alice's privkey>"))
        assert ret is None
        mock_post.assert_called_once_with(
            'http://foo/bar/cipherkey/uPwfLWMmP5BOfDXOx_O9D7cN0z3Kzvakd_iiGChmvYo=',
            data=b"<alice's cipherkey>"
        )
        mock_encrypt_with_password.assert_called_once_with(
            b'P@ssw0rd.', b'alice@test.com', b"<alice's privkey>")


async def test_perform_cipherkey_add_duplicated():
    component = StartAPIComponent('http://foo/bar')
    with patch('parsec.core.backend_start_api.aiohttp.ClientSession.post',
               new_callable=AsyncMock) as mock_post:
        mock_post.return_value.aenter.text.set_asyncret(b'')
        mock_post.return_value.aenter.status = 409

        with pytest.raises(PrivKeyHashCollision):
            await component.perform_cipherkey_add(EBackendCipherKeyAdd(
                'alice@test.com', 'P@ssw0rd.', b"<alice's privkey>"))


async def test_perform_identity_register():
    component = StartAPIComponent('http://foo/bar')
    with patch('parsec.core.backend_start_api.aiohttp.ClientSession.post',
               new_callable=AsyncMock) as mock_post:
        mock_post.return_value.aenter.text.set_asyncret(b'')
        mock_post.return_value.aenter.status = 200

        ret = await component.perform_identity_register(EBackendIdentityRegister(
            'alice@test.com', b"<alice's pubkey>"))
        assert ret is None
        mock_post.assert_called_once_with(
            'http://foo/bar/pubkey/alice@test.com', data=b"<alice's pubkey>")


async def test_perform_identity_register_error():
    component = StartAPIComponent('http://foo/bar')
    with patch('parsec.core.backend_start_api.aiohttp.ClientSession.post',
               new_callable=AsyncMock) as mock_post:
        mock_post.return_value.aenter.text.set_asyncret(b'')
        mock_post.return_value.aenter.status = 400

        with pytest.raises(BackendIdentityRegisterError):
            await component.perform_identity_register(EBackendIdentityRegister(
                'alice@test.com', b"<alice's pubkey>"))
