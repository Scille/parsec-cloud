<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Shamir API & types

## Abstract

This document describes design and implementation considerations to implement
Shamir-based user account recovery in Parsec. This feature is based on
[Shamir's secret sharing (SSS)](https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing)
algorithm.

Essentially, a User should be able to distribute the information required to
recover its account (the "secret") among a group of User of the same
Organization. The information is divided into parts (the "shares") from which
the secret can be reassembled only when quorum is achieved, i.e. a sufficient
number of shares (the "threshold") are combined, therefore enabling the
recovery of the user account.

The idea is that even if an attacker steals some shares, it is impossible for
the attacker to reconstruct the secret unless they have stolen the quorum
number of shares.

## 0 - Basic considerations

### Number of Shamir recovery setup per User

A User can have at most one Shamir recovery setup:

- a new User starts with no Shamir recovery setup
- an existing Shamir recovery setup can be deleted
- a new Shamir recovery setup cannot be setup if another one already exists

This means that only a single Shamir recovery setup (at most) is available as
a given time for a given User. This is sufficient because:

- Shamir recovery can achieve weight-based strategy (e.g. recovery can be
  achieved by 3 normal managers or by a single top ranked one that got
  assigned 3 shares instead of 1). This is enough even for advanced use-cases.
- Shamir recovery contains sensible (encrypted) data, so it should be possible
  to clear it as soon as it is no longer needed.
- An attacker is unlikely to take advantage of this: in order to modify the
  Shamir recovery setup, it must have got access to the user account. So
  the attack is about denying somebody access of its account, provided that
  the user has forgotten his main password and must use the Shamir recovery!

> **_Future evolution 1:_** restrict clearing of Shamir recovery setup to an
> organization Admin. This would prevent a potential attack and also prevent
> a regular User from changing their Shamir recovery without notifying the
> Admin. This should be set by the organization config system, considering
> small organization most likely don't want or cannot (i.e. too few users
> in the organization) enforce this rule.

### How does it work ?

To create the Shamir recovery:

1) Alice wants to create a Shamir recovery with Bob and Adam.
2) Alice creates a new device `alice@shamir1`, and encrypts her related keys
   using a symmetric key `SK`: `SK(alice@shamir1)`. `SK` is the secret in the
   Shamir algorithm.
3) Alice creates a `ShamirRecoveryShareCertificate` certificate for Bob
   (`SRSCBob`) and Adam (`SRSCAdam`). Each certificate contains a part of the
   `SK` secret, signed by Alice and encrypted with Bob/Adam's user key.
4) Alice sends a `shamir_recovery_setup` command to the Parsec server
   containing the `SK(alice@shamir1)`, `SRSCBob`, `SRSCAdam` and the
   Shamir `threshold`.

To use the Shamir recovery:

1) Alice has lost her account access:
   1) Alice asks Adam or Bob for a recovery invitation link.
   2) Adam (or Bob) uses the authenticated `invite_new` command to
      create a Shamir recovery invitation link (i.e. a special Parsec URL).
2) Alice uses the link to connect as invited to the Parsec server:
   1) Alice uses the `invite_info` command to retrieve information about the
      current Shamir recovery setup: `SK(alice@shamir1)`, `threshold`, User ID
      and human handle of recipients.
   2) Recipients can use the `invite_list` command to see the current Shamir
      recovery they can take part in.
3) Alice uses the `invite_x_claimer_*` commands to create a secure conduit with
   a recipient. The recipient uses the authenticated `invite_x_greeter_*` commands.
4) The recipient use `invite_4_greeter_communicate` to send to Alice her secret
    share(s). Alice uses the `invite_4_claimer_communicate` command (with an
    empty payload) to receive the secret share (i.e. `SRSCBob` and `SRSCAdam`).
5) If Alice hasn't reach a quorum (i.e. the number of secret shares obtained are
   less than `threshold`) she go back to 2). Otherwise she can compute the
   secret `SK` and decrypt `SK(alice@shamir1)`.
6) Alice uses `alice@shamir1` to re-create a new device (the recovered device).

## 1 - Create a Shamir recovery setup

