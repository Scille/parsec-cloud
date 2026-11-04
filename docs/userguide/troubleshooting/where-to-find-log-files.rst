.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_userguide_troubleshooting_where_to_find_log_files:

Where to find Parsec logs
-------------------------

Accessing logs from Parsec
==========================

Accessing logs when not logged-in to an organization
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. Open the ``Settings`` menu
2. In the ``Configuration`` section, you have the entry ``Logs``
3. Click on ``View logs`` button on that entry

.. image:: screens/find_logs/not_logged_show_logs.png
    :align: center
    :alt: Setting modal highlighting *Logs* entry when not logged to an organization


Accessing logs when logged-in to an organization
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. Click on your username (top right)
2. Select ``Settings``
3. In the ``Configuration`` section, you have the entry ``Logs``
4. Click on ``View logs`` button on that entry

.. image:: screens/find_logs/logged_show_logs.png
    :align: center
    :alt: Setting modal highlighting *Logs* entry when logged to an organization


Log files locations
===================

The desktop application place the log files following this platform matrix:

+-----------------+-------------------------------------------------------------+-------------------------------------------------------------------+
| OS              | Application logs                                            | Libparsec logs                                                    |
+=================+=============================================================+===================================================================+
| Windows         | ``%APPDATA%\parsec3\app\logs\main.log``                     | ``%APPDATA%\parsec3\libparsec\libparsec.log``                     |
+-----------------+-------------------------------------------------------------+-------------------------------------------------------------------+
| MacOS           | ``~/Library/Logs/parsec/main.log``                          | ``~/Library/Application Support/parsec3/libparsec/libparsec.log`` |
+-----------------+-------------------------------------------------------------+-------------------------------------------------------------------+
| Linux           | ``$XDG_CONFIG_HOME/parsec3/app/logs/main.log``              | ``$XDG_CONFIG_HOME/parsec3/libparsec/libparsec.log``              |
+-----------------+-------------------------------------------------------------+-------------------------------------------------------------------+
| Linux with Snap | ``~/snap/parsec/current/.config/parsec3/app/logs/main.log`` | ``$XDG_CONFIG_HOME/parsec3/libparsec/libparsec.log``              |
+-----------------+-------------------------------------------------------------+-------------------------------------------------------------------+

.. note::

  For libparsec logs, the file path is affected by the environment variable ``PARSEC_RUST_LOG_FILE``.

.. note::

  On Linux, if ``XDG_CONFIG_HOME`` environment variable is not defined, you can use ``~/.config`` instead.
