<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Registration Device

## 1 - Goals

1. Allow simplified use of multiple devices by not requiring device-to-device enrollment.
2. Provide configuration for per-organization opt-out of this solution for security reasons.

## 2 - Overview

## 2.1 - Different types of devices

In Parsec, a device is supposed to:

- Correspond to a single physical machine (e.g. desktop, phone, etc.)
- Never run in parallel (as it messes with the sync system)

However, with Parsec account, we introduce one exception to this: the "Registration Device".
The idea is to have a special device that only lives on the server and is used to
create new devices (hence bypassing the need for device-to-device enrollment).

This device's `LocalDevice` is stored server-side in the Account Vault (see [RFC 1014](1014-account-vault-for-device-stored-on-server.md)).

Since the Account Vault can be disabled on a per-organization basis (for security
reason, see [RFC 1014](1014-account-vault-for-device-stored-on-server.md)),
this feature is work in a best effort basis (i.e. traditional
device-to-device enrollment is still used otherwise).

### 2.2 - Relationship with existing enrollment

During the enrollment process, the claimer's client should decide what to do with
the new device.

This is done solely based on the organization configuration:

- If the organization allows Account Vault, the new device is an Registration Device.
  It is uploaded to the server and then used to create a new device that will be the
  actual device used locally.
- Otherwise, the new device is the actual device used locally.

> [!NOTE]
>
> - The reason not to allow the user to bypass the organization configuration is twofold:
>   - It simplifies implementation (i.e. everything is MUST, there is no SHOULD).
>   - It simplifies user experience since there is no user interaction needed.
>
> - The Parsec client should probably still provide some explanation about where the
>   device is stored and what it means (typically to avoid confusion when a user uses
>   two organizations with different config).
>   Typically "This organization disallows storing device keys on the server, manual
>   device invitation is needed to get access from another machine.".

## 2.3 - Access recovery on account vault reset

If the user recreate a vault from scratch (e.g. he has lost his password and hence user
does an account recovery), he is able to recreate the Registration Device from any
Parsec client that still has access to corresponding identity:

1. The user connect to Parsec Account from a Parsec client
2. The Parsec client gets the list of identities (i.e. organization Id  + user ID couple)
   the account is related to.
3. The Parsec client compares it to the list of devices locally stored on the machine.
4. If the organization's vault strategy allows it, the Parsec client prompt the user to
   create a new Registration Device for any identity available on the machine but
   not in the vault.

## 3 - Changes

### 3.1 - Datamodel: update `DeviceCertificate`

- In `DeviceCertificate`, remove `DevicePurpose`'s `WebAuth` variant
- In `DeviceCertificate`, add `DevicePurpose`'s `Registration` variant

> [!NOTE]
> Removing `WebAuth` is fine regarding backward compatibility since this value
> was never used (as it was introduced in preparation to Parsec account support).

### 3.2 - API: Add `RegistrationDevice` to `AccountVaultItem`

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
            "name": "RegistrationDevice",
            "discriminant_value": "REGISTRATION_DEVICE",
            "fields": [
              // Device ID is not provided here since it is not relevant:
              // this item is only used to give access to this organization/user
              // couple.
              {
                "name": "user_id",
                "type": "UserID"
              }
            ]
          }
        ]
      }
    ]
}
```

> [!NOTE]
> `WebLocalDeviceKey` is only allowed for organization having their `ClientSourceStrategy`
> configured with `NativeOrWeb` (see [RFC 1017](1017-web-client-allowed-on-per-org-basis.md)).

### 3.3 - Client

- Use organization configuration (obtained with `invite_info`, see [RFC 1014](1014-account-vault-for-device-stored-on-server.md)).
- Implement Registration Device creation at the end of the enrollment process.
- Implement device creation from Registration Device at the end of the enrollment process.
- Implement device creation from Registration Device outside of the enrollment process
  (i.e. replacing device-to-device enrollment).
