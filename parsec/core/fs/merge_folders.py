import pendulum
from itertools import count


def find_conflicting_name_for_child_entry(original_name: str, check_candidate_name) -> str:
    try:
        base_name, extension = original_name.rsplit(".")
    except ValueError:
        extension = None
        base_name = original_name

    now = pendulum.now()
    for tentative in count(1):
        tentative_str = "" if tentative == 1 else f" n°{tentative}"
        # TODO: Also add device id in the naming ?
        diverged_name = f"{base_name} (conflict{tentative_str} {now.to_datetime_string()})"
        if extension:
            diverged_name += f".{extension}"

        if check_candidate_name(diverged_name):
            return diverged_name

    assert False  # Should never be here


def merge_children(base, diverged, target):
    # If entry is in base but not in diverged and target, it is then already
    # resolved.
    all_entries = diverged.keys() | target.keys()
    conflicts = []
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

                def check_candidate_name(name):
                    return name not in target and name not in diverged and name not in resolved

                conflict_entry_name = find_conflicting_name_for_child_entry(
                    entry_name, check_candidate_name
                )
                resolved[entry_name] = target_entry
                conflict_entry_entry = resolved[conflict_entry_name] = diverged[entry_name]
                conflicts.append(
                    (
                        entry_name,
                        target_entry["id"],
                        conflict_entry_name,
                        conflict_entry_entry["id"],
                    )
                )

    return resolved, need_sync, conflicts


def merge_remote_folder_manifests(base, diverged, target):
    if base is None:
        base_version = 0
        base_children = {}
    else:
        base_version = base["version"]
        base_children = base["children"]
    assert base_version + 1 == diverged["version"]
    assert target["version"] >= diverged["version"]
    # Not true when merging user manifest v1 given v0 is lazily generated
    # assert diverged["created"] == target["created"]

    children, need_sync, conflicts = merge_children(
        base_children, diverged["children"], target["children"]
    )

    if not need_sync:
        updated = target["updated"]
    else:
        if target["updated"] > diverged["updated"]:
            updated = target["updated"]
        else:
            updated = diverged["updated"]

    merged = {**target, "updated": updated, "children": children}

    # Only user manifest has this field
    if "last_processed_message" in target:
        merged["last_processed_message"] = max(
            diverged["last_processed_message"], target["last_processed_message"]
        )

    # Only workspace manifest has this field
    if "participants" in target:
        merged["participants"] = list(set(target["participants"] + diverged["participants"]))

    return merged, need_sync, conflicts


def merge_local_folder_manifests(base, diverged, target):
    if base is None:
        base_version = 0
        base_children = {}
    else:
        base_version = base["base_version"]
        base_children = base["children"]
    assert base_version == diverged["base_version"]
    assert target["base_version"] > diverged["base_version"]
    # Not true when merging user manifest v1 given v0 is lazily generated
    # assert diverged["created"] == target["created"]

    children, need_sync, conflicts = merge_children(
        base_children, diverged["children"], target["children"]
    )

    if not need_sync:
        updated = target["updated"]
    else:
        # TODO: potentially unsafe if two modifications are done within the same millisecond
        if target["updated"] > diverged["updated"]:
            updated = target["updated"]
        else:
            updated = diverged["updated"]

    merged = {**target, "need_sync": need_sync, "updated": updated, "children": children}

    # Only user manifest has this field
    if "last_processed_message" in target:
        merged["last_processed_message"] = max(
            diverged["last_processed_message"], target["last_processed_message"]
        )

    # Only workspace manifest has this field
    if "participants" in target:
        merged["participants"] = list(
            sorted(set(target["participants"] + diverged["participants"]))
        )

    return merged, conflicts