Authenticated API:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "shamir_recovery_setup",
            "fields": [
                {
                    // The actual data we want to recover.
                    // It is encrypted with `data_key` that is itself split into shares.
                    // This should contain a serialized `LocalDevice`
                    "name": "ciphered_data",
                    "type": "Bytes"
                },
                {
                    // The token the claimer should provide to get access to `ciphered_data`.
                    // This token is split into shares, hence it acts as a proof the claimer
                    // asking for the `ciphered_data` had its identity confirmed by the recipients.
                    "name": "reveal_token",
                    "type": "InvitationToken"
                },
                {
                    // The Shamir recovery setup provided as a `ShamirRecoveryBriefCertificate`.
                    // It contains the threshold for the quorum and the shares recipients.
                    // This field has a certain level of duplication with the "shares" below,
                    // but they are used for different things (each encrypted share is
                    // only provided to its recipient, so the shamir recovery author won't
                    // get any).
                    "name": "shamir_recovery_brief_certificate",
                    "type": "Bytes"
                },
                {
                    // The shares provided as a `ShamirRecoveryShareCertificate` since
                    // each share is aimed at a specific recipient.
                    "name": "shamir_recovery_share_certificates",
                    "type": "List<Bytes>"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            {
                // Deserialization or signature verification error in the brief certificate.
                "status": "invalid_certificate_brief_corrupted"
            },
            {
                // Deserialization or signature verification error in a share certificate.
                "status": "invalid_certificate_share_corrupted"
            },
            {
                "status": "invalid_certificate_share_recipient_not_in_brief"
            },
            {
                "status": "invalid_certificate_duplicate_share_for_recipient"
            },
            {
                "status": "invalid_certificate_author_included_as_recipient"
            },
            {
                "status": "invalid_certificate_missing_share_for_recipient"
            },
            {
                "status": "invalid_certificate_share_inconsistent_timestamp"
            },
            {
                "status": "invalid_certificate_user_id_must_be_self"
            },
            {
                "status": "recipient_not_found"
            },
            {
                "status": "revoked_recipient",
                "fields": [
                    {
                        "name": "last_common_certificate_timestamp",
                        "type": "DateTime"
                    }

                ]
            },
            {
                "status": "shamir_recovery_already_exists",
                "fields": [
                    {
                        "name": "last_shamir_certificate_timestamp",
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
            }
        ]
    }
]
```

Consistency between `brief` and `shares` must be checked by the Parsec server:

- the number of shares must be greater or equal to the `threshold` and
  `threshold` must be greater or equal to 1.
- the recipients and their shares must be the same.
- the certificates datetimes & authors must be the same.

> **_Future evolution 1:_** Check the Shamir recovery setup against some
> organization-specific rules (e.g. the number of shares, the recipient's
> profiles, max number of share per recipient, etc.). See "Bonus" section below.

And the related certificates:

```json5
{
    "label": "ShamirRecoveryShareCertificate",
    "type": "shamir_recovery_share_certificate",
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
            /// User here must be the one owning the device used as author
            /// (i.e. it is the user to be recovered).
            "name": "user_id",
            "type": "UserID"
        },
        {
            /// Recipient is the user that will be able to decrypt the share.
            "name": "recipient",
            "type": "UserID"
        },
        {
            // The actual share as `ShamirRecoveryShareData`, signed by the author
            // and ciphered with the recipient's user key.
            "name": "ciphered_share",
            "type": "Bytes"
        }
    ]
}
```

Note: The share data is signed by the author in order to prevent attacks where a user puts someone else's share in the certificate in order to trick a recipient user into deciphering it.

```json5
{
    "label": "ShamirRecoveryShareData",
    "type": "shamir_recovery_share_data",
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
            // Weighted share to recover the secret key and the reveal token
            // The number of items in the list corresponds to the weight of the share
            "name": "weighted_share",
            "type": "List<ShamirShare>"
        }
    ]
}
```

Here, `ShamirShare` is a new cryptography primitive corresponding to a share produced by the Shamir algorithm.

The Shamir algorithm is meant to be fed with a serialized dump of a `ShamirRecoverySecret` instance, defined as such:

```json5
{
    "label": "ShamirRecoverySecret",
    "type": "shamir_recovery_secret",
    "other_fields": [
        {
            "name": "data_key",
            "type": "SecretKey"
        },
        {
            "name": "reveal_token",
            "type": "InvitationToken"
        }
    ]
}
```

This secret contains both:

- the secret key used to the encrypt the `ciphered_data` provided in the shamir recovery setup
- the reveal token to retrieve this `ciphered_data` from the server

```json5
{
    "label": "ShamirRecoveryBriefCertificate",
    "type": "shamir_recovery_brief_certificate",
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
            /// User here must be the one owning the device used as author
            /// (i.e. it is the user to be recovered).
            "name": "user_id",
            "type": "UserID"
        },
        {
            // Minimal number of shares to retrieve to reach the quorum and compute the secret
            "name": "threshold",
            "type": "NonZeroU8"
        },
        {
            // A recipient can have multiple shares (to have a bigger weight than others)
            "name": "per_recipient_shares",
            "type": "Map<UserID, NonZeroU8>"
        }
    ]
}
```

## 2 - Delete a shamir setup

Authenticated API:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "shamir_recovery_delete",
            "fields": [
                {
                    "name": "shamir_recovery_deletion_certificate",
                    "type": "Bytes"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            {
                // Deserialization or signature verification error.
                "status": "invalid_certificate_corrupted"
            },
            {
                "status": "invalid_certificate_user_id_must_be_self"
            },
            {
                "status": "shamir_recovery_not_found"
            },
            {
                // The deletion certificate refers to a shamir recovery whose recipients
                // differ (brief certificate's `per_recipient_shares` field vs deletion
                // certificate's `share_recipients` field).
                "status": "recipients_mismatch"
            },
            {
                "status": "shamir_recovery_already_deleted",
                "fields": [
                    {
                        "name": "last_shamir_certificate_timestamp",
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
            }
        ]
    }
]
```

