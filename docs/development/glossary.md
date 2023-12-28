<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Glossary

## General terms

### Organization

It represents an organized structure (team, company, government agency, business unit, etc.) that designates a group of *users*. It is created by an initial *Administrator*, who can then invite other users to join the Organization. Within the Organization, users can create and share data via *Workspaces*.

### Workspace

It represents the minimal unit in which data is shared. Workspaces contains folders and files and can be shared with other users within an Organization.

### User

An individual with the ability to authenticate to Parsec and manage Workspace shares within an Organization.

### Device

It represents the device (computer, mobile phone) registered by the user to access an Organization. Since the device is linked to *specific Organization*, it may be more clearly seen as keys enabling access to the Organization (i.e. the user will have at least one device per Organization).

## User Profile

It defines the user rights *within an Organization* and is set during the onboarding procedure.

### Standard profile

It allows to create and share Workspaces with other users within the Organization as well as to access data within Workspaces (depending on its *workspace role*). Essentially it enables all Parsec features except for user management.

### Administrator profile

In addition to all the functionalities allowed by the Standard profile, the Administrator profile enables user management. That is, the ability to invite and revoke other users in the Organization.

### Outsider profile

It allows to collaborate (read, write) on Workspaces they are invited to, but does not allow to create or share Workspaces.

The Outsider profile does not allow to see personal information (such as *email*, *user name* or *device name*) of other users.

## Workspace Role

It defines the user rights *within a Workspace*. Is first set for the Workspace creator or when the Workspace is shared to a user.

Since the role is linked to the Workspace, a user can have different roles on different Workspaces.

### Reader

The user has **read** access in the Workspace.

### Contributor

The user has **read** and **write** access in the Workspace.

### Manager

The user has **read** and **write** access in the Workspace. It can also **invite** other users to the Workspace as well as **demote** or **promote** other users roles (up to Manager) on the Workspace.

### Owner

The user has **read** and **write** access in the Workspace. It can also **invite** other users to the Workspace as well as **demote** or **promote** other users roles (up to Owner) on the Workspace.

The user can **trigger a Workspace re-encryption** in case of user deletion or compromise of a user's device.

A Workspace can have multiple owners. The Workspace creator has the Owner role by default.

## Operations

### Invitation

The action of inviting a new user to join an existing Organization. Tipically the user receive an email to start the Onboarding process.

<!-- TODO: Confirm following terms to avoid -->
* Terms to avoid: *Enroll*, *Enlist*, *Onboard*, *Greet*, *Join*.

> Note that *invitation* is specifically used for users. For devices, the term *adding* should be used instead.

### Onboarding

The Onboarding process starts from an invitation. It consists in several steps between the inviter (*greeter*) and the invitee (*claimer*) for the latter to join an Organization.

Both the greeter and the claimer must exchange information in order to finalize the onboarding process.

### Joining

A user joins an Organization when the Onboarding process is finalized successfully.

## Internals

### Parsec Server

The **Parsec server** is the application hosted on a remote computer interacting with the S3 data storage and the metadata PostgreSQL database.

* Terms to avoid: *Backend*

## Other terms

### Passphrase

Similar to a password but instead of being composed of possibly random characters (letters, digits, ...), it is composed of *words* (which can also include digits and special characters like a password).
