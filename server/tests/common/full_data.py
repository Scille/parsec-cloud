# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    DeviceLabel,
    EmailAddress,
    HashAlgorithm,
    HumanHandle,
    PrivateKey,
    RealmKeyRotationCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SecretKeyAlgorithm,
    SigningKey,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VlobID,
)
from tests.common import Backend, CoolorgRpcClients
from tests.common.utils import generate_new_device_certificates, generate_new_user_certificates


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

    zack_u_certs = generate_new_user_certificates(
        DateTime.now(),
        UserID.new(),
        HumanHandle(EmailAddress("zack@invalid.com"), "Zack"),
        UserProfile.STANDARD,
        PrivateKey.generate().public_key,
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    zack_d_certs = generate_new_device_certificates(
        zack_u_certs.certificate.timestamp,
        zack_u_certs.certificate.user_id,
        DeviceID.new(),
        DeviceLabel("pc42"),
        SigningKey.generate().verify_key,
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    outcome = await backend.user.create_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_certificate=zack_u_certs.signed_certificate,
        redacted_user_certificate=zack_u_certs.signed_redacted_certificate,
        device_certificate=zack_d_certs.signed_certificate,
        redacted_device_certificate=zack_d_certs.signed_redacted_certificate,
    )
    assert isinstance(outcome, tuple)

    marty_u_certs = generate_new_user_certificates(
        DateTime.now(),
        UserID.new(),
        HumanHandle(EmailAddress("marty@invalid.com"), "Marty Mc Fly"),
        UserProfile.ADMIN,
        PrivateKey.generate().public_key,
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    marty_d_certs = generate_new_device_certificates(
        marty_u_certs.certificate.timestamp,
        marty_u_certs.certificate.user_id,
        DeviceID.new(),
        DeviceLabel("Overboard"),
        SigningKey.generate().verify_key,
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    outcome = await backend.user.create_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_certificate=marty_u_certs.signed_certificate,
        redacted_user_certificate=marty_u_certs.signed_redacted_certificate,
        device_certificate=marty_d_certs.signed_certificate,
        redacted_device_certificate=marty_d_certs.signed_redacted_certificate,
    )
    assert isinstance(outcome, tuple)

    doc_u_certs = generate_new_user_certificates(
        DateTime.now(),
        UserID.new(),
        HumanHandle(EmailAddress("doc@invalid.com"), "Doc Emmett Brown"),
        UserProfile.ADMIN,
        PrivateKey.generate().public_key,
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    doc_d_certs = generate_new_device_certificates(
        doc_u_certs.certificate.timestamp,
        doc_u_certs.certificate.user_id,
        DeviceID.new(),
        DeviceLabel("Delorean"),
        SigningKey.generate().verify_key,
        author_device_id=coolorg.alice.device_id,
        author_signing_key=coolorg.alice.signing_key,
    )

    outcome = await backend.user.create_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_certificate=doc_u_certs.signed_certificate,
        redacted_user_certificate=doc_u_certs.signed_redacted_certificate,
        device_certificate=doc_d_certs.signed_certificate,
        redacted_device_certificate=doc_d_certs.signed_redacted_certificate,
    )
    assert isinstance(outcome, tuple)

    zack_update1 = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=DateTime.now(),
        user_id=zack_u_certs.certificate.user_id,
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
        user_id=zack_u_certs.certificate.user_id,
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
        user_id=marty_u_certs.certificate.user_id,
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
        user_id=marty_u_certs.certificate.user_id,
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
        user_id=marty_u_certs.certificate.user_id,
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
        user_id=zack_u_certs.certificate.user_id,
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
        user_id=doc_u_certs.certificate.user_id,
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
        user_id=doc_u_certs.certificate.user_id,
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
        user_id=zack_u_certs.certificate.user_id,
    )
    outcome = await backend.user.revoke_user(
        DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        revoked_user_certificate=zack_revoke.dump_and_sign(coolorg.alice.signing_key),
    )
    assert isinstance(outcome, RevokedUserCertificate)
