import pytest
from nacl.signing import SigningKey
from nacl.public import PrivateKey, SealedBox

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import connect_core, core_factory


@pytest.mark.trio
@pytest.mark.parametrize('cmd', [
    {'cmd': 'device_declare', 'device_name': 'device2'},
    {
        'cmd': 'device_configure',
        'configure_device_token': '123456',
        'user_id': 'alice',
        'device_name': 'device2',
        'device_verify_key': '123',
        'user_privkey_cypherkey': 'ABC',
    },
    {'cmd': 'device_get_configuration_try', 'configuration_try_id': '123456'},
    {'cmd': 'device_accept_configuration_try', 'configuration_try_id': '123456', 'cyphered_user_privkey': '123'},
    {'cmd': 'device_refuse_configuration_try', 'configuration_try_id': '123456', 'reason': 'Whatever'},
])
async def test_device_cmd_backend_offline(core, alice_core_sock, cmd):
    await alice_core_sock.send(cmd)
    rep = await alice_core_sock.recv()
    assert rep == {'status': 'backend_not_availabled'}


@pytest.mark.trio
async def test_device_declare_then_accepted(tmpdir, running_backend, backend_addr, core, alice, alice_core_sock):
    # 0) Initial device declare the new device

    await alice_core_sock.send({'cmd': 'device_declare', 'device_name': 'device2'})
    rep = await alice_core_sock.recv()
    assert rep['status'] == 'ok'
    configure_device_token = rep['configure_device_token']

    # 1) Existing device start listening for device configuration

    await alice_core_sock.send({'cmd': 'event_subscribe', 'event': 'device_try_claim_submitted'})
    rep = await alice_core_sock.recv()
    assert rep == {'status': 'ok'}

    # 2) Wannabe device spawn core and start configuration

    new_device_core_conf_dir = tmpdir.mkdir('new_device_core')
    async with core_factory(
        base_settings_path=new_device_core_conf_dir,
        backend_addr=backend_addr
    ) as new_device_core:

        device_signkey = SigningKey.generate()
        user_privkey_cypherkey_privkey = PrivateKey.generate()
        async with connect_core(new_device_core) as new_device_core_sock:
            await new_device_core_sock.send({
                'cmd': 'device_configure',
                'configure_device_token': configure_device_token,
                'user_id': 'alice',
                'device_name': 'device2',
                'device_verify_key': to_jsonb64(device_signkey.verify_key.encode()),
                'user_privkey_cypherkey': to_jsonb64(user_privkey_cypherkey_privkey.public_key.encode()),
            })

            # Here new_device_core should be on hold, waiting for existing device to
            # accept/refuse his configuration try

            # 3) Existing device receive configuration event

            await alice_core_sock.send({'cmd': 'event_listen'})
            rep = await alice_core_sock.recv()
            assert rep['status'] == 'ok'
            assert rep['event'] == 'device_try_claim_submitted'
            assert rep['subject']

            config_try_id = rep['subject']

            # 4) Existing device retreive configuration try informations

            await alice_core_sock.send({
                'cmd': 'device_get_configuration_try',
                'configuration_try_id': config_try_id,
            })
            rep = await alice_core_sock.recv()
            assert rep['status'] == 'ok'
            assert rep['configuration_status'] == 'waiting_answer'
            assert rep['device_name'] == 'device2'
            assert rep['device_verify_key']
            assert rep['user_privkey_cypherkey']
            user_privkey_cypherkey = PrivateKey(from_jsonb64(rep['user_privkey_cypherkey']))

            # 5) Existing device accept configuration

            box = SealedBox(user_privkey_cypherkey)
            cyphered_user_privkey = box.encrypt(alice.user_privkey.encode())

            await alice_core_sock.send({
                'cmd': 'device_accept_configuration_try',
                'configuration_try_id': config_try_id,
                'cyphered_user_privkey': to_jsonb64(cyphered_user_privkey)
            })
            rep = await alice_core_sock.recv()
            assert rep == {'status': 'ok'}

            # 6) Wannabe device get it answer: device has been accepted !

            rep = await new_device_core_sock.recv()
            assert rep['status'] == 'ok'
            assert rep['cyphered_user_privkey']
