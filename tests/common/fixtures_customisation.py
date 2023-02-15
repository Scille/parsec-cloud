# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Callable, TypeVar

import pytest

_FIXTURES_CUSTOMIZATIONS = {
    "coolorg_is_sequestered_organization",
    "alice_profile",
    "alice_initial_local_user_manifest",
    "alice2_initial_local_user_manifest",
    "alice_initial_remote_user_manifest",
    "alice_has_human_handle",
    "alice_has_device_label",
    "bob_profile",
    "bob_initial_local_user_manifest",
    "bob_initial_remote_user_manifest",
    "bob_has_human_handle",
    "bob_has_device_label",
    "adam_profile",
    "adam_initial_local_user_manifest",
    "adam_initial_remote_user_manifest",
    "adam_has_human_handle",
    "adam_has_device_label",
    "adam_is_revoked",
    "mallory_profile",
    "mallory_initial_local_user_manifest",
    "mallory_has_human_handle",
    "mallory_has_device_label",
    "backend_not_populated",
    "backend_has_webhook",
    "backend_force_mocked",
    "backend_over_ssl",
    "backend_forward_proto_enforce_https",
    "backend_spontaneous_organization_bootstrap",
    "logged_gui_as_admin",
    "fake_preferred_org_creation_backend_addr",
    "blockstore_mode",
    "real_data_storage",
    "gui_language",
    "workspace_storage_cache_size",
}

F = TypeVar("F")


def customize_fixtures(**customizations: Any) -> Callable[[F], F]:
    """
    Should be used as a decorator on tests to provide custom settings to fixtures.
    """
    assert not customizations.keys() - _FIXTURES_CUSTOMIZATIONS

    def wrapper(fn: F) -> F:
        try:
            getattr(fn, "_fixtures_customization").update(customizations)
        except AttributeError:
            setattr(fn, "_fixtures_customization", customizations)
        return fn

    return wrapper


@pytest.fixture
def fixtures_customization(request) -> dict[str, Any]:
    try:
        return request.node.function._fixtures_customization
    except AttributeError:
        pass
    try:
        # In the case of reruns, the original function can be found like so:
        function = getattr(request.node.module, request.node.originalname)
        return function._fixtures_customization
    except AttributeError:
        return {}
