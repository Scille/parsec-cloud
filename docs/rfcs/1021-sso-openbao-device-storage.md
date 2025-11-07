<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# SSO OpenBao Device Storage

## 1 - Goals

This RFC introduces SSO-based authentication with OpenBao for secure device keys storage. The main objectives are:

1. Enable SSO-based authentication for Parsec devices (i.e. device keys file) using external identity providers.
2. Leverage OpenBao as a secure storage backend for device encryption keys (opaque keys).
3. Provide a seamless user experience where device authentication is tied to organizational SSO credentials.
4. Maintain end-to-end encryption security by separating device files from their encryption keys.

## 2 - Overview

The SSO OpenBao approach implements a two-layer security model:

1. **Device Keys File**: Stored locally on the user's machine, encrypted with an opaque key
2. **Opaque Key**: Stored in OpenBao, accessible only after SSO authentication

This separation ensures that compromise of either component alone doesn't expose device private keys. An attacker would need both:

- Access to the encrypted device file on the user's machine
- Valid SSO credentials to retrieve the opaque key from OpenBao

> **Note:**
> Obviously the level of security is lower than traditional approaches (e.g.
> storing the opaque key on the OS Keyring or derive it from a strong password).
>
> However this is a trade-of to increase user-friendliness, the decision whether
> or not to use this is to be made by the server administrator according to its
> own threat model.

### 2.1 - Device File Workflow

#### 2.1.1 - Device Creation

1. User authenticates via SSO to OpenBao through his Parsec client
2. Parsec client generates a new opaque key
3. Parsec client uploads opaque key to OpenBao
4. Parsec client encrypts `LocalDevice` with the opaque key
5. Parsec client saves `DeviceFileOpenBao` to local storage

#### 2.1.2 - Device Access

1. Client reads `DeviceFileOpenBao` from local storage
2. User authenticates via SSO to OpenBao (if not already authenticated)
3. Client fetches the opaque key from OpenBao
4. Client decrypts `LocalDevice` using the retrieved opaque key

### 2.2 - OpenBao info storage strategy

To fetch/upload an OpenBao secret we need multiple things:

1. The address of the OpenBao server.
2. The authentication to use (e.g. "OIDC SSO with Github as identity provider").
3. The mount path of the authentication method (e.g. `auth/oidc/github`).
4. The mount path of the secret store (e.g. `secrets/parsec-keys`).
5. The path of the secret within the secret store.

However all those info are not stored in (and hence obtained from) the device
keys file since it would make them complex to change in the future.

So instead the OpenBao server configuration (i.e. server address, list of
supported authentication methods, mount paths) is to be obtained from the
Parsec server each time we need to load this device keys file.

## 3 - Data model

### 3.1 - New `DeviceFileOpenBao` schema

```json5
{
    // ⚠️ Note the device file (i.e. the stuff defined by this schema!) is not
    // stored on OpenBao.
    // Instead it is encrypted by the ciphertext key that is itself stored on
    // OpenBao.
    // This way getting access to the device private keys require both exfiltrating
    // the secret stored on OpenBao AND the device file stored on the end user's machine.
    "label": "DeviceFileOpenBao",
    "type": "openbao",
    "other_fields": [
        ... // Common device file fields: created_on/organization_id/human_handle/etc.
        {
            // `LocalDevice` encrypted with a secret key which is itself stored
            // on the OpenBao server.
            "name": "ciphertext",
            "type": "Bytes"
        },
        {
            // Arbitrary field only used by the GUI.
            //
            // In practice, it is expected to contain the plugin method mount
            // (e.g. `auth/github`, see https://openbao.org/api-docs/auth/jwt/)
            // corresponding to the authentication method that have been used
            // to authenticate to OpenBao during the creation of this device keys file.
            //
            // The idea here is to allow the GUI to use this again during subsequent
            // access of this device keys file in order to pre-select the
            // authentication method that should be used.
            "name": "openbao_preferred_auth",
            "type": "String"
        },
        {
            // Entity ID basically correspond to an account ID in OpenBao.
            // So the GUI should authenticate as this entity in OpenBao in order
            // to be able to fetch the secret containing the ciphertext key.
            "name": "openbao_entity_id",
            "type": "String"
        },
        {
            // The ciphertext key is stored in OpenBao as a secret that is has
            // for path `<entity_id>/<ciphertext key UUID>`.
            //
            // Note we don't just use the ciphertext key UUID as path since OpenBao
            // is expected to be configured with path-based access policy (i.e.
            // a given entity is only allowed to access the secrets starting with
            // its entity ID).
            "name": "openbao_ciphertext_key_path",
            "type": "String"
        }
    ]
}
```

### 3.2 - OpenBao Secret Format

The opaque key is stored in OpenBao as [a KV v2 secret](https://openbao.org/api-docs/secret/kv/kv-v2/)
with the following structure:

```json
{
    "data": {
        "opaque_key": "<secret key as base64>"
    }
}
```

## 4 - Protocol

### 4.1 - Server Configuration API

See [RFC 1022](1022-server-config-api.md)

## 5 - Security Considerations

### 5.1 - Threat Model

**Protected Against:**

- Compromise of user's local machine (device file alone is useless)
- Compromise of OpenBao server (opaque keys alone don't reveal device private keys)
- Loss of user credentials (device file remains on machine)

**Requires Additional Protection:**

- Simultaneous compromise of both user machine and SSO credentials
- Admin-level access to both Parsec server and OpenBao server

### 5.2 - Access Control

- OpenBao policies enforce entity-based path restrictions
- Each user can only access secrets under their entity ID
- Authentication requires valid SSO credentials
- Opaque keys are generated per-device and cannot be reused

### 5.3 - Audit & Monitoring

Organizations can leverage OpenBao's audit capabilities to:

- Monitor access to device encryption keys
- Detect unusual access patterns
- Maintain compliance with security policies
