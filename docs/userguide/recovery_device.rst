.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_recovery_device:

================================
Create and use a recovery device
================================

A *recovery device* is a backup of an existing device.
If you lose access to a Parsec's device (you forget the password, the Parsec keys had been lost, etc.),
the recovery device will allow you to recreate a new device and set a new password.

In order to `create a new recovery device`_ or to `use a recovery device`_ you need to open the recovery device dialog from the main menu:

1. Open the ``Recovery device`` menu.

   You need to open the sub-menu then click on ``Recovery device`` option (alternatively, you can use the shortcut ``Ctrl+I``).

   .. image:: screens/recovery_device_menu.png
      :alt: Parsec menu with recovery device option highlighted

2. That will open a modal dialog that looks like this:

   .. image:: screens/recovery_device_modal.png
      :alt: Parsec recovery device modal

Create a new recovery device
############################

1. In the Device recovery dialog, select ``Create a recovery device``

2. From here:

   a. Select the device you would like to create a recovery device for.
   b. Click the ``Select`` button to specify a file to store the recovery device.

   .. image:: screens/recovery_device_create_device.png
      :alt: Create a recovery device modal

3. Click ``Next``, you will be prompted to enter the password for the device you have selected

   .. image:: screens/recovery_device_create_device_re_auth.png
      :alt: Create a recovery device authentication modal

4. After confirmation, the recovery file is created and a passphrase is displayed. **You need to save the passphrase** as it will not be displayed again.

   .. image:: screens/recovery_device_create_device_confirmation.png
      :alt: Create a recovery device confirmation modal

   .. warning::

      The passphrase should be handled carefully like any other password.

      You can print out the passphrase and put-it in a safe to secure it.
      Nevertheless, you should not save it alongside the recovery file.


Use a recovery device
#####################

1. In the Device recovery dialog, select ``Recover a Device``

2. From here:

   a. Select the recovery file of the device you want to recover.
   b. Enter the passphrase for the recovery device.
   c. Enter the name of the new device.

   .. image:: screens/recovery_device_recover.png
      :alt: Recover a device modal

3. Click ``Next``, you will be prompted to enter a new password and confirm it.

   .. image:: screens/recovery_device_recover_new_password.png
      :alt: Enter the new password for the recovered device modal

4. A message is displayed to confirm the device has been recovered, you can click ``Continue``.

   .. image:: screens/recovery_device_recover_confirmation.png
      :alt: Device recovered confirmation modal

5. You can now access Parsec as usual using the new recovered device from the devices' list
