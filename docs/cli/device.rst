.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_device:

===================
Device CLI commands
===================

In this context, a device is not a physical device but the encrypted file containing the keys allowing the user to access the Organization.

For more see :ref:`Key Parsec concepts <doc_hosting_architecture_concepts>`

List
====

List all devices available locally. The information provided for each device is:

- short id
- organization name
- user name
- email
- device label
- parsec server address

.. code-block:: shell

    parsec-cli device list
    Found 2 device(s) in /tmp/parsec-testenv-831f316f-5fa7-41f4-a7f8-ba5c5658d79b/config/parsec3/libparsec:
    052 - WorkOrg: Alice <alice@example.com> @ pc (parsec3://127.0.0.1:7778?no_ssl=true)
    b1e - PersonalOrg: Alice <alice@example.com> @ laptop (parsec3://127.0.0.1:7778?no_ssl=true)


.. _cli_device_forget_local:

Forget-local
============

This command deletes locally the selected device. This does not involve any server operation.

.. important::

  Please note that if the device file exists elsewhere, it can still be used to
  access the organization on this server. In other words, this command does not
  replace user revocation in case of a compromised device.


.. code-block:: shell

    parsec-cli device forget-local -d c90
    You are about to forget the following local device:
    c90 - Org: Toto <toto@example.com> @ laptop
    Are you sure? yes
    The local device has been forgotten


Change authentication
=====================

This command changes the authentication method to password or keyring.
Other authentication methods are supported by the GUI.

.. code-block:: shell

    parsec-cli device change-authentication -d 052 --password
    Enter current password for the device:
    Enter new password for the device:
    Confirm password:
    Device authentication changed successfully



.. note::

    In this example, the authentication method has not changed per se, but
    the password itself has been updated.


Recovery file
=============

In case you lose access to your device or are unable to authenticate,
you can setup a recovery device. It consists of two parts that MUST be stored
separately: the encrypted recovery device file and the passphrase.

These two elements can be used to create a new device.

The inaccessible device cannot be removed from the server, but you can safely
remove it from your local files (see :ref:`forget local <cli_device_forget_local>`).

Export recovery device
----------------------

This step must be done before you lose access to you device.

.. code-block:: shell

    parsec-cli device export-recovery-device recovery.txt -d 052
    Enter password for the device:
    Recovery device saved at recovery.txt
    Save the recovery passphrase in a safe place: LIQV-PHTV-K76Y-TSWV-C44U-6ILR-O2JA-XBUC-L27I-J47E-KSID-7OY4-TEEA


Import recovery device
----------------------

This step should be done after you lose access to your device.

.. code-block:: shell

    parsec-cli device import-recovery-device --input recovery.txt --label recovered_device
    Enter passphrase for the recovery file:
    Enter password for the new device:
    Confirm password:
    New device created:
    b1e - Org: Alice <alice@example.com> @ recovered_device
