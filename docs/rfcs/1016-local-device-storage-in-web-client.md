<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# `LocalDevice` storage in web client

## 1 - Goals

1. Define a way to securely store the `LocalDevice` on a web client.
2. The `LocalDevice` should be only accessible from the web client that created it.
3. Account Vault key rotation should not affect the decryption of the `LocalDevice`.

## 2 - Overview

### 2.1 - `LocalDevice` stored on native client

The `LocalDevice` is a structure contains the secret data relative to a device
(mainly the local key and the private part of user & device keys).

On a Parsec client running natively, those data are typically stored as a device
keys file encrypted by a secret key stored in the OS keyring.

A password-based protection is also available, but it is being phased out (see RFC 1014),
as pushing yet-another password to the user is bad ergonomics.

The reader may catch a sens of irony here, as the new mandatory Parsec Account (see RFC 1013)
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

An obvious alternative for this would be the [Credential Management API](https://developer.mozilla.org/en-US/docs/Web/API/Credential_Management_API).
However, among its authentication methods:

- `FederatedCredential` is deprecated.
- `IdentityCredential`/`PasswordCredential`/`OTPCredential` are of limited availability (notably not
  available on Firefox and Safari).
- Only `PublicKeyCredential` is widely available, but it designed around signing operations
  (e.g. FIDO2) and not encryption like we need.

So instead we roll our own solution:

- The `LocalDevice` is stored (encrypted) in the web browser's
  [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage).
- The `LocalDevice` is encrypted by a secret key (the "web local device key")
  that is itself encrypted by the Account Vault key.

This way, the web local device key can be re-encrypted from anywhere whenever
the Account Vault key changes, but only the web client that created the
`LocalDevice` can use it.

> Note: The drawback to this solution is that the local device cannot be loaded
> if the server is not reachable (e.g. offline mode).
> This is acceptable for the web client however (especially since the web page
> itself is hosted by the server !).

## 3 - Changes

### 3.1 - Add `WebLocalDeviceKey` to `AccountVaultItem`

```json5
{
    "label": "AccountVaultItem",
    "type": "account_vault_item",
    "other_fields": [
        {
          "name": "organization_id",
          "type": "OrganizationID"
        },
        {
          // Data encrypted by the vault key
          "name": "encrypted_data",
          "type": "Bytes"
        },
        {
          "name": "data_type",
          "type": "VaultItemDataType",
        }
    ],
    "nested_types": [
      {
        "name": "VaultItemDataType",
        "discriminant_field": "type",
        "variants": [

          // Omitted items
          // […]

          {
            "name": "WebLocalDeviceKey",
            "discriminant_value": "WEB_LOCAL_DEVICE_KEY",
            "fields": [
              // User ID is not provided here since it is not relevant:
              // this item is only used by clients looking to decrypt a given device.
              {
                "name": "device_id",
                "type": "DeviceID"
              }
            ]
          }
        ]
      }
    ]
}
```

> Note: `WebLocalDeviceKey` is only allowed for organization having their `ClientSourceStrategy`
> configured with `NativeOrWeb` (see RFC 1017).
