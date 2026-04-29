<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Workspace archiving in Parsec v3

## 0 - Overview

This RFC ports the workspace archiving feature described in
[RFC 1002](./1002-workspace-archiving-and-deletion.md) to Parsec v3.

The key difference with Parsec v2 is that Parsec v3 clients **eagerly fetch certificates**
from the server as soon as they are available.
As a result, the archiving config can then be deduced from the certificates without
the need for a dedicated server command as it was the case in Parsec v2.

> [!NOTE]
> Since it was clear this feature was going to be ported to Parsec v3, some
> features (`RealmArchivingCertificate`, `minimum_archiving_period` config in server)
> are already implemented in the codebase (but hasn't been actually used so far).

## 1 - Data model

### 1.1 - Existing `RealmArchivingCertificate` schema

The `RealmArchivingCertificate` (introduced in RFC 1002) is reused as-is.

```json5
{
    "label": "RealmArchivingCertificate",
    "type": "realm_archiving_certificate",
    "other_fields": [
        {
            "name": "author",
            "type": "DeviceID"
        },
        {
            "name": "timestamp",
            "type": "DateTime"
        },
        {
            "name": "realm_id",
            "type": "VlobID"
        },
        {
            "name": "configuration",
            "type": "RealmArchivingConfiguration"
        }
    ],
    "nested_types": [
        {
            "name": "RealmArchivingConfiguration",
            "discriminant_field": "type",
            "variants": [
                {
                    // The realm can be read/written, this is the default
                    // status if no realm archiving certificate has been
                    // issued.
                    "name": "Available",
                    "discriminant_value": "AVAILABLE"
                },
                {
                    // The realm is now in read-only.
                    // This has precedence over workspace roles, so users
                    // with write-access (e.g. CONTRIBUTOR) would not be
                    // able to make changes on an archived realm.
                    "name": "Archived",
                    "discriminant_value": "ARCHIVED"
                },
                {
                    // The realm is now in read-only, on top of that the
                    // server is expected to delete the workspace data
                    // (i.e. vlob and blocks) once the deletion date has
                    // been reached.
                    "name": "DeletionPlanned",
                    "discriminant_value": "DELETION_PLANNED",
                    "fields": [
                        {
                            "name": "deletion_date",
                            "type": "DateTime"
                        }
                    ]
                }
            ]
        }
    ]
}
```

The validation constraints from RFC 1002 remain unchanged:

- `author` and `timestamp` are verified as for any other certificate.
- The user corresponding to `author` must be `OWNER` of the realm.
- The deletion date must respect the configured minimum archiving period.

> [!NOTE]
> This schema is already part of Parsec v3 and correctly handled by the `CertificateOps`.
> Hence no changes are required.

### 1.2 - Update `LocalUserManifest` manifest

`LocalUserManifest` has to be modified to include this per-workspace archiving status:

```json5
{
    "label": "LocalUserManifest",
    "type": "local_user_manifest",
    "other_fields": [
        // ...
        {
            "name": "local_workspaces",
            "type": "List<LocalUserManifestWorkspaceEntry>"
        },
        // ...
    ],
    "nested_types": [
        // ...
        {
            "name": "LocalUserManifestWorkspaceEntry",
            "fields": [
                {
                    "name": "id",
                    "type": "VlobID"
                },
                // ...
                // New field
                {
                    "name": "archiving_configuration",
                    // Type defined in `RealmArchivingCertificate`
                    "type": "RealmArchivingConfiguration",
                    // Introduced in Parsec 3.9.0
                    "introduced_in_revision": 390
                }
            ]
        }
    ]
}
```

> [!NOTE]
> `LocalUserManifest` works in a peculiar way: unlike other manifests, it is never uploaded
> to the server and instead its main job is to work as a cache on the info computed from
> the certificates.
>
> So adding this new `archiving_configuration` field is not strictly needed: instead the
> self-promotion need that the status could be computed from the certificates lazily
> (i.e. each time `Client::list_workspaces()` is called).
>
> However this would go against `Client::list_workspaces()` design, as it has been designed
> around the fact it should get its info from the `LocalUserManifest` that is itself
> pre-computed by a monitor whenever new certificates are fetched.

## 2 - Protocol

### 2.1 - New `realm_update_archiving` command

Authenticated API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "realm_update_archiving",
        "req": {
            "fields": [
                {
                    // Signed `RealmArchivingCertificate`
                    "name": "archiving_certificate",
                    "type": "Bytes"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            {
                // User is not part of the realm, or has no OWNER role in it
                "status": "author_not_allowed"
            },
            {
                "status": "invalid_certificate"
            },
            {
                "status": "realm_not_found"
            },
            {
                "status": "realm_deleted"
            },
            {
                // `deletion_date` in a `DeletionPlanned` configuration does
                // not satisfy the minimum archiving period.
                "status": "archiving_period_too_short"
            },
            {
                // Returned if another certificate in the server has a timestamp
                // posterior or equal to our current one.
                "status": "require_greater_timestamp",
                "fields": [
                    {
                        "name": "strictly_greater_than",
                        "type": "DateTime"
                    }
                ]
            },
            {
                // Returned if the timestamp in the certificate is too far away compared
                // to server clock.
                "status": "timestamp_out_of_ballpark",
                "fields": [
                    {
                        "name": "ballpark_client_early_offset",
                        "type": "Float"
                    },
                    {
                        "name": "ballpark_client_late_offset",
                        "type": "Float"
                    },
                    {
                        "name": "server_timestamp",
                        "type": "DateTime"
                    },
                    {
                        "name": "client_timestamp",
                        "type": "DateTime"
                    }
                ]
            }
        ]
    }
]
```

> [!NOTE]
> For convenience, the certificate is accepted even if it doesn't change the
> realm's current status.
> This behavior is similar to what is done for the the `RealmNameCertificate`
> and shouldn't cause any problems because it is always the last certificate
> that is taken into account.
> Rejecting a certificate that doesn't actually change the archiving
> configuration should be feasible, but it would require updating the
> certificate validation client-side and adding new error status.
> Since this type of certificate is not expected to be heavily used, we
> choose to go with the simplicity here.

### 2.2 - Commands updated with `realm_archived`/`realm_deleted` status

The table below mirrors the one from RFC 1002 but is adapted to the v3 command set.

| v3 command                 | archived | deleted |
|----------------------------|----------|---------|
| `realm_create`             |          |         |
| `realm_share`              |          | ✔️      |
| `realm_unshare`            |          | ✔️      |
| `realm_rename`             |          | ✔️      |
| `realm_rotate_key`         |          | ✔️      |
| `realm_get_keys_bundle`    |          | ✔️      |
| `realm_update_archiving`   |          | ✔️      |
| `vlob_create`              | ✔️       | ✔️      |
| `vlob_read_batch`          |          | ✔️      |
| `vlob_read_versions`       |          | ✔️      |
| `vlob_update`              | ✔️       | ✔️      |
| `vlob_poll_changes`        |          | ✔️      |
| `block_read`               |          | ✔️      |
| `block_create`             | ✔️       | ✔️      |

The two new response status to add to the affected commands are:

```json5
            [...]
            {
                "status": "realm_archived"
            },
            {
                "status": "realm_deleted"
            }
            [...]
```

### 2.3 - `OrganizationConfig` SSE event updated with `realm_minimum_archiving_period_before_deletion`

The `OrganizationConfig` variant of the `APIEvent` union (in `events_listen`) is extended with
a new field so that clients learn the minimum archiving period without an extra round-trip:

```json5
                    {
                        "name": "OrganizationConfig",
                        "discriminant_value": "ORGANIZATION_CONFIG",
                        "fields": [
                            // [...existing fields...]
                            {
                                // In seconds; default 2592000 (30 days)
                                "name": "realm_minimum_archiving_period_before_deletion",
                                "type": "Integer",
                                "introduced_in": "5.5"
                            }
                        ]
                    }
```

### 2.4 - Administration REST API

The administration REST API already exposes `minimum_archiving_period` (in seconds) both in the
`GET /administration/organizations/<organization_id>` response and as an optional field in the
`PATCH /administration/organizations/<organization_id>` request body.

Only change required is to rename this field as `realm_minimum_archiving_period_before_deletion` in
order to better understand what it is about.

## 3 - Server CLI

The `--organization-initial-realm-minimum-archiving-period-before-deletion` option is missing and should be
added to the the server CLI.

## 4 - Behavior

### 4.1 - When archived

Realm sharing configuration (i.e. workspace roles) is not lost when archiving or deleting.
Going from `AVAILABLE` to `ARCHIVED`/`DELETION_PLANNED` and back again to `AVAILABLE`
does not impact the existing user workspace roles.

### 4.2 -  When deletion date is reached

When a workspace has a `DeletionPlanned` configuration whose `deletion_date` has been reached,
the actual data cleanup on the server may not happen immediately (it is performed by a background
routine). This creates a window where the deletion date has passed but the data is still
physically present on the server.

The following rules apply during this window:

1. **GUI considers the realm as deleted as soon as the deadline is reached.**
   The client does not query the server to determine whether the data has actually been cleaned
   up (as it would not work when offline). Instead, it compares the current time against the
   deletion deadline and, once reached, no longer allow any access to the workspace.

> [!NOTE]
> Once the GUI has detected a workspace should be considered deleted, it also remove this
> workspace's local data (cache & not yet synchronized data).

2. **The server continues to give read-only access.**
   Until the cleanup routine has actually removed the data, the server treats the realm the same
   way as before the deletion deadline (i.e. `realm_archived` is returned for write commands,
   read commands succeed).

3. **An owner can still issue a new `RealmArchivingCertificate` to revert the status.**
   For example, an owner can switch the configuration back to `Available` or `Archived` in order
   to "un-delete" a realm that was marked for deletion by mistake. Because the GUI considers
   the realm deleted, this recovery action is only available through the Parsec CLI
   (see section 5).

> [!NOTE]
> The `realm_minimum_archiving_period_before_deletion` should minimize the need for "un-delete" by
> forcing the workspace to stay in an "about to be deleted" state long enough for the users to notice
> this way a mistake.
>
> However, since the client can no longer access the workspace once the deletion deadline as been
> reached, this create a typical pattern where only then the users realized the deletion was a mistake.
> Hence the need for a convenient way to undo the deletion using the CLI.

### 4.3 - What is deleted

Deleting a workspace means removing the following data:

- Blocks
- Vlobs
- Keys bundles

Certificates are not deleted because Parsec clients need to fetch them in order to determine
whether a realm, to which the user has access, is no longer accessible for a legitimate
reason (i.e. it has a `RealmArchivingCertificate` with a deletion date in the past).

## 5 - CLI commands

### 5.1 - Parsec CLI `workspace archive` command

A new `workspace archive` subcommand is added to the Parsec CLI. It allows an owner to change
the archiving configuration of a workspace:

```bash
parsec-cli workspace archive --device $DEVICE --workspace $WORKSPACE_ID --archived
parsec-cli workspace archive --device $DEVICE --workspace $WORKSPACE_ID \
    --deletion-planned-in-seconds 2592000
parsec-cli workspace archive --device $DEVICE --workspace $WORKSPACE_ID --available
```

### 5.2 - Update to Parsec CLI `workspace list` command

The `workspace list` command is updated to display the archiving status of each workspace alongside
its name and role. Possible displayed statuses:

- *(nothing)* for available workspace
- `[archived]`
- `[deletion planned: <date>]`
- `[deleted]`

### 5.3 - Server CLI realm deletion commands

These commands are not available in the Parsec CLI since deleting data from the PostgreSQL
database is considered too sensitive to be available from the administration API exposed by the Parsec server.

Instead they are part of the server CLI that directly accesses the PostgreSQL database.

> [!NOTE]
> An important point here is that these commands only modify the PostgreSQL database, it is still
> up to the server administrator to manually remove the data from the blockstore (e.g. S3).
> The rationale being that the blockstore is expected to have its own backup system configured
> in a way Parsec cannot bypass (otherwise an attacker getting access to a Parsec server could
> erase all the data and their backup from the blockstore...).
> So instead the server administrator is expected to manually remove the deleted workspaces'
> data from the blockstore.

Realm deletion is a three-step process:

1. Find the realms that are ready for deletion (command `list_deletable_realms`).
2. PostgreSQL cleanup, i.e. actual realm deletion (command `delete_realm`).
3. Manual blockstore cleanup, i.e. removing the realm's blocks (no command, see note above).

#### Step 1 — List deletion candidates (`list_deletable_realms`)

List the realms that are ready for deletion. Two kinds of realms can be candidates:

- **Planned for deletion**: realms whose last `RealmArchivingCertificate` has a
  `DeletionPlanned` configuration with a `deletion_date` that has been reached.
- **Orphaned**: realms that are only shared among revoked users. For these, the date of
  revocation of the last member is considered equivalent to an implicit deletion date.

A given realm can be both planned for deletion AND orphaned, in which case it appears
twice in the list (once as each kind).

> [!NOTE]
> It could be tempting to always delete orphan workspaces right away since in theory
> nobody can access them anymore. However this is not the case for a sequestered
> organization (where the workspace's decryption keys are accessible for a sequester
> service).

```bash
# List all realms eligible for deletion
parsec list_deletable_realms --organization $ORG_ID
```

The output lists each candidate with its realm ID, its kind (`orphaned` or
`deletion_planned`), and the associated date (`orphaned_since` or `deletion_date`).

#### Step 2 — Actual realm deletion (`delete_realm`)

This command internally does two things:

1. Export the IDs of all the realm's blocks
2. Proceed to delete the realm's metadata from the PostgreSQL database

Before deleting a realm's metadata, the IDs of all blocks belonging to the realm **must**
be exported. This is necessary because:

- The list of block IDs will no longer be available once the metadata is deleted.
- The blocks must be manually removed from the object storage by the server administrator
  (since the object storage has its own backup logic that Parsec cannot bypass).

> [!NOTE]
> There is no risk for the list of blocks to change between this step and the actual
> deletion since the realm is either not accessible by any user (i.e. orphaned realm) or
> is in read-only (i.e. archived realm planned for deletion).

The block IDs are then turned into slugs (i.e. paths in the object storage) according to
the object storage configuration.

```bash
# Export block slugs for a realm into a file
parsec delete_realm \
    --organization $ORG_ID \
    --realm 42ed56d053764eb899e3ba7f6e6ea4e3 \
    --dump-realm-blocks blocks-to-remove.txt
```

Options:

- `--dump-realm-blocks OUTPUT_FILE` write in `OUTPUT_FILE` the list of all the
  realm's blocks (in their slug format, i.e. the path there are referenced in
  the object storage). This list can then be used to clear the object storage
  from all those now useless objects.
  This option is required in order to ensure the user is aware that deleting the
  realm requires one more step.
- `--dry-run` Pretend to do the deletion.

#### Step 3 — Manual blockstore cleanup

As stated before, this step must be handled by the server administrator according
to his object store and backup policy.

For instance for a AWS S3 compatible storage.

```python
import boto3

BUCKET = "your-bucket-name"
FILE = "paths_to_delete.txt"

s3 = boto3.client("s3")

with open(FILE) as f:
    slugs = [line.strip() for line in f if line.strip()]

# Batch in chunks of 1000 (S3 API limit)
def chunks(lst, n):
    for i in range(0, len(lst), n):
        yield lst[i:i + n]

deleted, errors = 0, 0
for batch in chunks(slugs, 1000):
    objects = [{"Key": k} for k in batch]
    response = s3.delete_objects(
        Bucket=BUCKET,
        Delete={"Objects": objects, "Quiet": False}
    )
    deleted += len(response.get("Deleted", []))
    errors  += len(response.get("Errors", []))
    for err in response.get("Errors", []):
        print(f"  ERROR: {err['Key']} — {err['Message']}")

print(f"\nDone: {deleted} deleted, {errors} errors")
```
