.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_cheat_sheet:

===========
Cheat sheet
===========

This page aims to provide a quick overview of the most used commands.


Device
======

For more, see :ref:`device <parsec-cli-device>`

.. code-block:: shell

    parsec-cli device forget-local -d <DEVICE_ID>
    parsec-cli device list
    parsec-cli device change-authentication -d <DEVICE_ID> --<NEW_AUTH_METHOD>
    parsec-cli device export-recovery-device <DEST> -d <DEVICE_ID>
    parsec-cli device import-recovery-device --input <RECOVERY_FILE> --label <NEW_DEVICE_LABEL>



Invite
======

For more, see :ref:`invite <parsec-cli-invite>`

.. code-block:: shell

    parsec-cli invite cancel <INVITATION_TOKEN> -d <DEVICE_ID>
    parsec-cli invite claim '<INVITATION_LINK>'
    parsec-cli invite greet <INVITATION_TOKEN> -d <DEVICE_ID>
    parsec-cli invite list -d <DEVICE_ID>
    parsec-cli invite user <NEW_USER_EMAIL> -d <DEVICE_ID>
    parsec-cli invite device -d <DEVICE_ID>


User
====

For more, see :ref:`user <parsec-cli-user>`

.. code-block:: shell

    parsec-cli user list -d <DEVICE_ID>
    parsec-cli user revoke <USER_EMAIL> -d <DEVICE_ID>



Workspace
=========

For more, see :ref:`workspace <parsec-cli-workspace>`

.. code-block:: shell

    parsec-cli workspace list-users -d <DEVICE_ID> -w <WORKSPACE_ID>
    parsec-cli workspace archive -d <DEVICE_ID> -w <WORKSPACE_ID> --<STATUS>
    parsec-cli workspace create -d <DEVICE_ID> <NEW_WORKSPACE_NAME>
    parsec-cli workspace list -d <DEVICE_ID>
    parsec-cli workspace import -d <DEVICE_ID> -w <WORKSPACE_ID> <SOURCE> <DEST>
    parsec-cli workspace share -d <DEVICE_ID> -w <WORKSPACE_ID> -u <USER_ID> -r <ROLE>
    parsec-cli ls -d <DEVICE_ID> -w <WORKSPACE_ID>
    parsec-cli rm -d <DEVICE_ID> -w <WORKSPACE_ID> <PATH>
