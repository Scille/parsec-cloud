<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# TOTP for local device protection

## 1 - Goals

This RFC introduce the use of [Multi-Factor Authentication](https://en.wikipedia.org/wiki/Multi-factor_authentication)
(MFA) in the form of [Time-based One-Time Password](https://en.wikipedia.org/wiki/Time-based_one-time_password)(TOTP)
as an additional layer of protection on the local device.

Basically:

1. Alice starts her Parsec client
2. Alice selects the device she wants to use
3. The Parsec client then prompts Alice to enter a TOTP code.
   The TOTP code is then is transmitted to the server.
4. The server checks the TOTP code and returns a second decryption key.
5. The Parsec client prompts Alice to unlock the device according to its main
   protection (i.e. password). By doing this the client obtain a first decryption key.
6. The Parsec client use both first and second decryption keys to decrypt the
   device keys file.

## 2 - Overview

### 2.1 Where TOTP is to be used

Currently a local device can be protected by multiple solutions, for instance:

- Password
- OS keyring
- PKI (i.e. smartcard)
- OpenBao (i.e. SSO-based authentication)

Most of those solution are local-only, which is both a blessing and a curse:

- pro: Offline login is possible
- con: Offline attack is possible (typically when the user's machine gets stolen)

The use of a TOTP aims at involving the Parsec server in the login operation,
this way no secret can be obtained from the stolen machine (unless, of course,
the attacker also has access to the user's TOTP secret).

This is not a one-size-fits-all solution since its removes the possibility to
login while offline.

However since the TOTP server communication is only required during login (i.e. to
obtain the device keys file's decryption key), offline operation are still
possible after this step.
In a nutshell, with TOTP Parsec can still be used fine in a poor connection
configuration (e.g. traveling in a train), but cannot be used when no connection
is possible for extended periods of time (e.g. in a submarine ^^).

### 2.2 - Differences with traditional TOTP-based authentication

Typical TOTP-based authentication goes a as follow:

1. Alice provides her login and password to the server
2. The server checks the login and password
3. Alice provides her TOTP code
4. The server checks the TOTP code

The key point here is there is two separate challenge-response occurring:
first using login/password, and only then a second one involving the TOTP.

However this is not something we can do with Parsec: we want to protect the device keys
file with the TOTP, however it is the device signing key (which is part of the device
keys file) that is used to authenticate to the Parsec server.

So instead we must first do the TOTP check, and have its result used as a secret
required to decrypt the device keys file.

### 2.3 - Different TOTP usages

TOTP code can be obtained from different sources, typically:

- TOTP application (e.g. Google Authenticator)
- Email
- SMS

This RFC consider the TOTP code are only going to be obtained from a TOTP application.

However later development on the server side can be done to add additional ways.
This would typically require a new anonymous API command (`totp_info` ?) so that
the server can tell the client how the one-time-password is expected to be obtained
(form an authenticator app, or by querying an email/SMS through a
`totp_send_one_time_password`)

### 2.4 - How is used the secret obtained from the TOTP challenge ?

Once the TOTP challenge succeeded, the Parsec server send to the client a secret.
This secret has then to be used in conjuction with the secret obtained from the primary
protection (i.e. OS keyring) in order to decrypt the device keys file.

This is done by considering both secrets as separate keys and hence doing two decryption operations:

- Cleartext is first encrypted using the secret key obtained from the primary protection.
- Then it is again encrypted using the secret key obtained from the TOTP challenge, hence
  obtaining the final ciphertext.

Put it another way, the local device decryption is done this way:

```rust
let key_from_totp_challenge: SecretKey = ...
let key_from_primary_protection: SecretKey = ...
let cleartext = key_from_totp_challenge.decrypt(
    key_from_primary_protection.decrypt(ciphertext)
);
let device = LocalDevice::load(cleartext);
```

> [!NOTE]
> An alternative solution would be to derive the ciphertext decryption key from both secrets
> (i.e. `KeyDerivation::from(<key_from_primary_protection>).derive_secret_key_from_uuid(<secret_from_totp_challenge>).decrypt(<ciphertext>)`).
>
> We choose against it since having two separate encryption operations means we can know
> which operation has failed (typically useful to prompt the user his password is invalid).

### 2.5 - Per user TOTP config

Since TOTP is used here to protect a local device, the simpler approach would be to
have a different TOTP configuration for each device.
However this means the end user would have to store in his authenticator app one
TOTP secret per device, which is cumbersome and an error prone (the label for a
TOTP secret is supposed to be something short, here it would end up being something
like `Parsec - <organization_id> - <human_handle_email> - <device_label>`).

So instead the TOTP should be configured on a per-user base:

1. A `totp_setup_get_secret` command should be issued to obtain the TOTP secret.
   The secret is displayed to the end user that configure his authenticator app with it.
2. For each device to save with TOTP protection, a `totp_create_opaque_key` command
   is issued with a TOTP one-time-password as parameter.
3. To decrypt the device, a `totp_fetch_opaque_key` is issued with a TOTP
   one-time-password as parameter.

Whenever the TOTP challenge in `totp_create_opaque_key` or `totp_fetch_opaque_key` is
successful, then `totp_setup_get_secret` always returns `already_setup` (i.e. the
client can no longer obtain the TOTP secret since he is supposed to have it stored
in his authenticator app).

> [!NOTE]
> In a traditional application, TOTP can be disabled and later re-enabled. In
> which case a new TOTP secret is provided to the user.
>
> Here this is not possible: once the TOTP configured we cannot know if some
> local device is protected with it.
> So the TOTP configuration is setup once, and stays the same even if the user
> remove TOTP protection from all his device, then later on choose to re-add
> it.
> This is a bit error prone for the end user (since he may assume the secret in
> his authenticator app can be removed once he has disabled TOTP protection in
> all his devices), however this is no big deal:
>
> - The worst case is the end user can no longer protect his device with TOTP,
>   he is not locked out of it.
> - The outcome is similar to what would occur if the end user had lose his
>   authenticator app: in both case the server admin has to intervene to reset
>   the TOTP setup.

## 3 - Protocol

### 3.1 - Setup TOTP

Authenticated API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "totp_setup_get_secret",
        "req": {
            "fields": []
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        // TOTP secret to configure in the user's authenticator app.
                        "name": "totp_secret",
                        "type": "String"
                    }
                ]
            },
            {
                // The TOTP has already been setup.
                // Note if the end user no longer has access to the TOTP secret (e.g.
                // he has lost his phone containing his authenticator app), then
                // he should contact a server admin that can then do a TOTP reset so
                // that `totp_setup` can be called again.
                "status": "already_setup"
            }
        ]
    }
]
```

### 3.1 - Create a TOTP-protected opaque key to save a local device

Authenticated API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "totp_create_opaque_key",
        "req": {
            "fields": [
                {
                    // Value obtained from the authenticator app storing the TOTP secret
                    "name": "one_time_password",
                    "type": "String"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "opaque_key_id",
                        "type": "UUID"
                    },
                    {
                        // Note the server knows about this key as it is not encrypted!
                        //
                        // This is because TOPT's goal is to protect against unauthorized
                        // access *from the client*.
                        //
                        // Obviously this means this opaque key should always be used in
                        // conjunction with another key only known by the end-user.
                        //
                        // Similarly, it is no big deal to transmit the key to the client
                        // (instead of doing all the encryption/decryption operations on
                        // the server). The only risk is for a malicious client to remember
                        // the key, however the client can also just remember the decrypted
                        // data!
                        "name": "opaque_key",
                        "type": "SecretKey"
                    }
                ]
            },
            {
                "status": "invalid_one_time_password"
            }
        ]
    }
]
```

