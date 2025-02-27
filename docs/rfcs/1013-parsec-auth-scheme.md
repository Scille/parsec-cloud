<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Schema for parsec-auth

## Overview

This RFC defines the schema used for communicating with the parsec-auth service.

## Goals and Non-Goals

The goal is to define the required schema used when communicating with the parsec-auth service.
For simplicity, we will skip the fido2 part that will be defined in a later RFC.

## Design

### Account creation

```rust
mod AccountCreation {
  struct Step1EmailValidationReq {
    email: Email,
  }

  enum Step1EmailValidationRes {
    Ok,
    ErrInvalidEmail,
    ErrEmailAlreadyExists,
    ErrInternal
  }

  struct Step2AccountCreationBaseReq {
    email: Email,
    email_validation_token: EmailValidationToken,
    enc_sym_key: EncSymKey,
  }

  struct Step2AccountCreationPasswordReq {
    base: Step2AccountCreationBaseReq,
    secret: DerivedPassword,
    public_key: PublicKey,
  }

  enum Step2AccountCreationPasswordRes {
    Ok(AuthToken),
    ErrInvalidEmail,
    ErrEmailAlreadyExists,
    ErrInvalidValidationToken,
    ErrInternal
  }
}
```

### Authentication

```rust
mod Authentication {
  struct AuthenticationPasswordReq {
    email: Email,
    secret: DerivedPassword
  }

  enum AuthenticationPasswordRes {
    Ok(AuthToken),
    /// Either the email does not exist or the secret is invalid.
    ErrInvalidCredentials,
    ErrInternal
  }
}
```

### Uploading a new device

```rust
mod AccountInfo {
  #[use_auth_token]
  struct GetEncSymKeyReq {
    public_key: PublicKey,
  }

  enum GetEncSymKeyRes {
    Ok(EncSymKey),
    ErrUnauthenticated,
    ErrUnknownPublicKey,
    ErrInternal
  }
}
```

```rust
mod UploadDevice {
  #[use_auth_token]
  struct UploadDeviceReq {
    organization_id: OrganizationId,
    encrypted_device: EncryptedDevice,
  }

  enum UploadDeviceRes {
    Ok,
    ErrUnauthenticated,
    ErrInternal
  }
}
```

### List available devices

```rust
mod ListDevices {
  #[use_auth_token]
  struct ListDevicesReq {}

  enum ListDevicesRes {
    Ok(Vec<OrganizationId>),
    ErrUnauthenticated,
    ErrInternal
  }
}
```

### Retrieve device

```rust
mod GetDevice {
  #[use_auth_token]
  struct GetDeviceReq {
    organization_id: OrganizationId,
  }

  enum GetDeviceRes {
    Ok(EncryptedDevice),
    ErrUnauthenticated,
    ErrDeviceNotFound,
    ErrInternal
  }
}
```

### Adding a new authentication method

```rust
mod AddAuthMethod {
  #[use_auth_token]
  struct AddAuthMethodPasswordReq {
    secret: DerivedPassword,
    public_key: PublicKey,
    enc_sym_key: EncSymKey,
  }

  enum AddAuthMethodRes {
    Ok,
    ErrUnauthenticated,
    ErrInternal
  }
}
```

### Removing an authentication method

```rust
mod GetDevice {
  #[use_auth_token]
  struct GetAllDeviceReq {
  }

  enum GetAllDeviceRes {
    Ok(HashMap<OrganizationId, EncryptedDevice>),
    ErrUnauthenticated,
    ErrInternal
}
```

```rust
mod AccountInfo {
  #[use_auth_token]
  struct ListEncSymKeysReq {
  }

  enum ListEncSymKeysRes {
    Ok(HashMap<PublicKey, EncSymKey>),
    ErrUnauthenticated,
    ErrInternal
  }
}
```

```rust
mod RemoveAuthMethod {
  #[use_auth_token]
  struct RemoveAuthMethodPasswordReq {
    auth_method_to_remove: public_key,
    devices: HashMap<OrganizationId, EncryptedDevice>,
    /// Some validations should be done on the server side:
    /// - The auth method to remove should be in the list.
    /// - The list should not have new public keys.
    /// - The list should not be empty.
    enc_sym_keys: HashMap<PublicKey, EncSymKey>,
  }

  enum RemoveAuthMethodRes {
    Ok,
    ErrUnauthenticated,
    ErrInternal
  }
}
```

### Deleting the account

```rust
mod AccountDeletion {
  #[use_auth_token]
  struct AccountDeletionPasswordReq {
    secret: DerivedPassword,
  }

  enum AccountDeletionRes {
    Ok,
    ErrUnauthenticated,
    ErrInvalidCredentials,
    ErrInternal
  }
}
```
