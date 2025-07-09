.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_security_best_practices:

Security best practices
***********************

This section contains security best practices addressed to all Parsec users, regardless of their
role or technical knowledge.

Parsec will always do its best to ensure your data is secure. These best practices describe what
you can do to protect your data, reducing even further the risk of data leaks or loss.

Keep in mind that risk can never be completely eliminated. Always assess the confidentiality needs
of the data you need to access and take precautionary measures accordingly. If in doubt, contact
the owner of the organization or a security expert.

How to prevent data loss
========================

What is data loss?
------------------

Data loss refer to the permanent and definitive loss of data, which can no longer be recovered by
no means (such as backup or administrator access).

This usually happens when only a single user has access to data and later on it loses access.

How Parsec helps me reducing the risk of data loss
--------------------------------------------------

In Parsec, you and only you are the owner of the keys to decrypt your data. These keys are never
stored by Parsec so that, even if the server is compromised, an attacker will never be able to
access your data.

This strong cryptographic security means that if you lose access to your computer, or cannot login
for any reason, there is no way to recover data that only you had access to.

Parsec offer important features to make sure you reduce the risk of data loss.

How can I reduce the risk of data loss?
----------------------------------------

To avoid losing access to your data you should always considering the following actions.

:ref:`Add multiple devices <doc_userguide_manage_devices>` to access your organization. You should
*set up at least up two devices* so that if you lose access to one device, you can still login with
the other one.

:ref:`Create a recovery file <doc_userguide_recovery_files>` to access your organization. A recovery file is
a special file that, together with its password, will allow you to connect to your organization even if you lose access
to all your devices. You should *always create a recovery file* and store it securely.

:ref:`Share your workspaces <doc_userguide_parsec_workspaces_share>` with other users. This simple step will ensure
that even if you lose access to all your devices AND you don't have a recovery file, there will be at least another
user that can share the data back to you.

How to prevent data leaks
=========================

What are data leaks?
--------------------

Data leaks refer to the unauthorized exposure or disclosure of sensitive information, often
resulting from hacking, accidental sharing, or poor security practices.

They can lead to identity theft and other cybercrimes, making it crucial for individuals and
organizations to protect their data effectively.

How Parsec helps me reducing the risk of data leaks
---------------------------------------------------

When you access your files locally, or open them with Parsec's integrated viewer, Parsec does not
actually store them in your disk. Instead, files are only maintained in the computer memory and
kept always up to date with with your Parsec cloud storage.

This security guarantee means that, even in case of critical events such as a sudden computer
shutdown, Parsec ensures that your files will never be persisted nor stored in your disk.

How can I reduce the risk of data leaks?
----------------------------------------

As soon you download a file from Parsec to your computer, or copy a file from a Parsec drive to a
directory in your computer, Parsec can no longer secure them.

This does not necessarily mean they are at risk, but they are certainly exposed. The risk of data
leak will greatly depend on multiple aspects such as your system configuration, the location where
the files will be stored, your company security policy, etc.

Keep in mind that some software, like Microsoft Word or Google Drive, may automatically upload your
files to the cloud without your necessarily being aware of it.

.. _userguide-verify-parsec-link:

How to verify a Parsec link
===========================

What is a Parsec link?
---------------------------

A Parsec link is a link that is used by the Parsec application, you use such link when (non-exhaustive):

- :ref:`You are invited to a new organization <doc_userguide_join_organization>`.
- :ref:`You add a new device <userguide-add-new-device>`.
- :ref:`You share a file <userguide-share-file>`.

What should I verify in a Parsec link?
--------------------------------------

A Parsec link is formatted like so:

.. code-block:: jinja

   parsec3://<SERVER>/<ORGANIZATION>?<PARAMETERS>

.. note::

   The `URL scheme <url-scheme_>`_ ``parsec3`` may be replaced by ``https`` for links received by e-mail.

.. _url-scheme: https://en.wikipedia.org/wiki/List_of_URI_schemes


When you receive a link, you just need to check if the ``SERVER`` and ``ORGANIZATION`` parts match your organization's server. This can be found on the "Information" page, when you login to your organization.

When joining an organization, you should contact an administrator of the organization to provide or confirm this information.

How to enable web browser storage
=================================

Parsec needs to securely store your access keys (device) on your web browser so you can login to your organizations.

This section explains how to enable storage for Parsec depending on your web browser.

Chrome
------

1. Go to the settings menu (three dots in the upper right corner).
2. Click on "Privacy and security".
3. Click on "Site settings".
4. Under "Content", click on "Third-party cookies".
5. Make sure "Allow third-party cookies" is enabled or add Parsec website `https://app.parsec.cloud` to the list of allowed sites in "Site allowed to use third-party cookies".

Firefox
-------

1. Go to the settings menu (horizontal lines in the upper right corner).
2. Click on "Settings" then "Privacy & Security".
3. In the "Cookies and Site Data" section, click on "Manage Exceptions...".
4. Add Parsec website `https://app.parsec.cloud` to the list of allowed sites and then click "Allow".

.. important::

  Please note that even if you enabled "Delete cookies and site data when Firefox is closed", the exception for Parsec website will **remain**.

Safari
------

1. Go to "Safari" in the menu bar and select "Preferences".
2. Click on the "Privacy" tab.
3. Ensure that "Block all cookies" is unchecked to allow sites to store data.
