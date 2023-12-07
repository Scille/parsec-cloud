.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_adminguide_frozen_users:


Frozen users
============

It is possible for server administrators to freeze specific Parsec users.

This feature is not part of the Parsec revocation system, but a way for system administrators to temporarily block users from connecting to the server while waiting for organization administrators to actually revoke them. It is exposed as an HTTP route that requires the administration token. This way, it is possible for existing directory services to automatically freeze users that have been removed from the directory.

HTTP `users` route
------------------

This route is made available as `/administration/organizations/<raw_organization_id>/users` and requires an administration token.

It only supports the `GET` method which lists information for all users, including Parsec ID, user name, user email, and frozen status.

Here's an example using `curl` and `jq`:

  .. code-block:: bash

    $ curl https://$server/administration/organizations/$organization/users \
        -H "Authorization: Bearer $administration_token" | jq

A successful request returns a JSON dictionary similar to the one below:

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

HTTP `users/frozen` route
-------------------------

This route is made available as `/administration/organizations/<raw_organization_id>/users/frozen` and requires an administration token.

It only supports the `POST` method which modifies the `frozen` status for a given user.

Here's an example of generating the request data using `jq`:

  .. code-block:: bash

    $ DATA=$(jq -n --arg user_id "$user_id" --argjson frozen true '$ARGS.named')
    $ echo $DATA
    {
    "user_id": "940a380aedd44127863d952a66cfce1e",
    "frozen": true
    }

The request can also use the `user_email` field instead of `user_id` to identify the Parsec user:

  .. code-block:: bash

    $ DATA=$(jq -n --arg user_email "$user_email" --argjson frozen true '$ARGS.named')
    $ echo $DATA
    {
    "user_email": "alice@example.com",
    "frozen": true
    }

Here's an example of running the request using `curl` and `jq`:

  .. code-block:: bash

    $ curl https://$server/administration/organizations/$organization/users/frozen \
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

User identification: email address vs Parsec ID
-----------------------------------------------

There is a subtle difference between the two ways to identify a user. At any given time, an email address can be used to uniquely identify a non-revoked user from a given organization. In contrast, a Parsec user ID identifies uniquely any user from all organizations in the Parsec server, including revoked users. This means that over time, an email address can identify different Parsec users with different Parsec IDs, even from the same organization.

The frozen status configured by the `POST` method is specifically associated with the Parsec user ID, regardless of the identification method used in the request body. This has the following consequence: if a user is revoked and then a new user is created with the same email address, the frozen status will **not** be applied to the new user.

Error handling
--------------

The following standard errors are handled the same way as for the other administration routes:

- Organization not found: `404` with JSON body `{"error": "not_found}`
- Invalid administration token: `403` with JSON body `{"error": "not_allowed"}`
- Wrong request format: `400` with JSON body `{"error": "bad_data"}`

On top of it, an extra error is returned when the `POST` request contains a user that does not exist in the organization:

- User not found: `404` with JSON body `{"error": "user_not_found"}`
