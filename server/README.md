# Parsec server

## Architecture

The server is an [ASGI application](https://asgi.readthedocs.io/en/latest/introduction.html), hence
it can be seen as divided into two parts:

- The web application parts: The end goal here is to end up with a web application listening on a given port.
- The business application: This the lower-level layer the web application obviously relies on it, but
  is also used in testing, or to expose features in the CLI.

The business application's main class is `Backend` (defined in `parsec/backend.py`) that
agglomerate multiple components (defined in `parsec/components`, see below), each one of
them implementing a separated subset of the business logic (e.g. invitations).

> Note: In the tests a `Backend` class instance is very often obtain through the
> `backend` fixture, as this is a very convenient to customize the content of the
> server by directly using the business logic methods (e.g. `backend.vlob.create(...)`).

The web application's main class is obviously `AsgiApp` (we use
[FastAPI as ASGI framework](https://fastapi.tiangolo.com/), so this class is in
fact a `fastapi.FastAPI`) that is build from a factory that, among other things,
introspect the components to register the HTTP RCP API endpoints they expose.

This is defined in the `parsec/asgi` folder, the HTTP endpoints are:

- RCP API (`parsec/asgi/rpc.py`): this implement the HTTP API used by the Parsec client
  and defined in the crate `libparsec_protocol`.
- Administration API (`parsec/asgi/administration.py`): this is a JSON REST API designed
  for the administration of the server (e.g. create a new organization).
- Redirection (`parsec/asgi/redirect.py`): allow conversion between `https://` -> `parsec://`
  URLs (used in email given `parsec://` URLs are often not displayed correctly by email clients)
- Misc user-facing URLs (`parsec/asgi/__init__.py`), e.g. `index.html`, page not found etc.

It should be obvious now that the main part of the server is those components that both
implement the business application logic and the HTTP endpoints for the RPC API.

On top of that those components have two implementations:

- PostgreSQL `parsec/components/postgresql`
- In memory `parsec/components/memory`

While the PostgreSQL implementation is the only one to use in production, the memory
implementation is very convenient:

- Its simplicity allows for fast hack when testing new ideas.
- It frees us from a dependency so developer doesn't need to have a PostgreSQL
  server running to work on Parsec.
- It speeds up tests (obviously PostgreSQL implementation is much faster for a big volume
  of data, but this is irrelevant when testing ^^).

> Note: Only the business logic needs to be implemented in both postgresql and memory,
> RPC API endpoints are implemented once in the base class `Parsec/components/` given
> they themselves relies on the business logic methods.

## Data concurrency

The server needs to handle multiple queries concurrently.

For instance two concurrent requests to create an invitation to the same person
should have the same outcome than if those requests were consecutive (i.e. the
second request should detect an invitation already exists for the given email
and return it).

## Data causality

Given changes in the data come from different client, the server is responsible
to guarantee causality on all data.

### Example 1

- At T1 Alice query the server for all the certificates
- At T2 a new certificates from Bob reaches the server with timestamp T0 < T1 (Bob's
  clock might be off, or the query took a long time to reach the server)

In this case, if Alice got a certificate with a timestamp more recent that Bob's certificate,
then accepting Bob's certificate would break causality !

### Example 2

- At T1 the server receive block create from Alice in realm R1
- At T2 the server receive a certificate un-sharing with Alice realm R1 with timestamp T0

In this case, and considering the server has accepted the un-sharing certificate,
any client accessing the block before T2 will consider everything is fine, while
any client accessing the block after T2 will reject it given it appears to have
been created after Alice lost her access to the realm !

## Causality & concurrency model

From the server point of view, there is basically four different kinds of data:

- Certificates
- Blocks
- Vlobs
- Invitations

### Certificates

Any change done by a client that should be visible by the server is provided as a
certificate (i.e. a certificate is an unencrypted document signed by the client's device).

A very simple way to ensure causality and concurrency on certificates would be:

- Have a single big lock in the server that must be taken before any attempt to
  add a certificate (handles concurrency).
- Ensure the new certificate's timestamp is newer than any existing certificate
  (handles causality).

However this is pretty wasteful given most certificates are unrelated (e.g. certif A
add a new user, certif B does a key rotation in a realm). So instead we reduce the
size of the lock by introducing the concept of topic.

Each type of certificate is part of one of the following topics:

- `common`84beb4111e96b4e7231a2bfb51424ed5528d708c
- `sequester`, which depends on `common`
- `realm`, which depends on `common` and uses the realm ID as discriminant
- `shamir_recovery`, which depends on `common` and use the ID of the user to recover as discriminant

On top of that, some certificates (`RealmRoleCertificate` and `RevokedUserCertificate`)
impact one's capacity to upload vlobs/blocks in a realm (as a user must be member of the realm
with sufficient profile and not be revoked):

- `RevokedUserCertificate`'s causality check requires to check the timestamp of the last
  vlob/block created by the target user (i.e. the one being revoked) across all realms.
- `RealmRoleCertificate`'s causality check requires to check the timestamp of the last
  vlob/block created by the target user on the target realm.

Regarding blocks, they are not uploaded with a timestamp (i.e. a block is just an ID in a realm
pointing to some encrypted arbitrary bytes). Hence causality is always safe here: if the block
exists it is assumed it has been created when the user had the right to do it.

> Notes:
>
> - Blocks doesn't need timestamp simply because they only have meaning as part of a
>   file manifest (i.e. a vlob from the server point of view), which is the one having
>   timestamp !
> - In PostgreSQL there is a `created_on` field in the `block` table, however it is never
>   actually used and should only be seen as a useful information for database admin.

So here we go with a coarse approach and only keep track for each user of the timestamp
of the last vlob/block across all realms. This is less optimized for `RealmRoleCertificate`,
but the low volume of certificate creation makes this a non-issue.

In the end, the actual rule used in the server is:

- Have locks in the server for each couple topic + discriminant
- Take the lock for the topic + discriminant couple the certificate we want to add
  belongs to AND to other topics our topic may depend on (handles concurrency).
  Note this must be done in an order respecting the dependency (e.g. first `common`
  topic then `realm` + *realmID*).
- Ensure the new certificate's timestamp is newer than any existing certificate
  in the topics we have locked (handles causality).
- If the new certificate impact the capacity to write vlob, ensure its
  timestamp is newer than any existing vlob the target user has created.

> Notes:
>
> - Also organizations are totally isolated from each other on the server, so
>   in a way the organization ID is an additional discriminant on each certificate topic.
> - [As we are going to see](#what-is-locked-by-who-), all the other kind of data depends at diverse degrees to
>   the certificates (e.g. blocks are accessed by users that are member of a realm, hence
>   `common` and `realm` topics are involved).

#### User revocation and concurrent operation involving users

Locking a realm's topic works great as long as:

- The topic exists... which is not the case before the realm's creation !
- We know the topic should be locked... which is not the case when a concurrent operation
  gives access to a new topic (e.g. `realm_share`, `shamir_recovery_setup`) at the wrong time !

This is an issue when revoking a user, as we must make sure no concurrent operation leads
to the production of data by this user (or involving this user) after its revocation.

The commands impacted by this issue are all the ones giving any access to a non-revoked user
(i.e. if a command involves certificate upload and has a `recipient_revoked` error status,
then it should be here !).

So far far they are:

- `realm_create`
- `realm_share`
- `realm_unshare` (removing an access should never be an issue for user revocation check,
  but it's more consistent and safer to consider this is also impacted)
- `shamir_recovery_setup`

Now regarding the good news: this problem is already taken care of by the `common` topic !

Indeed, user revocation takes a write lock on `common` topic, while all commands involving
giving an access to a user must take a read lock on `common` (since giving an access involves
a certificate, certificate that has been signed by a user, user that itself must not get
revoked for the duration of the command, hence the `common` lock being read locked).

> Notes:
>
> - The relationship between user revoke and give-user-access commands are specified
>   in the [What is locked by who ?](#what-is-locked-by-who-) section with the `user_revoke_safe` column.
> - The [What is locked by who ?](#what-is-locked-by-who-) table is checked in CI by the
>   `misc/check_server_readme.py` script to ensure there is consistency between the
>   `user_revoke_safe` and `common` topic lock.

#### `RequireGreaterTimestamp` vs `CertificateBasedActionIdempotentOutcome`

On top of command-specific checks, a server may reject a valid certificate by returning
`RequireGreaterTimestamp` or `CertificateBasedActionIdempotentOutcome`.

`RequireGreaterTimestamp` is simple to understand: a data belonging to a locked topic has
a timestamp more recent than the one provided in the command, hence the client should
use a timestamp strictly superior than the one returned in the error to have a chance
to succeed.

`CertificateBasedActionIdempotentOutcome` is more subtle: the server has detected the
operation the user wants to do has already been done (e.g. revoking a user). Hence
there is no point for the client in updating the timestamp and retry, and instead it
should fetch the new certificates.

The "idempotent" in the name comes from the fact a client having successfully uploaded a
certificate on the server cannot directly integrate it in its local database (as some other
certificates may have been concurrently uploaded right before its own), but instead must
fetch it from the server.
So if the client receives a `CertificateBasedActionIdempotentOutcome` instead of a success,
it can nevertheless pretend a success occurred and fetch the new certificates to end up
in a similar state than if the operation actually succeeded.

### Blocks

Blocks are encrypted piece of data that belong to a given realm and cannot
be modified.

This makes concurrency & causality pretty simple:

- Each block is considered independent, hence there is no need for a lock dedicated to blocks.
- A block is identified by a client-provided UUID, then there is no risk for double-creation
  under concurrency (a trivial uniqueness check on insertion prevents this).
- Block creation date must be more recent than the last certificate in `common` topic and
  in the `realm` topic corresponding to the block's realm.

> Notes:
>
> - It could be tempting to only lock the `realm` topic (given it's this topic that contains
>   the certificate describing the role of a user in the realm). However a revoked user is also
>   forbidden from adding data, hence making it mandatory to also lock the `common` topic here.
> - Uploading blocks (and vlobs) are the most common task on the server, so we may want to
>   optimise it in the future to avoid having to lock two topics.
>   A possible solution would be to have an optimistic locking approach: first read the
>   last timestamp in the topics, then do the checks, then on the final data insertion check
>   again the timestamp haven't changed (and if they have we re-do everything from the start).

### Vlobs

Vlobs are encrypted metadata that belong to a given realm. Unlike block they can be
modified and hence have a version field that must increase linearly.

Concurrency & causality is handle in a similar fashion as for blocks.

TODO: explain checkpoint logic, and concurrent vlob insertion causing unique violation on checkpoint

> Note:
>
> This may seem counter-intuitive given vlobs require a database write, but blocks require
> both a database write and upload to the object storage. However you can think of both vlob
> and block upload as:
>
> 1) Fetch info from database.
> 2) Do stuff in Python.
> 3) Store the vlob/block in the database.
>
> In the case of vlobs, step 2 is only about checking access rights and vlob existence.
> On the other hand blocks also includes the write to the object storage.
>
> However this is irrelevant from the database point of view: in both case step 3 is only
> allowed if data in step 1 hasn't changed.
>
> Obviously this approach means we can end up with data in the  object storage while the
> SQL transaction got cancelled, however this is acceptable:
>
> - Given the block ID is determined client-side, re-trying to upload the block will lead
>   to another write in the object storage at the same location.
> - The object storage follows an eventual consistency strategy, hence the new block upload
>   is guaranteed to overwrite the previous one.
> - The only issue would be if multiple attempts of the same block ID would be done with
>   different data on each try. In this case eventual consistency means it is possible
>   for a short period of time to get a previous value when querying the block.
>   However this is purely theoretical since block ID is random (hence there should be
>   no concurrency between clients) and a client considers a block as immutable.

### Invitations

During the process of invitation, one of the client (the claimer) doesn't have a device
yet (as it is the whole point of the invitation process !).
Hence using certificates during this step is not possible (since a certificate is signed by a device).

No certificate involvement means:

- Invitations data are totally separated from other data (given other data always relies
  on certificates to do part of there validation).
- Invitation request doesn't contains a client-generated timestamp.

This last point is great from a causality point of view:

- With certificates the client propose a change that occurred at a given timestamp, then it's
  up to the server to check for causality to know if it should accept it.
- Here the client merely ask the server do to something, then the server checks if it is
  possible *now*, then do it *now*. No causality can be broken given we only work in the present.

Regarding concurrency, locks are needed to protect the creation of invitation against:

- Concurrent invitation creation and user creation leading an invitation for an existing user.
  This is handled by simply taking the `common` topic lock.
- Concurrent invitation creations of the same email creating multiple invitations
  (see [example in Data concurrency paragraph](#data-concurrency)). This is handled
  by a dedicated `invitation_creation` advisory lock that must be taken before the
  invitation creation procedure starts any checks involving the invitations.

### What is locked by who ?

The following array recapitulate which topics (and other related locks) each command
needs to lock (and in which mode).

Topic locks: `common`, `realm`, `sequester`, `shamir`
Manual locks: `invitation_creation`
Special checks:

- `last_vlob_timestamp`: Compare the to-be-inserted data's timestamp with the timestamp
  of the most recent vlob.
- `user_revoke_safe`: See [User revocation and concurrent operation involving users](#user-revocation-and-concurrent-operation-involving-users)

<!-- See `misc/check_server_readme.py` script that checks this table is up to date ! -->
<!-- start-table-what-is-lock-by-who -->
API Family    |  command                               | common | realm | sequester | shamir | invitation_creation | user_revoke_safe | last_vlob_timestamp |
--------------|----------------------------------------|--------|-------|-----------|--------|---------------------|------------------|---------------------|
tos           | tos_accept                             |        |       |           |        |                     |                  |                     |
tos           | tos_get                                |        |       |           |        |                     |                  |                     |
invited       | ping                                   |        |       |           |        |                     |                  |                     |
anonymous     | ping                                   |        |       |           |        |                     |                  |                     |
authenticated | ping                                   |        |       |           |        |                     |                  |                     |
authenticated | events_listen                          |        |       |           |        |                     |                  |                     |
authenticated | certificate_get                        |        |       |           |        |                     |                  |                     |
anonymous     | organization_bootstrap                 | write  |       |           |        |                     |                  |                     |
authenticated | invite_list                            |        |       |           |        |                     |                  |                     |
authenticated | list_frozen_users                      |        |       |           |        |                     |                  |                     |
invited       | invite_info                            |        |       |           |        |                     |                  |                     |
authenticated | invite_new_device                      | read   |       |           |        | write               |                  |                     |
authenticated | invite_new_user                        | read   |       |           |        | write               |                  |                     |
authenticated | invite_new_shamir_recovery             | read   |       |           | read   | write               |                  |                     |
authenticated | invite_cancel                          |        |       |           |        |                     |                  |                     |
authenticated | invite_complete                        |        |       |           |        |                     |                  |                     |
authenticated | invite_greeter_cancel_greeting_attempt |        |       |           |        |                     |                  |                     |
authenticated | invite_greeter_start_greeting_attempt  |        |       |           |        |                     |                  |                     |
authenticated | invite_greeter_step                    |        |       |           |        |                     |                  |                     |
invited       | invite_claimer_cancel_greeting_attempt |        |       |           |        |                     |                  |                     |
invited       | invite_claimer_start_greeting_attempt  |        |       |           |        |                     |                  |                     |
invited       | invite_claimer_step                    |        |       |           |        |                     |                  |                     |
invited       | invite_shamir_recovery_reveal          |        |       |           |        |                     |                  |                     |
authenticated | device_create                          | write  |       |           |        |                     |                  |                     |
authenticated | user_create                            | write  |       |           |        |                     |                  |                     |
authenticated | user_revoke                            | write  |       |           |        |                     | sequential       | all user's realms   |
authenticated | user_update                            | write  |       |           |        |                     |                  |                     |
authenticated | realm_create                           | read   |       |           |        |                     | forbidden        |                     |
authenticated | realm_rename                           | read   | write |           |        |                     |                  |                     |
authenticated | realm_rotate_key                       | read   | write |           |        |                     |                  |                     |
authenticated | realm_share                            | read   | write |           |        |                     | forbidden        | target realm        |
authenticated | realm_unshare                          | read   | write |           |        |                     | forbidden        | target realm        |
authenticated | realm_get_keys_bundle                  |        |       |           |        |                     |                  |                     |
authenticated | vlob_poll_changes                      |        |       |           |        |                     |                  |                     |
authenticated | vlob_read_batch                        |        |       |           |        |                     |                  |                     |
authenticated | vlob_read_versions                     |        |       |           |        |                     |                  |                     |
authenticated | vlob_create                            | read   | read  |           |        |                     |                  |                     |
authenticated | vlob_update                            | read   | read  |           |        |                     |                  |                     |
authenticated | block_create                           | read   | read  |           |        |                     |                  |                     |
authenticated | block_read                             |        |       |           |        |                     |                  |                     |
anonymous     | pki_enrollment_info                    |        |       |           |        |                     |                  |                     |
authenticated | pki_enrollment_list                    |        |       |           |        |                     |                  |                     |
anonymous     | pki_enrollment_submit                  | read   |       |           |        |                     |                  |                     |
authenticated | pki_enrollment_accept                  | write  |       |           |        |                     |                  |                     |
authenticated | pki_enrollment_reject                  | read   |       |           |        |                     |                  |                     |
authenticated | shamir_recovery_setup                  | read   |       |           | write  |                     | forbidden        |                     |
authenticated | shamir_recovery_delete                 | read   |       |           | write  |                     |                  |                     |
CLI sequester | create_service                         |        |       | write     |        |                     |                  |                     |
CLI sequester | update_service                         |        |       | write     |        |                     |                  |                     |
<!-- end-table-what-is-lock-by-who -->

The key takeaways are:

- Read commands never take any lock. This means such a command may succeed after
  a concurrent operation has invalidate its checks, but this is considered acceptable
  since this has no impact on causality.
- `organization_bootstrap` is a special case since it is only used once when no certificates
  are present (hence no risk to break causality). However we still lock the `common`
  certificate (given this topic is created as part of the organization creation) to
  protect again concurrent bootstraps.
- Most other commands doing changes take a read lock on `common` topic. This is because,
  at the very least, the user the command originated from must not get revoked in the meantime !
- On top of that, commands realm/vlob/block commands work on a given realm, and hence must
  obviously lock the corresponding realm topic.
  Note the lock is write where changing the realm configuration, and only read when actually
  inserting data in the realm (i.e. block/vlob commands).
- However `realm_create` doesn't take any lock on realm topic, this is because the realm
  doesn't exist yet ! Note that, unlike for invitation creation, there is a unique
  constraint in the database on organization_id+realm_id which handles the concurrency
  creation of the same realm (hence no need for manual lock such as `invitation_creation`).
- Invite steps commands (e.g. `invite_1_greeter_wait_peer`) do not take a read lock on
  `common` topic.
  This is a limitation due to how the those methods are implemented (and RFC 1011 will fix
  this once implemented). This is not a big issue though: given the conduit exchange doesn't
  care about the date of modification, we can pretend the exchange succeeded before the
  concurrent operation causing the potential issue has started.
- Sequester configuration is only modified using the server CLI (which is used by a server
  administrator so in this case no user is involved, hence no lock of the `common` topic).

### Implementation: PostgreSQL

In PostgreSQL we have multiple tables (`common_topic`, `realm_topic` etc.) dedicated to
store the last timestamp for a given topic + discriminant.

From there, each operation lock the row in the topic tables using:

- `SELECT ... FOR UPDATE` if the given topic is going to be modified in the current operation.
- `SELECT ... FOR SHARE` otherwise.

The `SELECT ... FOR SHARE` allows for [concurrent read access on the row](https://www.postgresql.org/docs/current/explicit-locking.html#LOCKING-ROWS).
This way multiple vlob create/update operations on the same workspace can run concurrently,
but a single realm share prevents all vlob create/update operations.

Finally the lock for invitation & realm creation cannot be implemented by regular ROW locking
(given there is no row to lock !), so we use
[an advisory lock instead](https://www.postgresql.org/docs/current/explicit-locking.html).

```sql
SELECT pg_advisory_xact_lock(<lock ID>, _id) FROM organization WHERE organization_id = <org>
 pg_advisory_xact_lock_shared
```

### Implementation: Memory

The memory implementation relies on async locks (see `MemoryOrganization.topics_lock` method).

The topic can be locked in read or write modes to simulate more closely the `FOR UPDATE` VS
`FOR SHARE` locking system used in PostgreSQL.

PostgreSQL's advisory locks (i.e. the lock used for invitation & realm creation) is implemented
in a similar fashion (given it correspond to a lock always taken in write mode).

> Note:
>
> The memory implementation is almost entirely asynchronous.
>
> Given we only use a single thread in our application, then a synchronous piece of code
> can be seen as atomic (i.e. only `await` change the scheduling of coroutines, no `await`
> in an async function means this function will be entirely executed before anything else
> can run).
>
> Hence it could be tempting rely on this and entirely remove all locks...
>
> However this is not perfect: adding an `await` to an async function seems harmless, but in
> our case it would break the assumption !
>
> On top of that, event dispatch is asynchronous in memory (this is needed to follow more
> closely what happen in PostgreSQL when a event may be received long after the request
> that sent it has finished).
> This is theoretically not an issue nevertheless given sending event is always the very
> last operation done, at which point the modification in database (and their corresponding
> checks) have been done.
>
> Finally, there is currently one place where this strategy really breaks: in a sequestered
> organization with a webhook sequester service, vlob update & create needs to first
> send an HTTP request (hence this must be async !) to the webhook before storing the vlob.
>
> So in the end, it's simply cleaner and safer to rely on locks instead of this
> smart-kid-implicit-lock-free approach !
