async def _sync_file(self, access, manifest):
    if not manifest["need_sync"]:
        # Placeholder means we need synchro !
        assert access["type"] != "placeholder"
        # This file hasn't been modified locally,
        # just download last version from the backend if any.
        target_manifest = await self.manifests_manager.fetch_from_backend(
            access["id"], access["rts"], access["key"]
        )
        if target_manifest["version"] != manifest["base_version"]:
            if manifest["need_sync"]:
                raise BackendConcurrencyError(
                    "File with access %r has locally diverged" % access
                )
            else:
                target_manifest = convert_to_local_manifest(target_manifest)
                self._manifests_cache[access["id"]] = target_manifest
                self._flush_entry(access, target_manifest)

    else:
        # The file has been locally modified, time to sync it with the backend !
        # TODO
        # fd = self._open_file(access, manifest)
        # ...
        snapshot_manifest = convert_to_remote_manifest(manifest)
        if access["type"] == "placeholder":
            id, rts, wts = await self.manifests_manager.sync_new_entry_with_backend(
                access["key"], snapshot_manifest
            )
            access.update(id=id, rts=rts, wts=wts, type="vlob")
        else:
            await self.manifests_manager.sync_with_backend(
                access["id"], access["wts"], access["key"], snapshot_manifest
            )
        # Finally fast forward the manifest with it new base version
        # TODO
        manifest = fast_forward_file_manifest(snapshot_manifest, manifest)
        # Note the manifest is still kept in the cache with it
        # previous placeholder id
        self._manifests_cache[access["id"]] = manifest
        self._flush_entry(access, manifest)

    return access
