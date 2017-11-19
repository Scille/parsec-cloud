import pytest
from unittest.mock import Mock

from parsec.core.manifest import (
    LocalFolderManifest, LocalFileManifest, LocalUserManifest, dump_local_manifest, load_local_manifest
)

from tests.common import alice


@pytest.mark.parametrize('manifest_cls', [
    LocalFileManifest,
    LocalFolderManifest,
    LocalUserManifest
])
def test_dump_local_manifest(manifest_cls):
    manifest = manifest_cls()
    dumped = dump_local_manifest(manifest)
    manifest2 = load_local_manifest(dumped)
    assert manifest == manifest2