### 3.2 - Access the TOTP-protected opaque key to load a local device

Anonymous API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "totp_fetch_opaque_key",
        "req": {
            "fields": [
                {
                    "name": "opaque_key_id",
                    "type": "UUID"
                },
                {
                    // Value obtained from the authenticator app storing the TOTP secret
                    "name": "one_time_password",
                    "type": "String"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        // Note the server knows about this key as it is not encrypted!
                        //
                        // This is because TOPT's goal is to protect against unauthorized
                        // access *from the client*.
                        //
                        // Obviously this means this opaque key should always be used in
                        // conjunction with another key only known by the end-user.
                        //
                        // Similarly, it is no big deal to transmit the key to the client
                        // (instead of doing all the encryption/decryption operations on
                        // the server). The only risk is for a malicious client to remember
                        // the key, however the client can also just remember the decrypted
                        // data!
                        "name": "opaque_key",
                        "type": "SecretKey"
                    }
                ]
            },
            {
                "status": "invalid_one_time_password"
            },
            {
                // No opaque key exists with this ID.
                // This status is only returned if the one-time-password is valid,
                // and can be used as a trick to ensure the end-user has correctly
                // configured his authenticator app with the TOTP secret (in such
                // case we send a dummy `opaque_key_id`).
                "status": "opaque_key_not_found"
            },
            {
                // Too many attempts have been done for this opaque key ID
                "status": "throttled",
                "fields": [
                    {
                        "name": "wait_until",
                        "name": "DateTime"
                    }
                ]
            }
        ]
    }
]
```

## 4 - Datamodel

### 4.1 - Optional TOTP field in device files schemas

Introduce a new `totp` field in each device file schema (`DeviceFilePassword`, `DeviceFileKeyring`, etc.):

```json5
{
    "label": "DeviceFilePassword",
    "type": "password",
    "other_fields": [
        // <-------------------------- Existing fields ----------------------->
        ... // Common device file fields: created_on/organization_id/human_handle/etc.
        {
            // `LocalDevice` encrypted with a secret key obtained by deriving
            // a password according to the `algorithm` field.
            "name": "ciphertext",
            "type": "Bytes"
        },
        {
            // Algorithm configuration to use to cook the user-provided password
            // into the main secret.
            "name": "algorithm",
            "type": "TrustedPasswordAlgorithm"
        },
        // <-------------------------- New field ----------------------------->
        {
            // When MFA is not used, `ciphertext` is directly encrypted by the
            // key obtained from the primary protection (i.e. the cooking of the
            // user-provided password).
            //
            // When MFA is used, `ciphertext` is encrypted by a key obtained by
            // deriving the key obtained from the primary protection with a
            // secret UUID obtained from the server though a MFA challenge.
            //
            // So in a nutshell something like:
            // ```rust
            // let mfa_secret_uuid = mfa_challenge_with_server(mfa_secret);
            // let key_from_main_protection = password_into_key(algorithm, password);
            // let ciphertext_key = KeyDerivation::from(key_from_main_protection)
            //     .derive_secret_key_from_uuid(<secret>);
            // let cleatext = ciphertext_key.decrypt(ciphertext);
            // ```
            "name": "totp",
            "type": "NonRequiredOption<TOTPOpaqueKeyAccess>"
        }
    ],
    "nested_types": [
        {
            "name": "TOTPOpaqueKeyAccess",
            "discriminant_field": "type",
            "variants": [
                {
                    "name": "TOTP",
                    "discriminant_value": "TOTP",
                    "fields": [
                        {
                            // Label as displayed in the Authenticator app
                            // (typically the authenticator app register the TOTP
                            // secret through an URL containing this label:
                            // `otpauth://totp/<label>?secret=<secret>`)
                            "name": "label",
                            "type": "String"
                        },
                        {
                            "name": "opaque_key_id",
                            "type": "UUID"
                        }
                    ]
                }
            ]
        }
    ]
}
```

## 5 - Libparsec API

### 5.1 - Per-user TOTP setup API

```rust
enum ClientTOTPOperationError {
    Offline,
}

