.. _doc_userguide_share_data:

Share data
==========

Create & share workspace
------------------------

In Parsec, data are stored into workspaces, each workspace having it own policy
for read and write access.

So before adding data to Parsec we must create a workspace:

.. image:: share_workspace.gif
    :align: center
    :alt: Sharing workspace process

The creator of the workspace automatically gets the ``Owner`` role, as shown
above it can then share the workspace with other users.

Regarding the differents sharing roles:

- Reader: Has only read-only access to the workspace
- Contributor: Has read&write access
- Manager: same access as Contributor, but can invite give
  Reader and Contributor roles to other users.
- Owner: same as Manager, but can also give Manager and Owner roles to other users.
  On top of that, owners are responsible for maintenance tasks such as
  :ref:`workspace re-encryption <doc_userguide_revoke_user_workspace_re_encryption>` or
  history garbage collector.

.. warning::

    Just like a user with a single device is bad because there is no fallback if
    something happens to it, having a workspace with a single user is dangerous !

    Strong cryptographic security prevent from data recovery if the user is
    lost or cannot log in. For this reason it's better to share the workspace
    with other users.

Upload data
-----------

Once the workspace created, it should appear in the file explorer as a regular
folder.

.. image:: upload_files.gif
    :align: center
    :alt: Working with files in the workspace

As you can see data copied from file explorer alos appear into the Parsec
client. On top of that the Parsec client also display the current synchronization
state of each file (showing wether the modification are only present locally or
if they have been synced with the server and hence are visible by everybody with
access to the workspace).

.. note::

    Parsec client can work while offline (however only the data present in
    local are available), synchronization will occures automatically as soon as
    the connection with the server is achieved.
