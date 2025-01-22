.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_install_parsec:


Install Parsec
==============


Install Parsec app
------------------

Parsec is available for Linux, Mac, and Windows operating systems.

.. tabs::

   .. group-tab:: Windows

      1. Download the latest stable version of the Windows installer:

      .. image:: screens/download-parsec.png
          :align: center
          :alt: download parsec
          :target: `Download Parsec`_

      2. Start the ``.exe`` and follow the instructions to install Parsec.

   .. group-tab:: Linux (AppImage)

      1. Download the latest stable version of the **AppImage** file:

      .. image:: screens/download-parsec.png
          :align: center
          :alt: download parsec
          :target: `Download Parsec`_

      2. Make the ``.AppImage`` file executable

        - Right-click/Properties/Allow executing file as program
        - Or use the `chmod` command:

          .. code-block:: shell

              chmod u+x Parsec.AppImage

      3. Run it (double-click on the ``.AppImage`` file)

   .. group-tab:: Linux (snap)

      Parsec is available on the `Snap Store <https://snapcraft.io/parsec/>`_:

      .. raw:: html

        <iframe src="https://snapcraft.io/parsec/embedded?button=black" frameborder="0" width="100%" height="375px" style="border: 1px solid #CCC; border-radius: 2px; padding: 1px 2px 3px 4px; margin-bottom: 1em;"></iframe>

      You can install Parsec from the command line:

      .. code-block:: shell

        sudo snap install parsec --classic --channel=v3

      If you are familiar with Snap, you may notice that Parsec is provided in *classic* mode (i.e. without sandbox). This is required because Parsec needs `Fuse <https://en.wikipedia.org/wiki/Filesystem_in_Userspace>`_ to mount your data as a virtual directory, which is not allowed by the Snap sandbox.

      .. note::

          In order to install multiple versions of Parsec, you first need to enable `parallel instances <https://snapcraft.io/blog/parallel-installs-test-and-run-multiple-instances-of-snaps>`_:

          .. code-block:: shell

              snap set system experimental.parallel-instances=true

          Then, you can install v3.0 (provided you already have v2.17 installed), give it a specific name:

          .. code-block:: shell

              snap install parsec_v3 --classic --channel=v3

   .. group-tab:: macOS

      1. Download the latest stable version of the **macOS installer**:

      .. image:: screens/download-parsec.png
          :align: center
          :alt: download parsec
          :target: `Download Parsec`_

      2. Start the ``.dmg`` installer and follow the instructions to install **Parsec app**.

      3. Install macFUSE (see below)

      .. note::

          Parsec requires `macFUSE <https://osxfuse.github.io/>`_ in order to provide a smooth integration with macOS and let you access your documents via Finder (macOS file manager).

      **Install macFUSE**

      This section describe how to install macFUSE.

      1. Get the latest version from the `macFUSE <https://osxfuse.github.io/>`_ website.

      .. image:: screens/macfuse_download.png
          :align: center
          :alt: macFUSE download screen

      2. Open the ``.dmg`` file and follow instructions to install.

        - If the opening fails, two options are available in `System Settings > Privacy and Security`: either check the `App Store and identified developers` box, or click `Open Anyway` if you don't want to change this setting, which will need to be done once to open the ``.dmg``, and possibly once more to start the installer.

        .. image:: screens/macfuse_current_allow.png
            :align: center
            :alt: macFUSE current allow screen

      3. Finally, reboot your Mac to complete the installation.

      .. note::

          On macOS 12 and older, if the installer fails to start, check the `App Store and identified developers` box in `System Preferences > Security & Privacy`:

          .. image:: screens/macfuse_previous_system_preferences.png
              :align: center
              :alt: macOS path to Security and Privacy

          .. image:: screens/macfuse_previous_allow_developer.png
              :align: center
              :alt: macOS previous allow identified developer

          To change this setting, click the lock first which will require admin rights.

          Once the installation is done, a `System Extension Updated` window will pop up. Click `Open Security Preferences`, and click the lock, then click `Allow`:

          .. image:: screens/macfuse_previous_system_extension.png
              :align: center
              :alt: Previous System Extension Updated window

          .. image:: screens/macfuse_previous_allow_extension.png
              :align: center
              :alt: macOS previous allow extension


Update Parsec app
-----------------

Automatic updates are supported and enabled. When a new version is released, Parsec app updates automatically on startup.

If Parsec app is running, A message is displayed. Simply click on the message to update the application.


Install an older version of Parsec
----------------------------------

.. warning::

    For security reasons, you should **always install the latest stable version of Parsec** as it contains the latest security fixes.

If you need to access older versions, they are available on `GitHub`_.

.. _Download Parsec: https://parsec.cloud/demarrer-parsec/
.. _GitHub: https://github.com/Scille/parsec/releases/latest
