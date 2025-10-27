.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_install_cli:

Administrative operations must be performed with Parsec CLI for Linux. They can be performed from another machine.

Install Linux CLI
=================

Parsec is also available as a command line interface (CLI) for Linux. It is a standalone binary that is provided in our GitHub releases.

.. _Parsec CLI v3.6.0-rc.2+dev: https://github.com/Scille/parsec-cloud/releases/download/v3.6.0-rc.2+dev/parsec-cli_3.6.0-rc.2+dev_linux-x86_64-musl

1. Download the CLI by following this link:

   `Parsec CLI v3.6.0-rc.2+dev`_.

2. Make the file executable

  - Right-click/Properties/Allow executing file as program
  - Or use the `chmod` command:

    .. code-block:: shell

        chmod +x parsec-cli_3.6.0-rc.2+dev_linux-x86_64-musl

3. Verify the installation by running the following command:

  .. code-block:: shell

      ./parsec-cli_3.6.0-rc.2+dev_linux-x86_64-musl --version

  The CLI should output its version:

  .. code-block::

      parsec-cli 3.6.0-rc.2+dev
