# Invisible minimal sync

## 0 - Basic considerations

Considering the following :

1. Alice creates a file in location `/foo/big.txt`
2. Alice write a big amount of data into `/foo/big.txt`
3. Alice create another file in location `/foo/small.txt`
4. Bob wants to access to `/foo/small.txt` asap

Here the simplest way to synchronize the data is:

1. Sync `/foo/small.txt`'s file manifest
2. Sync `/foo/big.txt`'s file manifest
3. Sync `/foo`'s folder manifest

However this is bad because `/foo`'s changes cannot be accessed by Bob as long as
`/foo/big.txt` is not done with synchronization.

For that reason Parsec currently has slightly more complexe way of doing synchronization:

1. Sync `/foo/small.txt`'s file manifest
2. Sync a minimal version `/foo/big.txt` (i.e. file manifest corresponding to an empty file)
3. Sync `/foo`'s folder manifest
4. Sync of `/foo/big.txt` actual data as version 2 of it file manifest

## 1 - The issues with minimal sync

The minimal sync approach is great to reduce synchronization congestion, however it creates
an empty files manifest that cannot be differentiate from an actual legitimate empty file
(e.g. by doing a `touch foo.txt`).

### 1.1 - Issue in prod

This empty file is especially bad for non-text documents because the software that should
open them is not expecting the file to be empty, hence bad error message that make Parsec look broken :'(

### 1.2 - Issue with tests

[See this issue](https://github.com/Scille/parsec-cloud/issues/4312)

Condiering the following test code:

```python
    # Create a shared workspace
    wid = await create_shared_workspace(EntryName("w"), bob_user_fs, alice_core)
    alice_workspace = alice_core.user_fs.get_workspace(wid)

    # Alice creates a non-empty files and sync it
    await alice_workspace.write_bytes("/test.txt", b"hello world")
    await alice_core.wait_idle_monitors()

    info = await alice_workspace.path_info("/test.txt")
    assert info["base_version"] == 2  # or is it 3 ????
```

The issue here is we have two manifests to synchronize: `/` workspace manifest and
`/test.txt` file manifest.
So the sync monitor may choose to start synchronization either by the file manifest or
the workspace manifest. If it starts by the workspace manifest, a minimal sync must be
done first on the file manifest !

So in the end, once fully synchronized the file manifest may be at version 2 or 3...

## Not a solution: ignore version 1 of the manifest

Given minimal sync may or may not occur, we cannot just ignore the version 1 of the manifest.
On top of that an empty file can be legitime (e.g. `touch foo.txt`) so we also cannot filter
on "version 1 and empty".

## Solution 1: `minimal` tag in file/folder manifest

TODO: description
TODO: Backward compatibility impact

## Solution 2: allow unknown id in file manifest entry

Folder manifest can have be crafted by a malicious user, hence the "entry ID in the folder
manifest's children should always reference a valid vlob containing a file/folder manifest"
cannot be enforced.

So no matter what we have to deal with the fact an entry ID is invalid:

- no vlob have this entry ID
- a vlob exists but it contains the wrong type of manifest (e.g. workspace manifest)
- a vlob exists but it contains garbage data
- a vlob exists and used to contains valid data, but not it latest version

TODO: Current handling of invalid entry ID
TODO: Backward compatibility impact
