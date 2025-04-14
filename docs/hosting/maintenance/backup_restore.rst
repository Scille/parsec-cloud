.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_maintenance_backup_restore:

Backup and restore
******************

.. important::

  This section assumes that you deployed Parsec following the instructions from
  :ref:`Server deployment section <doc_hosting_deployment>`. If you deployed
  Parsec differently, you might need to adapt this section to your custom
  deployment.

Notes on consistency
====================

User data, stored in the S3 object storage, is accessible thanks to the metadata
stored on the PostgreSQL database. Metadata contains references to file blocks
stored in the object storage.

During backup and restore, the following situations may occur:

* The SQL database is up to date, but some objects are missing in the S3 object
  storage: this means that some files listed in the metadata may be missing
  some objects. Parsec will consider it a *file corruption* (only for files
  with missing objects). The affected files will still be displayed on the
  application, but they cannot be downloaded or opened.

* The SQL database is not up to date, but all objects are present: this means
  that there will be some objects that are not referenced by any file.
  Non-referenced objects are considered *orphaned* and therefore will be ignored
  by Parsec. All files displayed on the application will still be accessible.

* The SQL database is not up to date and some objects are not referenced:
  In this scenario, the effect of the previous two points are cumulative.

It should be noted that no block is deleted or modified from object storage,
even in the case of deleting a file or folder for historical purposes.

.. important::

  To maintain data consistency between databases, it is therefore necessary to
  **back up the SQL database before backing up the object database**, since
  excess objects have no consequences. The backup date to be considered is that
  of the SQL database.

In conclusion, it is not necessary to guarantee exact consistency between the
databases, as the SQL database is the authoritative source, but it is only
required that the SQL database backup is earlier than the backup of the object database.

PostgreSQL database
===================

Before starting, make sure you have the necessary permissions to perform these
operations and that the PostgreSQL service is running. If you encounter any
errors, check the error messages for clues as to what might be going wrong.

Backing up the database
-----------------------

You can create a backup file of the database using ``pg_dump``.

Open a terminal or command prompt and run the following command:

.. code-block:: shell

  pg_dump -U $USER -h $HOST -p $PORT "$DATABASE_NAME" > backup.sql

Replacing:

* ``$USER``: your PostgreSQL username
* ``$HOST``: the address of the database host (use ``localhost`` if the database is on your computer)
* ``$PORT``: the port on which PostgreSQL listens (by default, this is usually port ``5432``)
* ``$DATABASE_NAME``: the name of the database

If your database has a password, you will be prompted to enter it.

The above command creates a file ``backup.sql`` which contains all the data and
structure of the SQL database.

Restoring the database
----------------------

To restore the database from the backup file, you must first ensure that the
target database exists. If it doesn't, create it with PostgreSQL.

Database creation (if needed)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: shell

  createdb -U $USER -h $HOST -p $PORT "$DATABASE_NAME"

Replacing:

* ``$USER``: your PostgreSQL username
* ``$HOST``: the address of the database host (use ``localhost`` if the database is on your computer)
* ``$PORT``: the port on which PostgreSQL listens (by default, this is usually port ``5432``)
* ``$DATABASE_NAME``: the name of the database

Database restore
^^^^^^^^^^^^^^^^

If the target database exists, you can restore the database with the backup file
with a single command depending on the format of the backup: use ``psql`` for
SQL files or ``pg_restore`` for binary files.

For an SQL file, as described in the previous section, use:

.. code-block:: shell

  psql -U USER -h HOST -p PORT -d "DATABASE_NAME" < backup.sql

For a binary file (if you used ``pg_dump`` with ``-Fc`` option), use:

.. code-block:: shell

  pg_restore -U USER -h HOST -p PORT -d "DATABASE_NAME" -1 backup.bin


Object storage (S3)
===================

This section covers S3 object storage backup and restore in AWS.

Before starting, make sure your AWS account has the necessary permissions to
access the S3 bucket and perform these operations.

Backing up the bucket
---------------------

Use ``aws`` to manage buckets compatible with Amazon's S3 service.

Synchronize the S3 bucket with a local directory:

.. code-block:: shell

  aws s3 sync s3://bucket_name /path/local/backup

Where:

* ``s3://bucket_name`` is the path to the S3 bucket
* ``/path/local/backup`` is the path to the local directory where you want to store the backup

This command will download all files in the "bucket_name" bucket to the local
directory specified.

Restoring the bucket
--------------------

Restore all objects to an S3 bucket

To restore data from backup, use the ``aws s3 sync`` command in the opposite
direction, i.e. from the local directory to the S3 bucket.

.. code-block:: shell

  aws s3 sync /path/local/backup s3://bucket_name

* ``s3://bucket_name`` is the path to the S3 bucket
* ``/path/local/backup`` is the path to the local directory where you stored the backup.

This command will send all files in the specified local directory to the
"bucket_name" bucket.

.. tip::

  **Incremental backup**: ``aws s3 sync`` is smart enough to copy only those
  files that have been modified. This makes subsequent backups faster after the
  first full backup.
