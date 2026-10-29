.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting_web_cannot_be_invited_or_create_a_device:

Cannot Join an Organization or Add a Device on Web
--------------------------------------------------

When joining a Parsec organization or adding a device via Parsec web,
the last step of the operation may fail if Parsec is not permitted to access the web browser's storage.

When you first access Parsec on web, a message will ask you to grant persistent storage permission.
Follow the steps below to check whether the permission is already granted and, if not, to grant it.

.. _refresh-webapp:

1. Refresh Parsec web application (`F5` or `Ctrl+R` or `Command+R`)
2. Does it show you a prompt asking for persistent storage permission?

   If it shows you the following:

   .. image:: ./screens/firefox-asking-to-grant-persistent-storage.png

   Click on `Allow`.

.. _did-you-set-permission:

3. Did you previously set the permission?

   At the right side of the URL on Firefox, but on the left of the Shield icon 🛡️, there should be an icon.

   If you do not have another icon at the right of the URL beside the Shield icon, go to :ref:`step 4 <manually-ask-permission>`

   If you click on it, it will show you a list of granted permissions for the current website:

   .. image:: ./screens/firefox-persistent-storage-perm-refused.png

   What is the permission status for `Store data in persistent storage`?

   a. If it's set to `Allow`, you provided the correct permission for the Parsec web application.
   b. If it's `Blocked`, revoke the status by clicking on the cross ❌, then go to :ref:`step 1 <refresh-webapp>`

.. _manually-ask-permission:

4. Parsec did not prompt you to grant it permission on storage, nor have you previously granted the permission

   In that case, you can manually ask for the permission:

   - Anywhere on the page, do Right-click then Inspect
   - On the side window that opened, select the `Console` tab
   - Then enter the following command:

     .. code-block:: javascript

        await navigator.storage.persist()


   - After entering the command, it will prompt you to grant the persistent storage permission:

     .. image:: ./screens/firefox-ask-grant-persistent-storage-via-console.png

     Click on `Allow`

   - If the browser did not show the prompt, it means that it's configured to prevent a web application from accessing the storage API used by Parsec web application

.. mermaid::

  flowchart TD

    A[Refresh Parsec web] --> B{"Does it show you a prompt asking for persistent storage permission? (cf&nbsp;2.)"}

    click B href "#refresh-webapp"

    B --> |Yes| C[Click on <code>Allow</code>]

    C --> G

    B --> |No| D{"Did you previously set the permission? (cf&nbsp;3.)"}

    click D href "#did-you-set-permission"

    D --> |Yes| E{What is the value of the permission?}
    D --> |No| F("Manually ask for the permission (cf&nbsp;4.)")

    click F href "#manually-ask-permission"

    E --> |Allow| G[Done]
    E --> |Blocked|  H(Revoke the permission)

    H --> A

    F --> |"<code>await navigator.storage.persist()</code>"| I{What is the return value of the command?}

    I --> |true| A
    I --> |false| K[Your browser prevents the application from accessing the storage API]
