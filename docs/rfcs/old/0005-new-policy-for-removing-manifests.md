# New policy for removing manifests

From [ISSUE-575](https://github.com/Scille/parsec-cloud/issues/575)

Entries can get removed in two different ways:

- removed by a `file_delete` or `folder_delete` transaction
- removed by a sync operation (when someone else removed the file/folder)

But this doesn't mean that the corresponding manifest can be removed right away since the file might still be used by open file descriptors. Instead the clean up for manifests has to be performed when the corresponding entry is no longer referenced. This would also imply to check for open file descriptors when closing the application. This ASAP clean-up policy turns out to be hard to implement, especially in the case of a sync operation.

But this is not the only valid clean-up policy, as nothing in the system requires those manifests to be removed. Instead we could simply keep them in the local storage. This would even help with accessing a workspace history, especially if the application is offline.

This implies that the manifests would not be cleaned up automatically. Instead there could be a maintenance task that can be performed at specific times (or never at all, since manifests are pretty light).

One remaining issues is the blocks that used the be cleaned up along with their corresponding manifests. Instead, we would rely completely on the garbage collector: the storage for a given workspace would simply fill up until it reaches the configured limit, and then the GC would act according to its LRU policy. Again, the workspace history would benefit from this greedy approach.
