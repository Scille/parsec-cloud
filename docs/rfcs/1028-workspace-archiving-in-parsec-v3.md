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

### 2.3 - `OrganizationConfig` SSE event updated with `minimum_archiving_period`

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
                                "name": "minimum_archiving_period",
                                "type": "Integer",
                                "introduced_in": "5.5"
                            }
                        ]
                    }
```

### 2.4 - Administration REST API

The administration REST API already exposes `minimum_archiving_period` (in seconds) both in the
`GET /administration/organizations/<organization_id>` response and as an optional field in the
`PATCH /administration/organizations/<organization_id>` request body.  No new changes are
required here.

## 3 - Server CLI

The `--organization-initial-minimum-archiving-period` option is missing and should be
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
> The `minimum_archiving_period` should minimize the need for "un-delete" by forcing the
> workspace to stay in an "about to be deleted" state long enough for the users to notice this
> way a mistake.
>
> However, since the client can no longer access the workspace once the deletion deadline as been
> reached, this create a typical pattern where only then the users realized the deletion was a mistake.
> Hence the need for a convenient way to undo the deletion using the CLI.

## 5 - CLI commands

### 5.1 - Parsec CLI `workspace archive` command

A new `workspace archive` subcommand is added to the Parsec CLI. It allows an owner to change
the archiving configuration of a workspace:

```bash
parsec-cli workspace archive --device $DEVICE --workspace $WORKSPACE_ID --archived
parsec-cli workspace archive --device $DEVICE --workspace $WORKSPACE_ID \
    --deletion-planned 2026-05-18T00:00:00Z
parsec-cli workspace archive --device $DEVICE --workspace $WORKSPACE_ID --available
```

### 5.2 - Update to Parsec CLI `workspace list` command

The `workspace list` command is updated to display the archiving status of each workspace alongside
its name and role. Possible displayed statuses:

- *(nothing)* for available workspace
- `[archived]`
- `[deletion planned: <date>]`
- `[deleted]`

### 5.3 - Server CLI `cleanup_realms` command

This command is not available in the Parsec CLI since deleting data from the PostgreSQL
database is considered too sensitive to be available from the administration API exposed by the Parsec server.

Instead it is part of the server CLI that directly access the PostgreSQL database.

> [!NOTE]
> An important point here is that this command only modify the PostgreSQL database, it is still
> up to the server administrator to manually remove the data from the blockstore (e.g. S3).
> The rational being that the blockstore is expected to have its own backup system configured
> in a way Parsec cannot bypass (otherwise an attacker getting access to a Parsec server could
> erase all the data and their backup from the blockstore...).
> So instead the server administrator is expected to manually remove the deleted workspaces'
> data from the blockstore.

This command aims at removing two kinds of workspaces:

- Orphan workspace, i.e. Workspaces that are only shared with revoked users.
- Workspaces that have been marked for deletion.

> [!NOTE]
> It could be tempting to always delete orphan workspace right away since in theory
> nobody can access them anymore. However this is not the case for a sequestered
> organization (where the workspace's decryption keys are accessible for a sequester
> service).
> For this reason we consider the date of revocation of the last member of the
> workspace to be equivalent to an implicit workspace deletion date, and
> `--if-deadline-overdue-by-x-days` option (see below) can be used to ensure
> the actual deletion only occurs after a certain time.

```bash
# Pretend to delete all workspaces that have reached the deletion deadline
parsec cleanup_realms --dry-run
# Delete all workspaces whose deletion deadline is older than 15 days
parsec cleanup_realms --dry-run --if-deadline-overdue-by-x-days 15
# Only delete a specify workspace (or return an error if the deletion conditions are not met)
parsec cleanup_realms --realm-id 42ed56d053764eb899e3ba7f6e6ea4e3
```
