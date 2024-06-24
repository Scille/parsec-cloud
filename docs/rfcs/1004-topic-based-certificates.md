<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Topic-based certificates for greedy client processing

## 1 - Background

In Parsec v3, the client eagerly fetch the certificates from server.

The server is only allowed to provide newer certificates (i.e. certificates with
timestamp strictly greater than the last one client knows about).

However this comes with multiple challenges:

1. Sequester service certificate are signed in advance (they are signed by
  the sequester authority key which should be kept in cold storage).
2. When a realm is shared with a user, his client must now process new realm role
  certificates that most likely have older timestamps.
3. Shamir certificate don't need to be public (should only be known by the author and
  it recipients), however as a certificate it timestamp would impact the processing
  of other certificates.

The obvious solution for point 2&3 is to give access to all certificate to any user,
however this is not a perfect solution:

- It doesn't solve 1.
- It gives away sensitive information to unrelated user (shamir recovery) or allows
  to deduce who's working with who by analyzing realm role certificates.
- It is wasteful on resources (in case of very big organization where realm sharing
  is very frequent)

So instead the approach discussed here is to split the certificates into isolated
topics which each their own list certificate timestamp information.

## 2 - General approach

Divide the certificates into multiple topics.

- common

  - `UserCertificate`
  - `DeviceCertificate`
  - `UserUpdateCertificate`
  - `RevokedUserCertificate`

- sequester

  - `SequesterAuthorityCertificate`
  - `SequesterServiceCertificate`

- shamir

  - `ShamirRecoveryBriefCertificate`
  - `ShamirRecoveryShareCertificate`

- realm

  - `RealmRoleCertificate`
  - `RealmArchivingCertificate`

Each user is allowed to tracks different topics:

- common: all users see everything
- sequester: all users see everything
- shamir:

  - only `ShamirRecoveryBriefCertificate` certificates the user is part of (i.e.
    user is author or among recipients)
  - only `ShamirRecoveryShareCertificate` certificates the user is recipient

- realm: certificates only accessible to users that are (or used to be) part of the
  realm. A user cannot see the certificates created after he has left the realm.

> **TODO:**
>
> - Now that sequester service certificate are greedily fetched, how to tell not to use
>   a deprecated service ? Add a `SequesterDeletedServiceCertificate` ?
> - How to detect the shamir is no longer valid ? Add a `ShamirDeletedRecoveryCertificate` ?
> - User with ADMIN profile may need to see all the realm (e.g. to implement the feature
>   to promote a user OWNER of a realm with previous OWNER revoked). Should this be a
>   a special case here, or should be implement a specific command for that (I guess
>   the latter solution is simpler)
> - When changing the user's profile to OUTSIDER, if the user is OWNER/MANAGER of a shared
>   realm things get messy: the operation is not allowed, so the admin should be prompted
>   about the realm that cause the issue... but what about if the admin is not part of the
>   realm ? Here it seems just allowing an OUTSIDER to stay OWNER/MANAGER is just much
>   simpler.

## Client-side operations

On client side, the certificate ops handle each topic in isolation, and only accepts to
add certificate with a newer timestamp.

So this is valid:

1) Add Alice user certificate with timestamp T1
2) Add a sequester service certificate with timestamp T3
3) Add Alice revoked user certificate with timestamp T2

At the end, the last timestamp is T3 for topic `common` and T2 for topic `sequester`.

## Server-side operations

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "certificate_get",
            "fields": [
                {
                    // Skip the certificates before this timestamp
                    "name": "common_after",
                    "type": "RequiredOption<DateTime>"
                },
                {
                    // Skip the certificates before this timestamp
                    "name": "sequester_after",
                    "type": "RequiredOption<DateTime>"
                },
                {
                    // Skip the certificates before this timestamp
                    "name": "shamir_after",
                    "type": "RequiredOption<DateTime>"
                },
                {
                    // Skip the certificates before this timestamp
                    "name": "realm_after",
                    // Key is the realm ID
                    "type": "Map<VlobID, RequiredOption<DateTime>>"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    // Certificates are provided in-order (i.e. with growing timestamps)
                    {
                        "name": "common_certificates",
                        "type": "List<Bytes>"
                    },
                    {
                        "name": "sequester_certificates",
                        "type": "List<Bytes>"
                    },
                    {
                        "name": "shamir_certificates",
                        "type": "List<Bytes>"
                    },
                    {
                        "name": "realm_certificates",
                        // Key is the realm ID
                        "type": "Map<VlobID, List<Bytes>>"
                    }
                ]
            }
        ]
    }
]
```

### Example

Alice, whose Bob has just shared a new realm (id: 42) with, calls `certificate_get` and gets:

```json5
{
    "common_certificates": [],
    "sequester_certificates": [],
    "shamir_certificates": [],
    "realm_certificates": [
        {
            42: [
                "realm_42_role_certificate_1",  // Certificate that created the realm
                "realm_42_role_certificate_2",  // Potential other certificates
                ...
                "realm_42_role_certificate_100_share_with_alice",
                "realm_42_role_certificate_101",  // Potential other certificates
                ...
            ]
        }
    ],
}
```

Then Alice got remove from the sharing by Bob. She calls `certificate_get` again (with
the timestamp of the last realm role certificate she knows of) and gets:

```json5
{
    "common_certificates": [],
    "sequester_certificates": [],
    "shamir_certificates": [],
    "realm_certificates": [
        {
            42: [
                "realm_42_role_certificate_200",  // Potential other certificates
                ...
                "realm_42_role_certificate_300_unshare_with_alice",
                // No other certificates provided from this point on !
            ]
        }
    ],
}
```

So from the server point of view, things goes this way:

- take all common certificates, filter by timestamp too old
- take all sequester & shamir certificates, filter with user, filter by timestamp too old
- take the last realm role certificate for each realm for the given user, then for each realm:

  - take all realm certificates, filter by timestamp too old
  - if the user has no longer access to the realm, also filter by timestamp too recent according
    to the last time the user had access
