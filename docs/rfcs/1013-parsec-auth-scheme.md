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
account_creation_step1_email_validation:
  req-fields:
   email: Email
  reps:
    - status: ok
    - status: invalid_email
    - status: email_already_exists
```

On `ok`, the server would have sent a mail with a unique token used for next the request used to register the authentication method:

```yml
account_creation_step2_password:
  req-fields:
    email: Email
    email_validation_token: EmailValidationToken
    enc_sym_key: EncSymKey
    secret: DerivedPassword
    public_key: PublicKey
  reps:
    - status: ok
      auth_token: AuthToken
    - status: invalid_email
    - status: email_already_exists
    - status: invalid_validation_token
```

`EncSymKey`, `DerivedPassword` and `PublicKey` are specialized `Bytes` types.

### Authentication

To retrieve a `AuthToken`, the client sent its email and derived password:

```yml
authenticaton_password:
  req-fields:
    email: Email
    secret: DerivedPassword
  reps:
    - status: ok
      auth_token: AuthToken
    # Either the email does not exist or the secret is invalid.
    - status: invalid_credentials
    - status: internal_error
```

### Uploading a new device

To upload a new device, the client first need to retrieve the current symmetric key:

```yml
get_enc_sym_key:
  use_auth_token: true
  req-fields:
    public_key: PublicKey
  reps:
    - status: ok
      enc_sym_key: EncSymKey
    - status: unauthenticated
    - status: unknown_public_key
```

Now that the client can encrypt the device with the symmetric key, it can upload it:

```yml
upload_device:
  use_auth_token: true
  req-fields:
    organization_id: OrganizationId
    encrypted_device: EncryptedDevice
  reps:
    - status: ok
    - status: unauthenticated
```

### List available devices

To list the organization that have a registered device, the client only need the auth-token:

```yml
list_devices:
  use_auth_token: true
  req-fields: {}
  reps:
    - status: ok
      organization_ids: Vec<OrganizationId>
    - status: unauthenticated
```

### Retrieve device

To retrieve the encrypted device, the client simply has to provide the organization ID:

```yml
get_enc_device:
  use_auth_token: true
  req-fields:
    organization_id: OrganizationId
  reps:
    - status: ok
      encrypted_device: EncryptedDevice
    - status: unauthenticated
    - status: device_not_found
```

### Adding a new authentication method

To add a new authentication method, the client just needs to provide the required information:

```yml
add_auth_method_password:
  use_auth_token: true
  req-fields:
    secret: DerivedPassword
    public_key: PublicKey
    enc_sym_key: EncSymKey
  reps:
    # We do not return a new auth token since the client already has one.
    - status: ok
    - status: unauthenticated
```

### Removing an authentication method

To remove an authentication method, the client needs to re-encrypt all the devices.
The first step is to retrieve the said devices:

```yml
get_all_enc_devices:
  use_auth_token: true
  req-fields: {}
  reps:
    - status: ok
      devices: HashMap<OrganizationId, EncryptedDevice>
    - status: unauthenticated
```

The second step is to get all encrypted symmetric keys (the client will need to filter out the one associated with the removed auth method):

```yml
get_all_enc_sym_keys:
  use_auth_token: true
  req-fields: {}
  reps:
    - status: ok
      enc_sym_keys: HashMap<PublicKey, EncSymKey>
    - status: unauthenticated

```

Now the client has everything to re-encrypt the devices and re-upload them:

```yml
remove_auth_method_password:
  use_auth_token: true
  req-fields:
    auth_method_to_remove: PublicKey
    # Some validations should be done, no devices should be added or removed.
    devices: HashMap<OrganizationId, EncryptedDevice>
    # Some validations should be done on the server side:
    # - The auth method to remove should be in the list.
    # - The list should not have new public keys.
    # - The list should not be empty.
    enc_sym_keys: HashMap<PublicKey, EncSymKey>
  reps:
    - status: ok
    - status: unauthenticated

```

### Deleting the account

To delete the account, the client needs to be authenticated and provide the derived password:

```yml
delete_account:
  use_auth_token: true
  req-fields:
    secret: DerivedPassword
  reps:
    - status: ok
    - status: unauthenticated
    - status: invalid_credentials
```