And the related certificate:

```json5
{
    "label": "ShamirRecoveryDeletionCertificate",
    "type": "shamir_recovery_deletion_certificate",
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
            // The timestamp of the shamir recovery this certificate removes.
            // Given the timestamp are strictly growing, unique identification
            // can be done with the couple user_id + setup_timestamp.
            "name": "setup_to_delete_timestamp",
            "type": "DateTime"
        },
        {
            // User here must be the one owning the device used as author
            // (i.e. a user can only remove its own Shamir recovery)
            "name": "setup_to_delete_user_id",
            "type": "UserID"
        },
        {
            // Must correspond to the brief certificate's `per_recipient_shares`.
            "name": "share_recipients",
            "type": "Set<UserID>"
        }
    ]
}
```

The certificate needs to include the previous certificate timestamp in the deletion
certificate to link both certificates together: the couple user_id + timestamp is
enough to uniquely identify a shamir recovery setup.

### How to decide if a deletion certificate is valid ?

A setup can be identified by the tuple (author_user_id, timestamp) that we'll call shamir_id.
To decide if a deletion certificate is valid, the following questions must be asked:

1. Is there a shamir setup with the corresponding shamir id ? No, means `setup_not_found`
2. Has this shamir id already a corresponding deletion certificate ? Yes, means `setup_already_deleted`
3. Is the share recipients list the same ? No, means `recipients_mismatch`

## 3 - Get the Shamir recovery certificates

The shamir related certificates are retrieved by the `get_certificates` route, with other certificates depending on the timestamp.
If the user has authored a shamir setup, they will get the corresponding brief.
If they are a share recipient, they will get their share and the associated brief.

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
                    // Skip the certificates before (or at) this timestamp
                    "name": "shamir_recovery_after",
                    "type": "RequiredOption<DateTime>"
                },
                // <-------------- Other fields omitted --------->

            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    // Certificates are provided in-order (i.e. with growing timestamps)
                    {
                        "name": "shamir_recovery_certificates",
                        "type": "List<Bytes>"
                    },
                    // <-------------- Other fields omitted --------->

                ]
            }
        ]
    }
]
```

Who need which certificate ?

| certificate | author | share recipient |
|-------------|--------|-----------------|
| brief       | x      | x               |
| share       |        | x               |
| deletion    | x      | x               |

## 4 - Use the Shamir recovery

### 4.1 - A recipient creates an invitation

Authenticated API for creating the invitation:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_new",
            "unit": "InvitationType"
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "token",
                        "type": "InvitationToken"
                    },
                    // Added in API 2.3 (Parsec v2.6.0)
                    // Field used when the invitation is correctly created but the invitation email cannot be sent
                    {
                        "name": "email_sent",
                        "type": "InvitationEmailSentStatus",
                        "introduced_in": "2.3"
                    }
                ]
            },
            // <----------- other reps omitted ------------>

            // TODO: A specific reps is needed for when the target user has not Shamir recovery available
            // `not_available` seems like the obvious name, but it is already used for an unrelated
            // situation, so not sure if we should reuse (and rename the status for the unrelated
            // situation ? this is the right time do to if we do a major version bump !) or create
            // another one...
            {
                "status": "not_available"
            }
        ],
        "nested_types": [
            {
                "name": "InvitationType",
                "discriminant_field": "type",
                "variants": [
                    {
                        "name": "ShamirRecovery",
                        "discriminant_value": "SHAMIR_RECOVERY",
                        "fields": [
                            {
                                "name": "claimer_user_id",
                                "type": "UserID"
                            },
                            {
                                "name": "send_email",
                                "type": "Boolean"
                            }
                        ]
                    },
                    // <-------------- User variant omitted --------->
                    // <-------------- Device variant omitted --------->
                ]
            }
        ]
    }
]
```

