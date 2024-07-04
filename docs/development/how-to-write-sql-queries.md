<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# How to write SQL queries

The goal of this document is to describe best practices and conventions for
writing SQL queries in Parsec.

## Components and queries

PostgreSQL components are implemented in`./server/parsec/components/postgresql/`.

Each component is responsible for a specific set of tables. So to find out
in which component you should write your query, ask yourself which component
is responsible for the set of tables you will be requesting.

For convenience, queries are defined at the top of each component file in order
to be close to the methods using them (as you often go back and forth between
the two).

Components might expose methods to other components in order to access specific tables.

## Naming convention

Query names start with `_` to indicate that they should be private to the
component file. The usual convention is to name them like:

- `_q_<action>_<data>` (i.e. `_q_revoke_user`)
- `_q_<action>_<data>_for_<parameter>` (i.e. `_q_get_user_id_for_device`)

## The `Q` class

`Q` instances are constructed from SQL query strings containing variables
(such as `$user_id`). These variables are replaced by [positional parameters](https://www.postgresql.org/docs/current/sql-expressions.html#SQL-EXPRESSIONS-PARAMETERS-POSITIONAL)
(such as `$1`).

When the Q object is called, the values are supplied after the query.

```python
_q_get_user = Q("SELECT name FROM user WHERE user_id = $user_id")
print(_q_get_user.sql)           # SELECT name FROM user WHERE user_id = $1
print(_q_get_user(user_id=42)) # ['SELECT name FROM user WHERE user_id = $1', 42]
```

## `_make_q_` functions

When two queries differ only slightly, a function can be used to generate them.

```python
def _make_q_lock_common_topic(for_update: bool = False, for_share=False) -> Q:
    assert for_update ^ for_share
    share_or_update = "SHARE" if for_share else "UPDATE"
    return Q(f"""
      SELECT last_timestamp
      FROM common_topic
      JOIN organization ON common_topic.organization = organization._id
      WHERE organization_id = $organization_id
      FOR {share_or_update}
    """)

_q_check_common_topic = _make_q_lock_common_topic(for_share=True)
_q_lock_common_topic = _make_q_lock_common_topic(for_update=True)
```

## Tips & Tricks

### The COALESCE expression

From [Conditional Expressions: COALESCE](https://www.postgresql.org/docs/current/functions-conditional.html#FUNCTIONS-COALESCE-NVL-IFNULL):

> The `COALESCE` function returns the first of its arguments that is not null.
> Null is returned only if all arguments are null. It is often used to
> substitute a default value for null values when data is retrieved for
> display.

In Parsec, you'll sometimes see code like this:

```sql
AND COALESCE(revoked_on > some_parameter, TRUE)
```

This takes advantage of the fact the following fact that ordinary comparison
operators yield null, not true or false, when either input is null. So the
previous example will return `TRUE` if `revoked_on` is null.
