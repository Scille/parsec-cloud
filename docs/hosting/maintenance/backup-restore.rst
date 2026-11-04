.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_maintenance_backup_restore:

==================
Backup and restore
==================

This section explains how to manage backups for PostgreSQL database and the Object Storage.

.. important::

  This section assumes that you deployed Parsec following the instructions from
  :ref:`Server deployment section <doc_hosting_deployment>`. If you deployed
  Parsec differently, you might need to adapt this section to your custom
  deployment.

Notes on data consistency
=========================

The user data accessible to the user depends primarily on the metadata stored in the PostgreSQL
database and secondly on the Object Storage. This is because the metadata contain references to
file blocks stored in the Object Storage.

During backup and restore, the following situations may occur:

* The PostgreSQL database is up to date, but some referenced objects are missing in the Object Storage:
  Parsec will consider files with missing objects as *corrupted*. These files should still be
  visible to the user, but they cannot be downloaded or opened.

* The PostgreSQL database is not up to date, and there are some objects non referenced by any in the Object Storage:
  Objects that are not referenced by any file are considered *orphaned* and therefore will be
  ignored by Parsec. All files displayed in Parsec should still be accessible.

* The PostgreSQL database is not up to date and some objects are not referenced:
  In this scenario, the effect of the previous two points are cumulative.

It should be noted that no block is deleted or modified from the Object Storage,
even in the case of deleting a file or folder for historical purposes.

.. important::

  To ensure data consistency between databases, you must **back up the PostgreSQL database
  *before* backing up the Object Storage**, as any excess objects will have no consequences.
  The backup date to consider is that of the PostgreSQL database.

In conclusion, it is not necessary to ensure exact consistency between the databases,
since the PostgreSQL database is the authoritative source; rather the PostgreSQL database
backup simply needs to be older (previous) than the  Object Storage backup.

PostgreSQL database
===================

Before starting, make sure you have the necessary permissions to perform these
operations and that the PostgreSQL service is running. If you encounter any
errors, check the error messages for clues as to what might be going wrong.

Backing up the database
-----------------------

You can create a backup file of the database using :command:`pg_dump`.

Open a terminal or command prompt and run the following command:

.. code-block:: shell

  pg_dump -U $USER -h $HOST -p $PORT "$DATABASE_NAME" > backup.sql

Where ``$USER`` is your PostgreSQL username, ``$HOST`` is the database host address (use ``localhost``
if the database is on your computer), ``$PORT`` is the port on which PostgreSQL listens (usually ``5432``)
and ``$DATABASE_NAME`` is the database name.

If your database has a password, you will be prompted to enter it.

The previous command will create a ``backup.sql`` file containing the structure of the
PostgreSQL database and all its data.

Restoring the database
----------------------

To restore the database from the backup file, you must first ensure that the
target database exists. If it doesn't, create it with PostgreSQL.

.. admonition:: Create the database (if needed)

  To create the database, run the following command:

  .. code-block:: shell

    createdb -U $USER -h $HOST -p $PORT "$DATABASE_NAME"

  Where ``$USER`` is your PostgreSQL username, ``$HOST`` is the database host address (use ``localhost``
  if the database is on your computer), ``$PORT`` is the port on which PostgreSQL listens (usually ``5432``)
  and ``$DATABASE_NAME`` is the database name.

After making sure the database exists, you can restore the database with the ``backup.sql`` file
with a single command depending on the format of the backup.

For an SQL file, use :command:`psql`:

.. code-block:: shell

  psql -U $USER -h $HOST -p $PORT -d "$DATABASE_NAME" < backup.sql

For a binary file (if you used :command:`pg_dump -Fc`), use :command:`pg_restore`:

.. code-block:: shell

  pg_restore -U $USER -h $HOST -p $PORT -d "$DATABASE_NAME" -1 backup.bin

Where ``$USER`` is your PostgreSQL username, ``$HOST`` is the database host address (use ``localhost``
if the database is on your computer), ``$PORT`` is the port on which PostgreSQL listens (usually ``5432``)
and ``$DATABASE_NAME`` is the database name.


Object Storage (S3)
===================

This section covers Object Storage backup and restore in AWS S3.

Before starting, make sure your AWS account has the necessary permissions to
access the S3 bucket and perform these operations.

Backing up the bucket
---------------------

Use :command:`aws` to manage buckets compatible with Amazon's S3 service.

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

To restore data from backup, use :command:`aws s3 sync` in the opposite
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
