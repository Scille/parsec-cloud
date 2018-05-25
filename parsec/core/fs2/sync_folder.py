from parsec.core.fs2.sync_file import sync_file


async def _recursive_children_sync():
    # Recursive sync of children first
    synced_children = {}
    for child_name, child_access in snapshot_manifest["children"].items():
        try:
            child_manifest = self._manifests_cache[child_access["id"]]
        except KeyError:
            # This entry is not present in local (or has been deleted since
            # we started the sync), no need to go further
            synced_children[child_name] = child_access
            continue
        try:
            child_access = await self._sync(child_access, child_manifest)
        except BackendConcurrencyError:
            # Placeholder are not yet in backend, cannot have concurrence on them !
            assert child_access["type"] != "placeholder"
            # File already modified, must rename it to avoid losing data!
            if child_access:
                child_target_manifest = await self.manifests_manager.fetch_from_backend(
                    child_access["id"], child_access["rts"], child_access["key"]
                )
            else:
                child_target_manifest = (
                    await self.manifests_manager.fetch_user_manifest_from_backend()
                )
            child_target_manifest = convert_to_local_manifest(child_target_manifest)
            child_diverged_manifest = self._manifests_cache.pop(child_access["id"])
            self._manifests_cache[child_access["id"]] = child_target_manifest

            # Move the diverged entry to avoid losing it
            diverged_access = new_placeholder_access()
            duplicated_name = child_name
            while True:
                duplicated_name += ".conflict"
                if (
                    duplicated_name not in snapshot_manifest["children"]
                    and duplicated_name not in synced_children
                ):
                    synced_children[duplicated_name] = diverged_access
                    break
            self._manifests_cache[diverged_access["id"]] = child_diverged_manifest
            mark_updated(snapshot_manifest)

            # TODO: protect aganst BackendconcurrencyError ?
            await self._sync(diverged_access, child_diverged_manifest)

        # Required if access is a placeholder
        # TODO: find a better way to do it...
        synced_children[child_name] = child_access


async def sync_folder(manifest_manager, local_tree_cache, access, manifest):
    assert is_folder_manifest(manifest)

    # Make a snapshot of ourself to avoid concurrency
    # Serialize as local folder manifest
    base_local_manifest = snapshot_manifest(manifest)
    snapshot_manifest = snapshot_manifest(manifest)

    snapshot_manifest["children"] = _recursive_children_sync(
        snapshot_manifest["children"]
    )

    # Sync ourself or check if a new version is available in the backend
    if not snapshot_manifest["need_sync"]:
        # This folder hasn't been modified locally,
        # just download last version from the backend if any.
        if not access:
            target_remote_manifest = (
                await self.manifests_manager.fetch_user_manifest_from_backend()
            )
        else:
            # Placeholder means we need synchro !
            assert access["type"] != "placeholder"
            target_remote_manifest = await self.manifests_manager.fetch_from_backend(
                access["id"], access["rts"], access["key"]
            )
        if (
            not target_remote_manifest
            or target_remote_manifest["version"] == manifest["base_version"]
        ):
            return access
    else:

        to_sync_manifest = convert_to_remote_manifest(snapshot_manifest)
        # Upload the folder manifest as new vlob version
        while True:
            try:
                if not access:
                    await self.manifests_manager.sync_user_manifest_with_backend(
                        to_sync_manifest
                    )
                elif access["type"] == "placeholder":
                    id, rts, wts = await self.manifests_manager.sync_new_entry_with_backend(
                        access["key"], to_sync_manifest
                    )
                    access.update(id=id, rts=rts, wts=wts, type="vlob")
                else:
                    await self.manifests_manager.sync_with_backend(
                        access["id"], access["wts"], access["key"], to_sync_manifest
                    )
                target_remote_manifest = to_sync_manifest
                break

            except BackendConcurrencyError:
                # Do a 3-ways merge to fix the concurrency error, first we must
                # fetch the base version and the new one present in the backend
                # TODO: should be available locally
                if not access:
                    base = await self.manifests_manager.fetch_user_manifest_from_backend(
                        version=to_sync_manifest["version"] - 1
                    )
                    target = (
                        await self.manifests_manager.fetch_user_manifest_from_backend()
                    )
                else:
                    base = await self.manifests_manager.fetch_from_backend(
                        access["id"],
                        access["rts"],
                        access["key"],
                        version=to_sync_manifest["version"] - 1,
                    )
                    target = await self.manifests_manager.fetch_from_backend(
                        access["id"], access["rts"], access["key"]
                    )
                # 3-ways merge between base, modified and target versions
                to_sync_manifest, _ = merge_remote_folder_manifests(
                    base, to_sync_manifest, target
                )
                to_sync_manifest["version"] = target["version"] + 1

    # Finally merge with the version present in cache which may have
    # been modified in the meantime
    target_local_manifest = convert_to_local_manifest(target_remote_manifest)
    manifest, _ = merge_local_folder_manifests(
        base_local_manifest, manifest, target_local_manifest
    )
    if not access:
        self._root_manifest_cache = manifest
    else:
        # Note the manifest is still kept in the cache with it
        # previous placeholder id
        self._manifests_cache[access["id"]] = manifest
    self._flush_entry(access, manifest)

    return access
