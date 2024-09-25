.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_adminguide_install_cli:

Administrative operations must be performed with Parsec CLI for Linux. They can be performed from another machine.

Install Linux Parsec CLI
========================

Parsec is also available as a command line interface (CLI) for Linux. It is a standalone binary that is provided in our GitHub releases.

1. Download the CLI by following this link:

   `Parsec CLI v3.0.1-a.0 <https://github.com/Scille/parsec-cloud/releases/download/v3.0.1-a.0/parsec-cli_3.0.1-a.0_linux_x86_64>`_.

2. Make the file executable

  - Right-click/Properties/Allow executing file as program
  - Or use the `chmod` command:

    .. code-block:: shell

        chmod +x parsec-cli_3.0.1-a.0_linux_x86_64

3. Verify the installation by running the following command:

  .. code-block:: shell

      ./parsec-cli_3.0.1-a.0_linux_x86_64 --version

  The CLI should output its version:

  .. code-block::

      parsec-cli 3.0.1-a.0
