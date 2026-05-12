.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_invite:
.. cspell:words ZFIHY, SZCA

=======================
Invite CLI commands
=======================


Definitions
===========

Claimer
    The guest accepting the invitation

Greeter
    The host inviting the claimer into the Organization. May be the creator of the invitation.

General principles
==================

The invitation follows a 3 step process:

1. The invitation is created (the command depends on the type of invitation)
2. The Claimer uses the invitation link to claim the invitation (with the invite claim command)
3. The Greeter validates the Claimer's claim with the invitation token (with the invite greet command)

Steps 2 and 3 must occur simultaneously because there are synchronization points between
the Claimer and the Greeter. Using the CLI these synchronization points will be noted as a
circling dot and will become a ✔ when the wait will end.

The creation step will provide:

- an invitation link that will be used by the Claimer
- an invitation token that will be used by the Greeter.

The invitation may have been sent by email depending on the server's configuration,
but the invitation creator can directly transmit it to the Claimer by any secure channel.

The invitation token can also be retrieved listing all invitations.

User Invitation
===============

Creation
--------

The following command creates the invitation for `<user@example.com>`.


.. code-block:: shell

    parsec-cli invite user 'user@example.com' --device 627
    Enter password for the device:
    Invitation token: d57c6a64b5bd6eca760c3c41de3b0f61
    Invitation URL: http://127.0.0.1:6770/redirect/Org?a=claim_user&p=xBDVfGpktb1uynYMPEHeOw9h

Claim
-----

That new user can claim the invitation.

.. code-block:: shell

    parsec-cli invite claim 'http://127.0.0.1:6770/redirect/Org?a=claim_user&p=xBDVfGpktb1uynYMPEHeOw9h'
    ✔ Retrieving invitation info
    Invitation greeter: Alice <alice@example.com>
    ✔ Waiting the greeter to start the invitation procedure
    Select code provided by greeter: TP9W
    Code to provide to greeter: U4SQ
    ✔ Waiting for greeter
    Enter device label:
    new_device
    Enter email:
    user@example.com
    Enter name:
    User
    ✔ Waiting for greeter
    Enter password for the new device:
    Confirm password:
    New device created:
    19f - Org: User <user@example.com> @ new_device


Greet
-----

The Greeter can greet the new user. The device here is any device belonging to the greeter.

.. code-block:: shell

    parsec-cli invite greet d57c6a64b5bd6eca760c3c41de3b0f61 -d 627
    Enter password for the device:
    ✔ Poll server for new certificates
    ✔ Retrieving invitation info
    ✔ Waiting for claimer
    Code to provide to claimer: TP9W
    ✔ Waiting for claimer
    Select code provided by claimer: U4SQ
    ✔ Waiting for claimer information
    New device label: [new_device]
    New user: [User <user@example.com>]
    Which profile?: STANDARD
    ✔ Creating the user in the server


Device Invitation
=================

Creation
--------

Any user can have any device they want. The device specified here is an existing device.

.. code-block:: shell

    parsec-cli invite device -d 19f
    Enter password for the device:
    Invitation token: 67d55ad91481d806628782c0ebe6307a
    Invitation URL: parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_device&p=xBBn1VrZFIHYBmKHgsDr5jB6

Claim
-----

The user can claim the new device.

.. code-block:: shell

    parsec-cli invite claim 'parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_device&p=xBBn1VrZFIHYBmKHgsDr5jB6'
    ✔ Retrieving invitation info
    Invitation greeter: User <user@example.com>
    ✔ Waiting the greeter to start the invitation procedure
    Select code provided by greeter: SZCA
    Code to provide to greeter: 9W6U
    ✔ Waiting for greeter
    Enter device label:
    new_new_device
    ✔ Waiting for greeter
    Enter password for the new device:
    Confirm password:
    New device created:
    78b - Org: User <user@example.com> @ new_new_device

Greet
-----

The user must have access to an existing device and execute this command
while they a claiming the new device.

.. code-block:: shell

    parsec-cli invite greet -d 19f 67d55ad91481d806628782c0ebe6307a`
    Enter password for the device:
    ✔ Poll server for new certificates
    ✔ Retrieving invitation info
    ✔ Waiting for claimer
    Code to provide to claimer: SZCA
    ✔ Waiting for claimer
    Select code provided by claimer: 9W6U
    ✔ Waiting for claimer information
    New device label: [new_new_device]
    ✔ Creating the device in the server



List pending invitations
========================

Use the following command to list invitations that have been created but not yet completed.


.. code-block:: shell

    parsec-cli invite list -d 627
    Enter password for the device:
    ✔ Poll server for new certificates
    1 invitations found.
    d57c6a64b5bd6eca760c3c41de3b0f61	pending	user (email=user@example.com)
