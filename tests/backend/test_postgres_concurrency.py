# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio
import triopg
from uuid import uuid4
from contextlib import contextmanager
from parsec._parsec import DateTime
from unittest.mock import patch

from parsec.backend.organization import OrganizationAlreadyBootstrappedError
from parsec.backend.user import UserAlreadyExistsError, UserActiveUsersLimitReached
from parsec.backend.pki import PkiEnrollmentNoLongerAvailableError

from tests.common import local_device_to_backend_user


# Testing concurrency interractions is hard given it involve precise timing
# (otherwise the test appear to be concurrent, but the queries are in fact
# executed one after another...)
# The solution we choose here is to send multiple concurrent queries and
# try to ensure they are reaching the sensitive part roughly at the same
# time.
# Of course this is far from perfect (and produce non-reproductible errors...)
# but better than nothing ;-)


@contextmanager
def ensure_pg_transaction_concurrency_barrier(concurrency: int = 2):
    # In theory we would want to plug into all triopg connection methods that
    # can send request to PostgreSQL (so execute, fetch etc.), but this is
    # cumbersome given triopg is a dynamic wrapper over asyncpg...
    # In spite of that we consider a transaction block is always opened for
    # operations that involve concurrency issues. This seems like a reasonable
    # bet given a single INSERT/UPDATE without transaction is atomic all by
    # itself (including the SELECT inlined in the query).

    from triopg._triopg import TrioTransactionProxy

    current_concurrency = 0
    concurrency_reached = trio.Event()

    class PatchedTrioTransactionProxy(TrioTransactionProxy):
        async def __aenter__(self, *args, **kwargs):
            nonlocal current_concurrency
            current_concurrency += 1
            if current_concurrency >= concurrency:
                concurrency_reached.set()
            await concurrency_reached.wait()

            return await super().__aenter__(*args, **kwargs)

    with patch("triopg._triopg.TrioTransactionProxy", PatchedTrioTransactionProxy):
        yield

    assert current_concurrency >= concurrency  # Sanity check


@pytest.mark.trio
@pytest.mark.postgresql
async def test_concurrency_bootstrap_organization(postgresql_url, backend_factory, coolorg, alice):
    results = []

    backend_user, backend_first_device = local_device_to_backend_user(alice, coolorg)

    async def _concurrent_boostrap(backend):
        try:
            await backend.organization.bootstrap(
                coolorg.organization_id,
                backend_user,
                backend_first_device,
                coolorg.bootstrap_token,
                coolorg.root_verify_key,
            )
            results.append(None)

        except Exception as exc:
            results.append(exc)

    async with backend_factory(
        config={"db_url": postgresql_url, "db_max_connections": 10}, populated=False
    ) as backend:
        # Create the organization
        await backend.organization.create(
            id=coolorg.organization_id, bootstrap_token=coolorg.bootstrap_token
        )

        # Concurrent bootstrap
        with ensure_pg_transaction_concurrency_barrier(concurrency=10):
            async with trio.open_nursery() as nursery:
                for _ in range(10):
                    nursery.start_soon(_concurrent_boostrap, backend)

    assert len(results) == 10
    assert len([r for r in results if isinstance(r, OrganizationAlreadyBootstrappedError)]) == 9

    async with triopg.connect(postgresql_url) as conn:
        res = await conn.fetchrow("SELECT count(*) FROM organization")
        assert res["count"] == 1
        res = await conn.fetchrow("SELECT count(*) FROM user_")
        assert res["count"] == 1
        res = await conn.fetchrow("SELECT count(*) FROM device")
        assert res["count"] == 1


@pytest.mark.trio
@pytest.mark.postgresql
async def test_concurrency_create_user(
    postgresql_url, backend_factory, backend_data_binder_factory, coolorg, alice, bob
):
    results = []

    backend_user, backend_first_device = local_device_to_backend_user(bob, alice)

    async def _concurrent_create(backend):
        try:
            await backend.user.create_user(
                coolorg.organization_id, backend_user, backend_first_device
            )
            results.append(None)

        except Exception as exc:
            results.append(exc)

    async with backend_factory(
        config={"db_url": postgresql_url, "db_max_connections": 10}, populated=False
    ) as backend:
        # Create&bootstrap the organization
        binder = backend_data_binder_factory(backend)
        await binder.bind_organization(coolorg, alice)

        # Concurrent user creation
        with ensure_pg_transaction_concurrency_barrier(concurrency=10):
            async with trio.open_nursery() as nursery:
                for _ in range(10):
                    nursery.start_soon(_concurrent_create, backend)

    assert len(results) == 10
    assert len([r for r in results if isinstance(r, UserAlreadyExistsError)]) == 9

    async with triopg.connect(postgresql_url) as conn:
        res = await conn.fetchrow("SELECT count(*) FROM organization")
        assert res["count"] == 1
        res = await conn.fetchrow("SELECT count(*) FROM user_ WHERE user_id = 'bob'")
        assert res["count"] == 1
        res = await conn.fetchrow(
            "SELECT count(*) FROM device WHERE user_ = (SELECT _id FROM user_ WHERE user_id = 'bob')"
        )
        assert res["count"] == 1


