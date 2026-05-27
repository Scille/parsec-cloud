.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting_cannot_login_organization:

Cannot login to my Organization
===============================

If you cannot login to your organization in Parsec, here is a list of things you can try to recover access.

Check your login method
-----------------------

Depending on the method you use to login, there may be some preliminary checks to perform.

Login with password
^^^^^^^^^^^^^^^^^^^

If you login with **password**, double-check it to make sure you are using the right one:

- If you are a SaaS customer, the password to login to your organization is not the same as the one used for your *Customer Area*.
- If you have set up multiple devices, each device may have a different password to login to the same organization
  (this also holds you are using both Parsec web and desktop application)

Login with system authentication
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you login with **system authentication**, checks will depend on your specific operating system and/or desktop manager.
In any case, check that the system keyring exists and that is unlocked. Depending on your system, a dialog may appear
asking you to unlock the keyring in order to allow Parsec application to access it.

Login with Single Sign-On (SSO)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you login with **SSO** (such as ProConnect), the service may be temporarily unavailable or there may be
network issues to reach the server. Check your Internet connection and try again later.

Login with SmartCard (PKI)
^^^^^^^^^^^^^^^^^^^^^^^^^^

If you login with **SmartCard**, double-check that you are choosing the right certificate and that it has not expired.


Check your MFA setup
--------------------

If you have enabled MFA, Parsec will require the **6-digit** code shown in your authenticator app to login.

Parsec application needs to reach the Parsec server in order to login with MFA. Check your Internet connection and make sure you are online.

Refer to the :ref:`MFA <doc_userguide_mfa_reset>` section to reset your MFA setup.


Recover your session using a Recovery file
------------------------------------------

If you've created a Recovery file, it should allow you to recover access to your organization.

Refer to the :ref:`Recovery files<doc_userguide_recovery_files>` section for more detail.


Contact an administrator
------------------------

If none of the above worked, you should contact an administrator for help.

To address the issue as efficiently as possible, please include the following information:

- The Parsec version you are using (displayed in the About dialog)
- The method you use to login (password, ...) and if you have enabled MFA or not
- Any error message displayed or unexpected behaviour observed (include screenshots if you can)
- A brief description of steps you follow to login


Consider revocation and re-joining the organization
---------------------------------------------------

For security reasons, the keys needed to authenticate you with Parsec never leave your device.
This means that it is not possible to recover a forgotten password.

As a last resort, the administrator may need to revoked your access and re-invite you to join the organization.
