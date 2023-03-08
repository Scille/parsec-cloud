.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_roles:

==================
Profiles and Roles
==================


User management
===============

There are two profiles for user management:

The User profile enables
************************

- the creation of Workspaces;
- the data management (creation, modification, history, integrity information);
- the data sharing inside a Workspace;
- the creation of its own Devices.


The Administrator profile enables
*********************************

- the same roles of the User profile;
- the creation of Users (either with Administrator profile or User profile);
- the deletion of Users regardless of their profile.

.. note::

    It is not possible to modify the user's profile: an Administrator will remain an Administrator; a User will remain a User. Therefore, the user must be deleted and then re-created in order to allocate him the new profile.


Device management
=================

Only the user, regardless the profile, can create devices for itself. Devices are clones. The number of devices created by a user is usually small.

When a user is deleted, all the associated devices are deleted. It is not possible to delete a single device.


Management of Workspaces and Documents
======================================

There are four roles having different rights in a workspace:

1. Reader: it has **read** access.
2. Contributor: it has **read** and **write** access.
3. Manager: it has **read** and **write** access and can also **grant roles** with the exception of the Owner role.
4. Owner: it has **read** and **write** access and can also **grant roles** including the Owner role. It can also **trigger a complete Workspace re-encryption** in case of a prior user deletion (for example following the compromise of a user's device). A Workspace can have multiple Owners. The Workspace creator has the Owner role by default.
