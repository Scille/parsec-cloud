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
- a Shamir recovery setup always overwrites the previous one
- an existing Shamir recovery setup can be cleared

This means that instead of storing all Shamir recovery setups for a given
User, only the last setup is available. This is sufficient because:

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
2) Alice creates a new device `alice@shamir1`, and encrypt its related keys
   using a symmetric key `SK`: `SK(alice@shamir1)`. `SK` is the secret in the
   Shamir algorithm.
3) Alice create a `ShamirRecoveryShareCertificate` certificate for Bob
   (`SRSCBob`) and Adam (`SRSCAdam`). Each certificate contains a part of the
   `SK` secret.
4) Alice send a `shamir_recovery_setup` command to the Parsec server
   containing the `SK(alice@shamir1)`, `SRSCBob`, `SRSCAdam` and the
   Shamir `threshold`.

To use the Shamir recovery:

1) Alice has lost its account access:
   1) Alice asks an organization Admin for a recovery invitation link.
   2) The organization Admin uses the authenticated `invite_new` command to
      create a Shamir recovery invitation link (i.e. a special Parsec URL).
2) Alice uses the link to connect as invited to the Parsec server:
   1) Alice uses the `invite_info` command to retrieve informations about the
      current Shamir recovery setup: `SK(alice@shamir1)`, `threshold`, User ID
      and human handle of recipients.
   2) Recipients can use the `invite_list` command to see the current Shamir
      recovery they can take part in.
3) Alice uses the `invite_x_claimer_*` commands to create a secure conduit with
   a recipient. The recipient uses the authenticated `invite_x_greeter_*` commands.
4) The recipient use `invite_4_greeter_communicate` to send to Alice its secret
    share(s). Alice uses the `invite_4_claimer_communicate` command (with an
    empty payload) to receive the secret share (i.e. `SRSCBob` and `SRSCAdam`).
5) If Alice hasn't reach a quorum (i.e. the number of secret shares obtained are
   less than `threshold`) she go back to 2). Otherwise she can compute the
   secret `SK` and decrypt `SK(alice@shamir1)`.
6) Alice uses `alice@shamir1` to re-create a new device (the recovered device).

## 1 - Create & update a Shamir recovery setup

Authenticated API:

```json5
[
    {
        "major_versions": [
            4  // TODO: we most likely want to do an API bump for this feature !
        ],
        "req": {
            "cmd": "shamir_recovery_setup",
            "fields": [
                {
                    "name": "setup",
                    // Set to `None` to clear previous Shamir recovery setup
                    "type": "RequiredOption<ShamirRecoverySetup>"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
            },
            {
                // Certificate is not signed by the authenticated user, or timestamp is invalid
                "status": "invalid_certification",
            },
            {
                // Cannot deserialize data into the expected certificate, or inconsistency
                // between  certificates and/or threshold
                "status": "invalid_data",
            },
            {
                // Future evolution 1: Shamir recovery has already been setup, should ask your admin to reset it first !
                "status": "already_set",
            }
        ],
        "nested_types": [
            {
                "name": "ShamirRecoverySetup",
                "fields": [
                    {
                        // The actual data we want to recover.
                        // It is encrypted with `data_key` that is itself split into shares.
                        // This should contains a serialized `LocalDevice`
                        "name": "ciphered_data",
                        "type": "Bytes"
                    },
                    {
                        // The token the claimer should provide to get access to `ciphered_data`.
                        // This token is split into shares, hence it acts as a proof the claimer
                        // asking for the `ciphered_data` had it identity confirmed by the recipients.
                        "name": "reveal_token",
                        "type": "Bytes"
                    },
                    {
                        // The Shamir recovery setup provided as a `ShamirRecoveryBriefCertificate`.
                        // It contains the threshold for the quorum and the shares recipients.
                        // This field has a certain level of duplication with the "shares" below,
                        // but they are used for different things (we provide the encrypted share
                        // data only when needed)
                        "name": "brief",
                        "type": "Bytes"
                    },
                    {
                        // The shares provided as a `ShamirRecoveryShareCertificate` since
                        // each share is aimed at a specific recipient.
                        "name": "shares",
                        "type": "Vec<Bytes>",
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

> **_Future evolution 2:_** Check the Shamir recovery setup against some
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
            "name": "recipient",
            "type": "UserID"
        },
        {
            // Share ciphered with recipient's user key
            "name": "ciphered_share",
            "type": "ShamirRecoveryShareData"
        }
    ]
}
```

