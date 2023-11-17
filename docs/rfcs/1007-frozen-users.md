<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

Frozen users
============

Frozen users are users that are not allowed to connect to the parsec server.

This feature is not part of the parsec revocation system, but a way for system administrators to temporarily block users from connecting to the server while waiting for organization administrators to actually revoke them. It is exposed as an HTTP route that requires administration token. This way, it is possible for existing directory services to automatically freeze users that have been removed from the directory.

HTTP `frozen_users` route
-------------------------

This route is made available as `/administration/organizations/<raw_organization_id>/frozen_users` and requires an administration token.

### `GET` method

The `GET` method is used to list frozen status for all users, identified by their email addresses.

The body of the `GET` response is a JSON structure with the following format:

```json
[
    {
        "user_email": "<user_email_1>",
        "frozen": true
    },
    [...]
    {
        "user_email": "<user_email_N>",
        "frozen": false
    }
]
```

The list contains all users of the organization, including those that are not frozen.

However, it does not contain the email addresses of users that have been revoked.

### `PATCH` method

The `PATCH` method is used to modify this status for a given set of users.

The body of the `PATCH` request must be a JSON structure with the following format:

```json
{
    "user_email": "<user_email>",
    "frozen": true
}
```

### Error handling

The following standard errors are handled the same way as for the other administration routes:
- Organization not found: `404` with JSON body `{"error": "not_found"}`
- Invalid administration token: `403` with JSON body `{"error": "not_allowed"}`
- Wrong request format: `400` with JSON body `{"error": "bad_data"}`

On top of it, an extra error is handled when the `PATCH` request contains a user that does not exist in the organization:
- User not found: `404` with JSON body `{"error": "user_not_found"}`


## Implementation

A separate `frozen_users` SQL table stores the email addresses of frozen users for each organization. This table is updated by the `PATCH` method and read by the `GET` method.

When an authenticated handshake is performed by a user, the server checks if their email address is present in the `frozen_users` table. If it is, the handshake is rejected with a result that depends on the parsec protocol that is being used.

### Parsec v2

With parsec v2, the authentication process is performed through a websocket.

In this case, a frozen user will get a serialized `HandshakeResultSchema` with the following content:

```python
{
    "handshake": "result",
    "result": "frozen_user",
    "help": "User has been frozen by the server administrator"
}
```

This is similar to what happens when a user have been revoked (except the `result` field is set to `revoked_device` instead of `frozen_user`).

### Parsec v3

With parsec v3, the authentication process is performed through a HTTP request.

In this case, a frozen user will simply get a specific `462` HTTP error indicating that the user has been frozen by the server administrator.