enum TOTPStatus {
    Stalled {
        // Secret to display to the end-user so that he can configure his authenticator app
        totp_secret: String,
    }
    AlreadySetup,
}

async fn client_totp_status() -> Result<TOTPStatus, ClientTOTPOperationError>

// Return true if the provided one-time-password has succeeded the server's TOTP challenge
async fn client_totp_try_challenge(one_time_password: String) -> Result<bool, ClientTOTPOperationError>
```

Typically usecase:

1. GUI uses `client_totp_status()` to check if the TOTP is already setup.
2. If not, the GUI displays the TOTP secret to the end-user and ask for a
   one-time-password (that should be obtained from the authenticator app
   once configured with the secret).
3. GUI uses `client_totp_try_challenge()` to validate the one-time-password was
   valid. Under the hood this function send a `totp_fetch_opaque_key` command
   to the server (with a dummy opaque key ID) which tells the server the TOTP
   secret has been correctly used and should no longer be provided to the client.

### 5.2 - Create/fetch TOTP-protected opaque key

```rust
async fn client_totp_create_opaque_key(one_time_password: String) -> Result<(UUID, SecretKey), ClientTOTPOperationError>

enum ClientTOTPFetchOpaqueKeyError {
    Offline,
    NotFound,
}
async fn client_totp_fetch_opaque_key(opaque_key_id: UUID, one_time_password: String) -> Result<SecretKey, ClientTOTPFetchOpaqueKeyError>
```

> [!NOTE]
>
> `client_totp_*` require a client handle, which is not available during organization bootstrap and claimer finalize.
>
> - Claimer finalize can be easily supported by adding a `claimer_XXX_finalize_totp_upload_opaque_key`
> before `claimer_XXX_finalize_save_local_device`.
> - However it is complex to use during bootstrap organization since it is
>   currently done in a single `bootstrap_organization()` call (hence cannot
>   pass a save strategy with a TOTP configured).
> - Another possible solution: don't bother with TOTP during bootstrap/claim. Instead pass
>   in the server configuration if TOTP protection is required, and display a popup
>   on login if the current device is not protected accordingly.

### 5.3 - Changes in `list_available_devices()`

Listing available device should provide information about the need for MFA:

```rust
pub enum AvailableDeviceType {
    Keyring,
    Password,
    // ...
}

