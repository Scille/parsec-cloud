# Why the hashes in the names?

The content of this folder is copied verbatim in the release.
So we name everything with a cache-busting pattern, meaning a file must be
renamed every time its content get updated!

> [!NOTE]
> A pre-commit rule is configured to ensure the file naming and references never
> goes out of sync with the file content.

## Adding a new file

1. Create it as `<name>-00000000.<extension>` (e.g. `parsec-00000000.png`)
2. Use this file name as usual in your template code
3. Run `python ../../misc/gen_static_assets_cache_busting_naming.py client`, this will
   update the file (and the templates referencing it) to its correct name.

## Modifying a file

1. Modify the file content
2. Run `python ../../misc/gen_static_assets_cache_busting_naming.py client`

## What about _this_ file

Since it is in the public directory, this `README.md` also end up in the release.
This is considered okay since it is small, doesn't cause any actual issue, and
avoiding this would require cumbersome configuration in the build pipeline.
