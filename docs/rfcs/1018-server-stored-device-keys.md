<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Server-stored device keys

## 1 - Goals

1. Prevent device & user keys form being stored on the user machine disk.
2. Still allow offline use of Parsec client.
3. Open the door to further protection of the device & user keys based on MFA
   during server authentication.

Key points:

- `LocalDevice`'s device & user keys fields are now optional
- Legacy device keys file are still supported
- Device stored on Parsec account vault contains the keys.
- New device keys file don't contains the keys. Instead the keys is stored
  on the server, encrypted by the device local key (high entropy secret).
- `CertificateOps` lazily fetches the device & user keys from the server whenever
  it needs them (similar to what is currently done for the workspace keys).

## 2 - Overview

### 2.1 - Device&User keys protection

When designing Parsec account, there was a concern about storing server-side the device keys file, since it means:

- Highly sensitive data (as it allows to impersonate a device)
- Not guaranteed to be protected by a high entropy secret (e.g. password, see
  [RFC 1014](1014-account-vault-for-device-stored-on-server.md) on high vs low entropy secrets)

However those issues may also apply to the current implementation where the device keys file
only lives on the user machine disk:

- The user machine can get stolen or compromised (e.g. malware, etc.), putting the device keys file
  at similar risks it would be if it was stored on the server with a honest-but-curious administrator.
- The level of protection cannot be guaranteed (e.g. not TPM-based OS keyring, weak password)

The fact the user&device keys are currently stored locally on the user machine disk means they
are always available when the device is under use. This is not strictly needed though since
the device keys are only used in conjunction with the server (to authenticate to the server,
sign new data, and encrypt/decrypt data to/from the server (local data are still encrypted at rest)).

This is the basis of the design changes discussed here:

- The user&device keys are stored server-side...
- ...encrypted by the device's local key.
- They are lazily fetched from the server when needed and only kept in memory.

The choice of local key to protect the device keys bundle may seem odd since the encrypted
data are indeed not stored locally. However it makes perfect sense since by doing this we
ensure the device still corresponds to a single physical machine (the one in possession of
the local key) that is merely relying on the server as a storage facility.

This is a similar approach to how are currently handled the workspace keys.

While making acceptable the storage of the user&device keys on the server, this
approach alone doesn't protect much against the risk of a stolen device: once a
successful brute-force attack on a weak password has been achieved, the attacker
can simply query the server for the user&device keys.

However this pave the way for introducing MFA (multi-factor authentication) to reject
suspicious access (e.g. from a different IP address).
A typical example would be to systematically require TOTP (time-based one-time password)
when login to Parsec.

### 2.3 - Special case for vault-stored devices

Unlike other devices, registration devices (see [RFC 1015](1015-registration-device.md)) are not locally stored.

Those devices are stored server-side in the Parsec account vault. Hence in this
case the device keys are still stored in the local device since:

- It avoid a round trip to the server to fetch the device keys bundle.
- We have to support the case of keys in the local device for legacy device keys files anyway.

> [!NOTE]
> In case of web, while still stored locally, the local device is encrypted
> with a high entropy key protected by the vault key and stored server-side (see [RFC 1016](1016-local-device-storage-in-web-client.md)).
> So in this case we might also leave the device keys in the local device.

## 4 - Datamodel

### 4.1 - Option A: Update `LocalDevice`

Changes:

- Turn `signing_key` fields optional
- Turn `private_key` fields optional
- Add optional `keys_bundle_token` field
- Add validation rule to ensure `signing_key`&`private_key` are both set or
  `keys_bundle_token` is set.

### 4.2 - (not chosen) Option B: New `DeviceLocalBundle`

Option A has the advantage of being backward compatible with the current, however it
doesn't offer a clean separation of `signing_key`&`private_key` on one side and
`keys_bundle_token` on the other side.

Hence option B that instead considers `LocalDevice` as a legacy type (only used for
device keys files).

