# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest


@pytest.mark.xfail(reason="TODO: PKI is not fully implemented yet")
async def test_anonymous_pki_enrollment_info_ok() -> None:
    raise NotImplementedError


@pytest.mark.xfail(reason="TODO: PKI is not fully implemented yet")
async def test_anonymous_pki_enrollment_info_enrollment_not_found() -> None:
    raise NotImplementedError
