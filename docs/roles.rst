.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_roles:

==================
Profiles and Roles
==================


User management
===============

There are two profiles for user management :

The User profile enables
************************

- the creation of a Workspace;
- the management of documentation (creation, modification, history, integrity information);
- the sharing of data inside a Workspace;
- the creation of its own devices.


The Administrator profile enables
*********************************

- every roles of the User profile;
- the creation of other users (whether Administrator or User)
- the deletion of whichever User whatever his profile.

.. note::

    It isn't possible to modify a profile : an Administrator will stay an Administrator; an User will stay an User. In that case of modification, it is required, after deletion, to create a new user and allocate him the new role.


Device management
=================

Only the user, whatever his profile, can create an undetermined number of devices for himself. Every devices are clones. The number of devices by user is usually small. The deletion of on device only is not possible. When a user is deleted, all his devices are deleted.


Management of Workspaces and Documents
======================================

There are four roles having different rights in a workspace :

1. Reader : he only has read access.
2. Contributor : he only has read and write access.
3. Manager : he can give rights except the one of owner. He has read and write access.
4. Owner : he can give rights including the one of Owner. There can be multiple owners. The Workspace creator is Owner by default. He has read and write access. Only an Owner can trigger a complete Workspace re-encryption in case of the prior deletion of an user (for example consecutive to the compromise of a device or an user).
