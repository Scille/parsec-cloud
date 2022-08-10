# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import trio
import pytest
from parsec._parsec import DateTime
from collections import defaultdict

from parsec.api.data import UserCertificateContent, DeviceCertificateContent, UserProfile
from parsec.api.protocol import UserID, apiv1_organization_bootstrap_serializer
from parsec.api.protocol.handshake import HandshakeOrganizationExpired

from tests.common import customize_fixtures, local_device_to_backend_user
from tests.backend.common import ping


_missing = object()


async def organization_bootstrap(
    sock,
    bootstrap_token,
    user_certificate,
    device_certificate,
    root_verify_key,
    redacted_user_certificate=_missing,
    redacted_device_certificate=_missing,
):
    data = {
        "cmd": "organization_bootstrap",
        "bootstrap_token": bootstrap_token,
        "user_certificate": user_certificate,
        "device_certificate": device_certificate,
        "root_verify_key": root_verify_key,
    }

    if redacted_user_certificate is not _missing:
        data["redacted_user_certificate"] = redacted_user_certificate
    if redacted_device_certificate is not _missing:
        data["redacted_device_certificate"] = redacted_device_certificate

    raw_rep = await sock.send(apiv1_organization_bootstrap_serializer.req_dumps(data))
    raw_rep = await sock.receive()
    return apiv1_organization_bootstrap_serializer.rep_loads(raw_rep)


@pytest.mark.trio
@customize_fixtures(backend_has_webhook=True)
async def test_organization_bootstrap(
    webhook_spy,
    backend_asgi_app,
    organization_factory,
    local_device_factory,
    alice,
    apiv1_backend_ws_factory,
    backend_authenticated_ws_factory,
    unused_tcp_port,
):
    neworg = organization_factory("NewOrg")
    await backend_asgi_app.backend.organization.create(
        id=neworg.organization_id, bootstrap_token=neworg.bootstrap_token
    )

    # 1) Bootstrap organization

    # Use an existing user name to make sure they didn't mix together
    newalice = local_device_factory(alice.device_id, neworg, profile=UserProfile.ADMIN)
    backend_newalice, backend_newalice_first_device = local_device_to_backend_user(newalice, neworg)

    async with apiv1_backend_ws_factory(backend_asgi_app, neworg.organization_id) as sock:
        rep = await organization_bootstrap(
            sock,
            bootstrap_token=neworg.bootstrap_token,
            user_certificate=backend_newalice.user_certificate,
            device_certificate=backend_newalice_first_device.device_certificate,
            root_verify_key=neworg.root_verify_key,
            redacted_user_certificate=backend_newalice.redacted_user_certificate,
            redacted_device_certificate=backend_newalice_first_device.redacted_device_certificate,
        )
    assert rep == {"status": "ok"}

    # Ensure webhook has been triggered
    assert webhook_spy == [
        (
            f"http://127.0.0.1:{unused_tcp_port}/webhook",
            {
                "device_id": "alice@dev1",
                "device_label": "My dev1 machine",
                "human_email": "alice@example.com",
                "human_label": "Alicey McAliceFace",
                "organization_id": "NewOrg",
            },
        )
    ]

    # 2) Now our new device can connect the backend

    async with backend_authenticated_ws_factory(backend_asgi_app, newalice) as sock:
        await ping(sock)

    # 3) Make sure alice from the other organization is still working

    async with backend_authenticated_ws_factory(backend_asgi_app, alice) as sock:
        await ping(sock)

    # 4) Finally, check the resulting data in the backend
    backend_user, backend_device = await backend_asgi_app.backend.user.get_user_with_device(
        newalice.organization_id, newalice.device_id
    )
    assert backend_user == backend_newalice
    assert backend_device == backend_newalice_first_device


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_organization_expired_create_and_bootstrap(
    backend_asgi_app, organization_factory, apiv1_backend_ws_factory
):
    neworg = organization_factory("NewOrg")

    # 1) Create an organization and mark it expired
    await backend_asgi_app.backend.organization.create(
        id=neworg.organization_id, bootstrap_token=neworg.bootstrap_token
    )
    await backend_asgi_app.backend.organization.update(id=neworg.organization_id, is_expired=True)

    # 2) Connection to backend for bootstrap purpose is not possible

    with pytest.raises(HandshakeOrganizationExpired):
        async with apiv1_backend_ws_factory(backend_asgi_app, neworg.organization_id):
            pass

    # 3) Now re-create the organization to overwrite the expiration date
    await backend_asgi_app.backend.organization.create(
        id=neworg.organization_id, bootstrap_token=neworg.bootstrap_token
    )

    # 4) This time, bootstrap is possible

    async with apiv1_backend_ws_factory(backend_asgi_app, neworg.organization_id):
        pass


