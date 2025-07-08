<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# `LocalDevice` storage for the web client

## 1 - Goals

1. Define a way to securely store the `LocalDevice` on a web client.
2. The `LocalDevice` should be only accessible from the web client that created it.
3. Account Vault key rotation should not affect the decryption of the `LocalDevice`.

## 2 - Overview

### 2.1 - `LocalDevice` stored on native client

The `LocalDevice` contains the secret data related to a device
(mainly the local key and the private part of user & device keys).

On a Parsec client running natively, those data are typically stored as a device
keys file encrypted by a secret key stored in the OS keyring.

A password-based protection is also available, but it is being phased out (see [RFC 1014](1014-account-vault-for-device-stored-on-server.md)),
as pushing yet-another password to the user is bad ergonomics.

The reader may catch a sens of irony here, as the new mandatory Parsec Account (see [RFC 1013](1013-parsec-account.md))
comes with a password-based authentication by default. However:

- A password for a web account such a Parsec Account is a well understood concept...
- ...unlike a password that is specific to a single machine (the user may expect that
  changing his password on a machine will change the password on all his machines)
- In the future, password-less authentication methods (e.g. smartcard-based, FIDO2) will
  be also available.
- Losing the password of a web account is no big deal (as it can be reset), this is not
  the case for a password used as a secret for end-to-end encryption.

### 2.1 - `LocalDevice` stored on web client

However on web the OS keyring is not available, hence this RFC to define what to do instead.

An option for this would be to use the [Credential Management API](https://developer.mozilla.org/en-US/docs/Web/API/Credential_Management_API).
However, among its authentication methods:

- `FederatedCredential` is deprecated.
- `IdentityCredential`/`PasswordCredential`/`OTPCredential` are of limited availability (notably not
  available on Firefox and Safari).
- Only `PublicKeyCredential` is widely available, but it designed around signing operations
  (e.g. FIDO2) and not encryption like we need.

So instead we roll out our own approach:

- The `LocalDevice` is encrypted by a secret key (the ciphertext key).
- The ciphertext key is serialized as a `AccountVaultItemOpaqueKey`, which is itself
  encrypted by the vault key and stored in the vault as a `AccountVaultItemOpaqueKeyEncryptedData`.
- The `LocalDevice` is stored (encrypted) in the web browser's data storage as
  a `DeviceFileAccountVault`.

This way, the web local device key can be re-encrypted from anywhere whenever
the Account Vault key changes, but only the web client that created the
`LocalDevice` can use it.

> [!Note]
> The drawback to this solution is that the local device cannot be loaded
> if the server is not reachable (e.g. offline mode).
> This is acceptable for the web client however (especially since the web page
> itself is hosted by the server !).

## 3 - Data model

### 3.1 New `DeviceFileAccountVault` schema

```json5
{
    "label": "DeviceFileAccountVault",
    "type": "account_vault",
    "other_fields": [
        {
            // This refers to when the device file has been originally created.
            "name": "created_on",
            "type": "DateTime"
        },
        {
            // This field gets updated every time the device file changes its protection.
            "name": "protected_on",
            "type": "DateTime"
        },
        {
            // Url to the server in the format `https://parsec.example.com:443`.
            // Note we don't use the `parsec3://` scheme here to avoid compatibility
            // issue if we later decide to change the scheme.
            "name": "server_url",
            "type": "String"
        },
        {
            "name": "organization_id",
            "type": "OrganizationID"
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
            "name": "human_handle",
            "type": "HumanHandle"
        },
        {
            "name": "device_label",
            "type": "DeviceLabel"
        },
        {
            // ID of the opaque key stored on the account vault that should be used
            // to decrypt the `ciphertext` field.
            "name": "ciphertext_key_id",
            "type": "AccountVaultItemOpaqueKeyID"
        },
        {
            // `LocalDevice` encrypted with the ciphertext key
            "name": "ciphertext",
            "type": "Bytes"
        }
    ]
}
```

### 3.2 - New `AccountVaultItemOpaqueKeyEncryptedData` schema

```json5
{
    "label": "AccountVaultItemOpaqueKeyEncryptedData",
    "type": "account_vault_item_opaque_key_encrypted_data",
    "other_fields": [
        {
            "name": "key_id",
            "type": "AccountVaultItemOpaqueKeyID"
        },
        {
            "name": "key",
            "type": "SecretKey"
        }
    ]
}
```

### 3.3 New `AccountVaultItemOpaqueKey` schema

```json5
{
    "label": "AccountVaultItemOpaqueKey",
    "type": "account_vault_item_opaque_key",
    "other_fields": [
        {
            "name": "key_id",
            "type": "AccountVaultItemOpaqueKeyID"
        },
        {
            // `AccountVaultItemOpaqueKeyEncryptedData` encrypted by the vault key
            "name": "encrypted_data",
            "type": "Bytes"
        }
    ]
}
```

### 3.4 - Add `OpaqueKey` to `AccountVaultItem`

See `AccountVaultItem` format defined in [RFC 1014](1014-account-vault-for-device-stored-on-server.md).
