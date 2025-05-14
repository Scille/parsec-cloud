<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Deprecated `LocalDevice`'s user realm fields

## 1 - Overview

`LocalDevice` currently contains field referring to "user realm", i.e. a way to
use a realm that is not the workspace but instead is used to stored user-specific data
synchronized between devices.

This is a concept that was removed in Parsec v3, hence those fields are never used and can be safely removed.

> [!NOTE]
>
> - Backward compatibility is not a concern since our deserialization format simply
>   ignores unknown fields.
> - If in the future the need for user-specific synchronized data, we will introduce
>   ad-hoc APIs and rely on the user's private key.

## 2 - Changes

- Remove `user_realm_id` field
- Remove `user_realm_key` field

> [!NOTE]
> Instead of hard remove from the schema, the `deprecated_in_revision` attribute
> should be used on those fields in order to keep track of their existence.
