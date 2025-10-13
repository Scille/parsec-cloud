<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# PKI Support

## Overview

Add support to use Parsec with an externally managed PKI

## Background & Motivation

Some organizations manage their own PKI, and want Parsec to rely on that PKI.

That custom PKI come in the form of certificates with their private key often loaded in a dedicated device (a smartcard).
They would like to use their certificates to access Parsec, so the users do not have to use a password or rely on the keyring of the OS.

There is also an opportunity to rely on that PKI to join an organization.

Put in a simpler term access Parsec using a Smartcard.

### Terms:

- **PKI**: Public Key Infrastructure
- **Smartcard**:

  In this RFC, we call a Smartcard a "device" that is used to store certificates, secret keys (symmetric cryptography) or private keys (asymmetric cryptography) securely.
  That "device" is used by Parsec through an API (either `CryptAPI` on windows or `pkcs11` on unix) to list available certificates and to delegate cryptographic operations (signing a payload or decrypting it).
  The "device" come in different forms, like a USB stick for a Yubikey, the certificate store on Windows or SoftHSM for the `pkcs11` API.

- **Submitter**: A user/device submitting a request to join an organization using the PKI feature.
- **Accepter**: A user/device admin of an organization in charge of accepting or not the join request.

## Goals and Non-Goals

Some prior work was already done to add some primitives (sign, verify, encrypt, decrypt) to work with the windows certificate store under the `libparsec_platform_pki`.

The goals are to define the higher elements to make the PKI work with Parsec for the invitation process, those higher elements are:

- [`pki_enrollment_select_certificate`]
- [`pki_enrollment_sign_payload`]
- [`pki_enrollment_create_local_pending`]
- [`pki_enrollment_load_peer_certificate`]
- [`pki_enrollment_load_submit_payload`]
- [`pki_enrollment_load_accept_payload`]
- [`pki_enrollment_load_local_pending_secret_part`]

[`pki_enrollment_select_certificate`]: https://github.com/Scille/parsec-extensions/blob/5f0ae5ac44d23d775d92e241b546fc0cac327a0a/parsec_ext/smartcard/__init__.py#L217-L251
[`pki_enrollment_sign_payload`]: https://github.com/Scille/parsec-extensions/blob/5f0ae5ac44d23d775d92e241b546fc0cac327a0a/parsec_ext/smartcard/__init__.py#L254-L276
[`pki_enrollment_create_local_pending`]: https://github.com/Scille/parsec-extensions/blob/5f0ae5ac44d23d775d92e241b546fc0cac327a0a/parsec_ext/smartcard/__init__.py#L279-L333
[`pki_enrollment_load_peer_certificate`]: https://github.com/Scille/parsec-extensions/blob/5f0ae5ac44d23d775d92e241b546fc0cac327a0a/parsec_ext/smartcard/__init__.py#L336-L355
[`pki_enrollment_load_submit_payload`]: https://github.com/Scille/parsec-extensions/blob/5f0ae5ac44d23d775d92e241b546fc0cac327a0a/parsec_ext/smartcard/__init__.py#L385-L406
[`pki_enrollment_load_accept_payload`]: https://github.com/Scille/parsec-extensions/blob/5f0ae5ac44d23d775d92e241b546fc0cac327a0a/parsec_ext/smartcard/__init__.py#L409-L430
[`pki_enrollment_load_local_pending_secret_part`]: https://github.com/Scille/parsec-extensions/blob/5f0ae5ac44d23d775d92e241b546fc0cac327a0a/parsec_ext/smartcard/__init__.py#L433-L473

## Design

