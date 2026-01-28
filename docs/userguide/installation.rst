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

   .. group-tab:: Linux (snap)

      Parsec is available on the `Snap Store <https://snapcraft.io/parsec/>`_:

      .. raw:: html

        <iframe src="https://snapcraft.io/parsec/embedded?button=black" frameborder="0" width="100%" height="375px" style="border: 1px solid #CCC; border-radius: 2px; padding: 1px 2px 3px 4px; margin-bottom: 1em;"></iframe>

      You can install Parsec from the command line:

      .. code-block:: shell

        sudo snap install parsec --classic --channel=v3

      If you are familiar with Snap, you may notice that Parsec is provided in *classic* mode (i.e. without sandbox). This is required because Parsec needs `Fuse <https://en.wikipedia.org/wiki/Filesystem_in_Userspace>`_ to mount your data as a virtual directory, which is not allowed by the Snap sandbox.

      .. warning::

        On Ubuntu 25.04 and later, a bug may prevent your workspaces from being exposed in the file system (see `bug report <https://bugs.launchpad.net/ubuntu/+source/apparmor/+bug/2139081>`_).

        As a workaround, you can run the following command while Parsec is not running:

        .. code-block:: shell

          sudo tee /etc/apparmor.d/local/fusermount3 << 'EOF'
          # Allow Parsec packaged as a Snap to communicate with fusermount3 through unix sockets
          # see https://bugs.launchpad.net/ubuntu/+source/apparmor/+bug/2139081
          unix (send, receive) peer=(label="snap.parsec*.parsec"),
          EOF

          sudo apparmor_parser -r /etc/apparmor.d/fusermount3

      .. note::

          In order to install multiple versions of Parsec, you first need to enable `parallel instances <https://snapcraft.io/blog/parallel-installs-test-and-run-multiple-instances-of-snaps>`_:

          .. code-block:: shell

              snap set system experimental.parallel-instances=true

          Then, you can install v3.0 (provided you already have v2.17 installed), give it a specific name:

          .. code-block:: shell

              snap install parsec_v3 --classic --channel=v3

   .. group-tab:: Linux (AppImage)

      .. warning::

        AppImage support is currently in alpha, you are likely to encounter crashes if your Linux distribution differs too much from Ubuntu 24.04.

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

      This section describes how to install macFUSE.

      1. Get the latest version from the `macFUSE website <https://osxfuse.github.io/>`_.

      .. image:: screens/macfuse_download.png
          :align: center
          :alt: macFUSE download screen

      2. Open the macFUSE ``.dmg`` file and follow instructions to install.

        - If you cannot open the ``.dmg`` file because it was not downloaded from the App Store, head to `System Settings > Privacy and Security` on your Mac and scroll down to the `Security` section. You can either check `App Store & Known Developers` in the `Allow applications from` dropdown menu to make this installation and further updates smoother, or click `Open Anyway` for a one-time authorization.

        - At the end of the installation, you will be prompted to approve this extension in `System Settings > Privacy and Security`. Scroll down to the `Security` section, and click `Allow` as illustrated below.

        .. image:: screens/macfuse_current_allow.png
            :align: center
            :alt: macFUSE approve screen

      3. You will then be prompted to restart your Mac to complete the installation. Be mindful that the **Parsec app** will not function properly until this restart is done.


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
