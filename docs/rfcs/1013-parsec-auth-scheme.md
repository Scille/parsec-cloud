<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Schema for parsec-auth

## Overview

This RFC defines the schema used for communicating with the parsec-auth service.

## Goals and Non-Goals

The goal is to define the required schema used when communicating with the parsec-auth service.
For simplicity, we will skip the fido2 part that will be defined in a later RFC.

## Design

### Account creation

To start the account creation, the client start by sending its email.

```yml
cmd: account_creation_step1_email_validation
req-fields:
 email: Email
reps:
  - status: ok
  - status: invalid_email
```

On `ok`, the server would have sent a mail with a unique token used for next the request used to register the authentication method:

```yml
cmd: account_creation_step2_registration
req-fields:
  email_validation_token: EmailValidationToken
  auth_medium_hmac_key: HMACKey
  auth_medium_encrypted_priv_key: AuthMediumEncryptedPrivKey
  auth_medium_encrypted_priv_key_key_algorithm: KeyAlgorithm
  auth_medium_public_key: PublicKey
reps:
  - status: ok
  - status: invalid_email_validation_token
```

`HMACKey`, `AuthMediumEncryptedPrivKey` and `PublicKey` are specialized `Bytes` types.

### Uploading a new device

Before uploading, the client needs to retrieve the list of auth medium to encrypt the device symmetric key for them:

```yml
cmd: auth_medium_list
auth_api: true
req-fields: {}
reps:
  - status: ok
    items:
      - id: AuthMediumID
        public_key: PublicKey
      # ...
```

Now that the client can encrypt the device with the symmetric key, it can upload it:

```yml
cmd: device_upload
auth_api: true
req-fields:
  organization_id: OrganizationID
  # Proof that that we can use the uploaded device.
  # It's the signature of `DeviceID` + AuthMediumID currently used to authenticate the request.
  # The sigature is verified by the server by asking the metadata server.
  device_ownership_proof: DeviceOwnershipProof
  encrypted_device: EncryptedDevice
  # Some validations should be done on the server side:
  # All current auth method should be listed (no new, no missing).
  per_auth_medium_encrypted_encrypted_device_key: HashMap<AuthMediumID, EncryptedEncryptedDeviceKey>
reps:
  - status: ok
  - status: auth_medium_mismatch
  - status: invalid_ownership_proof
  - status: per_auth_medium_encrypted_encrypted_device_key_missmatch
```

> [!NOTE]
> The user can upload multiple devices for the same organization.

### List available devices

To list the devices registered in the service, the client only needs to be authenticated:

```yml
cmd: device_list
auth_api: true
req-fields: {}
reps:
  - status: ok
    devices:
      - organization_id: OrganizationID
        device_id: DeviceID
        user_id: UserID
        human_handle: HumanHandle
        # Creation date in parsec-auth
        created_on: DateTime
        revoked_on: Option<DateTime>
      # ...
```

### Retrieve device

To retrieve the encrypted device, the client simply has to provide the organization ID:

```yml
cmd: device_get
auth_api: true
req-fields:
  organization_id: OrganizationId
  device_id: DeviceID
reps:
  - status: ok
    encrypted_encrypted_device_key: EncryptedEncryptedDeviceKeysFileKey
    encrypted_device: EncryptedDevice
  - status: device_not_found
```

### Adding a new authentication method

To add a new authentication method, the client just needs to provide the required information:

```yml
cmd: auth_medium_new
auth_api: true
req-fields:
  delete_auth_medium_id: Option<AuthMediumID>
  auth_medium_hmac_sym_key: DerivedPassword
  auth_medium_public_key: PublicKey
  auth_medium_encrypted_priv_key: AuthMediumEncryptedPrivKey
  auth_medium_encrypted_priv_key_key_algorithm: KeyAlgorithm
  # Containt a key for each device present in parsec-auth.
  per_device_encrypted_encrypted_device_key: HashMap<(OrganizationId, DeviceId), EncryptedEncryptedDeviceKey>
reps:
  - status: ok
    id: AuthMediumID
  - status: invalid_auth_medium_id
  - status: per_device_encrypted_encrypted_device_key_missmatch
```

> `delete_auth_medium_id` is optional and can be used to delete the authentication method by the ID.

### Removing an authentication method

To remove an authentication method

```yml
cmd: auth_medium_delete
auth_api: true
req-fields:
  auth_medium_id: AuthMediumID
reps:
  - status: ok
  - status: invalid_auth_medium_id
```

### Deleting the account

To delete the account, the client needs to be authenticated and provide the derived password:

```yml
cmd: delete_account
auth_api: true
req-fields: {}
reps:
  - status: ok
```