Authenticated API for listing the invitation:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_list"
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "invitations",
                        "type": "List<InviteListItem>"
                    }
                ]
            }
        ],
        "nested_types": [
            {
                "name": "InviteListItem",
                "discriminant_field": "type",
                "variants": [
                    {
                        "name": "ShamirRecovery",
                        "discriminant_value": "SHAMIR_RECOVERY",
                        "fields": [
                            {
                                "name": "token",
                                "type": "InvitationToken"
                            },
                            {
                                "name": "created_on",
                                "type": "DateTime"
                            },
                            {
                                "name": "claimer_user_id",
                                "type": "UserID"
                            },
                            {
                                "name": "status",
                                "type": "InvitationStatus"
                            }
                        ]
                    }
                    // <-------------- User variant omitted --------->
                    // <-------------- Device variant omitted --------->
                ]
            }
        ]
    }
]
```

### 4.2 - Claimer connect to the server

Invited API, we reuse the `invite_info` command:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_info"
        },
        "reps": [
            {
                "status": "ok",
                "unit": "InvitationType"
            }
        ],
        "nested_types": [
            {
                "name": "InvitationType",
                "discriminant_field": "type",
                "variants": [
                    // <-------------- User variant omitted --------->
                    // <-------------- Device variant omitted --------->
                    {
                        "name": "ShamirRecovery",
                        "discriminant_value": "SHAMIR_RECOVERY",
                        "fields": [
                            {
                                "name": "threshold",
                                "type": "NonZeroU8"
                            },
                            {
                                "name": "recipients",
                                "type": "List<ShamirRecoveryRecipient>"
                            }
                        ]
                    }
                ]
            },
            {
                "name": "ShamirRecoveryRecipient",
                "fields": [
                    {
                        "name": "user_id",
                        "type": "UserID"
                    },
                    {
                        "name": "human_handle",
                        "type": "RequiredOption<HumanHandle>"
                    },
                    {
                        "name": "shares",
                        "type": "NonZeroU8"
                    },
                    {
                        "name": "revoked_on",
                        "type": "RequiredOption<DateTime>"
                    }
                ]
            }
        ]
    }
]
```

Signed data (such as `ShamirRecoveryBriefCertificate`) is not provided here
because the claimer has no way to verify the certificate (i.e. it doesn't know
the root verify key yet).

Also, `ciphered_data` is not provided until the claimer to have concluded it SAS
code exchange with all the greeters (this would guarantee `ciphered_data` is
only provided to the actual User, and not to an attacker that have eavesdropped
the invitation link).

### 4.3 - Claimer dance for SAS code exchange

On claimer side, the `invite_x_claimer_*` API is reused.

