.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_userguide_revoke_user:

Revoking users
==============

Revoking a user is the operation that aims at removing his access rights to the organization. This is needed when:

- the user is no longer needed (e.g. a person leaving his company)
- the user has been compromised

.. note::

    It is not possible to revoke a single device, only the entire user can be revoked.
    This is intended because the compromised device has the knowledge of some
    cryptographic secrets shared among all user's devices.


Revocation
----------

Revocation is done from the client, this option is accessible by right-clicking on a user:

.. image:: screens/revoke_user.png
    :align: center
    :alt: Revoking user process

.. note::

    - Only an administrator of the organization can revoke a user
    - Revocation is irreversible!

Workspace re-encryption
-----------------------
.. _doc_userguide_revoke_user_workspace_re_encryption:

Once a user is revoked, its devices are no longer allowed to connect to the Parsec server hosting the organization. In practice, this means the user won't be able to make any changes or consult the data he had access to.

However, from a cryptographic point of view, the revoked user still knows the encryption keys of the workspaces that have been shared with them. For this reason, those workspaces must be re-encrypted to ensure data security.

Once a user is revoked, each owner of a previously shared workspace will be notified that a re-encryption operation is required. Owners can then perform re-encryption right away or wait to do it later (for example if multiple user are getting revoked in one batch).

.. image:: screens/reencrypt_workspace.png
    :align: center
    :alt: Workspace re-encryption process

.. note::

    - During re-encryption, a workspace cannot be synchronized
    - Re-encryption is fairly quick since only metadata are re-encrypted
