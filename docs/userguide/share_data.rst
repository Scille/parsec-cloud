.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_userguide_share_data:

Share data
==========

Create & share workspace
------------------------

In Parsec, data are stored into workspaces, each workspace having its own policy
for read and write access.

So before adding data to Parsec we must create a workspace:

.. image:: screens/create_workspace.png
    :align: center
    :alt: Creating workspace process

The creator of the workspace automatically gets the ``Owner`` role, as shown
above, and can then share the workspace with other users.

.. image:: screens/share_workspace.png
    :align: center
    :alt: Sharing workspace process

Regarding the different sharing roles:

- Reader: has read-only access to the workspace
- Contributor: has read and write access
- Manager: same as Contributor and can also Reader and Contributor roles to other users.
- Owner: same as Manager and can also give Manager and Owner roles to other users.
  In addition to this, Owners are responsible for maintenance tasks such as
  :ref:`workspace re-encryption <doc_userguide_revoke_user_workspace_re_encryption>`.

.. warning::

    Just like a user with a single device is bad because there is no fall-back if something happens to it, having a workspace with a single user is dangerous.

    Strong cryptographic security prevent data recovery if the user is lost or cannot log in. For this reason it is better to share the workspace with other users.

Upload data
-----------

Once the workspace is created, it appears in the file explorer as a regular folder.

.. note::

    Although workspaces are mounted by default, they can be unmounted or mounted back using the toggle at the bottom left of the workspace card. When a workspace is unmounted, his data are not accessible in Parsec, and it is not reachable through the regular file explorer of the computer.

    .. image:: screens/workspace_unmounted_mounted.png
        :align: center
        :alt: workspaces unmounted and mounted

Parsec also proposes its own file manager, accessible when clicking on a mounted workspace.

.. image:: screens/parsec_file_explorer.png
    :align: center
    :alt: Parsec in file explorer

Data copied from file explorer also appear in the Parsec client. In addition, the Parsec client also displays the current synchronization state of each file (showing whether the modifications are only present locally or they have been synced with the server and hence are visible by everyone with access to the workspace).

.. note::

    Parsec client can work while offline (however only data present locally
    are available), synchronization will occur automatically as soon as the
    connection with the server is established.
