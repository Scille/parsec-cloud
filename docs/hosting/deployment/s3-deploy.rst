.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_deployment_s3:

=======================
S3 Object Storage Setup
=======================

Parsec is compatible with different object storage solutions, including:

- `OpenStack Swift <https://docs.openstack.org/swift/latest/>`_
- `Amazon S3 <https://aws.amazon.com/s3/>`_ or any S3-compatible storage such as `Outscale's OOS <https://docs.outscale.com/en/userguide/OUTSCALE-Object-Storage-OOS.html>`_ (most storage providers offer S3-compatible APIs)
- A RAID configuration combining multiple storages

However, the most common deployment uses S3-compatible storage.

This guide provides instructions for setting up an S3-compatible bucket for use with Parsec.

.. note::

  Parsec has a very basic use of the object storage: it only reads existing objects and creates new objects.
  There is no deletion or listing of objects. Therefore, enabling versioning for the bucket is not strictly
  required, but it is **highly recommended** to prevent data loss from overwriting an existing object (since S3
  uses eventual consistency, the ``s3:PutObject`` policy always allows overwriting an existing object).

Prerequisites
=============

- An S3-compatible storage provider account
- The `AWS CLI <https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-getting-started.html>`_
- Appropriate permissions to create buckets and manage policies

.. _doc_hosting_deployment_s3_create_bucket:

Create the Bucket
-----------------

Create a new bucket for Parsec data storage:

.. code-block:: bash

  aws s3api create-bucket \
      --bucket <BUCKET_NAME>

Replace ``<BUCKET_NAME>`` with your desired bucket name.

.. note::

  The exact command may vary depending on your storage provider. Consult your provider's documentation
  for the appropriate command syntax.

  For example, with Outscale:

  .. code-block:: bash

    aws s3api create-bucket \
        --profile <PROFILE_NAME> \
        --endpoint https://oos.eu-west-2.outscale.com \
        --bucket <BUCKET_NAME>

.. _doc_hosting_deployment_s3_enable_versioning:

Enable Versioning
-----------------

Enable versioning on the bucket to protect against accidental overwrites:

.. code-block:: bash

  aws s3api put-bucket-versioning \
      --bucket <BUCKET_NAME> \
      --versioning-configuration Status=Enabled

.. _doc_hosting_deployment_s3_limit_access:

Limit Access with a Dedicated User
----------------------------------

For security, create a dedicated IAM user with restricted permissions to only the necessary operations.

The following bucket policy grants only the required permissions for Parsec to function:

.. code-block:: bash

  aws s3api put-bucket-policy \
      --bucket <BUCKET_NAME> \
      --policy '{
     "Version": "2012-10-17",
     "Statement": [
        {
           "Effect": "Allow",
           "Principal": {
              "AWS": "arn:aws:iam::<ACCOUNT_ID>:root"
           },
           "Action": [
              "s3:GetObject",
              "s3:PutObject"
           ],
          "Resource": "arn:aws:s3:::<BUCKET_NAME>/*"
        },
        {
           "Effect": "Allow",
           "Principal": {
              "AWS": "arn:aws:iam::<ACCOUNT_ID>:root"
           },
           "Action": [
              "s3:ListBucket"
           ],
          "Resource": "arn:aws:s3:::<BUCKET_NAME>"
        }
     ]
  }'

Replace the following placeholders:

- ``<BUCKET_NAME>``: Your bucket name
- ``<ACCOUNT_ID>``: The AWS account ID or IAM user that needs access
