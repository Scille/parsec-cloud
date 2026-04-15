.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_install_cli:

==========================
Install Parsec CLI (Linux)
==========================

In order to deploy and maintain Parsec Server, you would need to perform some operations with
Parsec CLI for Linux. These operations can be performed from another machine.

Parsec Command-Line Interface (CLI) is a standalone binary that can be downloaded from GitHub releases page.

.. _Parsec CLI v3.8.2-a.0+dev: https://github.com/Scille/parsec-cloud/releases/download/v3.8.2-a.0+dev/parsec-cli_3.8.2-a.0+dev_linux-x86_64-musl

1. Download Parsec CLI: `Parsec CLI v3.8.2-a.0+dev`_.

2. Make the file executable

  - Right-click/Properties/Allow executing file as program
  - Or use the `chmod` command:

    .. code-block:: shell

        chmod +x parsec-cli_3.8.2-a.0+dev_linux-x86_64-musl

3. Verify the installation by running the following command:

  .. code-block:: shell

      ./parsec-cli_3.8.2-a.0+dev_linux-x86_64-musl --version

  The Parsec CLI version should be displayed:

  .. code-block::

      parsec-cli 3.8.2-a.0+dev