```json5
{
    "label": "ShamirRecoveryShareData",
    "type": "shamir_recovery_share_data",
    "other_fields": [
        {
            // Share to compute `reveal_token`, so claimer can ask server
            // for `ciphered_data`
            "name": "reveal_token_share",
            "type": "Bytes"
        },
        {
            // Share to compute `data_key`, so claimer can decrypt `ciphered_data`
            "name": "data_key_share",
            "type": "Bytes"
        }
    ]
}
```

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
            // Minimal number of shares to retrieve to reach the quorum and compute the secret
            "name": "threshold",
            "type": "NonZeroU32"
        },
        {
            // A recipient can have multiple shares (to have a bigger weight than others)
            "name": "per_recipient_shares",
            "type": "Map<UserID, NonZeroU32>"
        }
    ]
}
```

## 2 - Get the Shamir recovery setup

This enables two use cases:

1) A User retrieving its own Shamir recovery setup. Useful for displaying it to
   the user and to ensure the setup is still valid (i.e. no recipient has been
   revoked)
2) An Admin retrieving all the Shamir recovery setups.

Authenticated API:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "shamir_recovery_self_info",
        },
        "reps": [
            {
                "status": "ok",
                "other_fields": [
                    {
                        // `None` means no configuration
                        // Otherwise, contains a `ShamirRecoveryBriefCertificate`
                        "name": "self",
                        "type": "RequiredOption<Bytes>"
                    }
                ]
            }
        ],
    }
]
```

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "shamir_recovery_others_list",
        },
        "reps": [
            {
                "status": "ok",
                "other_fields": [
                    {
                        "name": "others",
                        // Contains a list of `ShamirRecoveryBriefCertificate`
                        "type": "Vec<Bytes>"
                    }
                ]
            },
            {
                // Current user is not ADMIN
                "status": "not_allowed"
            }
        ],
    }
]
```

> **_Future evolution 3:_** Allow a User (other than Admins) to be able to see
> the Shamir recovery it is recipient of. This can be achieved by providing
> `shamir_recovery_others_list` for non-admins. If so, this should be enabled in
> the organization configuration (similarly to what is done for the Shamir
> recovery setup config template). For the moment going KISS: only providing it
> to admins seems a reasonable choice to start with.

## 3 - Use the Shamir recovery

### 3.1 - Admin creates an invitation

Authentication API:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_new",
            "unit": "UserOrDeviceOrShamirRecovery",
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
            // situation, so not sure if we should re-use (and rename the status for the unrelated
            // situation ? this is the right time do to if we do a major version bump !) or create
            // another one...
            {
                "status": "not_available"
            }
        ],
        "nested_types": [
            {
                "name": "UserOrDeviceOrShamirRecovery",
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

### 3.2 - Claimer connect to the server

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
                "unit": "UserOrDeviceOrShamirRecovery"
            }
        ],
        "nested_types": [
            {
                "name": "UserOrDeviceOrShamirRecovery",
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
                                "type": "NonZeroU32"
                            },
                            {
                                "name": "recipients",
                                "type": "Vec<ShamirRecoveryRecipient>",
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
                        "type": "UserID",
                    },
                    {
                        "name": "human_handle",
                        "type": "Option<HumanHandle>",
                    },
                    {
                        "name": "shares",
                        "type": "NonZeroU32"
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

### 3.3 - Claimer dance for SAS code exchange

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
            // Same for the others cmds
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
                "status": "not_found",
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

### 3.4 - Greeter dance for SAS code exchange

On greeter side, the `invite_x_claimer_*` API is reused as-is.

The `invite_list` and `invite_delete` commands can also be used as-is to manage the Shamir recovery invitations.

### 3.5 - Greeter & claimer actual secret share exchange

This is done with the `invite_4_claimer/greeter_communicate` command.
This command requires that both claimer and greeter provide a binary payload
that is then passed to the peer.

Claimer payload is empty.

Greeter payload:

```json5
{
    "label": "InviteShamirRecoveryShare",
    "type": "invite_shamir_recovery_share",
    "other_fields": [
        {
            // Decrypted share
            "name": "share",
            "type": "Bytes"
        },
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
                    "type": "Bytes"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "other_fields": [
                    {
                        "name": "ciphered_data",
                        "type": "Bytes"
                    }
                ]
            }
            // <-------------- Other reps omitted --------->
        ]
    }
]
```

Then `ciphered_data` can be decrypted with `data_key`. From then on, the recovery works
just like the recovery device system (see `parsec core import_recovery_device` CLI).

## Bonus

### **Future evolution 1**: Only Admin can clear Shamir recovery

Authenticated API, `organization_config`:

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
                        "name": "shamir_recovery_clear_strategy",
                        // Allowed values:
                        // - `ADMINS_ONLY`
                        // - `ADMINS_AND_SELF`
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

### **Future evolution 2**: Specify a Shamir recovery setup template

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
                        "type": "NonZeroU32"
                    },
                    {
                        "name": "shamir_recovery_max_shares",
                        // `None` for no limit (the default)
                        "type": "RequiredOption<NonZeroU32>"
                    },
                    {
                        "name": "shamir_recovery_max_shares_per_recipient",
                        // `None` for no limit (the default)
                        "type": "RequiredOption<NonZeroU32>"
                    },
                    {
                        "name": "shamir_recovery_recipient_allowed_profiles",
                        // Default would be all profiles
                        "type": "Vec<UserProfile>"
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

### **Future evolution 3**: Non-admin can perform Shamir recovery invite

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
                        "name": "shamir_recovery_invite_strategy",
                        // Allowed values:
                        // - `ADMINS_ONLY`
                        // - `ADMINS_AND_RECIPIENTS`

                        // "recipients only" doesn't seem useful: the recipients will
                        // have to do the SAS code exchange so they already have to give
                        // there concent at this point.
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
