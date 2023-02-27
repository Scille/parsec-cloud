# Update User Profile

## 0 - Basic considerations

Currently user's profile is stored in its user manifest and hence cannot change once the user is created.
This RFC aims at providing a way to update a user role by introducing a new type of manifest.

## 1 - Updating user profile

User creation is not changed. Later on, only a user with ADMIN profile can change other user's
profile by using the `user_update_profile` command.

An admin cannot change its own profile.

Authenticated API:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "user_update_profile",
            "fields": [
                {
                    "name": "certificate",
                    // Contains a `UserProfileUpdateCertificate`
                    "type": "Bytes"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            {
                // Authenticated user doesn't have an admin profile, or is trying to change it own profile
                "status": "not_allowed",
            },
            {
                // Certificate is not signed by the authenticated user, or timestamp it invalid
                "status": "invalid_certification",
            },
            {
                // Cannot deserialize data into the expected certificate
                "status": "invalid_data",
            },
            {
                // User already has the specified profile
                "status": "already_granted",
            },
            {
                // User doesn't exist
                "status": "not_found",
            },
            {
                "status": "user_revoked",
            }
        ]
    }
]
```

Certificate:

```json5
{
    "label": "UserProfileUpdateCertificate",
    "type": "user_profile_update_certificate",
    "other_fields": [
        {
            "name": "author",
            "type": "CertificateSignerOwned"
        },
        {
            "name": "timestamp",
            "type": "DateTime"
        },
        {
            "name": "user_id",
            "type": "UserID"
        },
        {
            "name": "profile",
            "type": "UserProfile",
        }
    ]
}
```

## 2 - Fetching user info

Authenticated API:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "user_get",
            "fields": [
                {
                    "name": "user_id",
                    "type": "UserID"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "user_certificate",
                        "type": "Bytes"
                    },
                    {
                        // `UserProfileUpdateCertificate`
                        "name": "user_profile_updates",
                        "type": "List<Bytes>"
                    },
                    {
                        "name": "revoked_user_certificate",
                        "type": "RequiredOption<Bytes>"
                    },
                    {
                        "name": "device_certificates",
                        "type": "List<Bytes>"
                    },
                    {
                        "name": "trustchain",
                        "type": "Trustchain"
                    }
                ]
            },
            {
                "status": "not_found"
            }
        ],
        "nested_types": [
            {
                "name": "Trustchain",
                "fields": [
                    {
                        "name": "devices",
                        "type": "List<Bytes>"
                    },
                    {
                        "name": "users",
                        "type": "List<Bytes>"
                    },
                    {
                        // `UserProfileUpdateCertificate`
                        "name": "user_profile_updates",
                        "type": "List<Bytes>"
                    },
                    {
                        "name": "revoked_users",
                        "type": "List<Bytes>"
                    }
                ]
            }
        ]
    }
]
```

## 3 - Implementation considerations

While not very complex in its API changes, this RFC has a major impact: it changes the
postulate that a user only mutability is revocation !

This will break most part of the code in unexpected ways if care is not taken.

### Server side

Typical issues we can run into:

1) Server receive an authenticated query for a `user_create` command
2) Server check the authenticated user is ADMIN
3) Server insert the new user into the database

If not done in a transaction (and with a database write lock on the authenticated user row !),
authenticated user can get its profile modified between 2) and 3) !

Worst: the profile modification can have occurred after 3), but still lead to an inconsistent
system if the datetime used is earlier than the one in the created user certificate.

We have already encounter this kind of issues with the realm role update system (the famous
"demi-ballpark" problem ^^), so similar countermeasure should be taken (i.e. ensure the
inserted certificate's datetime is not in the past compared to other items already in the
database that may interact with it).

## Client side

Current remote loader system use a 1h cache system. This works good enough considering
only revocation may invalidate the cache.

The possibilities are:

- The user is not revoked, the cache is up to date
- The user is revoked, the cache is up to date
- False positive: the user is revoked, the cache is not up to date (i.e. it consider the
  user is not revoked). The remote loader consider valid a data that have been created
  after revocation.

With profile change on top this makes things more complex:

- The user has profile X, the cache is up to date
- False positive: The user has profile Y, the cache thinks it has profile X.
  e.g. The remote loader consider valid a `user_create` that have been done after profile has switched from `ADMIN` to `REGULAR`.
- False negative: The user has profile Y, the cache thinks it has profile X.
  e.g. The remote loader consider invalid a `user_create` that have been done after profile has switched from `REGULAR` to `ADMIN`.

So on top of false positive, we now also have to handle false negative :(

The solution for this seems to add a special handle in the code when a validation failure
occurs to send a query to the server in order to make sure we aren't short of a certificate.
