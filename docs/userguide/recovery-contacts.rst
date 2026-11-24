.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_recovery_contacts:

Recovery contacts
=================

**Recovery contacts** are members of your organization you choose to help you
get back your access to Parsec if you ever lose it, for example if you forget
your password or lose your computer.

Unlike a :ref:`recovery file <doc_userguide_recovery_files>`, there is nothing
to store, neither for you nor for your recovery contacts: Parsec takes care of
everything automatically. Your recovery contacts only need to be available to
help you when the time comes.

How it works
------------

When you set up your recovery contacts, Parsec splits the information needed to
recover your access into several parts and gives one part to each of your
recovery contacts.

- You choose **at least 3** recovery contacts.
- Any **2** of them are enough to help you recover your access.
- A single recovery contact can **never** recover your access alone.

This way, you can still recover your access even if one of your recovery
contacts is unavailable (on holiday, has left the organization, etc.).

To recover your access, you will get in touch with your recovery contacts (by
email, by phone or in person) and perform a short **code exchange** with each of
them. Each successful code exchange counts as an **approval**. Once you have 2
approvals, you can access Parsec again.

.. note::

  Recovery contacts are specific to an organization. If you are a member of
  several organizations, you need to set up recovery contacts in each of them.

.. caution::

  Choose people you trust and that you can easily reach. Your recovery contacts
  will be able to help you recover your access, so make sure they would not do
  it without your consent.

.. note::

  This feature is based on an algorithm called
  `Shamir's secret sharing <https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing>`_.
  You do not need to know anything about it to use recovery contacts.

.. _doc_userguide_recovery_contacts_setup:

Set up your recovery contacts
-----------------------------

1. Click on your name on the main menu (top-right) and select ``Access recovery``.
2. In the ``Recovery contacts`` section, click ``Set up my recovery contacts``.

.. image:: screens/recovery_contacts_access_recovery_page.png
    :align: center
    :alt: Access recovery page

3. Read the explanation and click ``Choose my recovery contacts``.

.. image:: screens/recovery_contacts_setup_info.png
    :align: center
    :alt: Recovery contacts explanation

4. Search for the members you want to add, by name or email, and select at least
   3 of them.
5. Check the confirmation box and click ``Confirm my recovery contacts``.

.. image:: screens/recovery_contacts_setup_select.png
    :align: center
    :alt: Choosing recovery contacts

Your recovery contacts are now set up. You can see them at any time by clicking
``View my recovery contacts``.

.. image:: screens/recovery_contacts_setup_done.png
    :align: center
    :alt: Recovery contacts confirmed

.. tip::

  Let your recovery contacts know that you chose them, so they will not be
  surprised if you ask for their help one day.

.. note::

  - Users with the *External* profile cannot be chosen as recovery contacts.
  - If you have the *External* profile, you can only choose *Administrators*
    as your recovery contacts.
  - Your organization needs to have enough members to choose from. If it does
    not, Parsec will tell you how many members are required.

.. _doc_userguide_recovery_contacts_remove:

Remove or change your recovery contacts
---------------------------------------

1. Click on your name on the main menu (top-right) and select ``Access recovery``.
2. Click ``View my recovery contacts``, then ``Remove my recovery contacts``.
3. Confirm by clicking ``Remove recovery contacts``.

.. image:: screens/recovery_contacts_remove.png
    :align: center
    :alt: Removing recovery contacts

Your recovery contacts can no longer help you recover your access.

To change your recovery contacts, remove them and then
:ref:`set up new ones <doc_userguide_recovery_contacts_setup>`.

When a recovery contact is revoked
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

If one of your recovery contacts is revoked from the organization, they can no
longer help you. Parsec shows a warning at the top of the application and on the
``Access recovery`` page:

- ``Some recovery contacts have been revoked``: you can still recover your access
  with your remaining contacts, but we recommend choosing new ones.
- ``Your access can no longer be recovered``: too many recovery contacts have been
  revoked, fewer than 2 remain. You must remove your current recovery contacts and
  set up new ones.

.. image:: screens/recovery_contacts_revoked_warning.png
    :align: center
    :alt: Warning about revoked recovery contacts

.. _doc_userguide_recovery_contacts_others:

See who chose you as a recovery contact
---------------------------------------

Other members of your organization may have chosen you as one of their recovery
contacts. To see who they are:

1. Click on your name on the main menu (top-right) and select ``Access recovery``.
2. In the ``Recovery contacts`` section, click ``Assist as a recovery contact``.

.. image:: screens/recovery_contacts_assist_list.png
    :align: center
    :alt: List of people who chose you as a recovery contact

The list shows all the people you can help. If nobody chose you yet, the list is
empty.

.. note::

  Users with the *External* profile cannot be recovery contacts, so this button is
  not available to them.

.. _doc_userguide_recovery_contacts_assist:

Help someone recover their access
---------------------------------

When someone who chose you as a recovery contact loses their access, they will
ask for your help. You both need to be available at the same time to perform the
code exchange, for example during a call or in person.

1. Click on your name on the main menu (top-right) and select ``Access recovery``,
   then click ``Assist as a recovery contact``.
2. Find the person in the list.

   - Click ``Start`` to begin the recovery. The person will receive a
     **recovery link** by email.
   - If the email does not reach them, click ``Copy link`` and send them the
     recovery link by any other means.

.. important::

  Before helping, make sure that the person asking for help really is who they
  claim to be, for example by calling them or meeting them in person.

3. Wait for the person to open the recovery link and select you in their list of
   recovery contacts.

.. image:: screens/recovery_contacts_assist_wait.png
    :align: center
    :alt: Waiting for the person recovering their access

4. Give them the code displayed on your screen, then select the code they give
   you.

.. image:: screens/recovery_contacts_assist_codes.png
    :align: center
    :alt: Code exchange with the person recovering their access

.. caution::

  If the code the person gives you is not in the list, stop the process: someone
  may be trying to take over this person's access.

5. Your approval is sent. The person may still need the approval of another
   recovery contact before recovering their access.

.. image:: screens/recovery_contacts_assist_success.png
    :align: center
    :alt: Recovery approved

.. _doc_userguide_recovery_contacts_recover:

Recover your access with your recovery contacts
-----------------------------------------------

If you lost your access to Parsec and you set up recovery contacts, you can ask
them to help you.

1. Get in touch with one of your recovery contacts (by email, by phone or in
   person) and ask them to
   :ref:`start your recovery <doc_userguide_recovery_contacts_assist>`. They
   will send you a **recovery link**.
2. Open Parsec and click ``I lost my access`` (top-right of the welcome page), or
   ``Forgot your password?`` when trying to log in.
3. Select ``Recovery contacts`` and click ``Continue with this recovery method``.

.. image:: screens/recovery_contacts_recover_method.png
    :align: center
    :alt: Choose recovery contacts as recovery method

4. Parsec explains the steps of the recovery. Click ``Next`` to go through them,
   then ``Got it``.

.. image:: screens/recovery_contacts_recover_instructions.png
    :align: center
    :alt: Recovery instructions

5. Paste the recovery link you received and click ``Start recovery``.

.. image:: screens/recovery_contacts_recover_link.png
    :align: center
    :alt: Enter the recovery link

.. tip::

  You can also simply click on the recovery link you received: Parsec will open
  and the recovery will start directly.

6. The list of your recovery contacts is displayed, along with the number of
   approvals you still need. Ask one of them to click ``Start`` on their side
   (see :ref:`Help someone recover their access <doc_userguide_recovery_contacts_assist>`),
   then click ``Choose`` next to their name.

.. image:: screens/recovery_contacts_recover_choose.png
    :align: center
    :alt: Choose a recovery contact

7. Select the code your recovery contact gives you, then give them the code
   displayed on your screen.

.. image:: screens/recovery_contacts_recover_codes.png
    :align: center
    :alt: Code exchange with a recovery contact

.. caution::

  If the code your recovery contact gives you is not in the list, stop the
  process: someone may be trying to interfere with your recovery.

8. Repeat steps 6 and 7 with another recovery contact until you have the required
   number of approvals.
9. Once the validation is complete, click ``Set my authentication method`` and
   choose how you want to log in from now on.

.. image:: screens/recovery_contacts_recover_done.png
    :align: center
    :alt: Recovery approved by recovery contacts

Your access has been recovered! You can now log in to your organization.
