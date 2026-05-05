.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_architecture:

========================
Application Architecture
========================

Parsec allows you to easily and securely share your data in the cloud with complete
confidentiality thanks to client-side end-to-end encryption and zero-trust security.

Overview
========

.. figure:: figures/architecture_diagram.svg
    :align: center
    :alt: A single Parsec Server managing access and data for multiple Organizations

    A single Parsec Server managing access and data for multiple Organizations


Key Parsec components
=====================

The following key software components are involved in Parsec:

💻 Parsec client application (desktop, web)
  Responsible for data encryption and signing, as well as all security functions related to the user
  data to be protected. User data is fully encrypted client-side.

⚙️ Parsec Server
  Responsible for the access control layer as well as managing storage for encrypted data and metadata.
  The Parsec Server can be self-hosted, providing IT administrators full control over security,
  integrations and customization.

⛁ PostgreSQL DB
  Responsible for storing encrypted metadata.

⛁ Object Storage (such as OpenStack Swift, Amazon S3)
  Responsible for storing the encrypted data blocks.


Key Parsec concepts
===================

👤 User
  The person accessing an Organization.

💻 Workstation
  The physical support to access the Organization (such as desktop computer, laptop or mobile phone)

🔐 Device
  The logical support to access the Organization.

  A Device is an encrypted file stored locally (in the File System or in the Web Browser storage)
  containing the keys allowing the User to access the Organization and sign its changes.

  The User has a Device for each workstation, but may also have have multiple Devices for a single
  workstation (this is the case if the user access the Organization from both the Desktop and Web applications).

📁 Workspace
  The trust perimeter for sharing data.

  Data added to a Workspace is only shared and accessible to Users of the Workspace.

  User access rights to a Workspace is managed by the Workspace owner or designated managers.

  See :ref:`Parsec Workspaces <doc_userguide_parsec_workspaces>`.

👥 Organization
  The trust perimeter for User management.

  It can be seen as a User Directory of trusted Users who have been invited and granted access to the Organization.

  Depending on their profile, Users can see the list of all Users from the Organization.

  In contrast, only Users having access to a Workspace know about its existence (not even
  Administrators can know of its existence if the Workspace have not been shared with them)

  Users and Workspace always belong to a single Organization.

Security Model
==============

Parsec is designed to protect data security at all costs, regardless of who controls the Parsec Server,
the PostgreSQL database or the Object Storage.

In this sense, no trust is placed in the Parsec Server in order to reach Zero-Trust level of confidentiality.
Only users have the keys to decipher data stored in Parsec. Take a look at
:ref:`Parsec Zero-Trust Model <doc_userguide_introduction_zero_trust>` for more detail.

Data security perimeter
-----------------------

From a data security perspective, there are three zones:

🔐 Trusted zone: User Workstation
  The trusted zone is limited to the user's workstation on which the Parsec client application is installed.

  Within an organization, the workstations should comply with required regulations, including being up to
  date with operating system security patches and adhering to the organization's security policy (eg. PSSI).

✋ Non-trusted zone: Parsec Server
  The Parsec Server is hosted either in the cloud or on-premise and therefore is considered a less secure
  environment.

  As such, it can be queried from any workstation connected to the Internet.

  Parsec Server has no knowledge of the data being exchanged but does know which user is accessing it and
  thus can restrict write access to users with sufficient permissions.

✋ Non-trusted zone: Object Storage
  The encrypted blocks containing the user data to be protected are stored in the cloud and thus also
  in a less secure zone.

  Client applications access data via the Parsec Server, which applies a layer of access control.


Client-side encryption
----------------------

Parsec secures your data before it is stored in the cloud (or on-premise) Object Storage.

This is done in 3 steps:

#. Split each file into blocks before encryption;
#. Encrypt each block using the key shared by users having access to this data;
#. Encrypt metadata (such as directory structure and sharing information) using the key shared by users having access to this data;
