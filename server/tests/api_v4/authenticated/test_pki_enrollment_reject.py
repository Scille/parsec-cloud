# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest


@pytest.mark.xfail(reason="TODO: PKI is not fully implemented yet")
async def test_authenticated_pki_enrollment_reject_author_not_allowed() -> None:
    raise NotImplementedError


@pytest.mark.xfail(reason="TODO: PKI is not fully implemented yet")
async def test_authenticated_pki_enrollment_reject_enrollment_no_longer_available() -> None:
    raise NotImplementedError


@pytest.mark.xfail(reason="TODO: PKI is not fully implemented yet")
async def test_authenticated_pki_enrollment_reject_enrollment_not_found() -> None:
    raise NotImplementedError


@pytest.mark.xfail(reason="TODO: PKI is not fully implemented yet")
async def test_authenticated_pki_enrollment_reject_ok() -> None:
    raise NotImplementedError
