# Workspace archiving and deletion

## 0 - Basic considerations

At the moment, there is no way to delete or archive a workspace. The aim of this RFC is to specify the changes to the data model, server API, server code and client to handle this use case.

In particular, the owner of a workspace should be able to archive said workspace making it read-only for every users that is part of it. It should also be possible for the owner to set a deletion date after which the workspace is marked as deleted and becomes unavailable for all users. The data is then free to be deleted by a cleanup routine on the server, according to a configured policy. Note that owners are free to override the previous status (archiving and deletion date) as long as the workspace is not marked as deleted.


Additionally, a minimum archiving period can be configured by the admin at the organization level in order to prevent hasty deletion (e.g, a 30 days period can be configured so that any workspace stays recoverable for at least 30 days after it's been archived).

## 1 - General approach

In the same way a given device has to store the information about its role for the different workspaces it has access to, a workspace archived/deletion status has to remain available for the device even when it is offline. This way, the interface can still behave according to the current workspace status even when the connection to the server is not available.

For this reason, it makes sense to add new "realm archiving certificates" (similar to "realm role certificate") to the trust chain, even though the deletion itself is not managed in cryptographically secure way (i.e any actor with access to the server could remove data).

Similarly, the devices can keep up-to-date with the current workspace status by listening to dedicated events, and explicitly check for the current state when opening a new connection. This is especially convenient for new v3 parsec API that proactively downloads the certificates and store them in a persistent way. However the v2 parsec API is less suited for this as the current behavior for managing realm roles is to add this information to the user manifest which is something we want to avoid for the workspace archiving status (in order to avoid adding more legacy). For this reason, the target here is to adapt the existing APIs in order to ease the transition to the v3 API later on.

More specifically, the following API changes are required:

- Add the new archiving certificate to the data model, representing the current archiving/deletion status
- Add a authenticated command to upload this certificate for a given realm
- Add a dedicated event representing a change in the archiving/deletion status
- Add an authenticated command to get the all the current archiving status at once (this will be used only in API v2 when a new connection is created in order to reach the correct state as soon as the connection becomes available)
- Update the realm commands to return specific status when the realm is not available due to its archiving status
- Update the organization configuration API to include the minimum archiving period


## 2 - Data model


The archiving status can be configured using one of 3 variants, `Available`, `Archived` and `DeletionPlanned`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RealmArchivingConfiguration {
    Available,
    Archived,
    DeletionPlanned(DateTime),
```

This configuration is included in a certificate signed by the owner:
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
            "type": "RealmID",
        }
        {
            "name": "configuration",
            "type": "RealmArchivingConfiguration",
        }
    ]
}
```

The following constraints should be verified:
- `author` and `timestamp` are verified as any other certificate
- the user corresponding to `author` must be owner of the realm
- the deletion date should respect the configured minimum archiving period


**Note 1:** A new archiving configuration completely overrides the previous one. That means that it's irrelevant whether the workspace was already archived or not, the minimum archiving period is checked against the `timestamp` provided by the new manifest.


**Note 2:** This data model does not allow for planning a future archiving, only the deletion can be planned. TODO: check with Jérôme that it's ok.

**Note 3:** There is no configuration variant to delete a workspace right away. However, if the minimum archiving period is configured to zero, it allows de facto for immediate deletion by using a `DeletionPlanned` configuration where the deletion date is set to the certificate timestamp. It is then possible for the user interface to transparently handle this case if necessary (e.g providing a `Delete immediately` button if the configuration allows it).

**Note 4:** The archiving configuration can be update regardless of the maintenance status as those two features are independent.

## 3 - Protocol

The following command is added to the authenticated API:

```json5
[
    {
        "major_versions": [
            2,
            3
        ],
        "req": {
            "cmd": "realm_update_archiving",
            "fields": [
                {
                    "name": "archiving_certificate",
                    "type": "Bytes" // Contains a signed RealmArchivingCertificate
                }
            ]
        },
        "reps": [
            // Following return status are similar to `realm_update_roles`
            {
                "status": "ok"
            },
            {
                // Returned if the user does not have the `owner` role
                "status": "not_allowed"
            },
            {
                "status": "invalid_certification"
            },
            {
                "status": "invalid_data"
            },
            {
                "status": "not_found"
            },
            {
                "status": "require_greater_timestamp",
                "fields": [
                    {
                        "name": "strictly_greater_than",
                        "type": "DateTime"
                    }
                ]
            },
            {
                "status": "bad_timestamp",
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
                        "name": "backend_timestamp",
                        "type": "DateTime"
                    },
                    {
                        "name": "client_timestamp",
                        "type": "DateTime"
                    }
                ]
            },
            // Those are specific return status
            {
                "status": "realm_deleted"
            },
            {
                "status": "archiving_period_too_short"
            },
        ]
    }
]
```

When this command returns successfully, a `REALM_ARCHIVING_UPDATED` event should be sent to all users in the corresponding realms.

Also, an authenticated command is added to get the all the current archiving status at once.

The archiving status is defined using the following variants:

```json5
        "nested_types": [
            {
                "name": "RealmArchivingStatus",
                "discriminant_field": "type",
                "variants": [
                    {
                        "name": "Available",
                        "discriminant_value": "AVAILABLE"
                    },
                    {
                        "name": "Archived",
                        "discriminant_value": "ARCHIVED",
                        "fields": [
                            {
                                "name": "archiving_date",
                                "type": "Datetime"
                            }
                        ]
                    },
                    {
                        "name": "DeletionPlanned",
                        "discriminant_value": "DELETION_PLANNED",
                        "fields": [
                            {
                                "name": "archiving_date",
                                "type": "Datetime"
                            },
                            {
                                "name": "deletion_date",
                                "type": "Datetime"
                            }
                        ]
                    },
                    {
                        "name": "Deleted",
                        "discriminant_value": "DELETED",
                        "fields": [
                            {
                                "name": "archiving_date",
                                "type": "Datetime"
                            },
                            {
                                "name": "deletion_date",
                                "type": "Datetime"
                            }
                        ]
                    }
                ]
            }
        ]
```

The command will be used in API v2 when a new connection is created in order to reach the correct state as soon as the connection becomes available.

It's also used in API v2 to fetch to new archiving status when an `ARCHIVING_STATUS_UPDATED` event is received.

It is defined as such:

```json5
[
    {
        "major_versions": [
            2,
            3
        ],
        "req": {
            "cmd": "realm_archiving_config"
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "realm_archiving_configs",
                        "type": "List<RealmArchivingConfig>"
                    },
                ]
            },
            {
                "status": "not_found"
            }
        ],
        "nested_types": [
            {
                "name": "RealmArchivingConfig",
                "fields": [
                    {
                        "name": "realm_id",
                        "type": "RealmID"
                    },
                    {
                        "name": "archiving_status",
                        "type": "RealmArchivingStatus"
                    }
                ]
            }
        ]
    }
]
```

**Note:** The command does not allow to get the archiving status for a single realm, which would be potentially more efficient to fetch the new archiving status of a freshly updated realm. This is however not a big deal as this is a temporary command for the v2/v3 transition. With v3 API, the certificates will be eagerly downloaded and this is how the device will get the new archiving status.

In addition, the following commands are updated with new reply status:


| Check for archived/deleted status          | archived | deleted |
|--------------------------------------------|----------|---------|
| `realm_create`                             |          |         |
| `realm_status`                             |          | ✔️       |
| `realm_stats`                              |          | ✔️       |
| `realm_get_role_certificates`              |          |         |
| `realm_update_roles`                       |          | ✔️       |
| `realm_update_archiving`                   |          | ✔️       |
| `realm_start_reencryption_maintenance`     |          | ✔️       |
| `realm_finish_reencryption_maintenance`    |          | ✔️       |
| `vlob_create`                              | ✔️        | ✔️       |
| `vlob_read`                                |          | ✔️       |
| `vlob_update`                              | ✔️        | ✔️       |
| `vlob_poll_changes`                        |          | ✔️       |
| `vlob_list_versions`                       |          | ✔️       |
| `vlob_maintenance_get_reencryption_batch`  |          | ✔️       |
| `vlob_maintenance_save_reencryption_batch` |          | ✔️       |
| `block_read`                               |          | ✔️       |
| `block_create`                             | ✔️        | ✔️       |



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


The organization config API is also modified to include the minimum archiving period:
```json5
[
    {
        "major_versions": [
            2,
            3
        ],
        "req": {
            "cmd": "organization_config"
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    [...]
                    {
                        "name": "minimum_archiving_period",
                        "type": "Integer", // In seconds
                        "introduced_in": "2.10"
                    }
                ]
            },
            {
                "status": "not_found"
            }
        ]
    }
]
```

and:

```python
class OrganizationUpdateReqSchema(BaseSchema):
    # /!\ Missing field and field set to `None` does not mean the same thing:
    # - missing field: don't modify this field
    # - field set to `None`: `None` is a valid value to use for this field
    [...]
    minimum_archiving_period = fields.Integer(required=False, validate=lambda x: x >= 0)
```

Note: the value applied by default is `2592000`, i.e. 30 days.
