import pytest
import trio
from nacl.public import PrivateKey, SealedBox
from unittest.mock import patch

from parsec.utils import to_jsonb64, from_jsonb64
from parsec.core.devices_manager import dumps_user_manifest_access

from tests.common import freeze_time


@pytest.fixture
def mock_generate_token():
    with patch("parsec.backend.user._generate_token") as mock_generate_token:
        yield mock_generate_token


@pytest.fixture
async def configure_device_token(backend, alice):
    configure_device_token = "1234567890"
    with freeze_time("2017-07-07"):
        await backend.user.declare_unconfigured_device(
            configure_device_token, alice.user_id, "phone2"
        )
    return configure_device_token


@pytest.mark.trio
async def test_device_declare(alice_backend_sock, mock_generate_token):
    mock_generate_token.side_effect = ["<token>"]
    await alice_backend_sock.send({"cmd": "device_declare", "device_name": "phone2"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "configure_device_token": "<token>"}


@pytest.mark.trio
async def test_device_declare_already_exists(alice, alice_backend_sock):
    await alice_backend_sock.send({"cmd": "device_declare", "device_name": alice.device_name})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "already_exists",
        "reason": f"Device `{alice.device_name}` already exists.",
    }


@pytest.mark.parametrize(
    "bad_msg",
    [{"device_name": 42}, {"device_name": None}, {"device_name": "phone2", "unknown": "field"}, {}],
)
@pytest.mark.trio
async def test_device_declare_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "device_declare", **bad_msg})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_device_configure(
    alice, alice_backend_sock, anonymous_backend_sock, configure_device_token, mock_generate_token
):
    alice_sock, anonymous_sock = alice_backend_sock, anonymous_backend_sock

    mock_generate_token.side_effect = ["<config_try_id>"]
    verifykey = b"0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-"
    cipherkey = b"\x8b\xfc\xc1\x88\xb7\xd7\x16t\xce<\x7f\xd2j_fTI\x14r':\rF!\xff~\xa8\r\x912\xe3N"

    # 1) Existing device start listening for device configuration

    await alice_sock.send({"cmd": "event_subscribe", "event": "device.try_claim_submitted"})
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
            "exchange_cipherkey": to_jsonb64(cipherkey),
        }
    )

    # 3) Existing device receive configuration event

    await alice_sock.send({"cmd": "event_listen"})
    with trio.fail_after(1.0):
        rep = await alice_sock.recv()
    assert rep == {
        "status": "ok",
        "event": "device.try_claim_submitted",
        "device_name": "phone2",
        "config_try_id": "<config_try_id>",
    }

    # 4) Existing device retreive configuration try informations

    await alice_sock.send(
        {"cmd": "device_get_configuration_try", "config_try_id": "<config_try_id>"}
    )
    rep = await alice_sock.recv()
    assert rep == {
        "status": "ok",
        "configuration_status": "waiting_answer",
        "device_name": "phone2",
        "device_verify_key": "MLqfWdG0RJMN9qdb6Kr57mG4AZjBfmltfUP63lzmoS0=\n",
        "exchange_cipherkey": "i/zBiLfXFnTOPH/Sal9mVEkUcic6DUYh/36oDZEy404=\n",
    }
    exchange_cipherkey = PrivateKey(from_jsonb64(rep["exchange_cipherkey"]))

    # 5) Existing device accept configuration

    box = SealedBox(exchange_cipherkey)
    ciphered_user_privkey = box.encrypt(alice.user_privkey.encode())
    ciphered_user_manifest_access = box.encrypt(
        dumps_user_manifest_access(alice.user_manifest_access)
    )
    await alice_sock.send(
        {
            "cmd": "device_accept_configuration_try",
            "config_try_id": "<config_try_id>",
            "ciphered_user_privkey": to_jsonb64(ciphered_user_privkey),
            "ciphered_user_manifest_access": to_jsonb64(ciphered_user_manifest_access),
        }
    )
    rep = await alice_sock.recv()
    assert rep == {"status": "ok"}

    # 6) Wannabe device get it answer: device has been accepted !

    rep = await anonymous_sock.recv()
    assert rep["status"] == "ok"
    transmitted_ciphered_user_privkey = from_jsonb64(rep["ciphered_user_privkey"])
    assert transmitted_ciphered_user_privkey == ciphered_user_privkey


@pytest.mark.trio
async def test_device_configure_and_get_refused(
    alice_backend_sock, anonymous_backend_sock, configure_device_token, mock_generate_token
):
    mock_generate_token.side_effect = ["<config_try_id>"]
    alice_sock, anonymous_sock = alice_backend_sock, anonymous_backend_sock

    # 1) Existing device start listening for device configuration

    await alice_sock.send({"cmd": "event_subscribe", "event": "device.try_claim_submitted"})
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
            "exchange_cipherkey": to_jsonb64(b"<cipherkey>"),
        }
    )

    # 3) Existing device receive configuration event

    await alice_sock.send({"cmd": "event_listen"})
    with trio.fail_after(1.0):
        rep = await alice_sock.recv()
    assert rep == {
        "status": "ok",
        "event": "device.try_claim_submitted",
        "device_name": "phone2",
        "config_try_id": "<config_try_id>",
    }

    # 5) Existing device refuse the configuration

    await alice_sock.send(
        {
            "cmd": "device_refuse_configuration_try",
            "config_try_id": "<config_try_id>",
            "reason": "Not in the mood.",
        }
    )
    rep = await alice_sock.recv()
    assert rep == {"status": "ok"}

    # 6) Wannabe device get it answer: device is not accepted :'-(

    rep = await anonymous_sock.recv()
    assert rep == {"status": "configuration_refused", "reason": "Not in the mood."}


@pytest.mark.trio
async def test_device_configure_timeout(anonymous_backend_sock, configure_device_token, mock_clock):
    anonymous_sock = anonymous_backend_sock

    # Wannabe device start configuration

    await anonymous_sock.send(
        {
            "cmd": "device_configure",
            "user_id": "alice",
            "device_name": "phone2",
            "configure_device_token": configure_device_token,
            "device_verify_key": to_jsonb64(b"<verifykey>"),
            "exchange_cipherkey": to_jsonb64(b"<cipherkey>"),
        }
    )

    # Configuration should timeout after 5mn without answer
    mock_clock.rate = 1
    mock_clock.autojump_threshold = 0.1
    rep = await anonymous_sock.recv()
    assert rep == {
        "status": "timeout",
        "reason": "Timeout while waiting for existing device to validate our configuration.",
    }


@pytest.mark.trio
async def test_device_get_configuration_try_unknown(alice_backend_sock):
    await alice_backend_sock.send(
        {"cmd": "device_get_configuration_try", "config_try_id": "<dummy>"}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found", "reason": "Unknown device configuration try."}


@pytest.mark.trio
async def test_device_accept_configuration_try_unknown(alice_backend_sock):
    await alice_backend_sock.send(
        {
            "cmd": "device_accept_configuration_try",
            "config_try_id": "<dummy>",
            "ciphered_user_privkey": to_jsonb64(b"whatever..."),
            "ciphered_user_manifest_access": to_jsonb64(b"whatever..."),
        }
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found", "reason": "Unknown device configuration try."}


@pytest.mark.trio
async def test_device_refuse_configuration_try_unknown(alice_backend_sock):
    await alice_backend_sock.send(
        {
            "cmd": "device_refuse_configuration_try",
            "config_try_id": "<dummy>",
            "reason": "Not in the mood.",
        }
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found", "reason": "Unknown device configuration try."}
