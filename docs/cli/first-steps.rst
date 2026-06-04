.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_cli_first_steps:

===========
First steps
===========

Now that you have installed the Parsec CLI, let's start using it.

.. note::

  The outputs displayed in this documentation are examples and
  may vary from yours depending on your CLI version.

Help
====

When in need for information about the Parsec CLI, you can always use `help` as a command
of its own,


.. code-block:: shell

    parsec-cli help
    Parsec cli

    Usage: parsec-cli <COMMAND>

    Commands:
      server              Contains subcommands related to server operations
      device              Contains subcommands related to devices
      invite              Contains subcommands related to invitation
      organization        Contains subcommands related to organization
      user                Contains subcommands related to user
      workspace           Contains subcommands related to workspace
      certificate         Contains subcommands related to certificate
      ls                  List files in a workspace
      rm                  Remove a file from a workspace
      tos                 Contains subcommands related to Term of Service (TOS)
      shared-recovery     Contains subcommands related to shared recovery devices (shamir)
      mount-realm-export  Mount a realm export as a workspace
      help                Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help
      -V, --version  Print version

or as an option available for any command,


.. code-block:: shell

    parsec-cli ls --help
    List files in a workspace

    Usage: parsec-cli ls [OPTIONS] --workspace <WORKSPACE> [PATH]

    Arguments:
    [PATH]  The absolute workspace path to list contents (e.g. "/foo/bar") [default: /]

    Options:
    -w, --workspace <WORKSPACE>    Workspace ID [env: PARSEC_WORKSPACE_ID=]
        --password-stdin           Read the password from stdin instead of TTY. This flag needs to be explicitly set (it does not have a env var).
    -d, --device <DEVICE>          Device ID [env: PARSEC_DEVICE_ID=]
    -c, --config-dir <CONFIG_DIR>  Parsec config directory [env: PARSEC_CONFIG_DIR=] [default: /home/aurelia/.config/parsec3/libparsec]
    -h, --help                     Print help


or a group of commands,


.. code-block:: shell

    parsec-cli device --help
    Contains subcommands related to devices

    Usage: parsec-cli device <COMMAND>

    Commands:
    forget-local            Forget a local device It will still exist on the server but not locally anymore
    list                    List all devices
    change-authentication   Change authentication medium for a device
    export-recovery-device  Export recovery device
    import-recovery-device  Import recovery device
    overwrite-server-url    Change the server URL for the device, this is normally not needed
    help                    Print this message or the help of the given subcommand(s)

    Options:
    -h, --help  Print help


or subcommand

.. code-block:: shell

    parsec-cli device list --help
    List all devices

    Usage: parsec-cli device list [OPTIONS]

    Options:
    -c, --config-dir <CONFIG_DIR>  Parsec config directory [env: PARSEC_CONFIG_DIR=] [default: /home/aurelia/.config/parsec3/libparsec]
    -h, --help                     Print help


Commonly used parameters
------------------------

- ``--device``, ``-d``: the short id of the device used to authenticate the operation. This is needed when a command needs to perform authenticated operations.
- ``--password-stdin``: to read the password from stdin. This is useful to automate CLI usage.
