.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_architecture:

============
Architecture
============


Overview
========

Parsec is divided between a client (responsible for exposing data to the user and providing an encryption layer) and a server (storing the encrypted data and notifying clients about other users activity such as data modification or new sharing).

.. figure:: figures/architecture_diagram.svg
    :align: center
    :alt: Parsec single server, multi organizations showcase

    Parsec single server, multi organizations showcase

The Parsec server only requires a PostgreSQL DB for metadata (that is encrypted using devices keys for the most part) and an Amazon S3 or OpenStack Swift object storage for data blobs (that are encrypted using Workspaces keys which never leave users’ devices). Redundancy using multiple cloud providers is possible.


Parsec security model
=====================

Parsec secures sensitive data before they are stored on public clouds, proceeding in 3 steps :

- Splitting of files in blocks before encryption;
- Encryption of each block with a different symmetric key (BLOCK_ENC_KEY);
- Encryption of the metadata (tree structure, composition of files, multiple BLOCK_ENC_KEY, sharing information) with the private key of the user (USER_ENC_S_KEY).


Separation of actors
********************

- User: represents a natural person in Parsec. A user owns an asymmetric key (USER_ENC_S_KEY/USER_ENC_P_KEY) enabling the encryption of data intended only for the user, like its User Manifest (see below).
- The Workstation: the physical support -- desktop or mobile computer.
- Device: it is through a Device that the user accesses Parsec. A user can have multiple devices (e.g. a desktop and a laptop). Each device has its own asymmetric signature key (DEVICE_SIG_S_KEY/DEVICE_SIG_P_KEY) enabling signing modifications made by itself.


Parsec data model
*****************

- File Manifest: contains the file name, the list of block composing it and the associated BLOCK_ENC_KEY.
- Folder Manifest: index containing a set of entries, each entry being either a File Manifest or another Folder Manifest.
- Workspace Manifest: index similar to the Folder Manifest, that can also be shared between multiple users.
- User Manifest: root index of each user containing the Workspaces Manifests shared with the user.


Data sharing model
******************

- Workspace: a set of users sharing a trust perimeter. Parsec ensures sharing sensitive data by encrypting the Workspace Key (WS_ENC_KEY) using the key of the receiver of that data (USER_ENC_P_KEY) -- this step is repeated for each receiver.
- Organization: a set of Workspaces and a set of Users members of the organization. Workspace access can only be granted to members of the organization. A single Workspace cannot be shared between two distincts organizations.
