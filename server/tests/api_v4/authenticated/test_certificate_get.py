# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    BootstrapToken,
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    HashAlgorithm,
    HumanHandle,
    OrganizationID,
    PrivateKey,
    PrivateKeyAlgorithm,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SecretKeyAlgorithm,
    SigningKey,
    SigningKeyAlgorithm,
    UserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VlobID,
    authenticated_cmds,
)
from tests.common import (
    AsyncClient,
    AuthenticatedRpcClient,
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    setup_shamir_for_coolorg,
)
from tests.common.data import wksp1_alice_gives_role


@pytest.mark.parametrize("redacted", (False, True))
async def test_authenticated_certificate_get_ok_common_certificates(
    backend: Backend,
    client: AsyncClient,
    redacted: bool,
    cleanup_organizations: None,
) -> None:
    org_id = OrganizationID("Org")
    root_key = SigningKey.generate()

    t0 = DateTime(1999, 12, 31)  # Oldest time: nothing occurred at this point
    t1 = DateTime(
        2000, 1, 1
    )  # Orga is bootstrapped, Alice is created (used as client when not redacted)
    t2 = DateTime(2000, 1, 2)  # Bob is created
    t3 = DateTime(2000, 1, 3)  # Bob profile is changed
    t4 = DateTime(2000, 1, 4)  # Mallory is created (used as client when redacted)
    t5 = DateTime(2000, 1, 5)  # Bob is revoked
    t6 = DateTime(2000, 1, 6)  # Represent current time: nothing should occur after that
    expected_common_certificates = []

    # 1) Org is bootstrapped, Alice is created

    alice_user_id = UserID.test_from_nickname("alice")
    alice1_device_id = DeviceID.test_from_nickname("alice@dev1")
    alice_privkey = PrivateKey.generate()
    alice_signkey = SigningKey.generate()

    alice_user_certificate = UserCertificate(
        author=None,
        timestamp=t1,
        user_id=alice_user_id,
        profile=UserProfile.ADMIN,
        human_handle=HumanHandle(label="Alice", email="alice@example.invalid"),
        public_key=alice_privkey.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
    ).dump_and_sign(root_key)

    alice_redacted_user_certificate = UserCertificate(
        author=None,
        timestamp=t1,
        user_id=alice_user_id,
        profile=UserProfile.ADMIN,
        human_handle=None,
        public_key=alice_privkey.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
    ).dump_and_sign(root_key)

    alice_device_certificate = DeviceCertificate(
        author=None,
        timestamp=t1,
        user_id=alice_user_id,
        device_id=alice1_device_id,
        device_label=DeviceLabel("Dev1"),
        verify_key=alice_signkey.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    ).dump_and_sign(root_key)

    alice_redacted_device_certificate = DeviceCertificate(
        author=None,
        timestamp=t1,
        user_id=alice_user_id,
        device_id=alice1_device_id,
        device_label=None,
        verify_key=alice_signkey.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    ).dump_and_sign(root_key)

    bootstrap_token = BootstrapToken.new()
    await backend.organization.create(now=t1, id=org_id, force_bootstrap_token=bootstrap_token)
    outcome = await backend.organization.bootstrap(
        id=org_id,
        now=t1,
        bootstrap_token=bootstrap_token,
        root_verify_key=root_key.verify_key,
        user_certificate=alice_user_certificate,
        device_certificate=alice_device_certificate,
        redacted_user_certificate=alice_redacted_user_certificate,
        redacted_device_certificate=alice_redacted_device_certificate,
        sequester_authority_certificate=None,
    )
    assert isinstance(outcome, tuple)

    if redacted:
        expected_common_certificates.append(alice_redacted_user_certificate)
        expected_common_certificates.append(alice_redacted_device_certificate)
    else:
        expected_common_certificates.append(alice_user_certificate)
        expected_common_certificates.append(alice_device_certificate)

    # 2) Bob is created

    bob_user_id = UserID.test_from_nickname("bob")
    bob1_device_id = DeviceID.test_from_nickname("bob@dev1")
    bob_privkey = PrivateKey.generate()
    bob_signkey = SigningKey.generate()
    bob_user_certificate = UserCertificate(
        author=alice1_device_id,
        timestamp=t2,
        user_id=bob_user_id,
        profile=UserProfile.STANDARD,
        human_handle=HumanHandle(label="Bob", email="bob@example.invalid"),
        public_key=bob_privkey.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
    ).dump_and_sign(alice_signkey)

    bob_redacted_user_certificate = UserCertificate(
        author=alice1_device_id,
        timestamp=t2,
        user_id=bob_user_id,
        profile=UserProfile.STANDARD,
        human_handle=None,
        public_key=bob_privkey.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
    ).dump_and_sign(alice_signkey)

    bob_device_certificate = DeviceCertificate(
        author=alice1_device_id,
        timestamp=t2,
        user_id=bob_user_id,
        device_id=bob1_device_id,
        device_label=DeviceLabel("Dev1"),
        verify_key=bob_signkey.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    ).dump_and_sign(alice_signkey)

    bob_redacted_device_certificate = DeviceCertificate(
        author=alice1_device_id,
        timestamp=t2,
        user_id=bob_user_id,
        device_id=bob1_device_id,
        device_label=None,
        verify_key=bob_signkey.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    ).dump_and_sign(alice_signkey)

    outcome = await backend.user.create_user(
        organization_id=org_id,
        now=t2,
        author=alice1_device_id,
        author_verify_key=alice_signkey.verify_key,
        user_certificate=bob_user_certificate,
        device_certificate=bob_device_certificate,
        redacted_user_certificate=bob_redacted_user_certificate,
        redacted_device_certificate=bob_redacted_device_certificate,
    )
    assert isinstance(outcome, tuple)

    if redacted:
        expected_common_certificates.append(bob_redacted_user_certificate)
        expected_common_certificates.append(bob_redacted_device_certificate)
    else:
        expected_common_certificates.append(bob_user_certificate)
        expected_common_certificates.append(bob_device_certificate)

    # 3) Bob profile is changed

    bob_user_update_certificate = UserUpdateCertificate(
        author=alice1_device_id,
        timestamp=t3,
        user_id=bob_user_id,
        new_profile=UserProfile.ADMIN,
    ).dump_and_sign(alice_signkey)

    outcome = await backend.user.update_user(
        organization_id=org_id,
        now=t3,
        author=alice1_device_id,
        author_verify_key=alice_signkey.verify_key,
        user_update_certificate=bob_user_update_certificate,
    )
    assert isinstance(outcome, UserUpdateCertificate)

    expected_common_certificates.append(bob_user_update_certificate)

    # 4) Mallory is created

    mallory_privkey = PrivateKey.generate()
    mallory_signkey = SigningKey.generate()
    mallory_user_id = UserID.test_from_nickname("mallory")
    mallory1_device_id = DeviceID.test_from_nickname("mallory@dev1")
    mallory_user_certificate = UserCertificate(
        author=alice1_device_id,
        timestamp=t4,
        user_id=mallory_user_id,
        profile=UserProfile.OUTSIDER,
        human_handle=HumanHandle(label="Mallory", email="mallory@example.invalid"),
        public_key=mallory_privkey.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
    ).dump_and_sign(alice_signkey)

    mallory_redacted_user_certificate = UserCertificate(
        author=alice1_device_id,
        timestamp=t4,
        user_id=mallory_user_id,
        profile=UserProfile.OUTSIDER,
        human_handle=None,
        public_key=mallory_privkey.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
    ).dump_and_sign(alice_signkey)

    mallory_device_certificate = DeviceCertificate(
        author=alice1_device_id,
        timestamp=t4,
        user_id=mallory_user_id,
        device_id=mallory1_device_id,
        device_label=DeviceLabel("Dev1"),
        verify_key=mallory_signkey.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    ).dump_and_sign(alice_signkey)

    mallory_redacted_device_certificate = DeviceCertificate(
        author=alice1_device_id,
        timestamp=t4,
        user_id=mallory_user_id,
        device_id=mallory1_device_id,
        device_label=None,
        verify_key=mallory_signkey.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    ).dump_and_sign(alice_signkey)

    outcome = await backend.user.create_user(
        organization_id=org_id,
        now=t4,
        author=alice1_device_id,
        author_verify_key=alice_signkey.verify_key,
        user_certificate=mallory_user_certificate,
        device_certificate=mallory_device_certificate,
        redacted_user_certificate=mallory_redacted_user_certificate,
        redacted_device_certificate=mallory_redacted_device_certificate,
    )
    assert isinstance(outcome, tuple)

    if redacted:
        expected_common_certificates.append(mallory_redacted_user_certificate)
        expected_common_certificates.append(mallory_redacted_device_certificate)
    else:
        expected_common_certificates.append(mallory_user_certificate)
        expected_common_certificates.append(mallory_device_certificate)

    # 5) Bob is revoked

    bob_revoked_user_certificate = RevokedUserCertificate(
        author=alice1_device_id,
        timestamp=t5,
        user_id=bob_user_id,
    ).dump_and_sign(alice_signkey)

    outcome = await backend.user.revoke_user(
        organization_id=org_id,
        now=t5,
        author=alice1_device_id,
        author_verify_key=alice_signkey.verify_key,
        revoked_user_certificate=bob_revoked_user_certificate,
    )
    assert isinstance(outcome, RevokedUserCertificate)
    expected_common_certificates.append(bob_revoked_user_certificate)

    # Now create the client and do the actual test \o/

    if redacted:
        user_client = AuthenticatedRpcClient(
            client,
            organization_id=org_id,
            user_id=mallory_user_id,
            device_id=mallory1_device_id,
            signing_key=mallory_signkey,
            event=None,
        )
    else:
        user_client = AuthenticatedRpcClient(
            client,
            organization_id=org_id,
            user_id=alice_user_id,
            device_id=alice1_device_id,
            signing_key=alice_signkey,
            event=None,
        )

    # 1) Get all certificates

    rep = await user_client.certificate_get(
        common_after=None, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.common_certificates == expected_common_certificates

    # 2) Get all certificates (with a after timestamp too far in the past)

    rep = await user_client.certificate_get(
        common_after=t0, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.common_certificates == expected_common_certificates

    # 3) Get a subset of the certificates

    rep = await user_client.certificate_get(
        common_after=t1, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.common_certificates == expected_common_certificates[2:]

    # 4) Get a subset of the certificates (no new certificates at all)

    rep = await user_client.certificate_get(
        common_after=t6, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.common_certificates == []


async def test_authenticated_certificate_get_ok_realm_certificates(
    backend: Backend, coolorg: CoolorgRpcClients
) -> None:
    t0 = DateTime(2000, 12, 31)  # Oldest time: nothing occurred at this point
    t1 = DateTime(2001, 1, 1)
    t2 = DateTime(2001, 1, 2)
    t3 = DateTime(2001, 1, 3)
    t4 = DateTime(2001, 1, 4)
    t5 = DateTime(2001, 1, 5)
    t6 = DateTime(2001, 1, 6)

    # Create two realms with certificates
    wksp2_id = VlobID.new()
    wksp3_id = VlobID.new()
    wksp2_certificates = []
    wksp3_certificates = []

    # First retrieve the realms created from the template, we will just ignore them
    rep = await coolorg.alice.certificate_get(
        common_after=None, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert wksp2_id not in rep.realm_certificates
    assert wksp3_id not in rep.realm_certificates
    initial_realm_certificates = rep.realm_certificates

    # /!\ All realm related certificates type should be present here, don't forget to update
    # it if new certificates types are added

    # 1) Create realm wksp2

    certif_timestamp = t1
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=wksp2_id,
        role=RealmRole.OWNER,
        user_id=coolorg.alice.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.create(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=certif,
    )
    assert isinstance(outcome, RealmRoleCertificate)

    wksp2_certificates.append((certif_timestamp, certif))

    # 2) Create another realm wksp3 (to ensure the filter doesn't leak on other realms)

    certif_timestamp = t2
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=wksp3_id,
        role=RealmRole.OWNER,
        user_id=coolorg.alice.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.create(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=certif,
    )
    assert isinstance(outcome, RealmRoleCertificate)

    wksp3_certificates.append((certif_timestamp, certif))

    # 3) Initial key rotation

    certif_timestamp = t3
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=wksp2_id,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=b"<dummy key canary>",
        key_index=1,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.rotate_key(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"<dummy key bundle>",
        per_participant_keys_bundle_access={coolorg.alice.user_id: b"<dummy key bundle access>"},
        realm_key_rotation_certificate=certif,
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    wksp2_certificates.append((certif_timestamp, certif))

    # 4) Initial rename

    certif_timestamp = t4
    certif = RealmNameCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=wksp2_id,
        key_index=1,
        encrypted_name=b"<dummy encrypted name>",
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.rename(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_name_certificate=certif,
        initial_name_or_fail=False,
    )
    assert isinstance(outcome, RealmNameCertificate)

    wksp2_certificates.append((certif_timestamp, certif))

    # 5) Share realm with Bob

    certif_timestamp = t5
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=wksp2_id,
        role=RealmRole.MANAGER,
        user_id=coolorg.bob.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.share(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=certif,
        key_index=1,
        recipient_keys_bundle_access=b"<dummy key bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    wksp2_certificates.append((certif_timestamp, certif))

    # TODO: add RealmArchiving certificate once implemented !

    # 1) Get all certificates

    rep = await coolorg.alice.certificate_get(
        common_after=None, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates == {
        **initial_realm_certificates,
        wksp2_id: [c for _, c in wksp2_certificates],
        wksp3_id: [c for _, c in wksp3_certificates],
    }

    # 2) Get all certificates (with a timestamp too far in the past)

    rep = await coolorg.alice.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={wksp2_id: t0},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates == {
        **initial_realm_certificates,
        wksp2_id: [c for _, c in wksp2_certificates],
        wksp3_id: [c for _, c in wksp3_certificates],
    }

    # 3) Get a subset of the certificates

    rep = await coolorg.alice.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={wksp2_id: t3},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates == {
        **initial_realm_certificates,
        wksp2_id: [c for _, c in wksp2_certificates][2:],  # Skip two certificate
        wksp3_id: [c for _, c in wksp3_certificates],
    }

    # 4) Get a subset of the certificates (no new certificates at all)

    rep = await coolorg.alice.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={wksp2_id: t6},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates == {
        **initial_realm_certificates,
        # Wksp2 is omitted as there is no certificates to return
        wksp3_id: [c for _, c in wksp3_certificates],
    }

    # 5) Getting wksp2's certificates as Bob should provide the same things as for Alice,
    # event if Bob has just been added and hence all certificates are older than him.

    rep = await coolorg.bob.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates[wksp2_id] == [c for _, c in wksp2_certificates]


async def test_authenticated_certificate_get_ok_realm_after_unshare(
    backend: Backend, coolorg: CoolorgRpcClients
) -> None:
    rep = await coolorg.bob.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)

    expected_realm_certifs = rep.realm_certificates

    # Now remove Bob's access, he should still be able to get all the certificates
    # he used to have access to (plus his unshare certificate)...

    (_, raw_certif0) = await wksp1_alice_gives_role(
        coolorg, backend, recipient=coolorg.bob.user_id, new_role=None
    )
    expected_realm_certifs[coolorg.wksp1_id].append(raw_certif0)

    # ...but has no access on new certificates...

    (_, raw_certif1) = await wksp1_alice_gives_role(
        coolorg, backend, recipient=coolorg.mallory.user_id, new_role=RealmRole.READER
    )

    rep = await coolorg.bob.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates == expected_realm_certifs

    # ...until the realm is re-shared with him !

    (_, raw_certif2) = await wksp1_alice_gives_role(
        coolorg, backend, recipient=coolorg.bob.user_id, new_role=RealmRole.CONTRIBUTOR
    )
    expected_realm_certifs[coolorg.wksp1_id].append(raw_certif1)
    expected_realm_certifs[coolorg.wksp1_id].append(raw_certif2)

    rep = await coolorg.bob.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates == expected_realm_certifs


async def test_authenticated_certificate_get_ok_not_part_of_realm(
    backend: Backend, coolorg: CoolorgRpcClients
) -> None:
    rep = await coolorg.mallory.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={coolorg.wksp1_id: DateTime.from_timestamp_seconds(0)},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)

    assert rep.realm_certificates == {}


# TODO: test when user is no longer part of the realm


async def test_authenticated_certificate_get_ok_realm_certificates_no_longer_shared_with(
    backend: Backend, coolorg: CoolorgRpcClients
) -> None:
    t1 = DateTime(2001, 1, 1)
    t2 = DateTime(2001, 1, 2)
    t3 = DateTime(2001, 1, 3)
    t4 = DateTime(2001, 1, 4)
    t5 = DateTime(2001, 1, 5)

    rep = await coolorg.alice.certificate_get(
        common_after=None, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    wksp1_certificates = rep.realm_certificates[coolorg.wksp1_id]

    # At first Mallory is not part of wksp1
    # Then we share with here...

    certif_timestamp = t1
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.CONTRIBUTOR,
        user_id=coolorg.mallory.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.share(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=certif,
        key_index=1,
        recipient_keys_bundle_access=b"<dummy key bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    wksp1_certificates.append(certif)

    # ...do some stuff...

    certif_timestamp = t2
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=coolorg.wksp1_id,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=b"<dummy key canary>",
        key_index=2,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.rotate_key(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"<dummy key bundle>",
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<dummy key bundle access>",
            coolorg.bob.user_id: b"<dummy key bundle access>",
            coolorg.mallory.user_id: b"<dummy key bundle access>",
        },
        realm_key_rotation_certificate=certif,
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    wksp1_certificates.append(certif)

    # ...unshare with here...

    certif_timestamp = t3
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=coolorg.wksp1_id,
        role=None,
        user_id=coolorg.mallory.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.unshare(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=certif,
    )
    assert isinstance(outcome, RealmRoleCertificate)

    wksp1_certificates.append(certif)

    # ...and even more stuff

    certif_timestamp = t4
    certif = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=coolorg.wksp1_id,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=b"<dummy key canary>",
        key_index=3,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.rotate_key(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        keys_bundle=b"<dummy key bundle>",
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<dummy key bundle access>",
            coolorg.bob.user_id: b"<dummy key bundle access>",
        },
        realm_key_rotation_certificate=certif,
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    wksp1_certificates.append(certif)

    # Mallory should be able to see all certificates until she was unshared from the realm

    rep = await coolorg.mallory.certificate_get(
        common_after=None, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates[coolorg.wksp1_id] == wksp1_certificates[:-1]

    # Same thing when passing an after parameter

    rep = await coolorg.mallory.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={coolorg.wksp1_id: t2},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates[coolorg.wksp1_id] == wksp1_certificates[-2:-1]

    rep = await coolorg.mallory.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={coolorg.wksp1_id: t3},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert coolorg.wksp1_id not in rep.realm_certificates

    # Now re-share with Mallory...

    certif_timestamp = t5
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=certif_timestamp,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.READER,
        user_id=coolorg.mallory.user_id,
    ).dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.share(
        now=certif_timestamp,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=certif,
        key_index=3,
        recipient_keys_bundle_access=b"<dummy key bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    wksp1_certificates.append(certif)

    # ...she should see all certificates again

    rep = await coolorg.mallory.certificate_get(
        common_after=None, sequester_after=None, shamir_recovery_after=None, realm_after={}
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.realm_certificates[coolorg.wksp1_id] == wksp1_certificates


async def test_authenticated_certificate_get_ok_shamir(
    backend: Backend, coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")
    a_long_time_ago = DateTime.now()

    # no shamir certificate at first
    rep = await coolorg.alice.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.shamir_recovery_certificates == []

    # setup usual shamir
    (brief, share) = await setup_shamir_for_coolorg(coolorg)

    # checks from alice's (=author) point of view
    rep1 = await coolorg.alice.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=None,
        realm_after={},
    )

    rep2 = await coolorg.alice.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=a_long_time_ago,
        realm_after={},
    )

    assert rep1 == rep2
    assert isinstance(rep1, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep1.shamir_recovery_certificates == [brief]

    # no new shamir
    rep = await coolorg.alice.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=DateTime.now(),
        realm_after={},
    )
    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.shamir_recovery_certificates == []

    # from mallory's (=share recipient) point of view

    rep = await coolorg.mallory.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=a_long_time_ago,
        realm_after={},
    )

    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert set(rep.shamir_recovery_certificates) == set(
        [brief, share]
    )  # these have the same timestamp, so order does not matter

    # from bob's (=not involved) point of view

    rep = await coolorg.bob.certificate_get(
        common_after=None,
        sequester_after=None,
        shamir_recovery_after=a_long_time_ago,
        realm_after={},
    )

    assert isinstance(rep, authenticated_cmds.v4.certificate_get.RepOk)
    assert rep.shamir_recovery_certificates == []

    # TODO check coherence after deletion


async def test_authenticated_certificate_get_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.certificate_get(
            common_after=None, sequester_after=None, shamir_recovery_after=None, realm_after={}
        )

    await authenticated_http_common_errors_tester(do)