```json5
{
    "label": "DeviceLocalBundle",
    "type": "device_local_bundle",
    "other_fields": [
        {
            // Url to the server in the format `https://parsec.example.com:443`.
            // Note we don't use the `parsec3://` scheme here to avoid compatibility
            // issue if we later decide to change the scheme.
            "name": "server_url",
            "type": "Url"
        },
        {
            "name": "organization_id",
            "type": "OrganizationID"
        },
        {
            "name": "root_verify_key",
            "type": "VerifyKey"
        },
        {
            "name": "user_id",
            "type": "UserID"
        },
        {
            "name": "device_id",
            "type": "DeviceID"
        },
        {
            "name": "device_label",
            "type": "DeviceLabel"
        },
        {
            "name": "human_handle",
            "type": "HumanHandle"
        },
        {
            // Note user profile can change by uploading additional `UserUpdateCertificate`.
            // Hence this field only contains the initial profile the user had when it
            // was enrolled.
            "name": "initial_profile",
            "type": "UserProfile"
        },
        {
            "name": "local_symkey",
            "type": "SecretKey"
        },
        {
            "name": "keys",
            "type": "DeviceLocalBundleKeys"
        }
    ],
    "nested_types": [
        {
            "name": "DeviceLocalBundleKeys",
            "variants": [
                {
                    "name": "Embedded",
                    "discriminant_value": "EMBEDDED",
                    "fields": [
                        {
                            "name": "signing_key",
                            "type": "SigningKey"
                        },
                        {
                            "name": "private_key",
                            "type": "PrivateKey"
                        },
                    ]
                },
                {
                    "name": "Remote",
                    "discriminant_value": "REMOTE",
                    "fields": [
                        {
                            "name": "token",
                            "type": "DeviceKeysBundleToken"
                        },
                    ]
                }
            ]
        }
    ]
}
```

> [!NOTE]
> Option A is probably the best option anyway, since it requires less
> changes and we can limit the ugly part dealing with legacy to the deserialization
> code (i.e. once deserialized, the in-memory `LocalDevice` would be similar to
> `DeviceLocalBundle`).

### 4.3 - New `DeviceKeysBundle`

Device keys bundle:

```json5
{
    "label": "DeviceKeysBundle",
    "type": "device_keys_bundle",
    "other_fields": [
        {
            "name": "signing_key",
            "type": "SigningKey"
        },
        {
            "name": "private_key",
            "type": "PrivateKey"
        }
    ]
}
```

> [!NOTE]
>
> - Since the device keys bundle is encrypted with the device's local key,
>   the server is not able to swap the device keys bundle with another one.
>   Put it another way, decrypting the device keys bundle with the device local key
>   is a strong proof that the device keys bundle is the one belonging to the device.
> - If we consider in the future that further proof are required, the returned keys
>   can be checked against their corresponding public keys present in the device
>   and user certificates.

## 5 - API

### 5.1 - New `device_get_keys_bundle`

Anonymous API:

```json5
{
    "cmd": "device_get_keys_bundle",
    "req": {
        "fields": [
            {
                "name": "device_token",
                // Same as `BootstrapToken` & `InvitationToken`, we should probably
                // merge those three types.
                "type": "DeviceKeysBundleToken"
            }
        ]
    },
    "reps": [
        {
            "status": "ok",
            "fields": [
                {
                    // `DeviceKeysBundle` serialized and encrypted with the device's `local_symkey`
                    "name": "device_keys_bundle",
                    "type": "Bytes"
                }
            ]
        },
        {
            "status": "device_not_found"
        }
    ]
}
```

> [!NOTE]
> A more robust approach would be to rely on HMAC authentication instead
> of token-based authentication (as this avoid transmitting the token in clear).
>
> However this is considered overkill considering its added complexity (it requires
> introducing a new command family) and the fact returned data are encrypted anyway.

### 5.2 - New `device_store_keys_bundle`

Authenticated API:

```json5
{
    "cmd": "device_store_keys_bundle",
    "req": {
        "fields": [
            // Device ID is not specified since it is determined from the authentication
            {
                "name": "device_token",
                "type": "DeviceKeysBundleToken"
            },
            {
                // `DeviceKeysBundle` serialized and encrypted with the device's `local_symkey`
                "name": "device_keys_bundle",
                "type": "Bytes"
            }
        ]
    },
    "reps": [
        {
            "status": "ok"
        },
        {
            // If the device keys bundle already exists it is not overwritten
            "status": "already_exists"
        }
    ]
}
```

> [!NOTES]
>
> - Device token is chosen by the client (instead of being returned by the server)
>   as it allows to go idempotent when `already_exists` is returned, which in
>   turn simplifies future evolutions to have the enrollment process surviving
>   a client crash.
>
> - Ideally, storing the device certificates and its keys bundle would be
>   an atomic operation.
>   However this is not worth the added complexity:
>   - The special case this prevent already exist in practice with the legacy devices.
>   - This special case is also convenient when storing the device server-side in the vault.
>   - It would require modifying the enrollment process to have the claimer pass
>     the device keys bundle to the greeter along with the device public key.
>   - The enrollment process is already not atomic anyway: a claimer may crash
>     after the greeter has stored its new device but before its secret keys has
>     been stored ¯\\_(ツ)_/¯
