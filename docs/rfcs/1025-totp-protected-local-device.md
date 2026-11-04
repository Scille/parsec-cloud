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
4. The server checks the TOTP code and returns the _secondary_ decryption key.
5. The Parsec client prompts Alice to unlock the device according to its main
   protection (e.g. password). By doing this the client obtain the _primary_
   decryption key.
6. The Parsec client use both _primary_ and _secondary_ decryption keys to
   decrypt the device keys file.

## 2 - Overview

### 2.1 Where TOTP is to be used

Currently a local device can be protected by one of the following multiple solutions:

- Password
- OS keyring
- PKI (i.e. smartcard)
- OpenBao (i.e. SSO-based authentication)

Most of those solutions are local-only, which is both a blessing and a curse:

- pro: Offline login is possible
- con: Offline attack is possible (typically when the user's machine gets stolen)

> [!NOTE]
> "Offline login" sounds like an oxymoron, here it refers to the fact we can
> decrypt the local device keys (then use this device to start a Parsec client)
> with only the data present on the machine (so notably without having the
> Parsec server involved in any way).

The use of a TOTP aims at involving the Parsec server in the login operation,
this way no secret can be obtained from the stolen machine (unless, of course,
the attacker also has access to the user's TOTP secret).

This is not a one-size-fits-all solution since it removes the possibility to
login while offline.

However since the TOTP server communication is only required during login (i.e. to
obtain the device keys file's decryption key), offline operation are still
possible after this step.
In a nutshell, with TOTP Parsec can still be used fine in a poor connection
configuration (e.g. traveling in a train), but cannot be used when no connection
is possible for extended periods of time (e.g. in a submarine ^^).

### 2.2 - Differences with traditional TOTP-based authentication

Typical TOTP-based authentication goes as follow:

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

This means the TOTP check is sensitive to brute force attack.

> [!NOTE]
> Considering an attacker making one TOTP attempt every 30s (so every
> time the TOTP code changes), he would have a ~65% chance of succeeding the
> challenge after one year.

To mitigate this risk:

- The TOTP challenge requires to know the opaque key ID which is not public. So
  the attacker must be in possession of the local device keys file.
- An exponential backoff should kicks in if too many attempts are done (see
  [`totp_fetch_opaque_key` API](#33---access-the-totp-protected-opaque-key-to-load-a-local-device))

### 2.3 - Different TOTP usages

TOTP code can be obtained from different sources, typically:

- TOTP application (e.g. Google Authenticator)
- Email
- SMS

This RFC consider the TOTP code is only going to be obtained from a TOTP application.

However later development on the server side can be done to add additional ways.
This would typically require a new anonymous API command (`totp_info` ?) so that
the server can tell the client how the one-time-password is expected to be obtained
(form an authenticator app, or by querying an email/SMS through a
`totp_send_one_time_password`)

### 2.4 - How is used the secret obtained from the TOTP challenge ?

Once the TOTP challenge succeeds, the Parsec server sends a secret to the client.
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

So instead the TOTP should be configured on a per-user base (i.e. each human
has a single TOTP secret for each organization he is part of):

1. A `totp_setup_get_secret` command should be issued to obtain the TOTP secret.
   The secret is displayed to the end user that configures his authenticator app with it.
2. A `totp_setup_confirm` command is then issued with a TOTP one-time-password as
   parameter to inform the server the user's authenticator app is correctly configured.
   From this point on (and until the TOTP setup is reset) `totp_setup_get_secret`
   and `totp_setup_confirm` always return `already_setup`.
3. For each device to save with TOTP protection, a `totp_create_opaque_key` command
   is issued with a TOTP one-time-password as parameter.
4. To decrypt the device, a `totp_fetch_opaque_key` is issued with a TOTP
   one-time-password as parameter.

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

### 2.6 - Reset TOTP setup

When a user lose his TOTP authenticator app, his TOTP setup has to be reset.

Two actor should be able to do that:

- The organization admins: since they are responsible with enrolling new users,
  they might be tempted to revoke and re-enroll any user coming at them, which
  is obviously the wrong solution to solve a lost TOTP authenticator app!
- The server admins: this is required since otherwise an organization with a
  single admin would be stuck if this admin needs to reset his TOTP setup...

On top of that implementing the TOTP setup reset for organization admins requires
new APIs (notably to know if a given user has configured TOTP) and potentially
rethinking the GUI (currently we list the users in the GUI from the user
certificates which are purely local info, but now we would also have to request
the server to know about this TOTP setup status...).

For those reasons, we should:

- First implement a new command in the server CLI for the server admin.
- Leave the organization admin for another day ;-p

> [!NOTE]
>
> No opaque key is lost during TOTP setup reset. Once the TOTP setup is completed
> again, the user can decrypt his existing TOTP-protected devices using his new
> authenticator app.
>
> Put it another way: TOTP setup reset only changes the TOTP secret shared
> between the authenticator app and the server.

## 3 - Protocol

### 3.1 - Initial TOTP setup

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
                    // TOTP configuration according to RFC 6238.
                    // Only the secret is provided since we use the defaults
                    // values for the rest (i.e. SHA1, 30s period, 6 digits).
                    // see https://datatracker.ietf.org/doc/html/rfc6238
                    {
                        // TOTP secret to configure in the user's authenticator app.
                        "name": "totp_secret",
                        "type": "Bytes"
                    }
                ]
            },
            {
                // The TOTP has already been setup.
                // Note if the end user no longer has access to the TOTP secret (e.g.
                // he has lost his phone containing his authenticator app), then
                // he should contact a server admin that can then do a TOTP reset so
                // that `totp_setup_get_secret` can be called again.
                "status": "already_setup"
            }
        ]
    }
]
```

Once the TOTP secret has been configured in the user's authenticator app,
a second authenticated API is to be called:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "totp_setup_confirm",
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
                // Once `ok` status has been returned, the TOTP setup is considered
                // completed and any further attempt at calling `totp_setup_*` will
                // return `already_setup`.
                "status": "ok"
            },
            {
                "status": "invalid_one_time_password"
            },
            {
                // The TOTP has already been setup.
                // Note if the end user no longer has access to the TOTP secret (e.g.
                // he has lost his phone containing his authenticator app), then
                // he should contact a server admin that can then do a TOTP reset so
                // that `totp_setup_get_secret` can be called again.
                "status": "already_setup"
            }
        ]
    }
]
```