pub struct AvailableDeviceTOTPConfig {
    label: String,
    opaque_key_id: UUID,
}

pub struct AvailableDevice {
    pub key_file_path: PathBuf,
    // ...
    pub ty: AvailableDeviceType,
    pub totp: Option<AvailableDeviceTOTPConfig>,
}
```

### 5.4 - Changes in `Device(Save|Access)Strategy`

```rust
pub enum DeviceSaveStrategy {
    Keyring {
        totp_opaque_key: Option<SecretKey>,
    },
    Password {
        totp_opaque_key: Option<SecretKey>,
        password: Password,
    },
    // ...
}

pub enum DeviceAccessStrategy {
    Keyring {
        totp_opaque_key: Option<SecretKey>,
        key_file: PathBuf,
    },
    Password {
        totp_opaque_key: Option<SecretKey>,
        key_file: PathBuf,
        password: Password,
    },
    ...
}
```

## 6 - List of changes

- New authenticated API: `totp_setup_get_secret`
- New authenticated API: `totp_create_opaque_key`
- New anonymous API:  `totp_fetch_opaque_key`
- CLI server: reset TOTP setup for a given user
- Introduce a new `totp` field in each device file schema (`DeviceFilePassword`, `DeviceFileKeyring`, etc.)
- Libparsec: new function `client_totp_status`
- Liparsec: new function `client_totp_try_challenge`
- Liparsec: new function `client_totp_create_opaque_key`
- Liparsec: new function `client_totp_fetch_opaque_key`
- GUI: check TOTP status and setup it (using `client_totp_status` and `client_totp_try_challenge`)
- GUI: add "enable TOTP" option in save device dialogue (using `client_totp_create_opaque_key`)
- GUI: add "TOTP challeng" in load device dialogue (using `client_totp_fetch_opaque_key`)
