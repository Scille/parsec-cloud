# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec._parsec import Regex
from parsec.types import FrozenDict
import pytest
from parsec._parsec import DateTime

from parsec.api.protocol import DeviceID
from parsec.api.data import EntryName
from parsec.core.types import EntryID, LocalWorkspaceManifest
from parsec.core.fs.workspacefs.sync_transactions import merge_manifests


empty_pattern = Regex.from_regex_str(r"^\b$")


def gen_date():
    curr = DateTime(2000, 1, 1)
    while True:
        yield curr
        curr = curr.add(days=1)


# Guessing by it name, this test is directed by M. Night Shyamalan ;-)
@pytest.mark.parametrize("local_changes", (False, True))
def test_merge_speculative_with_it_unsuspected_former_self(local_changes, core_config):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)
    d3 = DateTime(2000, 1, 3)
    d4 = DateTime(2000, 1, 4)
    d5 = DateTime(2000, 1, 5)
    my_device = DeviceID("a@a")

    # 1) Workspace manifest is originally created by our device
    local = LocalWorkspaceManifest.new_placeholder(author=my_device, timestamp=d1)
    foo_id = EntryID.new()
    local = local.evolve(updated=d2, children=FrozenDict({EntryName("foo"): foo_id}))

    # 2) We sync the workspace manifest
    v1 = local.to_remote(author=my_device, timestamp=d3)

    # 3) Now let's pretend we lost local storage, hence creating a new speculative manifest
    new_local = LocalWorkspaceManifest.new_placeholder(
        author=my_device, id=local.id, timestamp=d3, speculative=True
    )
    if local_changes:
        bar_id = EntryID.new()
        new_local = new_local.evolve(updated=d4, children=FrozenDict({EntryName("bar"): bar_id}))

    # 4) When syncing the manifest, we shouldn't remove any data from the remote
    merged = merge_manifests(
        local_author=my_device,
        timestamp=d5,
        prevent_sync_pattern=empty_pattern,
        local_manifest=new_local,
        remote_manifest=v1,
    )

    if local_changes:
        assert merged == LocalWorkspaceManifest(
            base=v1,
            need_sync=True,
            updated=d5,
            children=FrozenDict({**v1.children, **new_local.children}),
            local_confinement_points=frozenset(),
            remote_confinement_points=frozenset(),
            speculative=False,
        )
    else:
        assert merged == LocalWorkspaceManifest(
            base=v1,
            need_sync=False,
            updated=v1.updated,
            children=v1.children,
            local_confinement_points=frozenset(),
            remote_confinement_points=frozenset(),
            speculative=False,
        )