@pytest.mark.trio
async def test_organization_bootstrap_bad_data(
    apiv1_backend_ws_factory, organization_factory, local_device_factory, backend_asgi_app, coolorg
):
    neworg = organization_factory("NewOrg")
    newalice = local_device_factory("alice@dev1", neworg)
    await backend_asgi_app.backend.organization.create(
        id=neworg.organization_id, bootstrap_token=neworg.bootstrap_token
    )

    bad_organization_id = coolorg.organization_id
    good_organization_id = neworg.organization_id

    root_signing_key = neworg.root_signing_key
    bad_root_signing_key = coolorg.root_signing_key

    good_bootstrap_token = neworg.bootstrap_token
    bad_bootstrap_token = coolorg.bootstrap_token

    good_rvk = neworg.root_verify_key
    bad_rvk = coolorg.root_verify_key

    good_device_id = newalice.device_id

    good_user_id = newalice.user_id
    bad_user_id = UserID("dummy")

    public_key = newalice.public_key
    verify_key = newalice.verify_key

    now = DateTime.now()
    bad_now = now.subtract(seconds=1)

    good_cu = UserCertificateContent(
        author=None,
        timestamp=now,
        user_id=good_user_id,
        public_key=public_key,
        profile=UserProfile.ADMIN,
        human_handle=newalice.human_handle,
    )
    good_redacted_cu = good_cu.evolve(human_handle=None)
    good_cd = DeviceCertificateContent(
        author=None,
        timestamp=now,
        device_id=good_device_id,
        device_label=newalice.device_label,
        verify_key=verify_key,
    )
    good_redacted_cd = good_cd.evolve(device_label=None)
    bad_now_cu = good_cu.evolve(timestamp=bad_now)
    bad_now_cd = good_cd.evolve(timestamp=bad_now)
    bad_now_redacted_cu = good_redacted_cu.evolve(timestamp=bad_now)
    bad_now_redacted_cd = good_redacted_cd.evolve(timestamp=bad_now)
    bad_id_cu = good_cu.evolve(user_id=bad_user_id)
    bad_not_admin_cu = good_cu.evolve(profile=UserProfile.STANDARD)

    bad_key_cu = good_cu.dump_and_sign(bad_root_signing_key)
    bad_key_cd = good_cd.dump_and_sign(bad_root_signing_key)

    good_cu = good_cu.dump_and_sign(root_signing_key)
    good_redacted_cu = good_redacted_cu.dump_and_sign(root_signing_key)
    good_cd = good_cd.dump_and_sign(root_signing_key)
    good_redacted_cd = good_redacted_cd.dump_and_sign(root_signing_key)
    bad_now_cu = bad_now_cu.dump_and_sign(root_signing_key)
    bad_now_cd = bad_now_cd.dump_and_sign(root_signing_key)
    bad_now_redacted_cu = bad_now_redacted_cu.dump_and_sign(root_signing_key)
    bad_now_redacted_cd = bad_now_redacted_cd.dump_and_sign(root_signing_key)
    bad_id_cu = bad_id_cu.dump_and_sign(root_signing_key)
    bad_not_admin_cu = bad_not_admin_cu.dump_and_sign(root_signing_key)

    for i, (status, organization_id, *params) in enumerate(
        [
            ("not_found", good_organization_id, bad_bootstrap_token, good_cu, good_cd, good_rvk),
            (
                "already_bootstrapped",
                bad_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
            ),
            (
                "invalid_certification",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                bad_rvk,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                bad_now_cu,
                good_cd,
                good_rvk,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                bad_id_cu,
                good_cd,
                good_rvk,
            ),
            (
                "invalid_certification",
                good_organization_id,
                good_bootstrap_token,
                bad_key_cu,
                good_cd,
                good_rvk,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                bad_now_cd,
                good_rvk,
            ),
            (
                "invalid_certification",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                bad_key_cd,
                good_rvk,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                bad_not_admin_cu,
                good_cd,
                good_rvk,
            ),
            # Tests with redacted certificates
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                good_cu,  # Not redacted !
                good_redacted_cd,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                good_redacted_cu,
                good_cd,  # Not redacted !
            ),
            (
                "bad_message",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                None,  # None not allowed
                good_redacted_cd,
            ),
            (
                "bad_message",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                good_redacted_cu,
                None,  # None not allowed
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                bad_now_redacted_cu,
                good_redacted_cd,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                good_redacted_cu,
                bad_now_redacted_cd,
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                good_redacted_cu,
                _missing,  # Must proved redacted_device if redacted user is present
            ),
            (
                "invalid_data",
                good_organization_id,
                good_bootstrap_token,
                good_cu,
                good_cd,
                good_rvk,
                _missing,  # Must proved redacted_device if redacted user is present
                good_redacted_cd,
            ),
        ]
    ):
        print(f"sub test {i}")
        async with apiv1_backend_ws_factory(backend_asgi_app, organization_id) as sock:
            rep = await organization_bootstrap(sock, *params)
        assert rep["status"] == status

    # Finally cheap test to make sure our "good" data were really good
    async with apiv1_backend_ws_factory(backend_asgi_app, good_organization_id) as sock:
        rep = await organization_bootstrap(
            sock,
            good_bootstrap_token,
            good_cu,
            good_cd,
            good_rvk,
            good_redacted_cu,
            good_redacted_cd,
        )
    assert rep["status"] == "ok"


