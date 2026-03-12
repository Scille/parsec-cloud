<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# [RFC 1027] Workspace Chain of Command

## 1 - Overview

When all OWNERs of a workspace are revoked, the workspace becomes orphaned: no one can manage roles, perform key rotation etc.
This RFC introduces a **self-promotion** mechanism allowing one of the highest-ranked remaining user to claim the OWNER role.

From the GUI point of view, this feature would work as follow:

1. In the workspace list, an orphaned workspace (where the current user is eligible to self-promote) is shown with a
   warning indicator signaling that the workspace has no active owner.
2. A dedicated action (e.g. a button or context-menu entry labelled **"Claim ownership"** or **"Become owner"**)
   is displayed alongside the workspace entry.
3. When the user triggers that action, a confirmation dialog explains the situation
   ("All owners of this workspace have been revoked. As one of the highest-ranked remaining member,
   you can promote yourself to owner.") and asks for explicit confirmation before proceeding.
4. On confirmation, `client_self_promote_to_workspace_owner` is called in the background.
5. Once the call succeeds, the workspace list refreshes: the warning indicator disappears
   and the user's role is shown as OWNER.

## 2 - Self-promotion rules

The role hierarchy is: **OWNER > MANAGER > CONTRIBUTOR > READER**.

When a realm has **zero non-revoked users with the OWNER role**, a user with the highest remaining role (rank N-1 relative to OWNER) may self-promote to OWNER. Concretely:

1. If there are MANAGERs → a MANAGER can self-promote.
2. If there are no MANAGERs but there are CONTRIBUTORs → a CONTRIBUTOR can self-promote.
3. If there are no CONTRIBUTORs but there are READERs → a READER can self-promote.

Only **one** user needs to self-promote. Once a new OWNER exists, normal role-granting rules resume.

> [!NOTE]
>
> - Multiple users trying to self-promote concurrently is handled by the fact
>   this self-promotion is based on uploading a realm role certificate.
>   Hence we are guaranteed only a single user will succeed, the others receiving
>   an ad-hoc error telling them to poll the server for new certificates.
> - Under normal condition, a user with OUTSIDER profile cannot be given OWNER
>   or MANAGER role. However self-promotion is allowed for OUTSIDER.
>   This is because it is only allowed once only OUTSIDERs remain in the
>   workspace (highly unlikely), and OUTSIDERs can already be OWNER/MANAGER of
>   a workspace (if they have become OUTSIDER *after* the role was given to them).

## 3 - protocol

Authenticated API:

```json5
[
    {
        // If all OWNERs have been revoked in a workspace, it is up to a user
        // with the highest role to self-promote himself to OWNER.
        "major_versions": [
            5
        ],
        "cmd": "realm_self_promote_to_owner",
        "req": {
            "fields": [
                {
                    // `RealmRoleCertificate` with:
                    // - `user_id` field set to the author's user ID
                    // - `role` field set to OWNER
                    "name": "realm_role_certificate",
                    "type": "Bytes"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            {
                // Author is not among the workspace's active members with the highest role.
                "status": "author_not_allowed"
            },
            {
                "status": "realm_not_found"
            },
            {
                // A non-revoked user is already OWNER of this workspace
                "status": "active_owner_already_exists",
                "fields": [
                    {
                        "name": "last_realm_certificate_timestamp",
                        "type": "DateTime"
                    }
                ]
            },
            {
                "status": "invalid_certificate"
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
            },
            {
                // Returned if another certificate or vlob in the server has a timestamp
                // posterior or equal to our current one.
                "status": "require_greater_timestamp",
                "fields": [
                    {
                        "name": "strictly_greater_than",
                        "type": "DateTime"
                    }
                ]
            }
        ]
    }
]
```

> [!NOTE]
> We introduce this new command instead of modifying `realm_share` since:
>
> - It simplifies server internals (no need to handle self-promotion in realm share code).
> - `realm_share` requires `recipient_keys_bundle_access` which is unneeded here (since
>   self-promoting is only possible if our user already has access to the workspace !).
> - This is consistent with un-sharing being implemented with a dedicated `realm_unshare` command.
> - This simplifies backward compatibility: if a newer client wants to do a self-promote with
>   a older server, the client will receive an error complaining this command doesn't exist instead
>   of a more puzzling invalid certificate.

## 4 - Data model

No change needed: the self-promotion uses the existing `RealmRoleCertificate`

## 5 - Certificate Validation

The `RealmRoleCertificate` validation algorithm should be updated (both on client and server side).

This is a breaking change (an older Parsec client will consider a self-promotion certificate to be invalid), though this is considered acceptable since:

- Revoking all OWNERs in a workspace is a unlikely event, so there should be ample time for all users to update to a newer version of Parsec.
- The issue is present only if some users use the newer version and others the older version. In such case it should be easy enough
  for the remaining users to upgrade to the newer version.

## 6 - libparsec API

Update in the `client_list_workspaces` function:

```rust
pub struct WorkspaceInfo {
    // Existing fields
    pub id: VlobID,
    pub current_name: EntryName,
    pub current_self_role: RealmRole,
    pub is_started: bool,
    pub is_bootstrapped: bool,

    // New fields
    /// If all OWNERs have been revoked, it is up to a user with the highest role
    /// to self-promote himself to OWNER.
    /// If `true`, we are among those users... This is our chance to become king of the Realm!
    pub can_self_promote_to_owner: bool,
}


pub async fn client_list_workspaces(
    client: Handle,
) -> Result<Vec<WorkspaceInfo>, ClientListWorkspacesError> {
    // ...
}
```

> [!NOTE]
> We should also update `WorkspacesSelfListChanged` event trigger rule to also
> trigger it whenever `can_self_promote_to_owner` changes.

New `client_self_promote_to_workspace_owner` function:

```rust
pub enum ClientSelfPromoteToWorkspaceOwner {
    #[error("Component has stopped")]
    Stopped,
    #[error("Workspace realm not found")]
    WorkspaceNotFound,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("An active user is already OWNER of this workspace")]
    ActiveOwnerAlreadyExists,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn client_self_promote_to_workspace_owner(
    client: Handle,
    realm_id: VlobID,
) -> Result<(), ClientSelfPromoteToWorkspaceOwner> {
    // ...
}
```

> [!NOTE]
> We introduce this new function instead of using `client_share_workspace` to
> simplify the use of this API (intent in the function name, more precise
> error variants).

## 7 - Alternatives Considered

**Promotion by an organization ADMIN:** Rejected because it increases complexity:

- Organization ADMIN cannot decrypt the workspace and hence doesn't know the workspace name
- Would require the GUI to display the workspaces requiring promotion, and to list the workspace's members
- Would involve organization profile in the user realm certificate validation code (currently only realm role is needed)

**Automatic self-promotion:** Rejected since, unlike the automatic key rotation, it should be up to the user to decide
who should become OWNER instead of having a random member depending on whoever detects the need first.
