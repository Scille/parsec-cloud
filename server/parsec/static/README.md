# Why the hashes in the names?

The static resources use a cache-busting pattern in their names.
This means the name should change every time the content changes!

## Adding a new file

1. Create it as `<name>-00000000.<extension>` (e.g. `parsec-00000000.png`)
2. Use this file name as usual in your template code
3. Run `python misc/update_server_static_assets_hash_prefix.py`, this will
   update the file (and the templates referencing it) to its correct name.

## Modifying a file

1. Modify the file content
2. Run `python misc/update_server_static_assets_hash_prefix.py`