@pytest.mark.trio
@pytest.mark.parametrize("flavour", ["no_create", "create_different_token", "create_same_token"])
@customize_fixtures(backend_spontaneous_organization_bootstrap=True, backend_not_populated=True)
async def test_organization_spontaneous_bootstrap(
    backend_asgi_app, organization_factory, local_device_factory, apiv1_backend_ws_factory, flavour
):
    neworg = organization_factory("NewOrg")
    # Spontaneous bootstrap must have empty token
    empty_token = ""

    # Step 1: organization creation (if needed)

    if flavour == "create_same_token":
        # Basically pretent we already tried the spontaneous
        # bootstrap but got interrupted
        step1_token = empty_token
    else:
        # Administration explicitly created an organization,
        # we shouldn't be able to overwrite it
        step1_token = "123"
    if flavour != "no_create":
        await backend_asgi_app.backend.organization.create(
            id=neworg.organization_id, bootstrap_token=step1_token
        )

    # Step 2: organization bootstrap

    newalice = local_device_factory(org=neworg, profile=UserProfile.ADMIN)
    backend_newalice, backend_newalice_first_device = local_device_to_backend_user(newalice, neworg)
    async with apiv1_backend_ws_factory(backend_asgi_app, neworg.organization_id) as sock:
        rep = await organization_bootstrap(
            sock,
            empty_token,
            backend_newalice.user_certificate,
            backend_newalice_first_device.device_certificate,
            neworg.root_verify_key,
        )
        if flavour == "create_different_token":
            assert rep == {"status": "not_found"}
        else:
            assert rep == {"status": "ok"}

    # Check organization informations

    org = await backend_asgi_app.backend.organization.get(id=neworg.organization_id)
    if flavour == "create_different_token":
        assert org.root_verify_key is None
        assert org.bootstrap_token == step1_token
    else:
        assert org.root_verify_key == neworg.root_verify_key
        assert org.bootstrap_token == empty_token


@pytest.mark.trio
@customize_fixtures(backend_spontaneous_organization_bootstrap=True, backend_not_populated=True)
async def test_concurrent_organization_bootstrap(
    backend_asgi_app, organization_factory, local_device_factory, apiv1_backend_ws_factory
):
    concurrency = 5
    unleash_bootstraps = trio.Event()
    neworg = organization_factory("NewOrg")
    empty_token = ""
    results = defaultdict(lambda: 0)

    async def _do_bootstrap(task_status=trio.TASK_STATUS_IGNORED):
        newalice = local_device_factory(
            org=neworg, profile=UserProfile.ADMIN, base_human_handle="alice <alice@example.com>"
        )
        backend_newalice, backend_newalice_first_device = local_device_to_backend_user(
            newalice, neworg
        )
        async with apiv1_backend_ws_factory(backend_asgi_app, neworg.organization_id) as sock:
            task_status.started()
            await unleash_bootstraps.wait()
            rep = await organization_bootstrap(
                sock=sock,
                bootstrap_token=empty_token,
                user_certificate=backend_newalice.user_certificate,
                device_certificate=backend_newalice_first_device.device_certificate,
                redacted_user_certificate=backend_newalice.redacted_user_certificate,
                redacted_device_certificate=backend_newalice_first_device.redacted_device_certificate,
                root_verify_key=neworg.root_verify_key,
            )
            results[rep["status"]] += 1

    async with trio.open_nursery() as nursery:
        for i in range(concurrency):
            await nursery.start(_do_bootstrap)
        unleash_bootstraps.set()

    assert results == {"ok": 1, "already_bootstrapped": concurrency - 1}
