# Shamir API & types

## 0 - Basic considerations

Each user only has at most one Shamir recovery:

- a new user starts with no Shamir recovery
- each Shamir recovery setup overwrite the previous one
- it is possible to clear the Shamir recovery

Instead of storing all the Shamir recovery setups ever created, only the last one is available. This is because:

- Shamir recovery can achieve weight-based strategy (e.g. recovery can be achieve by 3 normal managers or by a single
  top ranked one that got assigned 3 shares instead of 1). So a single configuration is enough even for advanced usecases.
- Shamir recovery contains sensible -while encrypted- data, so it's better to make it possible to clear it as soon at it is no longer needed.
- An attacker is unlikely to take advantage of this: to modify the Shamir recovery it must have got access to the user account. So
  the attack is about denying somebody access of his account, provided that this user has forgotten his main password and must use
  the Shamir recovery !

**Future evolution 1**: we could only allow organization admin to be able to clear other user's Shamir recovery. This would defeat the
potentiel attack and also prevent regular user from changing their Shamir recovery without notifying the admin.
This should be set by the organization config system, considering small organization most likely don't want or cannot (i.e.
too few users in the organization) enforce this rule.

The basic mechanism is:

To create the Shamir recovery:

0) Alice wants to create a Shamir recovery with Bob and Adam
1) Alice creates a new device `alice@shamir1`, and encrypt it related keys using a symmetric key `SK`: `SK(alice@shamir1)`.
   `SK` is the secret in the Shamir algorithm.
2) Alice create a `ShamirRecoveryShareCertificate` certificate for Bob and Adam: `SRSCBob` & `SRSCAdam`. Those certificates
   each contain a part of the `SK` secret.
3) Alice send a `shamir_recovery_setup` command to the Parsec server containing the `SK(alice@shamir1)`, `SRSCBob`, `SRSCAdam` and the `threshold` of the Shamir.

To use the Shamir recovery:

0) Alice has lost its account access.
     1) She goes to an organization admin.
     2) The admin uses the authenticated `invite_new` command to create a Shamir recovery invitation link (i.e. a special Parsec URL).
1) Alice uses the URL to connect as invited to the Parsec server
     1) She uses the `invite_info` command to retrieve informations
        about the current Shamir recovery: `SK(alice@shamir1)`, `threshold`, UserID & human handle of recipients.
     2) Each recipient can use the `invite_list` to see the current Shamir recovery he can take part in.
2) Alice use the `invite_x_claimer_*` commands to create a secure conduit with a recipient. The recipient on it side use
   the authenticated `invite_x_greeter_*` commands.
3) The recipient use `invite_4_greeter_communicate` to send to Alice his secret share(s). Alice uses the
   `invite_4_claimer_communicate` command (with an empty payload) to receive the secret share (i.e. `SRSCBob` and `SRSCAdam`).
4) If Alice hasn't reach the quorum (i.e. the secret shares she has are < `threshold`) she go back to 2). Otherwise she
   can compute the secret `SK` and decrypt `SK(alice@shamir1)`.
5) Alice uses `alice@shamir1` to re-create a new device that will be her recovered device.