> [!NOTE]
> This at this point `totp_setup_get_secret` can be called to obtain the TOTP secret,
> there is no point in having a protection against brute force attempts here.

### 3.2 - Subsequent TOTP setup after admin reset

see [TOTP setup reset](#26---reset-totp-setup)

Anonymous API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "totp_setup_get_secret",
        "req": {
            "fields": [
                {
                    "name": "user_id",
                    "type": "UserID"
                },
                {
                    // Authentication token, this should be provided by the server admin
                    // once he has reset the TOTP config.
                    "name": "token",
                    "type": "AccessToken"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    // TOTP configuration according to RFC 6238.
                    // Only the secret is provided since we use the defaults
                    // values for the rest (i.e. SHA1, 30s period, 6 digits).
                    // see https://datatracker.ietf.org/doc/html/rfc6238
                    {
                        // TOTP secret to configure in the user's authenticator app.
                        "name": "totp_secret",
                        "type": "Bytes"
                    }
                ]
            },
            {
                // This status is returned if:
                // - The authentication token is invalid (obviously)
                // - The user is revoked or frozen
                // - TOTP setup has already been completed
                "status": "bad_token"
            }
        ]
    }
]
```

Once the TOTP secret has been configured in the user's authenticator app,
a second anonymous API is to be called:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "totp_setup_confirm",
        "req": {
            "fields": [
                {
                    "name": "user_id",
                    "type": "UserID"
                },
                {
                    // Authentication token, this should be provided by the server admin
                    // once he has reset the TOTP config.
                    "name": "token",
                    "type": "AccessToken"
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
                // Once `ok` status has been returned, the TOTP setup is considered
                // completed and any further attempt at calling `totp_setup_*` will
                // return `bad_token`.
                "status": "ok"
            },
            {
                "status": "invalid_one_time_password"
            },
            {
                // This status is returned if:
                // - The authentication token is invalid (obviously)
                // - The user is revoked or frozen
                // - TOTP setup has already been completed
                "status": "bad_token"
            }
        ]
    }
]
```

> [!NOTE]
> This at this point `totp_setup_get_secret` can be called to obtain the TOTP secret,
> there is no point in having a protection against brute force attempts here.

### 3.2 - Create a TOTP-protected opaque key to save a local device

Authenticated API:

```json5
[
    {
        "major_versions": [
            5
        ],
        "cmd": "totp_create_opaque_key",
        "req": {
            "fields": []
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "opaque_key_id",
                        "type": "TOTPOpaqueKeyID"
                    },
                    {
                        // Note the server knows about this key as it is not encrypted!
                        //
                        // This is because TOTP's goal is to protect against unauthorized
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
            }
        ]
    }
]
```

> [!NOTE]
> Unlike for fetching the opaque key (see below), there is no TOTP challenge here.
> This is for two reasons:
>
> - The server is not guarding an existing secret here, so TOTP challenge is
>   not strictly needed (it would be more a sanity check to ensure the TOTP has
>   been correctly setup, which is something that can be done in other ways).
> - Creating an opaque key before doing the TOTP setup is not an issue (though
>   it is unlikely to occur in practice).
> - TOTP challenge must be protected against brute force attack which is
>   non-trivial (typically `totp_fetch_opaque_key` should do throttled on a
>   per opaque key ID basis, but here we would need to do it on a per-user
>   basis...).

