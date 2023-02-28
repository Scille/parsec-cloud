# Make backend responsible for timestamp

From [ISSUE-908](https://github.com/Scille/parsec-cloud/issues/908)

## Intro

First a bit of definition: a timestamps in Parsec is a datetime that mark a
signed document sent to the backend. Currently there is three kind of documents
that contains a timestamp:

- manifests: which are send encrypted to the backend and stored as vlob
- messages: which are also send encrypted and destined to a specific user
- certificates: which are provided in cleartext to the backend

## Current situation

Currently those timestamps are controlled by the client generating the document.
This is the obvious way to go given the timestamp is part of the signed document
so it cannot be modified by the backend !

So there is two different scenarios when uploading a new document:

1) For certificates: given the document is in cleartext, the backend can extract
   the timestamp and verify it is in range with the current timestamp
2) For manifests&messages: the document is encrypted, so the client must also
   provide to the backend the timestamp along with the document.
   This way backend can do the range check. Later when another client retrieve
   the data from the backend, it will receive both the document and the clear
   timestamp so it can verify the client provided signed the document with the
   same timestamp he provided to the backend.

However this approach is not a perfect solution:

- each client has it own clock, so we can have client with a clock lagging
  behind or ahead
- the lag in the connection between client and server

Those two points mix together to create something insolvable: solving the
bad clock issue would require to reduce the shift allowed between client
timestamp and backend current time, but this would rule out client with slow
connections (e.g. 4G mobile).

In order to mitigate this issue, a check has been added in the backend to refuse
update vlob update with timestamp set prior to previous modification's one (see PR-758).

In the end this timestamp can be seen as a window time within it the action
took place:

```txt
timestamp - TIMESTAMP_MAX_DT < real_timestamp < timestamp + TIMESTAMP_MAX_DT
```

Here `real_timestamp` designates the timestamp when the backend registered
the change in it database. So we could also simply called it `backend_timestamp`.

## Why it's a trouble

As said above, the timestamp cannot strictly be relied on.

### Scenario 1: role + vlob modifications

Considering the following scenario:

T1: `Alice` gives access to `Bob` to workspace by sending a certif to the
backend with a timestamp Tx
T2: `Bob` modify data in the workspace by sending a manifest to the backend
with a timestamp Ty

With T1 and T2 roughly within TIMESTAMP_MAX_DT (which is possible given currently
TIMESTAMP_MAX_DT == 30s !)
If `Alice`'s clock is wrong and `Bob` one's right, we could have Tx > T2 and Ty == T2.
On the contrary, if `Bob`'s clock is wrong, we could have Tx == T2 and Ty < T2.

In both cases we end up with a modification that appears to have occurred before
`Bob` has the right to do it.

### Scenario 2: only role modifications

Now consider another scenario:

T1: `Alice` gives access to `Bob` to a workspace with certif timestamp Tx
T2: `Mike` removes `Bob` access  to the same workspace with certif timestamp Ty

If we have Tx == T1 and Ty < T1, `Bob` won't get it access removed (we see `Bob`
was first removed from the workspace, then added...)

## Improvement: the easy way

### More checks

We could generalize what has been done for vlobs in PR-758 by enforcing checks in the backend:

1) new role certif timestamp > previous role certif timestamp of the same user/workspace
2) vlob timestamp > user role certif timestamp of the modified workspace
3) device timestamp <= all signed data by this device

Check 1) would trivially mitigate scenario 2), while check 2) scenario 1).

### The History question

This way the only inconsistent order we could have would be between unrelated
resources (for instance vlob 1  has been modified before vlob 2, but timestamps
say otherwise).
Obviously "unrelated resources" is not totally true, for instance if we're
looking for the history of a path at a given timestamp, we end up with multiple
vlob (each representing a part of the path). So we could have something like this:

- `foo` with v1 with timestamp Tf1 and v2 with timestamp Tf'2
- `bar.txt`, child of `foo` v1, with v1 with timestamp Tb1 and v2 with timestamp Tb2
- another entry id `bar.txt`, child of `foo` v2, with v1 with timestamp Tb'3 and v2 with timestamp Tb'4

Now say we want history of `foo/bar.txt` at Tx.

If Tf'2 <= Tx, we will fetch the second `bar.txt` entry, but this one may have
Tb'3 > Tx.
I guess there is plenty of other strange usecases where looking at history at a
given time gives different results depending on the algorithm and the path
(from parent to child or the other way around) we choose.

