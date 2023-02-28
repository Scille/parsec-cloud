# Available when offline" workspace mode

From [ISSUE-2266](https://github.com/Scille/parsec-cloud/issues/2266)

At the moment, Parsec is lazy when it comes to download file data, meaning a specific piece of data is not going to be downloaded until the user tries to access it. The idea of the "available when offline" feature is to let the user select some specific data to be kept locally so this data is guaranteed to be available when offline.

## 1 - The feature

First, we should decide on the scope of the feature. More specifically, should the user be able to:

- Select a full workspace to be kept in offline mode
- Select a specific file or directory to be kept in offline mode

The second approach is more granular and require a greater amount of work, for a couple of reasons:

- The tree structure of file system in a workspace is dynamic
- The user interface is also more complex as it should be made clear to the user as
which directory is tagged as offline or not.

For the rest of this write-up, I'll focus on the first approach, more specifically:

- The user can select a workspace to be switched to "available when offline"
- Follow a transition period during which the corresponding data is downloaded and stored locally
- The current status of the workspace is displayed to the user
- Once the transition period is over, the user can fully access every file in the workspace even when offline
- The "available when offline" doesn't prevent synchronization (up and down) but the guarantees should remain at all times
- The user can disable the "available when offline" mode to clean up space on the disk

Extra features might be added on top of this, which will not be considered in this write up:

- Warn the user of the amount of data which is required to download when switching modes
- Warn the user of the amount of data which is required to download when down-synchronizing a large files

## 2 - The implementation

From the local storage point of view, there are two ways of implementing this feature:

- either create a third database dedicated to offline mode (the first two being the cache database and the local data database)
- or alter the design of the cache database to disable the cache limit

As of now, it's difficult to tell which one is the cheapest or best solution, this should be investigated further.

Then a state machine should be implemented at workspace level to manage the transitions between the following states:

- Normal state
- Transition to "available when offline" state
- Available when offline state

In order to maintain the "available when offline" state, the workspace cannot down-synchronize manifests without performing the following task:

- Check that the manifest is a file manifest
- List the new block IDs that are part of this manifest
- Download the corresponding blocks and store them in the local storage
- And only then, acknowledge this manifest

Note: this should also be done to the transitional state, so that a single pass over all the file manifests at a given moment should be enough to guaranteed the transition to the stable state.

The synchronization system being highly asynchronous, a major challenge of this implementation is to make sure that operations described above work consistently even other operations are carried out in the meantime. Those operations being:

- User read access to the file system: seems trivial
- User write access to the file system: seems trivial
- Down-synchronization: those operations might be redundant, if two versions of a manifest are uploaded close to each-other
- Up-synchronization: this might trigger a down-synchronization which loops to the previous point

Another challenge is the implementation of a cleanup routine, since we currently do not remove blocks when they're not referenced anymore (instead we simply rely on the cache and its "remove the oldest block" policy). There are two approaches to cleaning-up those unreferenced blocks:

- Have a periodic cleanup routine, relying on a workspace sweep to list all referenced block ids. Note: Race conditions can easily be avoided by comparing the last access time of the blocks to the start time of the sweep.
- Remove the old blocks as soon as they are not referenced by comparing manifests AFTER an down-synchronization has been performed. Note: This only work under the assumption that blocks cannot be referenced more than once

A hybrid approach might also have advantages in order to clean up unreferenced blocks that might remain in the database, in the case where the application has been stopped after acknowledging a manifest but before performing the clean up for instance. The routine might also be able to identify missing blocks preemptively, in order to provide more robustness to the system.

The last item that needs to done is the exposing of those feature in the desktop GUI in an ergonomic way.
