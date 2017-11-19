import pytest
from freezegun import freeze_time

from parsec.core.manifest import (
    SyncedEntry, PlaceHolderEntry, LocalFolderManifest, merge_folder_manifest)


class MockedSyncedEntry(SyncedEntry):
    def __init__(self, id):
        super().__init__(id, '<%s-syncid>' % id, b'<key>', '<rts>', '<wts>')


class MockedPlaceHolderEntry(PlaceHolderEntry):
    def __init__(self, id):
        super().__init__(id, b'<key>')


with freeze_time('2017-11-11'):
    class TestMergeLocalFolderManifest:

        @pytest.mark.parametrize('base', [
            LocalFolderManifest(),
            LocalFolderManifest(children={
                'foo': MockedSyncedEntry(id=2),
            })
            # Note we don't test with PlaceHolderEntry because base is
            # synchronized with backend, hence cannot contains placeholders.
        ])
        def test_no_modifications(self, base):
            merged = merge_folder_manifest(base, base, base)
            assert merged == base

        @pytest.mark.parametrize('base,target', [
            (  # Add an entry
                LocalFolderManifest(),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
            ),
            (  # Remove an entry
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(),
            ),
            (  # re-create an entry (i.e. entry id has changed)
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=3),
                })
            ),
            # Note we don't test with PlaceHolderEntry because base and target
            # are synchronized with backend, hence cannot contains placeholders.
        ])
        def test_merge_not_needed(self, base, target):
            merged = merge_folder_manifest(base, target, target)
            assert merged == target


        @pytest.mark.parametrize('base,diverged,target,expected_merged', [
            (  # Entry removed on diverged side
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                }),
            ),
            (  # Entry removed on target side
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                }),
            ),

            (  # Entry added on diverged side
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
            ),
            (  # Entry added on target side
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
            ),

            (  # Entry re-created on diverged side
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedPlaceHolderEntry(id=3),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedPlaceHolderEntry(id=3),
                }),
            ),
            (  # Entry re-created on target side
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=3),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=3),
                }),
            ),

            (  # Entry removed on target side and re-created on diverged side
                LocalFolderManifest(children={
                    'Foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'Foo': MockedPlaceHolderEntry(id=3),
                }),
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                    'Foo': MockedPlaceHolderEntry(id=3),
                }),
            ),
            (  # Entry re-created on target side and removed on diverged side
                LocalFolderManifest(children={
                    'Foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                }),
                LocalFolderManifest(children={
                    'Foo': MockedSyncedEntry(id=3),
                }),
                LocalFolderManifest(children={
                    'Foo': MockedSyncedEntry(id=3),
                }),
            ),

            (  # Same entry created on modifed side and synchronized on target side
                LocalFolderManifest(),
                LocalFolderManifest(children={
                    'foo': MockedPlaceHolderEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
            )
        ])
        def test_merge_without_conflicts(self, base, diverged, target, expected_merged):
            merged = merge_folder_manifest(base, diverged, target)
            assert merged == expected_merged


        @pytest.mark.parametrize('base,diverged,target,expected_merged', [
            (  # Entry created on both side
                LocalFolderManifest(),
                LocalFolderManifest(children={
                    'foo': MockedPlaceHolderEntry(id=3),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),  # Note we keep target's id
                    'foo.conflict': MockedPlaceHolderEntry(id=3),
                }),
            ),
            (  # Entry created and synchronized on both side
                LocalFolderManifest(),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=3),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),
                }),
                LocalFolderManifest(children={
                    'foo': MockedSyncedEntry(id=2),  # Note we keep target's id
                    'foo.conflict': MockedSyncedEntry(id=3),
                }),
            ),
        ])
        def test_merge_with_conflicts(self, base, diverged, target, expected_merged):
            merged = merge_folder_manifest(base, diverged, target)
            assert merged == expected_merged
