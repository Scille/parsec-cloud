# Libparsec Mountpoint

As it name suggest, this crate implement a filesystem mountpoint to expose Parsec workspaces.

The implementation uses [FUSE](https://github.com/libfuse/libfuse/) on Linux and [WinFPS](https://winfsp.dev/) on Windows.

## Interactive testing

The quickest way to test the mountpoint is to use the `minimal` example:

```shell
RUST_LOG=libparsec_platform_mountpoint=DEBUG cargo run -p libparsec_platform_mountpoint --example minimal .
```

This mount a workspace in the current directory. The workspace is based on the
`minimal_client_ready` testbed template.

Also note the `RUST_LOG` environ variable to enable logs on each mountpoint access.

With this configuration, everything is kept in-memory so don't store too much !

## Implementation details

File system are historically designed to be fully implemented locally, and hence
having full control over the stored data. This in turn allows atomic operations,
cache and cache invalidation. In particular, the OS kernel is then very eager
to rely on caching to avoid as much as possible to have to do a costly context
switching to interrogate the FUSE/WinFSP (see [Fuse I/O Modes](https://www.kernel.org/doc/html/latest/filesystems/fuse-io.html)).

However the whole point of Parsec is to allow both local and remote file changes.
And even without that, Parsec allows for local modification to be done from it GUI.

In both case, the local data gets modified without informing the mountpoint. Which
means outdated caches and unexpected changes occurring :/

That is for the bad news, now things are in fact not *that* bad: those issues are not
specific to Parsec and have have plagued all softwares exposing through mountpoint a
remote resource (e.g. nfs, sshfs etc.).

The answer is basically always the same: a filesystem is a very convenient but
feature-limited abstraction over data (e.g. there is no api to atomically remove a
non-empty directory, atomic writing of a file requires writing to a temporary destination
then doing a rename), so it's no big deal to have some weird behavior when remote change
has just occurred. And if need stronger guarantees, then you should access your data
through another mechanism.

So in the end it's all a question of tweaking the cache timeout according to how big
do we accept the inconsistency window (and remember a timeout to zero doesn't mean no
inconsistency: even in this case we have no knowledge of the changes accepted by the
server that haven't reached us yet !).

## A problem not specific to the mountpoint

Funny enough, this issue of outdated view over data can also exist in a similar way
within libparsec when offline (or when a recent change hasn't reached the client yet).

For instance:

1. Alice is offline, she remove folder `/foo` that is empty
2. Bob is online, he creates `/foo/bar` and synchronize this change with the server
3. Alice is back online, she upload here removal of the `/foo` ¯\_(ツ)_/¯

Here the issue comes from the fact the removal changes one entry, but requires prior
checks on a different entry (i.e. the children).

## What about Windows's exclusive handles ?

In Windows opening a file/folder provides an exclusive handle on it so that no concurrent
modification can occur (we all have tried to rename a file in the explorer, only to get
a "file in use" error !).

So in theory Windows should be very lost (or angry ? ^^) if it gets an exclusive handle
on a directory, checks with `can_delete` it can be removed, only to get an error in
the final `cleanup` that should do the actual removal...

But in fact Windows seems cool with that (I guess there is not much it can do about it).
So in the end this exclusive handle only impacts Windows itself.

## `EntryID` vs `FsPath`

In both WinFSP and FUSE, a resource is first resolved before doing the actual operation.

- In WinFSP this is the `open` that takes a path and return a file context.
- In FUSE this is the `lookup` that takes a path and return an inode.

The resource resolution object is then provided when the actual operation function
is called. In turn, the operation function call the libparsec's `WorkspaceOps` APIs
to do the actual operation.

This architecture implies two things:

- The libparsec's `WorkspaceOps`APIs must be provided with the resource to modify.
  This can be done by passing it path (as provided to the resolution) or it entry ID
  (determined from the path during resolution).
- There is always a chance for a concurrent change occurring between the resolution
  and the operation.

Let's consider the following situation:

1. WinFSP callback `open` is called on `/foo` (which is an empty directory in a Parsec workspace).
   This succeed and an exclusive handle `#42` on `/foo` is returned.
2. WinFSP callback `can_delete` is called on handle `#42`, this succeed as the folder exists and is empty.
3. A remote change is integrated into the workspace.
4. WinFSP callback `cleanup` is called to actually remove the resource pointed by handle `#42`.

If we pass the resource path to libparsec, then we will remove the wrong element if
`/foo` has been overwritten during step 3.

If we pass the resource entry ID to libparsec, then we will remove a directory
unrelated to `/foo` if the folder (or one if it's parents) has been moved during
step 3.

In the end there is no "right" solution.
