.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_install_cli:

==========
Parsec CLI
==========

In order to deploy and maintain Parsec Server, you would need to perform some operations with
Parsec :abbr:`CLI (Command-Line Interface)` for Linux. These operations can be performed from another machine.

Parsec :abbr:`CLI (Command-Line Interface)` is a standalone binary that can be downloaded from
GitHub releases page.

.. _Parsec CLI: https://github.com/Scille/parsec-cloud/releases/download/v3.9.1/parsec-cli_3.9.1_linux-x86_64-musl

#. Download `Parsec CLI`_.

#. Make the file executable:

   - Right-click on file, then :menuselection:`Properties --> Allow executing file as program`
   - Or use the :command:`chmod` command:

   .. code-block:: shell

     chmod +x parsec-cli_3.9.1_linux-x86_64-musl

#. Verify the installation by running the following command:

   .. code-block:: shell

     ./parsec-cli_3.9.1_linux-x86_64-musl --version

   Which should display the installed version:

   .. code-block:: text

     parsec-cli 3.9.1
