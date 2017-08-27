import pytest


async def test_identity_info(johndoe_client, johndoe):
    ret = await johndoe_client.send_cmd('identity_info')
    assert ret == {'status': 'ok', 'loaded': True, 'id': johndoe.id}


async def test_identity_info_not_loaded(client):
    ret = await client.send_cmd('identity_info')
    assert ret == {'status': 'ok', 'loaded': False, 'id': None}


@pytest.mark.parametrize('cmd', [
    'backend_status',
    'identity_unload'
])
async def test_identity_not_loaded(client, cmd):
    ret = await client.send_cmd(cmd)
    assert ret == {'label': 'Identity not loaded', 'status': 'identity_not_loaded'}


async def test_identity_load(client, johndoe):
    ret = await client.send_cmd('identity_load', id=johndoe.id, key=johndoe.privkey)
    assert ret == {'status': 'ok'}
    ret = await client.send_cmd('identity_info')
    assert ret == {'id': 'John_Doe', 'loaded': True, 'status': 'ok'}
    ret = await client.send_cmd('identity_unload')
    assert ret == {'status': 'ok'}


async def test_login(client, johndoe, johndoe_cipherkey_in_backend):
    ret = await client.send_cmd('identity_login', id=johndoe.id, password=johndoe.password)
    assert ret == {'status': 'ok'}
    ret = await client.send_cmd('identity_info')
    assert ret == {'status': 'ok', 'loaded': True, 'id': johndoe.id}


async def test_bad_password_login(client, johndoe):
    ret = await client.send_cmd('identity_login', id=johndoe.id, password='dummy')
    assert ret == {'status': 'privkey_not_found', 'label': 'Bad id or password'}


async def test_unknown_id_login(client):
    ret = await client.send_cmd('identity_login', id='Unknown', password='P@ssw0rd.')
    assert ret == {'status': 'privkey_not_found', 'label': 'Bad id or password'}


async def test_signup(client):
    ret = await client.send_cmd('identity_signup', id='Zack', password='P@ssw0rd.')
    assert ret == {'status': 'ok'}
    # Should be able to login with the new identity
    ret = await client.send_cmd('identity_login', id='Zack', password='P@ssw0rd.')
    assert ret == {'status': 'ok'}
    ret = await client.send_cmd('identity_info')
    assert ret == {'status': 'ok', 'loaded': True, 'id': 'Zack'}


async def test_already_exist_signup(client):
    ret = await client.send_cmd('identity_signup', id='Zack', password='P@ssw0rd.')
    assert ret == {'status': 'ok'}
    # Should not be able to overwrite the identity...
    ret = await client.send_cmd('identity_signup', id='Zack', password='P@ssw0rd.')
    assert ret == {'status': 'privkey_hash_collision', 'label': 'This hash already exists...'}
    # ...even with a different the password
    ret = await client.send_cmd('identity_signup', id='Zack', password='New_P@ssw0rd.')
    assert ret == {'status': 'backend_identity_register_error',
                   'label': 'Identity `Zack` already has a public key'}
