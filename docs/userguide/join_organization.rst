.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_join_organization:

Join an organization
====================

You have been invited to join an organization on Parsec! Before you get started,
please make sure you have :ref:`installed the Parsec app <doc_userguide_install_parsec>`.

1. Start the invitation process
-------------------------------

If you have received an **invitation email**, simply click the link in the email.
The Parsec app will start and guide you through the invitation process.

.. note::

  If Parsec does not start automatically, you can still join the organization
  with an **invitation link**. Paste the link on the login screen and click ``Join``.

2. Create a secure channel
--------------------------

In order to create a secure channel between the **guest** (you) and
the **host**, you will need to carry out a **code exchange** via an
**authentic channel** (such as a classic telephone call).

.. important::

  Authentic channels, such as classic phone calls, are not confidential,
  but they guarantee information is not tampered (you are sure that what
  your contact says is what you hear). This is why you should avoid using SMS.

When you are ready, click ``I understand!`` and wait for the host to start
the process on its side. You will both need to be connected to Parsec at the
same time.

1. From the list, select the code provided by your host.
2. Provide your code and wait for the host to select it.

.. |host_code_get| image:: screens/join_organization_host_code_get.png
    :align: top
    :width: 45 %
    :alt: List of codes to select the host code

.. |host_code_share| image:: screens/join_organization_guest_code_share.png
    :align: top
    :width: 45 %
    :alt: Your guest coded to be provided to the host

|host_code_get|  |host_code_share|

.. caution::

  If the code provided by you or by the host is not present in the list,
  it means there is a very high probability that one of you is victim of a
  `Man-in-the-middle attack <https://en.wikipedia.org/wiki/Man-in-the-middle_attack>`_.
  You can read more about that process at the :ref:`Cryptography <doc_cryptography>` section.


3. Set up your account
----------------------

Finally, enter your **contact details** and choose the preferred **authentication method**
(this can be changed later from your profile).
