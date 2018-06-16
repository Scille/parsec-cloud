def merge_children(base, diverged, target):

    # If entry is in base but not in diverged and target, it is then already
    # resolved.
    all_entries = diverged.keys() | target.keys()
    resolved = {}
    need_sync = False

    for entry_name in all_entries:
        base_entry = base.get(entry_name)
        target_entry = target.get(entry_name)
        diverged_entry = diverged.get(entry_name)

        if diverged_entry == target_entry:
            # No modifications or same modification on both sides, either case
            # just keep things like this
            if target_entry:
                resolved[entry_name] = target_entry
            continue

        elif target_entry == base_entry:
            # Entry has been modified on diverged side only
            need_sync = True
            if diverged_entry:
                resolved[entry_name] = diverged_entry

        elif diverged_entry == base_entry:
            # Entry has been modified en target side only
            if target_entry:
                resolved[entry_name] = target_entry

        else:
            # Entry modified on both side...
            if not target_entry:
                need_sync = True
                # Entry removed on target side, apply diverged modifications
                # not to loose them
                # TODO: rename entry to `<name>.deleted` ?
                resolved[entry_name] = diverged_entry

            elif not diverged_entry:
                need_sync = True
                # Entry removed on diverged side and modified (no remove) on
                # target side, just apply them
                # TODO: rename entry to `<name>.deleted` ?
                resolved[entry_name] = target_entry

            else:
                need_sync = True
                # Entry modified on both side (no remove), conflict !
                resolved[entry_name] = target_entry
                conflict_entry_name = entry_name
                while True:
                    conflict_entry_name = "%s.conflict" % conflict_entry_name
                    if (
                        conflict_entry_name not in target
                        and conflict_entry_name not in diverged
                        and conflict_entry_name not in resolved
                    ):
                        resolved[conflict_entry_name] = diverged[entry_name]
                        break

    return resolved, need_sync


def merge_remote_folder_manifests(base, diverged, target):
    if base is None:
        version = 0
        base_children = {}
    else:
        version = base["version"]
        base_children = base["children"]
    assert version + 1 == diverged["version"]
    assert target["version"] >= diverged["version"]

    children, need_sync = merge_children(base_children, diverged["children"], target["children"])

    if not need_sync:
        updated = target["updated"]
    else:
        if target["updated"] > diverged["updated"]:
            updated = target["updated"]
        else:
            updated = diverged["updated"]

    merged = {**target, "updated": updated, "children": children}

    return merged, need_sync


def merge_local_folder_manifests(base, diverged, target):
    if base is None:
        version = 0
        base_children = {}
    else:
        version = base["base_version"]
        base_children = base["children"]
    assert version == diverged["base_version"]
    assert target["base_version"] > diverged["base_version"]

    children, need_sync = merge_children(base_children, diverged["children"], target["children"])

    if not need_sync:
        updated = target["updated"]
    else:
        # TODO: potentially unsafe if two modifications are done within the same millisecond
        if target["updated"] > diverged["updated"]:
            updated = target["updated"]
        else:
            updated = diverged["updated"]

    return {**target, "need_sync": need_sync, "updated": updated, "children": children}
