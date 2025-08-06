.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_manage_organization:

Manage your organization
========================

Now that our organization is ready, we can start inviting new users.

In Parsec, inviting a new user to join your organization is a critical operation
that aims at building trust towards the user and the device that will be used to
connect to Parsec.

The process requires both the guest (the invited user) and the host (you) to be
connected to the Parsec server at the same time.

Invite a user
-------------

1. In the sidebar, go to ``Users``
2. Click ``Invite a user`` and enter its email address to send an **invitation link**

  - A blue button will appear at the top, informing you that you have an
    invitation waiting to be validated.

  .. image:: screens/new_user_invitation_sent.png
      :align: center
      :alt: Pending invitations

3. Ask the guest to open the invitation link and follow the steps to
   :ref:`Join an organization <doc_userguide_join_organization>`.

4. Click on ``Greet`` and wait for the guest to proceed to the
   :ref:`Token exchange <doc_userguide_join_organization_token_exchange>`.

5. Confirm the user details and set its :ref:`user profile <doc_userguide_manage_organization_profiles>`.


Revoke a user
-------------

Revoking a user will remove its access rights to the organization. This is
specially required when:

- the user device has been compromised or lost
- the user is no longer part of the organization

1. In the sidebar, go to ``Users``
2. Select ``Revoke this user``.
3. Click ``Revoke`` to confirm (**this action cannot be undone**)

.. caution::

  Revocation is irreversible. If the revoked user needs to re-join the organization,
  it will need to go through the standard procedure to :ref:`join an organization <doc_userguide_join_organization>`

.. note::

  It is not possible to revoke a single user device. If one of the user's
  devices has been compromised, the user and all its devices need to be revoked.
  This is intended because the compromised device has the knowledge of some
  cryptographic secrets shared among all the user's devices.


.. _doc_userguide_manage_organization_profiles:

User profiles
-------------

The **user profiles** defines what the user is allowed to do within the
organization.

External profile
^^^^^^^^^^^^^^^^

This profile allows a user to:

- **Collaborate in workspaces**

.. note::

  Users with the External profile are not allowed to see the name and
  email of other users within the organization.

Member profile
^^^^^^^^^^^^^^

This profile allows a user to:

- Collaborate in workspaces.
- **Create and share workspaces.**
- **See the name and email of other users within the organization.**

Administrator profile
^^^^^^^^^^^^^^^^^^^^^

This profile allows a user to:

- Collaborate in workspaces.
- Create and share workspaces.
- See the name and email of other users within the organization.
- **Invite new users to join the organization, and set their profile.**
- **Remove users from the organization, regardless of their profile.**

Change the profile
------------------

If you need to change the profile of a user, go to ``Users`` (in the sidebar) in the upper-left corner. Either select the user or right-click on them and select ``Change profile``.

.. image:: screens/change_profile.png
    :align: center
    :alt: change profile screenshot
    :width: 800

Select the user's new profile and click on ``Change``.

.. image:: screens/change_profile_dialog.png
    :align: center
    :alt: change profile dialog screenshot
    :width: 400

You can change the profiles of multiple users at once by selecting them and clicking on ``Change profiles``.

.. image:: screens/change_multiple_profiles.png
    :align: center
    :alt: change multiple profiles screenshot
    :width: 800

You cannot change the profile from and to `External`. If there are users with `External` profile among the selected users, the dialog will warn you that their profile will not be affected.

.. image:: screens/change_profile_external_warn.png
    :align: center
    :alt: change profile external warning screenshot
    :width: 400