A single change is required: passing the recipient `user_id` as parameter
(the `invite_x_claimer_*` was used in one-to-one and no parameter was requested)

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            // Same for the others invite_x_claimer_* cmds
            "cmd": "invite_1_claimer_wait_peer",
            "fields": [
                {
                    "name": "greeter_user_id",
                    "type": "UserID"
                }
                // <-------------- Other fields omitted --------->
            ]
        },
        "reps": [
            {
                // Currently returned if invitation is no longer available
                // now also returned if `greeter_user_id` has an invalid value
                "status": "not_found"
            }
            // <-------------- Other reps omitted --------->
        ],
    }
]
```

So `greeter_user_id` is always provided:

- User/Device invitation: its only valid value is the `greeter_user_id` provided in `invite_info`.
- Shamir recovery invitation: it is one of the `recipients` provided in `invite_info`.

Internally, the invite API is implemented (both on claimer and greeter side)
in the Parsec server with a generic `conduit_exchange` command.
Currently `conduit_exchange` use the invitation token as identifier for
communication: a greeter and an claimer communicate by doing `conduit_exchange`
with the same invitation token.
However this is no longer possible because in Shamir recovery the claimer will
talk to multiple different greeters in parallel. The solution is then to use the
pair "invitation token" + "greeter UserID" as identifier.

### 4.4 - Greeter dance for SAS code exchange

On greeter side, the `invite_x_claimer_*` API is reused as-is.

The `invite_list` and `invite_delete` commands can also be used as-is to manage the Shamir recovery invitations.

### 4.5 - Greeter & claimer actual secret share exchange

This is done with the step 7 of the greeting attempt (greeter step `SendPayload`/ claimer step `GetPayload`).

Greeter payload:

```json5
{
    "label": "InviteShamirRecoveryConfirmation",
    "type": "invite_shamir_recovery_confirmation",
    "other_fields": [
        {
            // Weighted share to recover the secret key and the reveal token
            // The number of items in the list corresponds to the weight of the share
            "name": "weighted_share",
            "type": "List<ShamirShare>"
        }
    ]
}
```

Note that the claimer does not have a way to identify from which setup the share
is from. This information is not sent by the greeter.
In theory this means that a recipient could send an incompatible share to the claimer:

- When a new setup is made in the middle of the invitation recovery
- When a malicious Parsec server provide a previous setup to one (or multiple) recipient(s)

This is not a big concern because in all those cases the claimer will end up with an invalid
secret and won't be able to decrypt the recovery data.

Once enough shares are collected, the secret can be computed.

The claimer gets access to `reveal_token` and `data_key`, it can then retrieve `ciphered_data`:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_shamir_recovery_reveal",
            "fields": [
                {
                    "name": "reveal_token",
                    "type": "InvitationToken"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "ciphered_data",
                        "type": "Bytes"
                    }
                ]
            },
            {
                "status": "bad_invitation_type"
            },
            {
                "status": "bad_reveal_token"
            }
        ]
    }
]
```

Then `ciphered_data` can be decrypted with `data_key`. From then on, the recovery works
just like the recovery device system (see `parsec core import_recovery_device` CLI).

## Bonus

### **Future evolution 1**: Specify a Shamir recovery setup template

Shamir recovery allows plenty of different configurations (single recipient, different
weight per recipient etc.), but we want to be able to set some limits here using the
organization config system.

There are two approaches for this:

1) Use a very specific configuration template
2) Use limit-based rules

Approach 2) is the simplest, for instance we could limit have something like:

authenticated API, `organization_config`

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "organization_config"
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "shamir_recovery_min_shares",
                        // Default should be 1
                        "type": "NonZeroU8"
                    },
                    {
                        "name": "shamir_recovery_max_shares",
                        // `None` for no limit (the default)
                        "type": "RequiredOption<NonZeroU8>"
                    },
                    {
                        "name": "shamir_recovery_max_shares_per_recipient",
                        // `None` for no limit (the default)
                        "type": "RequiredOption<NonZeroU8>"
                    },
                    {
                        "name": "shamir_recovery_recipient_allowed_profiles",
                        // Default would be all profiles
                        "type": "List<UserProfile>"
                    },
                    // <------------ Already existing options omitted --------->
                ]
            },
            {
                "status": "not_found"
            }
        ]
    }
]
```

On the other hand, approach 1) allow things like "recovery requires Alice, Bob and Adam" or "recovery
requires Alice and Bob, or Adam alone". However given we cannot trust the server
on such precise configuration, a new certificate type must be introduced which is cumbersome :(
Also we should be able to provide approach 2) as part of approach 1).

### **Future evolution 2**: notify shamir author when shamir secret become irrecoverable

If Alice has setup a shamir with Bob, Mallory and John, each of then having one share and a threshold of two.
Then Bob and Mallory leave the organization. So this setup becomes unusable.

The goal would be to have a notification to prompt the user to setup a new shamir.
Depending on the configuration, if any of the share recipient is deleted a warning could be sent first
event if the shamir could still be used.

Two propositions could mitigate that:

- each time a user is deleted, if they were a share recipient a notification could be sent to users on the other end of this shamir setup.
- at each connection, check if share recipients are still valid.
