<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Frozen users

Frozen users are users that are not allowed to connect to the parsec server.

This feature is not part of the parsec revocation system, but a way for system administrators to temporarily block users from connecting to the server while waiting for organization administrators to actually revoke them. It is exposed as an HTTP route that requires administration token. This way, it is possible for existing directory services to automatically freeze users that have been removed from the directory.

## HTTP `users` route

This route is made available as `/administration/organizations/<raw_organization_id>/users` and requires an administration token.

### `GET` method

The `GET` method is used to list information for all users, including parsec ID, user name, user email and frozen status.

The body of the `GET` response is a JSON structure with the following format:

```jsonc
[
    {
        "user_id": "<parsec_user_id_1>",
        "user_name": "<user_name_1>",
        "user_email": "<user_email_1>",
        "frozen": true // or false
    },
    // [...]
    {
        "user_id": "<parsec_user_id_N>",
        "user_name": "<user_name_N>",
        "user_email": "<user_email_N>",
        "frozen": true // or false
    }
]
```

The list contains all users of the organization, including those that are not frozen.

However, it does not contain the users that have been revoked.

## HTTP `users/frozen` route

This route is made available as `/administration/organizations/<raw_organization_id>/users/frozen` and requires an administration token.

### `PATCH` method

The `PATCH` method is used to modify the `frozen` status for a given user.

The body of the `PATCH` request must be a JSON structure with either one of following formats:

```jsonc
{
    "user_email": "<user_email>",
    "frozen": true // or false
}
```

Or:

```jsonc
{
    "user_id": "<parsec_user_id>",
    "frozen": true // or false
}
```

This way, a user can either be identified by its email or by its parsec ID.

The body of the `PATCH` response is a JSON structure summarizing the updated information for the selected user:

```jsonc
{
    "user_id": "<parsec_user_id>",
    "user_name": "<user_name>",
    "user_email": "<user_email>",
    "frozen": true // or false
}
```

### User identification: email address vs parsec ID

There is a subtle difference between the two ways to identify a user. At any given time, an email address can be used to uniquely identify an non-revoked user from a given organization. In contrast, a parsec user ID identifies uniquely any user from all organizations in the parsec server, including revoked users. This means that over time, an email address can identify different parsec users with different parsec IDs, even from the same organization.

The frozen status configured by the `PATCH` method is specifically associated with the parsec user ID, regardless of the identification method used in the request body. This has a the following consequence: if a user is revoked and then a new user is created with the same email address, the frozen status will **not** be applied to the new user.


### Updating frozen status for several users at once

For simplicity, we do not support updating the frozen status for several users at once. This is not a problem as this feature is only meant to be used by directory services to freeze users that have been removed from the directory. In this case, the directory service will only need to freeze one user at a time.

However, this feature might be implemented in the future if necessary.


## Error handling

The following standard errors are handled the same way as for the other administration routes:

- Organization not found: `404` with JSON body `{"error": "not_found"}`
- Invalid administration token: `403` with JSON body `{"error": "not_allowed"}`
- Wrong request format: `400` with JSON body `{"error": "bad_data"}`

On top of it, an extra error is handled when the `PATCH` request contains a user that does not exist in the organization:

- User not found: `404` with JSON body `{"error": "user_not_found"}`


## Implementation

An extra `frozen` column is added to the  SQL table that stores the user information for each organization. This column is initialized with `False` values and is updated by the `PATCH` method.

When an authenticated handshake is performed by a user, the server checks the `frozen` field in the `user_` table. If it is, the handshake is rejected with a result that depends on the parsec protocol that is being used.

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
