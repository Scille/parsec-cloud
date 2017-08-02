import pytest
from effect2.testing import perform_sequence, const, noop

from parsec.core.privkey import EPrivKeyLoad, EPrivKeyExport, EPrivKeyBackendCmd, PrivKeyBackendComponent
from parsec.core.identity import EIdentityLoad, EIdentityGet

from tests.test_crypto import ALICE_PRIVATE_RSA, mock_crypto_passthrough
from tests.core.test_identity import alice_identity


def test_perform_privkey_export(alice_identity, mock_crypto_passthrough):
    component = PrivKeyBackendComponent(url='ws://dummy')
    eff = component.perform_privkey_export(EPrivKeyExport('P4ssw0rd.'))
    expected_backcmd_msg = {
        'hash': b"dO`\xd6\x01 \xe1e\x95|\xbarZ\x15`\xb2\xbd'\xfcpp\xcb\x97\x02\xde\x9f\xb0Y\xf0I\xd2j",
        'cipherkey': b'<mock-exported-key with password P4ssw0rd.>'
    }
    sequence = [
        (EIdentityGet(), const(alice_identity)),
        (EPrivKeyBackendCmd('privkey_add', expected_backcmd_msg), noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None


def test_perform_privkey_load(alice_identity, mock_crypto_passthrough):
    component = PrivKeyBackendComponent(url='ws://dummy')
    eff = component.perform_privkey_load(EPrivKeyLoad(alice_identity.id, 'P4ssw0rd.'))
    expected_backcmd_msg = {
        'hash': b"dO`\xd6\x01 \xe1e\x95|\xbarZ\x15`\xb2\xbd'\xfcpp\xcb\x97\x02\xde\x9f\xb0Y\xf0I\xd2j"
    }
    sequence = [
        (EPrivKeyBackendCmd('privkey_get', expected_backcmd_msg), const(ALICE_PRIVATE_RSA)),
        (EIdentityLoad(alice_identity.id, password='P4ssw0rd.', key=ALICE_PRIVATE_RSA), noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp is None

# TODO: test EPrivKeyBackendCmd
