# Automatic user revoke

## Abstract

This document describes design and implementation considerations to enable
automatic user revoke based on an external service.

An organization deploying Parsec on-premise needs to be able to automatically
revoke a user when it leaves the organization. The idea is to avoid a manual
task that may be forgotten by the organization administrators.

## 0 - Basic considerations

Parsec could expose a service allowing to revoke a user (or a list of users)
of a given organization. This service may be triggered by an external tool,
such as a web hook or a cron job, depending on the organization needs.

Another possibility will be to expose a service that will "only" notify the
administrator that one or more user need to be revoked. The administrator will
have to manually perform revocation, but it will be constantly "remainded"
(for example via permanent notification) that his action needs to be performed.
This is somehow similar to the current "re-encryption" action that needs to be
performed by workspaces owner.

It is assumed that the organization deploying Parsec has already a way to
identify the person leaving. A specific development (outside Parsec scope)
will probably be required in order to link the person leaving with its
corresponding Parsec user.

## 1 - Current constraints

This looks a little more complicated than expected, because of the following
constraints:

- Revocation is a **user action**, i.e. it can only be triggered by a logged-in
  administrator and not by the server itself.

- Once a user has been revoked, all workspaces in which he/she has participated
  need to be **re-encrypted**. Again, this is a user action, but with an
  additional constraint: only the workspace owners can re-encrypt it (not the
  administrators).

## 2 - Feature proposal

**TBC**

### A - Parsec service enabling automatic user revoke

The user revoke could be either *user-agnostic* (does not require user action)
or performed by some kind of *bot* (automated user) having the rights to revoke
users.

### B - Parsec service enabling notifications of user revoke to be performed

In this case, user revoke is still manually performed by an administrator.
Parsec will keep track of users to be revoked and will display it as a
notification or in a specific page for the administrators.
