# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Awaitable, Callable

import pytest

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    SigningKey,
    SigningKeyAlgorithm,
    VlobID,
)
from tests.common import Backend, CoolorgRpcClients


@pytest.fixture(params=("common_certificate", "realm_certificate", "shamir_certificate", "vlob"))
def alice_generated_data(
    request: pytest.FixtureRequest,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> Callable[[DateTime], Awaitable[None]]:
    """
    Parametrized fixture to generate all the different types of data a user can create.

    This is to be used when testing APIs that change the user capabilities (e.g.
    `realm_unshare`) to ensure the change cannot be done if its timestamp is older
    than data previously created by the user.

    Note, while being part of the realm's data, blocks are not present here. This
    is simply because blocks don't contain timestamp information.
    """
    match request.param:
        case "common_certificate":

            async def _alice_generated_data(now: DateTime) -> None:
                device_id = DeviceID.new()
                verify_key = SigningKey.generate().verify_key

                device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=DeviceLabel("New device"),
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                redacted_device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=None,
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.user.create_device(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    device_certificate=device_certif,
                    redacted_device_certificate=redacted_device_certif,
                )
                assert isinstance(outcome, DeviceCertificate)

        case "realm_certificate":

            async def _alice_generated_data(now: DateTime) -> None:
                realm_role_certif = RealmRoleCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    realm_id=VlobID.new(),
                    user_id=coolorg.alice.user_id,
                    role=RealmRole.OWNER,
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.realm.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    realm_role_certificate=realm_role_certif,
                )
                assert isinstance(outcome, RealmRoleCertificate)

        case "shamir_certificate":
            # TODO: implement this once Shamir support is done !
            pytest.skip(reason="shamir not implemented")

        case "vlob":

            async def _alice_generated_data(now: DateTime) -> None:
                outcome = await backend.vlob.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    vlob_id=VlobID.new(),
                    key_index=1,
                    timestamp=now,
                    blob=b"<vlob data>",
                )
                assert outcome is None

        case unknown:
            assert False, unknown

    return _alice_generated_data


@pytest.fixture(params=("common_certificate", "realm_certificate", "vlob"))
def alice_generated_realm_wksp1_data(
    request: pytest.FixtureRequest,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> Callable[[DateTime], Awaitable[None]]:
    """
    Parametrized fixture to generate all the different types of data a user can create
    in a realm.

    This is to be used when testing APIs that change the user capabilities (e.g.
    `realm_unshare`) to ensure the change cannot be done if its timestamp is older
    than data previously created by the user.

    Note, while being part of the realm's data, blocks are not present here. This
    is simply because blocks don't contain timestamp information.
    """
    match request.param:
        case "common_certificate":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                device_id = DeviceID.new()
                verify_key = SigningKey.generate().verify_key

                device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=DeviceLabel("New device"),
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                redacted_device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=None,
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.user.create_device(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    device_certificate=device_certif,
                    redacted_device_certificate=redacted_device_certif,
                )
                assert isinstance(outcome, DeviceCertificate), outcome

        case "realm_certificate":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                realm_name_certificate = RealmNameCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    realm_id=coolorg.wksp1_id,
                    key_index=1,
                    encrypted_name=b"<dummy encrypted name>",
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.realm.rename(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    realm_name_certificate=realm_name_certificate,
                    initial_name_or_fail=False,
                )
                assert isinstance(outcome, RealmNameCertificate), outcome

        case "vlob":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                outcome = await backend.vlob.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    vlob_id=VlobID.new(),
                    key_index=1,
                    timestamp=now,
                    blob=b"<vlob data>",
                )
                assert outcome is None, outcome

        case unknown:
            assert False, unknown

    return _alice_generated_realm_data
