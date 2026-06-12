.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_install_cli:

==========================
Install Parsec CLI (Linux)
==========================

In order to deploy and maintain Parsec Server, you would need to perform some operations with
Parsec :abbr:`CLI (Command-Line Interface)` for Linux. These operations can be performed from another machine.

Parsec :abbr:`CLI (Command-Line Interface)` is a standalone binary that can be downloaded from
GitHub releases page.

.. _Parsec CLI: https://github.com/Scille/parsec-cloud/releases/download/v3.9.1-a.0.dev.20616+4736382/parsec-cli_3.9.1-a.0.dev.20616+4736382_linux-x86_64-musl

1. Download `Parsec CLI`_.

2. Make the file executable

   - Right-click on file, then :menuselection:`Properties --> Allow executing file as program`
   - Or use the :command:`chmod` command:

     .. code-block:: shell

        chmod +x parsec-cli_3.9.1-a.0.dev.20616+4736382_linux-x86_64-musl

3. Verify the installation by running the following command:

   .. code-block:: shell

      ./parsec-cli_3.9.1-a.0.dev.20616+4736382_linux-x86_64-musl --version

   The Parsec CLI version should be displayed:

   .. code-block:: shell

      parsec-cli 3.9.1-a.0.dev.20616+4736382

4. For convenience, put the executable in ``~/.local/bin``

   .. code-block:: shell

      cp parsec-cli_3.9.1-a.0.dev.20616+4736382_linux-x86_64-musl ~/.local/bin

   and make a symbolic link

   .. code-block:: shell

      cd  ~/.local/bin
      ln -s parsec-cli_3.9.1-a.0.dev.20616+4736382_linux-x86_64-musl parsec-cli

   You will be able to execute ``parsec-cli`` from anywhere.

.. tip::

   To update, follow the previous steps with the new version.