Here is a possible user flow of Alice (Submitter) joining the organization CoolOrg using the PKI support:

 1. Alice wants to join CoolOrg organization by using the provided submission link to submit a request.
 2. Alice select the x509 certificate (from the smartcard) to use for the enrollment ([`pki_enrollment_select_certificate`])

    > [!IMPORTANT]
    > The certificate needs to have a `CommonName` ([oid-2.5.4.3](https://oid-base.com/get/2.5.4.3)) and a `EmailAddress` ([oid-1.2.840.113549.1.9.1](https://oid-base.com/get/1.2.840.113549.1.9.1)) attributes in the certificate's subject.

 3. Alice creates a user encryption key pair and a device signing key pair.
 4. Alice saves the private parts of the user & device keys pairs on the filesystem ([`pki_enrollment_create_local_pending`]). Those data are stored encrypted by the Smartcard's encryption key.
 5. Alice submits the request (authorized to re-submit later on with updated information) with: ([`pki_enrollment_submit`]):

    - The PKI certificate.
    - A payload consisting of the public parts of the user & device keys pairs and the desired device label ([`PkiEnrollmentSubmitPayload`]) (signed using [`pki_enrollment_sign_payload`]).

 6. Mallory (a CoolOrg admin which also has the PKI configured, this time the OS certificate store hold the private key and x509 certificate) sees the enrollment request from Alice ([`pki_enrollment_list`]).
 7. Mallory verifies the provided x509 certificate ([`pki_enrollment_load_peer_certificate`]) and provided data ([`pki_enrollment_load_submit_payload`]).

    > [!NOTE]
    > The verification of the certificate implies that we have access to the root certificates to be able to verify the trust chain.

 8. If Mallory accepts the request (after being validated), it creates the Alice's user and device certificates with the provided user & device public keys.
 9. Mallory signs both certificates with its device signing key. Then prepares the "accept" response payload ([`PkiEnrollmentAnswerPayload`])
10. Mallory selects the x509 certificate (from the certificate store of the OS) ([`pki_enrollment_select_certificate`])
11. Mallory signs the response payload with the certificate ([`pki_enrollment_sign_payload`])
12. Mallory sends the signed accept response payload alongside the used x509 certificate to the server ([`pki_enrollment_accept`])

    > [!NOTE]
    > The payload destined to Alice is signed with the x509 certificate, but the request needs to be authenticated to the server so it's signed with Mallory's device.

13. Alice checks the server to see the state of the request ([`pki_enrollment_info`])
14. Alice retrieves the accept response from the server
15. Alice starts validating the provided admin's x509 certificate ([`pki_enrollment_load_peer_certificate`]) and accept response payload signature ([`pki_enrollment_load_accept_payload`])
16. From that accept response payload and the saved local pending part ([`pki_enrollment_load_local_pending_secret_part`]), Alice is able to create a local device and use it to access the organization.

Alternative, if Mallory wasn't able to verify the payload (because of invalid certificate or signature) during step 8 or decided to reject the request at step 9 ([`pki_enrollment_reject`]),
Mallory's client would just have sent a reject response to Alice's request. Alice could then try to submit a new request.

[`pki_enrollment_submit`]: https://github.com/Scille/parsec-cloud/blob/v2.17.0/oxidation/libparsec/crates/protocol/schema/anonymous_cmds/pki_enrollment_submit.json5
[`pki_enrollment_list`]: https://github.com/Scille/parsec-cloud/blob/v2.17.0/oxidation/libparsec/crates/protocol/schema/authenticated_cmds/pki_enrollment_list.json5
[`PkiEnrollmentAnswerPayload`]: https://github.com/Scille/parsec-cloud/blob/df7bc6891989830c4d93f0b88ddaac9ade6b620c/libparsec/crates/types/schema/pki/pki_enrollment_answer_payload.json5
[`pki_enrollment_accept`]: https://github.com/Scille/parsec-cloud/blob/v2.17.0/oxidation/libparsec/crates/protocol/schema/authenticated_cmds/pki_enrollment_accept.json5
[`pki_enrollment_info`]: https://github.com/Scille/parsec-cloud/blob/v2.17.0/oxidation/libparsec/crates/protocol/schema/anonymous_cmds/pki_enrollment_info.json5
[`pki_enrollment_reject`]: https://github.com/Scille/parsec-cloud/blob/cb5cce5972e8e8228069762bbf2c0e8becd23c29/oxidation/libparsec/crates/protocol/schema/authenticated_cmds/pki_enrollment_reject.json5

### Select a certificate

[`pki_enrollment_select_certificate`] will present a native dialog to select a certificate detected by windows (generated by `CryptUI` API)
and verify that the chosen certificate is able to sign a payload by default by using [`pki_enrollment_sign_payload`].

### Sign a payload

[`pki_enrollment_sign_payload`] take a payload to sign using a referenced certificate

> [!NOTE]
> It's already provided by [`sign_message`]

[`sign_message`]: https://github.com/Scille/parsec-cloud/blob/a74cb0f3cfd29a9deb2878c3ddd8182ffad79ba9/libparsec/crates/platform_pki/src/lib.rs#L69-L72

### Create the local part of the async enrollment

[`pki_enrollment_create_local_pending`] create a pending enrollment ([`LocalPendingEnrollment`]) consisting of the following information:

- Certificate ref (provided)
- Server addr (provided)
- Creation date (provided)
- Enrollment ID (provided)
- Submit payload (provided)
- Encrypted key (generated): A random key used to encrypt/decrypt `cleartext`/`ciphertext`, that is encrypted using the external PKI.
- Ciphertext (generated): Is a MSGPack data containing the private key (provided) and signing key (provided) of the device that was encrypted by the random key.

The pending enrollment contains almost everything to create a device only missing the organization verification key that will be sent with the "accept" payload response ([`pki_enrollment_load_accept_payload`])

[`LocalPendingEnrollment`]: https://github.com/Scille/parsec-cloud/blob/df7bc6891989830c4d93f0b88ddaac9ade6b620c/libparsec/crates/types/schema/pki/local_pending_enrollment.json5

### Submit the request to join an organization

Part of the anonymous API [`pki_enrollment_submit`] allow a user to request join an organization by using an external PKI.

```jsonc
{
  "cmd": "pki_enrollment_submit",
  "req": {
    "fields": [
      {
        // UUID of the current request, generated by the sender.
        "name": "enrollment_id",
        "type": "EnrollmentID"
      },
      {
        // Allow to submit a new request to replace a pending one.
        // A request is identified by its `submitter_der_x509_certificate`.
        "name": "force",
        "type": "Bool"
      },
      {
        // The certificate used by the sender to sign the payload
        "name": "der_x509_certificate",
        "type": "Bytes"
      },
      {
        // The signature of the payload
        "name": "payload_signature",
        "type": "Bytes"
      },
      {
        // PkiEnrollmentSubmitPayload serialized in msgpack format
        "name": "payload",
        "type": "Bytes"
      }
    ]
  },
  "reps": [
    {
      // The request was accepted by the server and need to be reviewed by an organization admin.
      "status": "ok",
      "fields": [
        {
          "name": "submitted_on",
          "type": "DateTime"
        }
      ]
    },
    {
      // An enrollment request already exist for the provided certificate.
      "status": "already_submitted",
      "fields": [
        {
          "name": "submitted_on",
          "type": "DateTime"
        }
      ]
    },
    {
      "status": "id_already_used"
    },
    {
      // The server check if an user doesn't already exist using the email contained in the certificate.
      "status": "email_already_used"
    },
    {
      // The certificate was already used with an accepted request.
      "status": "already_enrolled"
    },
    {
      // The server tries to deserialize the payload to check if it use the correct format.
      "status": "invalid_payload"
    }
  ]
}
```

[`PkiEnrollmentSubmitPayload`] is already defined on the main branch with the correct fields and don't need to be updated.

[`PkiEnrollmentSubmitPayload`]: https://github.com/Scille/parsec-cloud/blob/df7bc6891989830c4d93f0b88ddaac9ade6b620c/libparsec/crates/types/schema/pki/pki_enrollment_submit_payload.json5

### List the pending request

To list the submitted request (i.e. the pending one), we use the command [`pki_enrollment_list`], it's part of the authenticated API.

```jsonc
{
  "cmd": "pki_enrollment_list",
  "req": {},
  "reps": [
    {
      "status": "ok",
      "fields": [
        {
          "name": "enrollments",
          "type": "List<PkiEnrollmentListItem>"
        }
      ]
    },
    {
      // The command should only be used by admin users
      "status": "not_allowed"
    }
  ],
  "nested_types": [
    {
      "name": "PkiEnrollmentListItem",
      "fields": [
        {
          // UUID of the request
          "name": "enrollment_id",
          "type": "EnrollmentID"
        },
        {
          // When the request was made
          "name": "submitted_on",
          "type": "DateTime"
        },
        {
          // The certificate used by the submitter to sign the request.
          // The server does not perform any validation on the certificate,
          // those verifications need to be made by the "accepter"
          "name": "der_x509_certificate",
          "type": "Bytes"
        },
        {
          // The signature of the payload
          "name": "payload_signature",
          "type": "Bytes"
        },
        {
          // The payload, it's a `PkiEnrollmentSubmitPayload` formatted in msgpack
          "name": "payload",
          "type": "Bytes"
        }
      ]
    }
  ]
}
```

### Load a certificate

[`pki_enrollment_load_peer_certificate`] load a provided x509 certificate in DER format and only extract the issuer and subject from it.
It also creates a fingerprint for the provided certificate.

### Load submitted payload

[`pki_enrollment_load_submit_payload`] Take a payload and a certificate, the certificate is validated against a set of trusted root certificate and it can be used to sign data (`digital_signature`).
Once the certificate validated, we can validate the payload signature against the certificate public verification key

> [!NOTE]
> Both steps can be concurrent

### Accept the enrollment request

[`pki_enrollment_accept`]

```jsonc
{
  "cmd": "pki_enrollment_accept",
  "req": {
    "fields": [
      {
        // ID of the accepted request
        "name": "enrollment_id",
        "type": "EnrollmentID"
      },
      {
        // Certificate used to sign the payload by the "accepter"
        "name": "der_x509_certificate",
        "type": "Bytes"
      },
      {
        // Signature of the payload
        "name": "payload_signature",
        "type": "Bytes"
      },
      {
        // `PkiEnrollmentAnswerPayload` in msgpack format
        "name": "payload",
        "type": "Bytes"
      },
      {
        "name": "device_certificate",
        "type": "Bytes"
      },
      {
        "name": "redacted_device_certificate",
        "type": "Bytes"
      },
      {
        "name": "user_certificate",
        "type": "Bytes"
      },
      {
        "name": "redacted_user_certificate",
        "type": "Bytes"
      }
    ]
  },
  "reps": [
    {
      "status": "ok"
    },
    {
      // The user does not have the correct permission to perform that action
      "status": "not_allowed"
    },
    {
      // The payload is not correctly formatted
      "status": "invalid_payload_data"
    },
    {
      // The provided certificate are not valid
      "status": "invalid_certification"
    },
    {
      // The server did not found a request for the provided ID
      "status": "not_found"
    },
    {
      // The request is no longer in pending state (either accepted or rejected)
      "status": "no_longer_available"
    },
    {
      // The "new" user already exist in the organization
      "status": "already_exists"
    }
  ]
}
```

[`PkiEnrollmentAnswerPayload`] is already present on the main branch with the correct fields.

### Check the status of a request

[`pki_enrollment_info`] is used by the Submitter to fetch the status of its request.

```jsonc
{
  "cmd": "pki_enrollment_info",
  "req": {
    "fields": [
      {
        "name": "enrollment_id",
        "type": "EnrollmentID"
      }
    ]
  },
  "reps": [
    {
      "status": "ok",
      "unit": "PkiEnrollmentInfoStatus"
    },
    {
      "status": "not_found"
    }
  ]
}
```

### Load local part of the async enrollment

[`pki_enrollment_load_local_pending_secret_part`] load the local part created during [Create the local part of the async enrollment](#create-the-local-part-of-the-async-enrollment).

1. From the enrollment ID, it loads the local part from the filesystem config directory.
2. Decrypt the random key using the certificate present in the PKI of the system.
3. Decrypt the ciphered text using the random key to extract the private and signing key after being deserialized using MSGPack.
4. Return both keys.

### Reject the enrollment request

[`pki_enrollment_reject`] is used by an organization's admin to reject a request.

```jsonc
{
  "cmd": "pki_enrollment_reject",
  "req": {
    "fields": [
      {
        "name": "enrollment_id",
        "type": "EnrollmentID"
      }
    ]
  },
  "reps": [
    {
      "status": "ok"
    },
    {
      // The user does not have the correct permission to perform that action
      "status": "not_allowed"
    },
    {
      // The server did not found a request for the provided ID
      "status": "not_found"
    },
    {
      // The request is no longer in pending state (either accepted or rejected)
      "status": "no_longer_available"
    }
  ]
}
```