## 1 - Create&update the Shamir recovery

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
                // Certificate is not signed by the authenticated user, or timestamp it invalid
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
                        // Token the claimer should provide to get access to `ciphered_data`.
                        // This token is split in shares, hence it acts as a proof the claimer
                        // asking for the `ciphered_data` had it identity confirmed by the recipients.
                        "name": "reveal_token",
                        "type": "Bytes"
                    },
                    {
                        // Configuration is provide as a `ShamirRecoveryBriefCertificate`, it
                        // contains the threshold for the quorum and for who each share is for
                        // This field has a certain level of duplication with the shares one,
                        // this is because they are used for different thing (we provide the
                        // encrypted share data only when needed)
                        "name": "brief",
                        "type": "Bytes"
                    },
                    {
                        // Each share is aimed at a given recipient, hence the shares
                        // are provided as a `ShamirRecoveryShareCertificate`
                        "name": "shares",
                        "type": "Vec<Bytes>",
                    }
                ]
            }
        ]
    }
]
```

Consistency between `brief` and `shares` must be checked by the parsec server:

- the number of shares must be greater or equal to the threshold (and threshold must be >= 1)
- the recipients and their shares must be the same
- the certificates datetime & author must be the same

**Future evolution 2**: On top of that, we may want to check the Shamir configuration against some rules specific
to the organization (e.g. number of shares, profile of recipients, max number of share per
recipient), see bonus part.

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

## 2 - Get the Shamir recovery configuration

This has two purposes:

- Retrieving its own Shamir recovery configuration.
  This is useful to display it to the user and
  ensure this configuration is still usable (i.e. no recipient has been revoked)
- Retrieve all the Shamir recovery for admins.

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

**Future evolution 3**: We might also want to provide `shamir_recovery_others_list`
for non-admin so that one could see the Shamir recovery he is recipient of.
If so, this should be enabled in the organization configuration (similarly to what is
done for the Shamir recovery setup config template).
For the moment going KISS and only providing it to admins seems like the reasonable
thing to do.

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

We don't provide signed data here (such as `ShamirRecoveryBriefCertificate`) given the claimer
has not way to verify the certificate (i.e. he doesn't know the root verify key yet).

We also don't provide `ciphered_data` right now, but wait for the claimer
to have concluded it SAS code exchange with all the greeters (this would guarantee we
only provide `ciphered_data` to the actual user, and not to an attacker that have
eavesdropped the invitation link).

### 3.3 - Claimer dance for SAS code exchange

On claimer side, the `invite_x_claimer_*` API is reused.

This required a single change: passing the `user_id` of the recipient as parameter (
this is because the `invite_x_claimer_*` used to be used in one-to-one, so no parameter
was requested)

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

- in the user/device invitation it only valid value is the `greeter_user_id` provided in `invite_info`.
- in the Shamir recovery invitation, it is one of the `recipients` provided in `invite_info`

From an internal point of view, the invite API is implemented (both on claimer and greeter side)
in the Parsec server with a generic `conduit_exchange` command.
Currently `conduit_exchange` use the invitation token as identifier for communication: a greeter and an claimer
communicate by doing `conduit_exchange` with the same invitation token.
However this is no longer possible given in Shamir recovery the claimer will talk to multiple different greeters in parallel.
The solution is hence to use the couple invitation token + greeter UserID as identifier.

### 3.4 - Greeter dance for SAS code exchange

On greeter side, the `invite_x_claimer_*` API is reused as-is.

`invite_list` & `invite_delete` can also be used as-is to manage the Shamir recovery invitations.

### 3.6 - Greeter & claimer actual secret share exchange

This is done with the `invite_4_claimer/greeter_communicate` command.
This command requires that both claimer and greeter provide a binary payload this is then
passed to the peer.

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

Note that the greeter doesn't send a way for the claimer to identify from which setup the share is from.
In theory this means we could end up with recipients sending incompatible share to the claimer:

- In case a new setup is done in the middle of the invitation recovery
- In case a malicious Parsec server provide a previous setup to one (or multiple) recipient(s)

This is not a big concern given in all those case the claimer will end up with an invalid
secret and won't be able to decrypt the recovery data.

Once enough shares are collected, the secret can be computed.
The claimer hence get access to `reveal_token` and `data_key`, it can then retrieve `ciphered_data`:

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

### **Future evolution 1**: Only admin can clear Shamir recovery

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

### **Future evolution 2**: Specify the Shamir recovery configuration template

Shamir recovery allows plenty of different configuration (single recipient, different
weight per recipient etc.), but we want to be able to set some limits here using the
organization config system.

There is two approach for this:

1) User very specific configuration template
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

### **Future evolution 3**: Non-admin can do Shamir recovery invite

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
