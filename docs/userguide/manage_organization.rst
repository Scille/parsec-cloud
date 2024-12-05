.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_manage_organization:

Manage your organization
========================

Now that our organization is ready, we can start inviting new membres.

In Parsec, inviting a new member to join your organization is a critical operation
that aims at building trust towards the member and the device that will be used to
connect to Parsec.

The process requires both the guest (the invited member) and the host (you) to be
connected to the Parsec server at the same time.

Invite a member
-------------

1. Go to ``Manage my organization``
2. Click ``Invite a member`` and enter its email address to send an **invitation link**

  - A yellow button will appear at the top, informing you that you have an
    invitation waiting to be validated.

  .. image:: screens/new_user_invitation_sent.png
      :align: center
      :alt: Pending invitations

3. Ask the guest to open the invitation link and follow the steps to
   :ref:`Join an organization <doc_userguide_join_organization>`.

4. Click on ``Greet`` and wait for the guest to proceed to the
   :ref:`Token exchange <doc_userguide_join_organization_token_exchange>`.

5. Confirm the member details and set its :ref:`member profile <doc_userguide_manage_organization_profiles>`.


Revoke a member
-------------

Revoking a member will remove its access rights to the organization. This is
specially required when:

- the member device has been compromised or lost
- the member is no longer part of the organization

1. Go to ``Manage my organization``
2. Select ``Revoke this member``.
3. Click ``Revoke`` to confirm (**this action cannot be undone**)

.. caution::

  Revocation is irreversible. If the revoked member needs to re-join the organization,
  it will need to go through the standard procedure to :ref:`join an organization <doc_userguide_join_organization>`

.. note::

  It is not possible to revoke a single member device. If one of the member's
  devices has been compromised, the member and all its devices need to be revoked.
  This is intended because the compromised device has the knowledge of some
  cryptographic secrets shared among all the member's devices.


.. _doc_userguide_manage_organization_profiles:

Member profiles
-------------

The **member profiles** defines what the member is allowed to do within the
organization.

External profile
^^^^^^^^^^^^^^^^

This profile allows a member to:

- **Collaborate in workspaces**

.. note::

  Members with the External profile are not allowed to see the name and
  email of other member within the organization.

Standard profile
^^^^^^^^^^^^^^^^

This profile allows a member to:

- Collaborate in workspaces.
- **Create and share workspaces.**
- **See the name and email of other member within the organization.**

Administrator profile
^^^^^^^^^^^^^^^^^^^^^

This profile allows a member to:

- Collaborate in workspaces.
- Create and share workspaces.
- See the name and email of other members within the organization.
- **Invite new members to join the organization, and set their profile.**
- **Remove members from the organization, regardless of their profile.**
