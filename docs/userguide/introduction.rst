.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_introduction:


Introduction
============

Parsec allows you to easily and securely share your data in the cloud with complete
confidentiality thanks to client-side end-to-end encryption and zero-trust security.

.. image:: screens/parsec_snapshot.png
    :align: center

Who is Parsec for?
------------------

Parsec is designed for organizations and individuals seeking to build secure workspaces
to share sensitive and confidential data via a private or public cloud on the Internet.

Parsec can be used by government agencies, SMEs, start-ups, consulting firms, R&D entities,
or independent professionals, providing zero-trust data sharing with a guarantee of
confidentiality, integrity, authenticity and data history.

The Zero-Trust model
--------------------

Parsec is the result of a strategic vision:

  *Secure data sharing is a major challenge for the coming years and network perimeter protection
  alone is no longer sufficient to meet data security needs, which are much broader in scope:
  mobility of actors, security in the cloud, integrity & confidentiality guarantees and accountability.*

The security strategy must become **Zero-Trust**, essentially meaning that no trust is placed in the
server to allow secure data access and sharing.

Every user, every terminal and every network flow must be authenticated and authorized at all times.
Your data is secured *before* it is stored in the cloud with a multi-step process involving file
splitting, data & metadata encryption.

.. important::

  Parsec guarantees data integrity & confidentiality even though it does not guarantee that users
  will have access to the latest version of the data.

  In the event of a server compromise, an attacker taking control of Parsec server has little control
  over what they can do with your data. For example, they can roll back the database without users
  realizing that they are not seeing the latest version. However, Parsec guarantees that the data cannot be
  accessed or altered in any way by the attacker or by anyone who has access to the server.

Key features
------------

📂 Secure virtual drive on your computer
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A secure virtual drive on your computer allows you to access your data
stored in Parsec using your usual software, just as you would on your
local hard drive.

🗓️ Never lose your data
^^^^^^^^^^^^^^^^^^^^^^^

Your data is encrypted and synchronized with Parsec regularly and incrementally.
You can browse data history and restore previous versions at any time.

🔐 Client-side cryptographic security
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Client-side `E2EE <https://en.wikipedia.org/wiki/End-to-end_encryption>`_ ensures
your data is only accessed by you and the people you share it with.
Encryption key rotation is also managed automatically on the client side to ensure
that users whose access has been revoked cannot decrypt future data.

✒️ Cryptographic signatures
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Each change is automatically signed by its author enabling identification
in an unforgeable manner.

🤝 Simplified enrollment
^^^^^^^^^^^^^^^^^^^^^^^^

Invite users to securely join your organization via link and token code exchange.
Leverage your existing SSO or PKI to enable secure join requests.

🧑‍💻 Easy to self-host
^^^^^^^^^^^^^^^^^^^^^^^

You can easily deploy Parsec on your own infrastructure. Take a look at the
:ref:`Hosting guide <doc_hosting_intro>`.
