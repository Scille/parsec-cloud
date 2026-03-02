.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_erase_organization:

Erase an organization
=====================

Where & how the data are stored
-------------------------------

For any given organization, data are split as follow:

- A PostgreSQL database containing the certificates (e.g. user/devices/workspaces) and
  encrypted workspaces metadata (e.g. content of folder, list of blocks for each file version).
- A blockstore (e.g. S3) containing the blocks (i.e. encrypted pieces of data that compose the files).
- On top of that, each Parsec client having access to the organization has an encrypted local database
  containing a copy of the certificates, metadata for the workspaces it has access to, and a subset
  of the blocks (depending on local cache configuration).

Erasing data
------------

When no longer in use (or for legal reason) an organization can be erased from the Parsec server.

In practice this means:

1. Removing everything (certificates & metadata) related to the organization in the PostgreSQL database.
2. Removing the blocks from the blockstore.
3. Removing the remaining data from the clients.

Step 1: remove from PostgreSQL
------------------------------

Erasing an organization from Parsec is both an uncommon and (obviously !) a destructive operation.
As such it is not available from the Administration API but instead must be triggered from the server CLI directly.

.. code-block:: bash

   # On Parsec server
   parsec erase_organization --organization <OrgName> --db <database_url>

.. warning::

   This operation cannot be undone. Make sure you have a backup of any data you may need before
   proceeding.

.. note::

   After erasing an organization, it is possible to create a new organization with the same name
   since no trace of the previous one remains.

   Even if they share the same name, the erased and the new organization are strictly unrelated
   since they have a different root verify key (i.e. root key used to verify all certificates
   in the organization).

   Typically this means a Parsec client trying to access the erased organization will complain
   it doesn't exist on the server even if a new organization with the same name exist.

Step 2: Blockstore cleanup
--------------------------

Once step 1 done, the blocks' decryption keys has been lost. In other words, everything
stored in the blockstore and related to the organization is irrecoverable.
Hence removing those data from the blockstore should be seen as an optional step to reclaim
needlessly occupied space.

This can be done by manually removing from the blockstore the top level directory named after the organization.
For example, if the organization was named ``CoolOrg``, remove the ``CoolOrg/`` prefix from the bucket.

.. note::

   Blockstores have their own backup strategy. Typically AWS S3 allows for a bucket to
   have an history so that a data removal can be cancelled.
   You should pay attention to this to ensure the blocks have actually been removed.

Step 3: Clients cleanup
-----------------------

Once the organization erased from the server, the Parsec client will display an error
about the organization not being found on the server.
However the client can still work in offline mode (if the server is not reachable, the
client cannot know the organization has been erased from the server!),

For this reason, an end-user is still able to use his Parsec client to work on the
organization using the local cache (e.g. creating a new file in a workspace or
reading an existing file that is in cache).

To prevent the users from accessing the local cache, the local configuration and data should be manually removed:

- Linux:
   - config: ``$XDG_DATA_HOME or $HOME/.local/share/parsec3/<device_id>`` (e.g. ``/home/alice/.local/share/parsec3/e68b7131394749a4bbd279bd087e6ae6``)
   - data: ``$XDG_CONFIG_HOME or $HOME/.config/parsec3/libparsec/devices/<device_id>`` (e.g. ``/home/alice/.config/parsec3/libparsec/devices/e68b7131394749a4bbd279bd087e6ae6``)
- macOS:
   - config: ``$HOME/Library/Application Support/parsec3/<device_id>`` (e.g. ``/Users/Alice/Library/Application Support/parsec3/e68b7131394749a4bbd279bd087e6ae6``)
   - data: ``$HOME/Library/Application Support/parsec3/libparsec/devices/<device_id>`` (e.g. ``/Users/Alice/Library/Application Support/parsec3/libparsec/devices/e68b7131394749a4bbd279bd087e6ae6``)
- Windows:
   - config: ``{FOLDERID_RoamingAppData}\parsec3\<device_id>`` (e.g. ``C:\Users\Alice\AppData\Roaming\parsec3\e68b7131394749a4bbd279bd087e6ae6``)
   - data: ``{FOLDERID_RoamingAppData}\parsec3\libparsec\devices\<device_id>`` (e.g. ``C:\Users\Alice\AppData\Roaming/parsec3/libparsec/devices/e68b7131394749a4bbd279bd087e6ae6``)
