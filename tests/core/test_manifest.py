import pytest
from unittest.mock import Mock

from parsec.core.manifest import (
    LocalFolderManifest, LocalFileManifest, LocalUserManifest, dump_manifest, load_manifest
)

from tests.common import alice


@pytest.mark.parametrize('manifest_cls', [
    LocalFileManifest,
    LocalFolderManifest,
    LocalUserManifest
])
def test_dump_manifest(manifest_cls):
    manifest = manifest_cls()
    dumped = dump_manifest(manifest)
    manifest2 = load_manifest(dumped)
    assert manifest == manifest2
