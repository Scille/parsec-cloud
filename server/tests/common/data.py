# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Awaitable, Callable

import pytest

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    RealmRole,
    RealmRoleCertificate,
    SigningKey,
    SigningKeyAlgorithm,
    VlobID,
)
from tests.common import Backend, CoolorgRpcClients


@pytest.fixture(
    params=("common_certificate", "realm_certificate", "shamir_certificate", "vlob", "block")
)
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

                await backend.user.create_device(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    device_certificate=device_certif,
                    redacted_device_certificate=redacted_device_certif,
                )

        case "realm_certificate":

            async def _alice_generated_data(now: DateTime) -> None:
                realm_role_certif = RealmRoleCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    realm_id=VlobID.new(),
                    user_id=coolorg.alice.user_id,
                    role=RealmRole.OWNER,
                ).dump_and_sign(coolorg.alice.signing_key)

                await backend.realm.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    realm_role_certificate=realm_role_certif,
                )

        case "shamir_certificate":
            # TODO: implement this once Shamir support is done !
            pytest.skip(reason="shamir not implemented")

        case "vlob":

            async def _alice_generated_data(now: DateTime) -> None:
                await backend.vlob.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    vlob_id=VlobID.new(),
                    key_index=1,
                    timestamp=now,
                    blob=b"<vlob data>",
                )

        case "block":

            async def _alice_generated_data(now: DateTime) -> None:
                await backend.block.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    key_index=1,
                    block_id=BlockID.new(),
                    block=b"<block data>",
                )

        case unknown:
            assert False, unknown

    return _alice_generated_data


@pytest.fixture(params=("realm_certificate", "vlob", "block"))
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
    """
    match request.param:
        case "realm_certificate":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                realm_role_certif = RealmRoleCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    realm_id=VlobID.new(),
                    user_id=coolorg.alice.user_id,
                    role=RealmRole.OWNER,
                ).dump_and_sign(coolorg.alice.signing_key)

                await backend.realm.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    realm_role_certificate=realm_role_certif,
                )

        case "vlob":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                await backend.vlob.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    vlob_id=VlobID.new(),
                    key_index=1,
                    timestamp=now,
                    blob=b"<vlob data>",
                )

        case "block":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                await backend.block.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    key_index=1,
                    block_id=BlockID.new(),
                    block=b"<block data>",
                )

        case unknown:
            assert False, unknown

    return _alice_generated_realm_data
