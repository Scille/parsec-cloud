<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# PKI Device Storage

## 1 - Goals

This RFC introduces PKI-based authentication. The main objectives are:

1. Enable PKI-based authentication for Parsec devices (i.e. device keys file) using external identity providers.
2. Provide a seamless user experience where device authentication is tied to PKI credentials (e.g. smartcard).

## 2 - Overview

PKI-based device creation is rather straightforward:

1. User selects a PKI credential to use (typically selecting smartcard containing a secret key)
2. Parsec client generates a new opaque key
3. Parsec client encrypts the opaque key using the secret key from the smartcard
4. Parsec client encrypts `LocalDevice` with the opaque key
5. Parsec client saves `DeviceFileSmartcard` to local storage

PKI-based device access works in a similar fashion

1. Client reads `DeviceFileSmartcard` from local storage
2. User is prompted to unlock the PKI credential he used to secure the given `DeviceFileSmartcard`
3. Parsec client decrypts the opaque key using the PKI credential's private key
4. Parsec client decrypts the `LocalDevice` using the opaque key

## 3 - Data model

### 3.1 - New `DeviceFileSmartcard` schema

```json5
{
    "label": "DeviceFileSmartcard",
    "type": "smartcard",
    "other_fields": [
        ... // Common device file fields: created_on/organization_id/human_handle/etc.
        {
            // Certificate here refers to the X509 certificate that describes what is
            // in the smartcard.
            "name": "certificate_ref",
            "type": "X509CertificateReference"
        },
        {
            // Used to encrypt the secret key
            "name": "algorithm_for_encrypted_key",
            "type": "PKIEncryptionAlgorithm"
        },
        {
            // `SecretKey` encrypted by asymmetric key from the smartcard.
            "name": "encrypted_key",
            "type": "Bytes"
        },
        {
            // `LocalDevice` encrypted with the secret key from the `encrypted_key` field.
            "name": "ciphertext",
            "type": "Bytes"
        }
    ]
}
```
