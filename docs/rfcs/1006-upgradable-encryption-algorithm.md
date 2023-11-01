<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

!!!! TODO !!!!!

- Client legacy handling for RFC 1005 (detect the realm key when there is no key rotation certificate)
- Sequester service signature & encryption data armor format
  - add RSA key size (e.g. `4096@RSASSA-PSS-SHA256`)
  - add Chacha20 support for encryption


<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Upgradable encryption algorithm

## 0 - Abstract

This RFC proposes a way to be able to upgrade the algorithm used when encrypting
and hashing workspace data.
On top of that it proposes to upgrade the encryption algorithm currently used.

## 1 - Background

Currently the encryption algorithms used in Parsec on the workspace data are:

- XSalsa20-Poly1305 for symmetric encryption
- Sha256 for hashing

Those algorithms are fine for now (well regarded, no known exploit on them), but
we need a way to migrate out of them in the even this becomes no longer the case.

This is currently not possible as the algorithms are hardcoded in Parsec, and hence
releasing a new Parsec version with a different algorithm would means breaking
compatibility with any existing organization.

On top of that, the choice of using XSalsa20-Poly1305 was suboptimal as it is less
scrutinized than it close cousin (X)Chacha20-Poly1305,
[especially it IETF version used in TLS 1.3](https://www.rfc-editor.org/rfc/rfc8439).

Finally, note asymmetric encryption is outside of the scope of this RFC. This is because
asymmetric encryption is the corner stone Parsec trust is build on, hence in case
of exploit it is likely the organization must recreated anyway (with a patched Parsec
using another asymmetric encryption algorithm).

## 2 - General approach

- Encryption and hashing algorithms are specified in the realm key rotation certificate.
- Blocks are encrypted by the same key as the manifests (unlike now where it is encrypted
  by dedicated keys that are stored in the file manifest).
- Only legacy file manifest contains keys for block, in which case the algorithm is always
  XSalsa20-Poly1305.
- Chacha20-Poly1305 is introduced as the new symmetric encryption algorithm. Hence all new
  data are generated using it, and Salsa20 is only use to read legacy data.
- Using multiple algorithms at the same time (for anything but backward compatibility)
  is a non-goal as it only add to the complexity.

## 3 - Changes: Specifying the algorithms

### 3.1 - Realm key rotation certificate

RFC 1005 introduced the `RealmKeyRotationCertificate` certificate which contains
`encryption_algorithm` and `hash_algorithm`:

```json5
{
    "label": "RealmKeyRotationCertificate",
    "type": "realm_key_rotation_certificate",
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
            "name": "realm_id",
            "type": "VlobID"
        },
        {
            "name": "key_index",
            "type": "Index"
        },
        {
            // here !
            "name": "encryption_algorithm",
            "type": "SecretKeyAlgorithm"
        },
        {
            // here !
            "name": "hash_algorithm",
            "type": "HashAlgorithm"
        },
        {
            "name": "key_canary",
            "type": "Bytes"
        }
    ]
}
```

The only allowed value is:

- `SHA256` for `hash_algorithm`
- `XSALSA20-POLY1305` for `encryption_algorithm`

We add a new allowed value: `CHACHA20-POLY1305`.

> **Note:**
>
> As RFC 1005 is not in production, `XSALSA20-POLY1305` will never actually
> be used. So it will be an allowed value (it will still be used to decrypt
> legacy data though).

### 3.2 - Sequester authority certificate

```json5
{
    "label": "SequesterAuthorityCertificate",
    "type": "sequester_authority_certificate",
    "other_fields": [
        {
            "name": "author",
            "type": "RequiredOption<DeviceID>"
        },
        {
            "name": "timestamp",
            "type": "DateTime"
        },
        {
            "name": "verify_key_der",
            "type": "SequesterVerifyKeyDer"
        },
        {
            // here !
            "name": "signing_algorithm",
            // Non-required for backward compatibility. If missing, defaults
            // to `RSASSA-PSS-4096-SHA256`
            "type": "NonRequiredOption<SequesterSigningKeyAlgorithm>"
        }
    ]
}
```

`SequesterSigningKeyAlgorithm` has one possible values: `RSASSA-PSS-4096-SHA256`.

### 3.3 - Sequester service certificate

```json5
{
    "label": "SequesterServiceCertificate",
    "type": "sequester_service_certificate",
    "other_fields": [
        // No author field here given we are signed by the sequester authority
        {
            "name": "timestamp",
            "type": "DateTime"
        },
        {
            "name": "service_id",
            "type": "SequesterServiceID"
        },
        {
            "name": "service_label",
            "type": "String"
        },
        {
            "name": "encryption_key_der",
            "type": "SequesterPublicKeyDer"
        },
        {
            // here !
            "name": "encryption_algorithm",
            // Non-required for backward compatibility. If missing, defaults
            // to `RSAES-OAEP-4096-XSALSA20-POLY1305`
            "type": "NonRequiredOption<SequesterEncryptionKeyAlgorithm>"
        }
    ]
}
```

`SequesterEncryptionKeyAlgorithm` has one possible values: `RSAES-OAEP-4096-XSALSA20-POLY1305`.

## 4 - Changes: Modification in the file manifest

Change the `BlockAccess` type used in `blocks`:

```json5
{
    "label": "BlockAccess",
    "fields": [
        {
            "name": "id",
            "type": "VlobID",
        },
        {
            "name": "offset",
            "type": "Size"
        },
        {
            "name": "size",
            "type": "NonZeroU64"
        },
        {
            "name": "digest",
            // Used to be `HashDigest`, backward compatible change
            "type": "Bytes"
        },
        {
            "name": "key",
            // Used to be a required field, msgpack encoding makes this change backward compatible
            // Field only present for legacy, in such case the SecretKey must be XSalsa20-Poly1305
            // (i.e. 32 bytes long)
            "type": "NonRequiredOption<SecretKey>"
        }
    ]
}
```

> **Note:**
>
> `digest` field can no longer have a precise `HashDigest` type as it size depend
> on the algorithm, hence we only know it is made of bytes.
