import pytest
from nacl.public import PrivateKey, SealedBox

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import freeze_time, connect_backend
from tests.backend.test_user import mock_generate_token


@pytest.fixture
async def configure_device_token(backend, alice):
    configure_device_token = "1234567890"
    with freeze_time("2017-07-07T00:00:00"):
        await backend.user.declare_unconfigured_device(
            configure_device_token, alice.user_id, "phone2"
        )
    return configure_device_token


@pytest.mark.trio
async def test_device_declare(backend, alice, mock_generate_token):
    mock_generate_token.side_effect = ["<token>"]
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "device_declare", "device_name": "phone2"})
        rep = await sock.recv()
    assert rep == {"status": "ok", "configure_device_token": "<token>"}


@pytest.mark.trio
async def test_device_declare_already_exists(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "device_declare", "device_name": alice.device_name})
        rep = await sock.recv()
    assert rep == {"status": "already_exists", "reason": "Device `test` already exists."}


@pytest.mark.parametrize(
    "bad_msg",
    [{"device_name": 42}, {"device_name": None}, {"device_name": "phone2", "unknown": "field"}, {}],
)
@pytest.mark.trio
async def test_device_declare_bad_msg(backend, alice, bad_msg):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "device_declare", **bad_msg})
        rep = await sock.recv()
        assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_device_configure(backend, alice, configure_device_token, mock_generate_token):
    mock_generate_token.side_effect = ["<config_try_id>"]
    verifykey = b"0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-"
    cypherkey = b"\x8b\xfc\xc1\x88\xb7\xd7\x16t\xce<\x7f\xd2j_fTI\x14r':\rF!\xff~\xa8\r\x912\xe3N"

    async with connect_backend(backend, auth_as="anonymous") as anonymous_sock, connect_backend(
        backend, auth_as=alice
    ) as alice_sock:

        # 1) Existing device start listening for device configuration

        await alice_sock.send({"cmd": "event_subscribe", "event": "device_try_claim_submitted"})
        rep = await alice_sock.recv()
        assert rep == {"status": "ok"}

        # 2) Wannabe device start configuration

        await anonymous_sock.send(
            {
                "cmd": "device_configure",
                "user_id": "alice",
                "device_name": "phone2",
                "configure_device_token": configure_device_token,
                "device_verify_key": to_jsonb64(verifykey),
                "user_privkey_cypherkey": to_jsonb64(cypherkey),
            }
        )

        # 3) Existing device receive configuration event

        await alice_sock.send({"cmd": "event_listen"})
        rep = await alice_sock.recv()
        assert rep == {
            "status": "ok",
            "event": "device_try_claim_submitted",
            "subject": "<config_try_id>",
        }

        # 4) Existing device retreive configuration try informations

        await alice_sock.send(
            {"cmd": "device_get_configuration_try", "configuration_try_id": "<config_try_id>"}
        )
        rep = await alice_sock.recv()
        assert rep == {
            "status": "ok",
            "configuration_status": "waiting_answer",
            "device_name": "phone2",
            "device_verify_key": "MLqfWdG0RJMN9qdb6Kr57mG4AZjBfmltfUP63lzmoS0=\n",
            "user_privkey_cypherkey": "i/zBiLfXFnTOPH/Sal9mVEkUcic6DUYh/36oDZEy404=\n",
        }
        user_privkey_cypherkey = PrivateKey(from_jsonb64(rep["user_privkey_cypherkey"]))

        # 5) Existing device accept configuration

        box = SealedBox(user_privkey_cypherkey)
        cyphered_user_privkey = box.encrypt(alice.user_privkey.encode())
        await alice_sock.send(
            {
                "cmd": "device_accept_configuration_try",
                "configuration_try_id": "<config_try_id>",
                "cyphered_user_privkey": to_jsonb64(cyphered_user_privkey),
            }
        )
        rep = await alice_sock.recv()
        assert rep == {"status": "ok"}

        # 6) Wannabe device get it answer: device has been accepted !

        rep = await anonymous_sock.recv()
        assert rep["status"] == "ok"
        transmitted_cyphered_user_privkey = from_jsonb64(rep["cyphered_user_privkey"])
        assert transmitted_cyphered_user_privkey == cyphered_user_privkey