@pytest.mark.trio
@pytest.mark.postgresql
async def test_concurrency_create_user_with_limit_reached(
    postgresql_url,
    backend_factory,
    backend_data_binder_factory,
    coolorg,
    alice,
    local_device_factory,
):
    results = []

    async def _concurrent_create(backend, user):
        backend_user, backend_first_device = local_device_to_backend_user(user, alice)
        try:
            await backend.user.create_user(
                coolorg.organization_id, backend_user, backend_first_device
            )
            results.append(None)

        except Exception as exc:
            results.append(exc)

    async with backend_factory(
        config={"db_url": postgresql_url, "db_max_connections": 10}, populated=False
    ) as backend:
        # Create&bootstrap the organization
        binder = backend_data_binder_factory(backend)
        await binder.bind_organization(coolorg, alice)

        # Set a limit that will be soon reached
        await backend.organization.update(alice.organization_id, active_users_limit=3)

        # Concurrent user creation
        with ensure_pg_transaction_concurrency_barrier(concurrency=10):
            async with trio.open_nursery() as nursery:
                for _ in range(10):
                    nursery.start_soon(
                        _concurrent_create, backend, local_device_factory(org=coolorg)
                    )

    assert len(results) == 10
    assert len([r for r in results if isinstance(r, UserActiveUsersLimitReached)]) == 8
    assert len([r for r in results if r is None]) == 2

    async with triopg.connect(postgresql_url) as conn:
        res = await conn.fetchrow("SELECT count(*) FROM organization")
        assert res["count"] == 1
        res = await conn.fetchrow("SELECT count(*) FROM user_")
        assert res["count"] == 3
        res = await conn.fetchrow("SELECT count(*) FROM device")
        assert res["count"] == 3


@pytest.mark.trio
@pytest.mark.postgresql
async def test_concurrency_pki_enrollment_accept(
    postgresql_url, backend_factory, backend_data_binder_factory, coolorg, alice, bob
):
    results = []
    enrollment_id = uuid4()

    backend_user, backend_first_device = local_device_to_backend_user(bob, alice)

    async def _concurrent_enrollment_accept(backend):
        try:
            await backend.pki.accept(
                organization_id=coolorg.organization_id,
                enrollment_id=enrollment_id,
                accepter_der_x509_certificate=b"whatever",
                accept_payload_signature=b"whatever",
                accept_payload=b"whatever",
                accepted_on=DateTime.now(),
                user=backend_user,
                first_device=backend_first_device,
            )
            results.append(None)

        except AssertionError:
            # Improve pytest --pdb behavior
            raise

        except Exception as exc:
            results.append(exc)

    async with backend_factory(
        config={"db_url": postgresql_url, "db_max_connections": 10}, populated=False
    ) as backend:
        # Create&bootstrap the organization
        binder = backend_data_binder_factory(backend)
        await binder.bind_organization(coolorg, alice)

        # Create the PKI enrollment
        await backend.pki.submit(
            organization_id=coolorg.organization_id,
            enrollment_id=enrollment_id,
            force=False,
            submitter_der_x509_certificate=b"whatever",
            submitter_der_x509_certificate_email="whatever",
            submit_payload_signature=b"whatever",
            submit_payload=b"whatever",
            submitted_on=DateTime.now(),
        )

        # Concurrent PKI enrollement accept
        with ensure_pg_transaction_concurrency_barrier(concurrency=10):
            async with trio.open_nursery() as nursery:
                for _ in range(10):
                    nursery.start_soon(_concurrent_enrollment_accept, backend)

    assert len(results) == 10
    assert len([r for r in results if isinstance(r, PkiEnrollmentNoLongerAvailableError)]) == 9

    async with triopg.connect(postgresql_url) as conn:
        res = await conn.fetchrow("SELECT count(*) FROM organization")
        assert res["count"] == 1
        res = await conn.fetchrow("SELECT count(*) FROM user_ WHERE user_id = 'bob'")
        assert res["count"] == 1
        res = await conn.fetchrow(
            "SELECT count(*) FROM device WHERE user_ = (SELECT _id FROM user_ WHERE user_id = 'bob')"
        )
        assert res["count"] == 1
        res = await conn.fetchrow("SELECT count(*) FROM pki_enrollment")
        assert res["count"] == 1
        res = await conn.fetchrow("SELECT enrollment_state FROM pki_enrollment")
        res["enrollment_state"] == "ACCEPTED"