#### The time frame everywhere solution

A solution for the history question could be to always consider thing with
a time window. This way having multiple events at the same time (well, within
the same time frame) is considered a perfectly normal thing.

Obviously this would necessitate to turn TIMESTAMP_MAX_DT into a hard constant
that cannot be changed.

I guess introducting a Timestamp class to represent and take into account
jitter would help things:

```python
t1 = pendulum.now()
t2 = t1.add(seconds=1)
t3 = t1.add(seconds=100)
ts1 = Timestamp(t1)
ts2 = Timestamp(t2)
ts3 = Timestamp(t3)
assert t1 == t2
assert t1 != t2
assert t1.lower_bound == t1.add(seconds=-TIMESTAMP_MAX_DT)
assert t1.upper_bound == t1.add(seconds=+TIMESTAMP_MAX_DT)
```

The hard point would probably to find a wind to represent multiple events
at the same time in the GUI...

#### The ordering solution

A way to solve the the "time frame means multiple simultaneous events" trouble
would be to introduce a rule for ordering events within the same time frame.
For instance if vlob 42 v2 is modified within the same time frame that vlob 66 v5,
we will alway consider vlob 42 v2 occurred before vlob 66 v5 because 42 < 66.

A better rule could be to take the lower bound of the time frame.
Another more intelligent rule would be to order from parent to child but
this is much more complicated to enforce given relationships are not always
visible at first sight...

I guess we should explore more those kind of solutions

## Improvement: the hard way

The starting point is simple: we lack a single, centralized clock.
So everything would be solved if the backend would be in charge of the timestamp.

Strictly speaking, it's not a 100% perfect solution given we can still have
concurrency when running multiple instances of the backend, but I guess this
can be neglected.
Same thing with timing lag between the timestamp generated in the backend code
and the actual time the data are stored in the database.

Beside, this doesn't totally save us from the messy history problem given the
backend being in charge of the timestamps, once compromised it can specifically
choose bad timestamps to blur the history.

Given how currently works the encrypted data, the implementation on them is not
really complex (just store in the backend it own timestamp instead of the one
provide by the client which is now just check once before database insertion).

However the impact is much bigger on the certificates:

- each api involving certificate create/read has to be changed to return the
  backend timestamp. This must be done in a way that keep compatibility with
  older version so we must add plenty of new fields.
- trustchain handling code in core has to be changed to pass new fields around
- local storage (see below)
- test fixtures (see below)

On top of that, this give the `BaseAPISignedData` based classes a weird behavior:

```python
# Before
obj = MySignedData(
    author=author_id,
    timestamp=now,
    data="whatever"
)
serialized = obj.dump_and_sign(author_priv_key)
obj2 = MyCertif.verify_and_load(serialized, author_pub_key)

# After
obj = MySignedData(
    author=author_id,
    timestamp=now,  # We create the certif, so client timestamp here
    data="whatever"
)
serialized = obj.dump_and_sign(author_priv_key)
# Now we must provide backend timestamp during deserialization
obj2 = MyCertif.verify_and_load(serialized, author_pub_key, backend_timestamp)
# But once deserialized, we want to serialize it again (typically to store it into local storage)
serialized2 = obj2.dump_and_encrypt(local_storage_key)
```

So before we had a simple equivalence between the serialized and object versions
of the certificate. After the change this is no longer the case :'-(

In fact we may even divide the after `MySignedData` class into two kinds (or maybe
add a flag into it) to mark the difference between object with client timestamp
or backend timestamp. Typically `MySignedData` and `LocalMySignedData`, `MySignedData`
being the object equivalent to the deserialized data (with client timestamp)
and local the one with backend timestamp that aims at being stored into the
local storage.

I guess this would complexify the code base, especially the tests (given how
we mix core and backend data in the fixtures...)

For instance:
<https://github.com/Scille/parsec-cloud/blob/cf831083182b65b670ff9ea51a18e998d1230ef4/tests/backend/user/test_device_get_invitation_creator.py#L106-L108>

Here we should add new fields `devices_trustchains`, `users_trustchains` and
`revoked_users_trustchains` (each being a list of timestamps). However we must
sort those field in the same order as the sort being done on there corresponding
certificate field (e.g. `devices[0]` and `devices_trustchains[0]` should point
to the same stuff)... this is getting out of hand :'-(
