from parsec.core.fs.base import BaseEntry


class MergeProtocol:
    def __init__(self, entry):
        self.entry = entry

    def __eq__(self, other):
        return self["id"] == other["id"] if isinstance(other, (MergeProtocol, dict)) else False

    def __getitem__(self, attr):
        if attr == "children":
            return self.entry._children

        elif attr == "created":
            return self.entry._created

        elif attr == "updated":
            return self.entry._updated

        elif attr == "version":
            return self.entry._version

        elif attr == "id":
            return self.entry.id

        else:
            return KeyError("%s not found" % attr)


def simple_rename(base, diverged, target, entry_name):
    resolved = {entry_name: target["children"][entry_name]}
    name = entry_name
    while True:
        name = "%s.conflict" % name
        if name not in target["children"] and name not in diverged["children"]:
            resolved[name] = diverged["children"][entry_name]
            break

    return resolved


def merge_children(base, diverged, target, on_conflict=simple_rename, inplace=None):
    if isinstance(base, BaseEntry):
        base = MergeProtocol(base)

    if isinstance(diverged, BaseEntry):
        diverged = MergeProtocol(diverged)

    if isinstance(target, BaseEntry):
        target = MergeProtocol(target)

    if isinstance(inplace, BaseEntry):
        inplace = MergeProtocol(inplace)

    if inplace is None:
        inplace = {"children": {}}

    # If entry is in base but not in diverged and target, it is then already
    # resolved.
    all_entries = diverged["children"].keys() | target["children"].keys()
    resolved = inplace["children"]
    modified = False

    for entry in all_entries:
        base_entry = base["children"].get(entry)
        target_entry = target["children"].get(entry)
        diverged_entry = diverged["children"].get(entry)

        if isinstance(base_entry, BaseEntry):
            base_entry = MergeProtocol(base_entry)

        if isinstance(target_entry, BaseEntry):
            target_entry = MergeProtocol(target_entry)

        if isinstance(diverged_entry, BaseEntry):
            diverged_entry = MergeProtocol(diverged_entry)

        entry_modified = True

        if diverged_entry == target_entry:
            # No modifications or same modification on both sides, either case
            # just keep things like this
            resolved[entry] = target_entry
            entry_modified = False

        elif target_entry == base_entry:
            if diverged_entry:
                # Entry has been modified on diverged side only
                resolved[entry] = diverged_entry

        elif diverged_entry == base_entry:
            # Entry has been modified en target side only
            if target_entry:
                resolved[entry] = target_entry

        else:
            # Entry modified on both side...
            if not target_entry:
                # Entry removed on target side, apply diverged modification
                # not to loose it (unless it is a remove of course)
                if diverged_entry:
                    resolved[entry] = diverged_entry

            elif not diverged_entry:
                # Entry removed on diverged side and modified (no remove) on
                # target side, just apply them
                resolved[entry] = target_entry

            else:
                # Entry modified on both side (no remove), last chance to
                # resolve this is if the entry is a placeholder that has been
                # synchronized
                if target_entry["id"] == diverged_entry["id"]:
                    # Keep the synced entry, forget about the placeholder one
                    resolved[entry] = target_entry
                else:
                    # Conflict !
                    resolved.update(on_conflict(base, diverged, target, entry))

        modified = modified or entry_modified

    return resolved, modified


def merge_folder_manifest(base, diverged, target, on_conflict=simple_rename, inplace=None):
    resolved, modified = merge_children(
        base, diverged, target, on_conflict=on_conflict, inplace=inplace
    )

    if target["updated"] > diverged["updated"]:
        updated = target["updated"]
    else:
        updated = diverged["updated"]

    return {
        **target,
        "version": target["version"] + 1,
        "updated": updated,
        "children": resolved,
    }, modified
