# Use email as main human identifier

From [ISSUE-1011](https://github.com/Scille/parsec-cloud/issues/1011)

## Use email as main user ID

According to [RFC-0008](0008-smoothening-the-invite-process.md), using email during the user enrollment process seems to simply the process.

This could be occasion to solve another issue in the current Parsec architecture.

## The problem

Currently Parsec doesn't use email at all, humans are represented and interact with the system through User and Devices.

In Parsec User can be revoked for multiple reasons:

1) the corresponding human should no longer be part of the organization
2) the User's private data has been compromised

In point 2) what's really going on is an administrator is going to revoke the User in Parsec, then recreate a new User to the human can access again to the system.

In fact the same can occur with point 1) given it is entirely possible the human may come back to the organization later on.

This means we end up with multiple Users in Parsec with refers to the same human.
The trick is a User is defined by it unique UserID.
So for instance we can have human "Alice" first represented by User "alice", then by "alice2", then "alice-2020", then "AlicyMcAliceFace" etc.
Of course there is at most one User not revoked that represent a given human.
However 1) the name can be hard to tell and 2) other human may try to do impersonation by registering a better name (for instance creating "alice3" when "alice2" is the current valid UserID).

## The solution

The email would be used as a top level identifier on the human (a HumanHandle).
A given HumanHandle would be referenced by n Users, with at most 1 non-revoked User (this should be enforced by the backend API).

The UserCertificate would store the HumanHandle (and a human_name field to store the full name of the human provided by himself during enrollment and verified by the inviter as discussed in [RFC-0008](0008-smoothening-the-invite-process.md)).

For compatibility, those fields should be optional (defaulting if needed to the UserID).

The HumanHandle would be usable in the Backend API for user_get and user_find. Those APIs could be provided with a regular UserID or a HumanHandle, in which case HumanHandle is a pointer on the current non-revoked User.

## Get a readable UserID from the email ?

We could enforce rules on the user_id to be of the form `{human_handle}-{number}` in order to keep UserID readable for human.

However email is a really complex format (for instance ```!#$%&'*+-/=?^_`.{|}~@example.com``` is a valid email !) and UserID must be at most 32 characters long.
So it would mean to develop a cooking format to turn email into a readable UserID.

On top of that this would increase the complexity when creating a User for a human which already had a previous User revoked (given we would want to have both Users with the same UserID name pattern).
We need to have a cooking format not using randomness or to interrogate the backend (with all the concurrency issues that implies...) to get the previous UserID corresponding of the given email and patch it.

In the end I don't think it's worth it to go this way, using UUID for UserID linked to HumanHandle is just much simpler.
Another interesting point is using UUID may allow us to totally hide human identity to the Backend in the future (by having a secured entity separated from the backend storing the UserCertificates).

## Non-human actors

The current behavior (UserID not connected to HumanHandle) should be kept for two reasons

- compatibility
- allow non-human system to connect Parsec

Obviously the GUI hasn't to deal with those concerns so it hasn't to keep the old enrollment system (only the CLI should keep it)
