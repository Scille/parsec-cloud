# Sequester - RealmExtract creation CLI

From [ISSUE-2465](https://github.com/Scille/parsec-cloud/issues/2465)

see [RFC-0021](0021-sequester-realm-extract-file-format.md) for data format

requires [ISSUE-2474](https://github.com/Scille/parsec-cloud/issues/2474)

## CLI usage

### First, retrieve the list of workspaces

```shell
$ parsec backend sequester human_accesses --db postgres://... --organization-id CoolOrg --query doe
Found 2 results:
Human John Doe <john.doe@example.com>

  User 9e082a43b51e44ab9858628bac4a61d9 (created on: 2000-01-02T00:00:00Z)

    Realm 8006a491f0704040ae9a197ca7501f71
      - 2000-01-04T00:00:00Z: Got MANAGER access
      - 2000-01-03T00:00:00Z: Access removed
      - 2000-01-02T00:00:00Z: Got READER access

    Realm 109c48b7c931435c913945f08d23432d
      - 2000-01-01T00:00:00Z: Got OWNER access

  User 02e0486752d34d6ab3bf8e0befef1935 (created on: 2000-01-01T00:00:00Z, revoked on: 2000-01-02T00:00:00Z)

Human Jane Doe <jane.doe@example.com>

  User baf59386baf740bba93151cdde1beac8 (created on: 2000-01-01T00:00:00Z):

    Realm 8006a491f0704040ae9a197ca7501f71
      - 2000-01-01T00:00:00Z: Got OWNER access
```

All parameters are mandatory, and `--query` is similar to what is used in `parsec core human_find` CLI command

### Then export a given workspace

```shell
$ parsec backend sequester export_realm  --db postgres://... --blockstore s3:... --organization-id <org> --realm-id <wid> --output /path/to/parsec-export-realm.sqlite
1/2 Exporting Metadata [100%]
2/2 Exporting Blocks (Roughly 90Go to download) [65%]
```

If `--output` path points on a folder, a database name should be automatically picked, typically `parsec-sequester-export-realm-<wid>.sqlite`

#### What to export

The export is divided into 3 steps:

1) export vlobs
2) export other metadatas
3) export blocks

We should pay attention to concurrency (typically if we export device certificates before vlobs we may end up with a vlob created by a newly created device and lack the corresponding certificate...).

So the simple way to avoid this is to first export the vlobs, then export all the user/devices/revocation/roles certificates (without trying to be smart here: the amount of certificates is small so it's fast anyway)

#### CLI output

The output of the cli should be divided in (at least) two parts:

0) export of the certificates: this can be be merged with 1) if it's simpler. This should be very fast
1) export of the metadata: this should be fast enough (the amount of data to retrieve from the database should be typically around 100 Mb), a progress display might be interesting here (but want to use a single SELECT no matter what to avoid pagination issues, so what we can do to display the progress depends on triopg's cursor)
2) export of the blocks: this is much slower, but we know precisely what we have to retrieve (since we've just downloaded the metadata we have the number of blocks and their size). Progress display is mandatory here !

#### Resume on cancellation

Given the amount of data that should be downloaded from S3, it would be good to resume the upload.
So the idea is:

- if the output file doesn't exist, if it format is invalid (should check the value of the `magic` field in the `info` table ) or if it doesn't contain the `vlob_atom` table
  > Start from the beginning
- if the `vlob_atom` table exists
  > We consider the vlob export step is done
- if the `user_/device/realm_role`  table exists
  > We consider the user certificate export step is done
- finally create the `block` table if it doesn't exists
- retrieve from the `block` table the list of already downloaded blocks and remove them from the list of to download blocks
- do the actual download

So the key point here is to create the table and set it content in the same transaction.
