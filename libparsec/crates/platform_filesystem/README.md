This crate does common files filesystem operations mainly used to manage
devices (although the concept of device is not present in this crate).

- save content to a file
- load content from a file
- list recursively files in a directory with a given extension
- remove a file
- rename a file


The platforms supported are
- native: where you have access to a nice filesystem (if it was the only platform, there would not be a need for this crate)
- web: to be able to stuff (aka devices) on a web browser

This crates (contrary to platform_storage) does not operate a database.

> [!NOTE]
> The content of this crate has previously been part of platform_device_loader.