### 3.3 - Access the TOTP-protected opaque key to load a local device

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
                    "name": "user_id",
                    "type": "UserID"
                },
                {
                    "name": "opaque_key_id",
                    "type": "TOTPOpaqueKeyID"
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
                        // This is because TOTP's goal is to protect against unauthorized
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
                // This status is returned if:
                // - The one-time-password is invalid (obviously)
                // - The user is revoked or frozen
                // - No opaque key exists with the provided ID
                // - TOTP setup hasn't been completed (or has been reset)
                "status": "invalid_one_time_password"
            },
            {
                // Too many attempts with an invalid one time password have
                // been done to this opaque key ID.
                // Throttling follow a simple `wait_time_in_seconds = 2 ^ number_of_failed_attemps`
                // The absence of higher bound makes it very aggressive (e.g. after
                // 20 errors you have to wait 12 days for the next attempt...),
                // this is by design:
                // - Throttling is done per-opaque-key ID, which is only known by
                //   the machine containing the device keys file to decrypt.
                // - So there is no risk for an external attacker doing a deny of service
                //   by with the throttling to prevent user from login.
                // - A legitimate user can still reset the throttle by asking the server
                //   admin to reset his TOTP setup.
                // Also note returning the `throttled` status doesn't count as a failed
                // attempt, so an attacker cannot overflow the wait time by just spamming
                // attempts in a quick burst.
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
            // When TOTP is not used, `ciphertext` contains data encrypted by the
            // key obtained from the primary protection (i.e. the cooking of the
            // user-provided password).
            //
            // When TOTP is used, `ciphertext` is contains data encrypted:
            // - First using the secret key obtained from the primary protection.
            // - Then using the secret key obtained from the TOTP challenge.
            // Put it another way, the local device decryption is done this way:
            //
            // ```rust
            // let key_from_primary_protection: SecretKey = ...
            // let key_from_totp_challenge: SecretKey = do_totp_challenge(
            //     <totp_opaque_key_id>,
            //     <one_time_password>
            // ).await;
            // let cleartext = key_from_totp_challenge.decrypt(
            //     key_from_primary_protection.decrypt(ciphertext)
            // );
            // let device = LocalDevice::load(cleartext);
            // ```
            "name": "totp_opaque_key_id",
            "type": "NonRequiredOption<TOTPOpaqueKeyID>",
            // Introduced in Parsec 3.8.0
            "introduced_in_revision": 380
        }
    ]
}
```

## 5 - Libparsec API

### 5.1 - TOTP setup API

The API is divided into two parts.

A client one using the authenticated commands (for the initial setup):

```rust
enum TOTPStatus {
    Stalled {
        // Secret to display to the end-user so that he can configure his authenticator app
        totp_secret: Bytes,
    }
    AlreadySetup,
}

async fn client_totp_setup_status(
    client: Handle,
) -> Result<TOTPSetupStatus, ClientTotpSetupStatusError> {
    // ...
}

pub async fn client_totp_setup_confirm(
    client: Handle,
    one_time_password: String,
) -> Result<(), ClientTOTPSetupConfirmError> {
    // ...
}
```

And another using anonymous commands (for the setup after reset):

```rust
pub async fn totp_setup_status_anonymous(
    config: ClientConfig,
    addr: ParsecTOTPResetAddr,
) -> Result<TOTPSetupStatus, TotpSetupStatusAnonymousError> {
    // ...
}

pub async fn totp_setup_confirm_anonymous(
    config: ClientConfig,
    addr: ParsecTOTPResetAddr,
    one_time_password: String,
) -> Result<(), TotpSetupConfirmAnonymousError> {
    // ...
}
```

### 5.2 - Create/fetch TOTP-protected opaque key

```rust
pub async fn client_totp_create_opaque_key(
    client: Handle,
) -> Result<(TOTPOpaqueKeyID, SecretKey), ClientTotpCreateOpaqueKeyError> {
    // ...
}

pub async fn totp_fetch_opaque_key(
    config: ClientConfig,
    addr: ParsecOrganizationAddr,
    user_id: UserID,
    opaque_key_id: TOTPOpaqueKeyID,
    one_time_password: String,
) -> Result<SecretKey, TotpFetchOpaqueKeyError> {
    // ...
}
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

Listing available device should provide information about the need for TOTP:

```rust
pub enum AvailableDeviceType {
    Keyring,
    Password,
    // ...
}

pub struct AvailableDevice {
    pub key_file_path: PathBuf,
    // ...
    pub ty: AvailableDeviceType,
    pub totp_opaque_key_id: Option<TOTPOpaqueKeyID>,
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
    // ...
}
```

## 6 - List of changes

- New authenticated API: `totp_setup_get_secret`
- New anonymous API: `totp_setup_get_secret`
- New authenticated API: `totp_create_opaque_key`
- New anonymous API:  `totp_fetch_opaque_key`
- CLI server: reset TOTP setup for a given user
- Introduce a new `totp` field in each device file schema (`DeviceFilePassword`, `DeviceFileKeyring`, etc.)
- Libparsec: new function `client_totp_setup_status` & `totp_setup_status_anonymous`
- Libparsec: new function `client_totp_setup_confirm` & `totp_setup_confirm_anonymous`
- Libparsec: new function `client_totp_create_opaque_key`
- Libparsec: new function `totp_fetch_opaque_key`
- GUI: check TOTP status and setup it (using `client_totp_status` and `client_totp_try_challenge`)
- GUI: add "enable MFA" option in save device dialogue (using `client_totp_create_opaque_key`)
- GUI: add "TOTP challenge" in load device dialogue (using `client_totp_fetch_opaque_key`)
