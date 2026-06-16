.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_mfa:

===================
User Authentication
===================

This section describes how to configure and manage user authentication in Parsec.

As described in the :ref:`Authentication <doc_hosting_client_deploy_authentication>` section,
user authentication is (mostly) a client-side operation in Parsec:
it determines the way the user's local device file will be protected.
Please refer to that section for more details.

Parsec also supports :abbr:`MFA (Multi-Factor Authentication)`
via :abbr:`TOTP (Time-based One-Time Password)` codes.
This is in addition to the primary protection (password, keyring, etc.).


Authentication advisories
=========================

When starting the server, the recommended authentication methods can be specified as shown in the example below.

.. code-block:: console

    # Recommend only PKI (SmartCard) and PASSWORD (with mandatory MFA)
    $ python -m parsec run [...] \
      --advisory-device-file-protection PASSWORD+TOTP  \
      --advisory-device-file-protection PKI

Possible values are :

- `PASSWORD`
- `PASSWORD+TOTP`
- `KEYRING`
- `KEYRING+TOTP`
- `PKI`
- `PKI+TOTP`
- `OPENBAO`
- `OPENBAO+TOTP`

.. important::

    Only the specified methods will be available for users in Parsec client application.

    However, since the encrypted device file is only available locally,
    the Parsec Server cannot *enforce* the specified methods (hence the *advisory* name).

    MFA is a special case because it involves communication with the Parsec Server.
    If you want to force users to enable MFA use *only* the variant including ``+TOTP``
    (e.g. ``PASSWORD+TOTP``, but not ``PASSWORD``)

.. _doc_hosting_mfa_reset:

MFA setup reset
===============

When a user loses access to their authenticator app (e.g. lost or replaced phone),
their MFA setup must be reset by a **server administrator** before the user can
reconfigure it (see :ref:`doc_userguide_mfa`).

To reset a user's MFA setup, the server administrator needs to provide:

- The Parsec server address, as a Parsec URL ``parsec3://hostname:port``
- The ``administration_token`` configured on the Parsec server
- The organization ID
- The user to reset, identified by their email address or their Parsec user ID

Typically to reset by email address:

.. code-block:: shell

    parsec-cli user totp-reset --addr parsec3://example.com --token s3cr3t --organization MyOrganization --user-email alice@example.com --send-email

Example of output:

.. code-block:: text

    TOTP reset for user 940a380aedd44127863d952a66cfce1e
    Reset URL: parsec3://example.com/MyOrganization?a=totp_reset&p=...
    An email with the reset URL has been sent to alice@example.com
