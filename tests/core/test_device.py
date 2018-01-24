import pytest

from tests.common import connect_core, core_factory


@pytest.mark.trio
@pytest.mark.parametrize('cmd', [
    {'cmd': 'device_declare', 'device_name': 'device2'},
    {
        'cmd': 'device_configure',
        'device_id': 'alice@device2',
        'password': 'S3cr37',
        'configure_device_token': '123456',
    },
    # TODO: mocking configuraton_try_id is a PITA right now...
    # {'cmd': 'device_accept_configuration_try', 'configuration_try_id': '123456'},
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

        async with connect_core(new_device_core) as new_device_core_sock:
            await new_device_core_sock.send({
                'cmd': 'device_configure',
                'device_id': 'alice@device2',
                'password': 'S3cr37',
                'configure_device_token': configure_device_token,
            })

            # Here new_device_core should be on hold, waiting for existing device to
            # accept/refuse his configuration try

            # 3) Existing device receive configuration event

            await alice_core_sock.send({'cmd': 'event_listen'})
            rep = await alice_core_sock.recv()
            assert rep['status'] == 'ok'
            assert rep['event'] == 'device_try_claim_submitted'
            assert rep['device_name'] == 'device2'
            assert rep['configuration_try_id']

            config_try_id = rep['configuration_try_id']

            # 4) Existing device accept configuration

            await alice_core_sock.send({
                'cmd': 'device_accept_configuration_try',
                'configuration_try_id': config_try_id,
            })
            rep = await alice_core_sock.recv()
            assert rep == {'status': 'ok'}

            # 5) Wannabe device get it answer: device has been accepted !

            rep = await new_device_core_sock.recv()
            assert rep['status'] == 'ok'

    # Device config should have been stored on local storage so restarting
    # core is not a trouble

    async with core_factory(
        base_settings_path=new_device_core_conf_dir,
        backend_addr=backend_addr
    ) as restarted_new_device_core:
        async with connect_core(restarted_new_device_core) as new_device_core_sock:

            # 6) Now wannabe device can login as alice

            rep = await new_device_core_sock.send({'cmd': 'list_available_logins'})
            rep = await new_device_core_sock.recv()
            assert rep == {
                'status': 'ok',
                'devices': ['alice@device2'],
            }

            rep = await new_device_core_sock.send({
                'cmd': 'login',
                'id': 'alice@device2',
                'password': 'S3cr37'
            })
            rep = await new_device_core_sock.recv()
            assert rep == {'status': 'ok'}

            rep = await new_device_core_sock.send({
                'cmd': 'info',
            })
            rep = await new_device_core_sock.recv()
            assert rep == {
                'status': 'ok',
                'loaded': True,
                'id': 'alice@device2',
            }
