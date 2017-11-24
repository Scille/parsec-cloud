def simple_rename(base, diverged, target, entry_name):
    resolved = {
        entry_name: target.children[entry_name]
    }
    name = entry_name
    while True:
        name = '%s.conflict' % name
        if name not in target.children and name not in diverged.children:
            resolved[name] = diverged.children[entry_name]
            break
    return resolved


def merge_children(base, diverged, target, on_conflict=simple_rename):
    # If entry is in base but not in diverged and target, it is then already
    # resolved.
    all_entries = diverged['children'].keys() | target['children'].keys()
    resolved = {}
    for entry in all_entries:
        base_entry = base['children'].get(entry)
        target_entry = target['children'].get(entry)
        diverged_entry = diverged['children'].get(entry)
        if diverged_entry == target_entry:
            # No modifications or same modification on both sides, either case
            # just keep things like this
            resolved[entry] = target_entry
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
                if target_entry.id == diverged_entry.id:
                    # Keep the synced entry, forget about the placeholder one
                    resolved[entry] = target_entry
                else:
                    # Conflict !
                    resolved.update(on_conflict(base, diverged, target, entry))
    return resolved


def merge_folder_manifest(base, diverged, target, on_conflict=simple_rename):
    resolved = merge_children(base, diverged, target, on_conflict)

    if target['updated'] > diverged['updated']:
        updated = target['updated']
    else:
        updated = diverged['updated']

    return {
        **target,
        'version': target['version'] + 1,
        'updated': updated,
        'children': resolved
    }