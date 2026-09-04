.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_install_cli:

.. |parsec-cli| replace:: Parsec :abbr:`CLI (Command-Line Interface)`
.. |parsec-version| replace:: ``parsec-cli 3.9.4-a.0.dev.20700+72e13e8``

==========================
Install Parsec CLI (Linux)
==========================

In order to deploy and maintain Parsec Server, you would need to perform some operations with
|parsec-cli| for Linux. These operations can be performed from another machine.

|parsec-cli| is a standalone binary that can be downloaded from
GitHub releases page.

.. _Parsec CLI: https://github.com/Scille/parsec-cloud/releases/download/v3.9.4-a.0.dev.20700+72e13e8/parsec-cli_3.9.4-a.0.dev.20700+72e13e8_linux-x86_64-musl

1. Download `Parsec CLI`_.

2. Make the file executable

   - Right-click on file, then :menuselection:`Properties --> Allow executing file as program`
   - Or use the :command:`chmod` command:

     .. code-block:: shell

        chmod +x parsec-cli_3.9.4-a.0.dev.20700+72e13e8_linux-x86_64-musl

3. Verify the installation by running the following command:

   .. code-block:: shell

      ./parsec-cli_3.9.4-a.0.dev.20700+72e13e8_linux-x86_64-musl --version

   The Parsec CLI version should be displayed as |parsec-version|

4. For convenience, put the executable in ``~/.local/bin``

   .. code-block:: shell

      cp parsec-cli_3.9.4-a.0.dev.20700+72e13e8_linux-x86_64-musl ~/.local/bin

   and make a symbolic link

   .. code-block:: shell

      cd  ~/.local/bin
      ln -s parsec-cli_3.9.4-a.0.dev.20700+72e13e8_linux-x86_64-musl parsec-cli

   You will be able to execute ``parsec-cli`` from anywhere.

.. tip::

   To update, follow the previous steps with the new version.

Enable auto-completion
======================

.. version-added:: 3.10.0

   To have your shell auto-complete the ``parsec-cli`` command arguments,
   you will need to have the following line in your shell RC file:

   .. code-block:: shell

      eval "$(parsec-cli auto-complete $SHELL)"

   .. note::

      The above code-snippet is for ``bash``, adapt it to your shell.

Install man pages
=================

.. version-added:: 3.10.0

   You can install the |parsec-cli| man pages by doing the following:

   1. Create the local man pages folder:

      .. code-block:: shell

         mkdir -p ~/.local/share/man/man1

      .. tip::

         To make the pages available system-wide, replace the folder path with ``/usr/local/share/man/man1`` on the commands above and below.

   2. Generate the man pages:

      .. code-block:: shell

         parsec-cli man-page --mode separate ~/.local/share/man/man1

   3. Update the man pages database:

      .. code-block:: shell

         mandb

   4. Verify:

      .. code-block:: shell

         man parsec-cli

      .. important::

         If ``man`` cannot find the page, ensure that the path where you put the pages is in the variable ``$MANPATH``.
         You can also verify if ``mandb`` explored the folder.
