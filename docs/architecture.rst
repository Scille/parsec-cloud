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

The Parsec server only requires a PostgreSQL DB for metadata (that is encrypted using devices keys for the most part) and an Amazon S3 or OpenStack Swift object storage for data blobs (that are all encrypted using Workspaces keys, that never left users’ devices). Redundancy using multiple cloud providers is possible.


Parsec security model
=====================

PARSEC secures sensitive data before they are stored on public clouds, proceeding in 3 steps :

- Splitting of files in blocks before encryption;
- Encryption of each block with a different symmetric key (BLOCK_ENC_KEY);
- Encryption of the metadata (tree structure, composition of files, multiple BLOCK_ENC_KEY, sharing information) with the private key of the user (USER_ENC_S_KEY).



Separation of the actors
************************

- User : represents a natural person in Parsec. An user owns an asymmetric key (USER_ENC_S_KEY / USER_ENC_P_KEY) that enables him to encrypt data for him alone, like his User Manifest (see below).
- The Workstation : the physical support -- desktop or mobile computer.
- Device : it is through a Device that the user accesses Parsec. Each user potentially has multiple devices (e.g. one for his desktop and one for his laptop). Each terminal owns it's own asymmetric signature key (DEVICE_SIG_S_KEY / DEVICE_SIG_P_KEY) enabling him to sign the modification he has made.


Parsec data model
*****************

- File Manifest : contains the name of the file, the list of block composing it and the associated BLOCK_ENC_KEY.
- Folder Manifest : index containing a set of entries, each entry being a File Manifest or another Folder Manifest.
- Workspace Manifest : index similar to the Folder Manifest, but that can be shared between multiple users.
- User Manifest : root index of each user containing the Workspaces Manifests shared with him.


Data sharing model
******************

- Workspace : a set of users sharing a trust perimeter. Parsec do the sharing of sensitive data by encrypting the Workspace Key (WS_ENC_KEY) using the key of the receiver of that data (USER_ENC_P_KEY) -- that step is repeated for each receiver.
- Organization : a set of Workspaces and a set of Users members of that organization. The access to a workspace can only be awarded to members of the organization. Two distincts organizations can't share the same Workspace.
