.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_freeze_users:

============
Freeze users
============

Server administrators can *freeze* users via the Administration API. A frozen user will be temporarily blocked from connecting to Parsec server.

This mechanism allows to automatically block users who have been deleted from a directory service (such as OpenLDAP or Active Directory), while waiting for the Organization administrator to revoke them.
It is exposed in the form of HTTP routes that only requires an administration token.

.. note::

  The *freeze* operation is performed by a *Server administrator* and can be undone. The *revoke* operation is performed by an *Organization administrator* and cannot be undone, it is the definitive removal of the user's rights within the organization.

HTTP ``/users`` route
=====================

This route is made available as ``/administration/organizations/<raw_organization_id>/users`` and requires an administration token.

It supports the ``GET`` method which lists information for all users, including:

- Parsec ID
- user name
- user email
- frozen status

Here's an example using ``curl`` and ``jq``:

  .. code-block:: bash

    $ curl https://$server/administration/organizations/$organization/users \
        -H "Authorization: Bearer $administration_token" | jq

A successful request returns a JSON object with the following structure:

  .. code-block:: json

    {
      "users": [
        {
          "user_name": "Alice",
          "frozen": true,
          "user_email": "alice@example.com",
          "user_id": "940a380aedd44127863d952a66cfce1e"
        },
        {
          "user_name": "Bob",
          "frozen": false,
          "user_email": "bob@example.com",
          "user_id": "7d04b0df51a74158b35b6386eecdf4e0"
        }
      ]
    }

HTTP ``/users/freeze`` route
============================

This route is made available as ``/administration/organizations/<raw_organization_id>/users/freeze`` and requires an administration token.

It supports the ``POST`` method which modifies the ``frozen`` status for a given user.

Here's an example of generating the request data using ``jq``:

  .. code-block:: bash

    $ DATA=$(jq -n --arg user_id "$user_id" --argjson frozen true '$ARGS.named')
    $ echo $DATA
    {
    "user_id": "940a380aedd44127863d952a66cfce1e",
    "frozen": true
    }

The request can also use the ``user_email`` field instead of ``user_id`` to identify the Parsec user (see the :ref:`note on user identification <note-on-user-identification>` section below for more information):

  .. code-block:: bash

    $ DATA=$(jq -n --arg user_email "$user_email" --argjson frozen true '$ARGS.named')
    $ echo $DATA
    {
    "user_email": "alice@example.com",
    "frozen": true
    }

Here's an example of running the request using ``curl`` and ``jq``:

  .. code-block:: bash

    $ curl https://$server/administration/organizations/$organization/users/freeze \
        -H "Authorization: Bearer $administration_token" \
        --request POST --data $DATA | jq

A successful request returns a JSON dictionary similar to the one below:

  .. code-block:: json

    {
      "frozen": true,
      "user_email": "alice@example.com",
      "user_id": "940a380aedd44127863d952a66cfce1e",
      "user_name": "Alice"
    }

HTTP Error handling
===================

The following errors can be returned by both ``/users`` and ``/users/freeze`` routes:

- ``404: Organization not found`` with JSON body ``{"error": "not_found}``
- ``403: Invalid administration token`` with JSON body ``{"error": "not_allowed"}``
- ``400: Wrong request format`` with JSON body ``{"error": "bad_data"}``

The following error is returned by the ``/users/freeze`` request if the user does not exist in the organization:

- ``404: User not found`` with JSON body ``{"error": "user_not_found"}``

.. _note-on-user-identification:

Note on user identification
===========================

There is a subtle difference between using **Parsec user ID** or **email address** to identify a user.

- The **Parsec user ID** uniquely identifies a user *within the Parsec server*, regardless of its organization. When a user is revoked, its user ID identifies the revoked user.
- The **email address** identifies an active (i.e. non-revoked) user *within an organization*. When a user is revoked, its email address can be reused to (re)join the organization.

This means that, over time, an email address can be shared between multiple user IDs, either from different organizations or within the same organization if it has been revoked.

Consider the following scenario:

=====================================  ====================================
Org 1                                  Org 2
=====================================  ====================================
ID **1** alice@mail.com (**revoked**)  ID **4** bob@mail.com (**active**)
ID **2** bob@mail.com (**active**)     ID **5** alice@mail.com (**active**)
ID **3** alice@mail.com (**active**)
=====================================  ====================================

The frozen status specified in the ``POST`` request is always associated with a Parsec user ID, even if an email address is specified.

Regarding the previous scenario, here are some possible requests and their outcomes:
- Freeze user with ID **1**: will have no consequences since the user is already *revoked*.
- Freeze user with ID **4**: will effectively freeze the user. Its *active* status is maintained in case of unfreeze.
- Freeze user with email **alice@mail.com** from **Org 1**: will freeze user with ID **3** (user with ID **1** is not considered as it is already revoked)

.. warning::

  Notice that if a user is revoked from an organization and then re-invited with the same email address, its previous frozen status will **not** be applied to the new user.


.. _cli-freeze-user-command:

CLI  ``user freeze`` command
----------------------------

The ``user freeze`` command allows to change frozen status for a user:

.. code-block:: bash

  $ parsec-cli user freeze --addr "parsec3://$server" --token $administration_token --organization $organization --user-email user@example.com
  User fcc1d10eed3f4cb6a155b705ac5fcfbd - username <user@example.com> is not frozen

.. code-block:: bash

  $ parsec-cli user freeze --addr "parsec3://$server" --token $administration_token --organization $organization --user-email user@example.com --unfreeze
  User fcc1d10eed3f4cb6a155b705ac5fcfbd - username <user@example.com> is not frozen


The provided ``$user`` can either be a parsec ID or an email address. Use the ``--help`` for more information.
