# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    DevicePurpose,
    EmailAddress,
    HashAlgorithm,
    HumanHandle,
    PrivateKey,
    PrivateKeyAlgorithm,
    RealmKeyRotationCertificate,
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
)
from tests.common import Backend, CoolorgRpcClients


async def insert_full_data(
    coolorg: CoolorgRpcClients,
    backend: Backend,
):
    """
    Consider this function as a helper to copy/paste what you need when writing
    other tests.
    On top of that it is useful when you need generate plenty of data (e.g. to
    generate the data for PostgreSQL migration tests).
    """
    await backend.block.create(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        block_id=BlockID.new(),
        key_index=1,
        block=b"block 1",
    )
    await backend.block.create(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        block_id=BlockID.new(),
        key_index=1,
        block=b"block 2",
    )

    zack_u = UserCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=UserID.new(),
        human_handle=HumanHandle(EmailAddress("zack@invalid.com"), "Zack"),
        public_key=PrivateKey.generate().public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=UserProfile.STANDARD,
    )
    zack_ur = UserCertificate(
        author=coolorg.alice.device_id,
        timestamp=zack_u.timestamp,
        user_id=zack_u.user_id,
        human_handle=None,
        public_key=zack_u.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=UserProfile.STANDARD,
    )
    zack_d = DeviceCertificate(
        author=coolorg.alice.device_id,
        timestamp=zack_u.timestamp,
        purpose=DevicePurpose.STANDARD,
        user_id=zack_u.user_id,
        device_id=DeviceID.new(),
        device_label=DeviceLabel("pc42"),
        verify_key=SigningKey.generate().verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    )
    zack_dr = DeviceCertificate(
        author=coolorg.alice.device_id,
        timestamp=zack_u.timestamp,
        purpose=DevicePurpose.STANDARD,
        user_id=zack_u.user_id,
        device_id=zack_d.device_id,
        device_label=None,
        verify_key=zack_d.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    )
    outcome = await backend.user.create_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_certificate=zack_u.dump_and_sign(coolorg.alice.signing_key),
        redacted_user_certificate=zack_ur.dump_and_sign(coolorg.alice.signing_key),
        device_certificate=zack_d.dump_and_sign(coolorg.alice.signing_key),
        redacted_device_certificate=zack_dr.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, tuple)

    marty_u = UserCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=UserID.new(),
        human_handle=HumanHandle(EmailAddress("marty@invalid.com"), "Marty Mc Fly"),
        public_key=PrivateKey.generate().public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=UserProfile.ADMIN,
    )
    marty_ur = UserCertificate(
        author=coolorg.alice.device_id,
        timestamp=marty_u.timestamp,
        user_id=marty_u.user_id,
        human_handle=None,
        public_key=marty_u.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=UserProfile.ADMIN,
    )
    marty_d = DeviceCertificate(
        author=coolorg.alice.device_id,
        timestamp=marty_u.timestamp,
        purpose=DevicePurpose.STANDARD,
        user_id=marty_u.user_id,
        device_id=DeviceID.new(),
        device_label=DeviceLabel("Overboard"),
        verify_key=SigningKey.generate().verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    )
    marty_dr = DeviceCertificate(
        author=coolorg.alice.device_id,
        timestamp=marty_u.timestamp,
        purpose=DevicePurpose.STANDARD,
        user_id=marty_u.user_id,
        device_id=marty_d.device_id,
        device_label=None,
        verify_key=marty_d.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    )
    outcome = await backend.user.create_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_certificate=marty_u.dump_and_sign(coolorg.alice.signing_key),
        redacted_user_certificate=marty_ur.dump_and_sign(coolorg.alice.signing_key),
        device_certificate=marty_d.dump_and_sign(coolorg.alice.signing_key),
        redacted_device_certificate=marty_dr.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, tuple)

    doc_u = UserCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=UserID.new(),
        human_handle=HumanHandle(EmailAddress("doc@invalid.com"), "Doc Emmett Brown"),
        public_key=PrivateKey.generate().public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=UserProfile.ADMIN,
    )
    doc_ur = UserCertificate(
        author=coolorg.alice.device_id,
        timestamp=doc_u.timestamp,
        user_id=doc_u.user_id,
        human_handle=None,
        public_key=doc_u.public_key,
        algorithm=PrivateKeyAlgorithm.X25519_XSALSA20_POLY1305,
        profile=UserProfile.ADMIN,
    )
    doc_d = DeviceCertificate(
        author=coolorg.alice.device_id,
        timestamp=doc_u.timestamp,
        purpose=DevicePurpose.STANDARD,
        user_id=doc_u.user_id,
        device_id=DeviceID.new(),
        device_label=DeviceLabel("Delorean"),
        verify_key=SigningKey.generate().verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    )
    doc_dr = DeviceCertificate(
        author=coolorg.alice.device_id,
        timestamp=doc_u.timestamp,
        purpose=DevicePurpose.STANDARD,
        user_id=doc_u.user_id,
        device_id=doc_d.device_id,
        device_label=None,
        verify_key=doc_d.verify_key,
        algorithm=SigningKeyAlgorithm.ED25519,
    )

    outcome = await backend.user.create_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_certificate=doc_u.dump_and_sign(coolorg.alice.signing_key),
        redacted_user_certificate=doc_ur.dump_and_sign(coolorg.alice.signing_key),
        device_certificate=doc_d.dump_and_sign(coolorg.alice.signing_key),
        redacted_device_certificate=doc_dr.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, tuple)

    zack_update1 = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=zack_u.user_id,
        new_profile=UserProfile.OUTSIDER,
    )
    outcome = await backend.user.update_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_update_certificate=zack_update1.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, UserUpdateCertificate)
    zack_update2 = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=zack_u.user_id,
        new_profile=UserProfile.ADMIN,
    )
    outcome = await backend.user.update_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_update_certificate=zack_update2.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, UserUpdateCertificate)

    marty_update1 = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=marty_u.user_id,
        new_profile=UserProfile.STANDARD,
    )
    outcome = await backend.user.update_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_update_certificate=marty_update1.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, UserUpdateCertificate)

    blob1 = VlobID.new()
    outcome = await backend.vlob.create(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=blob1,
        key_index=1,
        timestamp=DateTime.now(),
        blob=b"blob 1 v1",
    )
    assert outcome is None

    realm_key_rotation_certificate = RealmKeyRotationCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        realm_id=coolorg.wksp1_id,
        key_index=2,
        encryption_algorithm=SecretKeyAlgorithm.BLAKE2B_XSALSA20_POLY1305,
        hash_algorithm=HashAlgorithm.SHA256,
        key_canary=b"<dummy canary>",
    )
    outcome = await backend.realm.rotate_key(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_key_rotation_certificate=realm_key_rotation_certificate.dump_and_sign(
            coolorg.alice.signing_key
        ),
        per_participant_keys_bundle_access={
            coolorg.alice.user_id: b"<alice dummy keys bundle access>",
            coolorg.bob.user_id: b"<bob dummy keys bundle access>",
        },
        keys_bundle=b"<dummy keys bundle>",
    )
    assert isinstance(outcome, RealmKeyRotationCertificate)

    x = await backend.vlob.create(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=VlobID.new(),
        key_index=2,
        timestamp=DateTime.now(),
        blob=b"blob 2 v1",
    )
    assert x is None

    outcome = await backend.vlob.update(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=blob1,
        key_index=2,
        version=2,
        timestamp=DateTime.now(),
        blob=b"blob 1 v2",
    )
    assert x is None

    marty_rr1 = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=marty_u.user_id,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.READER,
    )
    outcome = await backend.realm.share(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        key_index=2,
        realm_role_certificate=marty_rr1.dump_and_sign(coolorg.alice.signing_key),
        recipient_keys_bundle_access=b"<marty keys bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    marty_rr2 = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=marty_u.user_id,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.CONTRIBUTOR,
    )
    outcome = await backend.realm.share(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        key_index=2,
        realm_role_certificate=marty_rr2.dump_and_sign(coolorg.alice.signing_key),
        recipient_keys_bundle_access=b"<marty keys bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    zack_rr1 = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=zack_u.user_id,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.MANAGER,
    )
    outcome = await backend.realm.share(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        key_index=2,
        realm_role_certificate=zack_rr1.dump_and_sign(coolorg.alice.signing_key),
        recipient_keys_bundle_access=b"<marty keys bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    doc_rr1 = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=doc_u.user_id,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.CONTRIBUTOR,
    )
    outcome = await backend.realm.share(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        key_index=2,
        realm_role_certificate=doc_rr1.dump_and_sign(coolorg.alice.signing_key),
        recipient_keys_bundle_access=b"<doc keys bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    doc_rr2 = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=doc_u.user_id,
        realm_id=coolorg.wksp1_id,
        role=None,
    )
    outcome = await backend.realm.unshare(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=doc_rr2.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RealmRoleCertificate)

    zack_revoke = RevokedUserCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=zack_u.user_id,
    )
    outcome = await backend.user.revoke_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=zack_revoke.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RevokedUserCertificate)