@pytest.mark.trio
async def test_device_configure_und_get_refused(
    backend, alice, configure_device_token, mock_generate_token
):
    mock_generate_token.side_effect = ["<config_try_id>"]

    async with connect_backend(backend, auth_as="anonymous") as anonymous_sock, connect_backend(
        backend, auth_as=alice
    ) as alice_sock:

        # 1) Existing device start listening for device configuration

        await alice_sock.send({"cmd": "event_subscribe", "event": "device_try_claim_submitted"})
        rep = await alice_sock.recv()
        assert rep == {"status": "ok"}

        # 2) Wannabe device start configuration

        await anonymous_sock.send(
            {
                "cmd": "device_configure",
                "user_id": "alice",
                "device_name": "phone2",
                "configure_device_token": configure_device_token,
                "device_verify_key": to_jsonb64(b"<verifykey>"),
                "user_privkey_cypherkey": to_jsonb64(b"<cypherkey>"),
            }
        )

        # 3) Existing device receive configuration event

        await alice_sock.send({"cmd": "event_listen"})
        rep = await alice_sock.recv()
        assert rep == {
            "status": "ok",
            "event": "device_try_claim_submitted",
            "subject": "<config_try_id>",
        }

        # 5) Existing device refuse the configuration

        await alice_sock.send(
            {
                "cmd": "device_refuse_configuration_try",
                "configuration_try_id": "<config_try_id>",
                "reason": "Not in the mood.",
            }
        )
        rep = await alice_sock.recv()
        assert rep == {"status": "ok"}

        # 6) Wannabe device get it answer: device is not accepted :'-(

        rep = await anonymous_sock.recv()
        assert rep == {"status": "configuration_refused", "reason": "Not in the mood."}


@pytest.mark.trio
async def test_device_configure_timeout(autojump_clock, backend, alice, configure_device_token):

    async with connect_backend(backend, auth_as="anonymous") as anonymous_sock:

        # Wannabe device start configuration

        await anonymous_sock.send(
            {
                "cmd": "device_configure",
                "user_id": "alice",
                "device_name": "phone2",
                "configure_device_token": configure_device_token,
                "device_verify_key": to_jsonb64(b"<verifykey>"),
                "user_privkey_cypherkey": to_jsonb64(b"<cypherkey>"),
            }
        )

        # Configuration should timeout after 5mn without answer (autojump_clock
        # fixture make this instantaneous)
        rep = await anonymous_sock.recv()
        assert rep == {
            "status": "timeout",
            "reason": "Timeout while waiting for existing device to validate our configuration.",
        }


@pytest.mark.trio
async def test_device_get_configuration_try_unknown(backend, alice):

    async with connect_backend(backend, auth_as=alice) as alice_sock:

        await alice_sock.send(
            {"cmd": "device_get_configuration_try", "configuration_try_id": "<dummy>"}
        )
        rep = await alice_sock.recv()
        assert rep == {"status": "not_found", "reason": "Unknown device configuration try."}


@pytest.mark.trio
async def test_device_accept_configuration_try_unknown(backend, alice):

    async with connect_backend(backend, auth_as=alice) as alice_sock:

        await alice_sock.send(
            {
                "cmd": "device_accept_configuration_try",
                "configuration_try_id": "<dummy>",
                "cyphered_user_privkey": to_jsonb64(b"whatever..."),
            }
        )
        rep = await alice_sock.recv()
        assert rep == {"status": "not_found", "reason": "Unknown device configuration try."}


@pytest.mark.trio
async def test_device_refuse_configuration_try_unknown(backend, alice):

    async with connect_backend(backend, auth_as=alice) as alice_sock:

        await alice_sock.send(
            {
                "cmd": "device_refuse_configuration_try",
                "configuration_try_id": "<dummy>",
                "reason": "Not in the mood.",
            }
        )
        rep = await alice_sock.recv()
        assert rep == {"status": "not_found", "reason": "Unknown device configuration try."}
