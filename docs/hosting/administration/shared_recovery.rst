.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_administration_shared_recovery:

.. cspell:words xBCEREHKItJ0lPzzEuk-8q0N


Shared recovery
===============

..  warning::

  This section describes an advanced function that is currently only available
  via Parsec CLI. For a user friendly way to recover access to your organization
  see :ref:`Recovery files<doc_userguide_recovery_files>`.

The shared recovery allows a user to recover access to its organization by
distributing the information required to recover its account (the "secret")
among a group of users of the organization. The information is divided into parts
(the "shares") from which the secret can be reassembled only when quorum is
achieved, i.e. a sufficient number of shares (the "threshold") are combined,
therefore enabling the recovery of the user account.

This is based on *Shamir's secret sharing algorithm*. The idea is that even if an
attacker steals some shares, it is impossible for the attacker to reconstruct
the secret unless they have stolen the quorum number of shares.


Overview
--------

The shared recovery process involves the following steps:

1. the user creates a shared recovery setup by choosing

  - the list of users to send shares (*recipients*)
  - the number of shares for each recipient (*weight*)
  - the number of shares required to recover the account (*threshold*)

2. any of the recipient sends an invitation to the user to recover their account
3. the user contacts the recipients one by one until the threshold is reached

The ``<DEVICE_ID>`` mentioned in the commands below always refer to the device
of the user running the command (user or administrator). You can find out which
is your device ID by running:

  .. code-block:: bash

    parsec-cli device list

Shared recovery creation
------------------------

To setup shared recovery for a device, run the following command:

  .. code-block:: bash

    parsec-cli shared-recovery create --device <DEVICE_ID>

Run the help of this command to know all the options.

When no recipient is specified, all the Administrations of the organization
will be recipients with a single share. Note that users with External profile
are not able to choose their recipients as they do not have access to the
organization user list.

If the threshold is not specified, it will be asked interactively.

Recover access with shared recovery
-----------------------------------

The shared recovery process must be initiated by one of the recipients, so the
user needs to contact them and ask them for an invitation to recover its account.

A recipient can create an invitation by using the user's email:

  .. code-block:: bash

    parsec-cli invite shared-recovery user@example.com --device $DEVICE

The user will receive an email with the invitation url, and will be able to
claim the invitation with the following command:

  .. code-block:: bash

    parsec-cli invite claim $INVITATION_URL

The user will have to select recipients one by one, performing the SAS code
exchange, until until enough shares have been gathered. At which point
the new device is registered and access is fully recovered.

An example scenario
-------------------

The following is a simple scenario to show you the shared recovery process.
All commands are executed in the same machine for simplicity.

Given an organization with the following users:

  .. code-block:: bash

    parsec-cli device list
    870 - Org: Arnold <arnold@example.com> @ label
    bc1 - Org: Alice <alice@example.com> @ laptop
    ea9 - Org: Bob <bob@example.com> @ laptop

Bob is a Member of the organization. Alice and Arnold are Administrators.

First Bob needs to create their shared recovery setup.

  .. code-block:: bash

    # Bob
    parsec-cli shared-recovery create --device ea9
    Enter password for the device:
    ✔ Poll server for new certificates
    ... Creating shared recovery setup
    Choose a threshold between 1 and 2
    The threshold is the minimum number of recipients that one must gather to recover the account: 2
    ✔ Shared recovery setup has been created

All the Administrators (Alice and Arnold) are recipients, as no recipients was
provided. Bob chooses interactively the threshold.
So Bob's shared recovery is all setup.

Oh no! Bob has lost access to their device. It must contact an Alice or Arnold in
order to be invited again through a shared recovery process.

Alice creates the invitation and share the URL with Bob.

  .. code-block:: bash

    # Alice
    parsec-cli invite shared-recovery  bob@example.com --device bc1
    ✔ Poll server for new certificates
    Invitation URL: parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_shamir_recovery&p=xBCEREHKItJ0lPzzEuk-8q0N

Bob can now start the invitation process.

  .. code-block:: bash

    #Bob
    parsec-cli invite claim "parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_shamir_recovery&p=xBCEREHKItJ0lPzzEuk-8q0N"
    ✔ Retrieving invitation info
    2 shares needed for recovery
    Choose a person to contact now:
    > Alice <alice@example.com> - 1 share(s)
      Arnold <arnold@example.com> - 1 share(s)

Bob must choose a person to contact first.
Let's choose Alice first.

In the meantime, Alice must be ready to greet Bob.
First, retrieve the invitation token.

  .. code-block:: bash

    # Alice
    parsec-cli invite list --device bc1
    ✔ Poll server for new certificates
    2 invitations found.
    844441ca22d27494fcf312e93ef2ad0d	pending	shamir recovery (Bob <bob@example.com>)

Then it can be use to greet Bob.
And proceed to a SAS code exchange.

  .. code-block:: bash

    # Alice
    parsec-cli invite greet --device bc1 844441ca22d27494fcf312e93ef2ad0d
    ✔ Poll server for new certificates
    ✔ Retrieving invitation info
    ✔ Waiting for claimer
    Code to provide to claimer: 5CDY
    ✔ Waiting for claimer
    Select code provided by claimer: C8UX

Now Bob has one share of the two they need.
So they can repeat the process with Arnold.

  .. code-block:: bash

    # Bob
    parsec-cli invite claim "parsec3://127.0.0.1:6770/Org?no_ssl=true&a=claim_shamir_recovery&p=xBCEREHKItJ0lPzzEuk-8q0N"
    # ...
    Out of 2 shares needed for recovery, 1 were retrieved.
    Choose a person to contact now: Arnold <arnold@example.com> - 1 share(s)
    Invitation greeter: Arnold <arnold@example.com>
    ✔ Waiting the greeter Arnold <arnold@example.com> to start the invitation procedure
    Select code provided by greeter: DL9Q
    Code to provide to greeter: 2VWL
    ✔ Waiting for greeter
    ✔ Waiting for greeter
    Enter device label: label
    ✔ Recovering device
    Enter password for the new device:
    Confirm password:

Once the SAS codes are exchanged, Bob can setup their new device with a label and password.
And so the shared recovery process is fully completed.
