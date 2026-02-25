.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_mfa_reset:

MFA setup reset
===============

When a user loses access to their authenticator app (e.g. lost or replaced phone),
their MFA setup must be reset by a **server administrator** before the user can
reconfigure it (see :ref:`doc_userguide_mfa`).

To reset a user's MFA setup, the server administrator needs to provide:

- The Parsec server address, as a parsec URL ``parsec3://hostname:port``
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
