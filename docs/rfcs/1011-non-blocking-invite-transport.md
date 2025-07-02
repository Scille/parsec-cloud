<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Non-blocking invite transport

## Overview

This RFC describes the new invite transport model, adapted to work with non-blocking HTTP routes and providing a better semantic for handling attempts at building a secure channel between peers.

## Background & Motivation

The invite system is based on the creation of a secure channel between two peers, a claimer and a greeter. This is done using the Short Authentication String algorithm (SAS) and requires a couple of messages to be exchanged by the peers, along with some acknowledgments.

In the following paragraphs, we'll use the term `invite protocol` to describe the rules that define the **content** of those messages and what the clients are expected to do with them. We'll use the term `invite transport` to describe the semantics behind the **handling** of those messages between the clients and the server. This RFC is mostly about the `invite transport` while `invite protocol` is almost left untouched.

In Parsec v2, the communication between the clients and the server is based on websockets which allows for building blocking APIs. The `invite transport` is based on blocking commands for each step of the message passing protocol. Each step consists of a handshake between the peers where two messages are exchanged. The corresponding command is blocking while waiting for the other peer. If one of the peers is disconnected during the process, the attempt is reset and the peers have to start over from the beginning.

In Parsec v3, the communication between the clients and the server is now based on regular HTTP routes (along with Server-Sent Events). Blocking HTTP requests would be very cumbersome considering that gateways are basically free to close the connection however they please after a while. Worse, a connection loss in the context of the building of an invitation channel would cause the process to start over. Since the invite process already weighs heavily on the user experience, keeping this unreliable process is highly unadvisable. A more thorough analysis is available in the [issue #7018 - Update the internal invitation logic for better network failure resilience](https://github.com/Scille/parsec-cloud/issues/7018).

In addition, we can notice that the constraint of having both peers connected to the server at the same time in order to make the exchange doesn't provide any substantial gain. On the contrary, it's fine to have both peers simply depositing and retrieving their payload as they please, as long as it is done in a consistent way. The retrieval is the blocking part, which can be either implemented with a blocking request or non-blocking one, returning a "not ready" status when the payload is not available. This is the approach used here in order to avoid dealing with blocking requests altogether (although it would be valid in this context since retrying after a network error is mostly equivalent to retrying after a "not ready" status).

Since the invite transport is being redesigned, we're including two more changes to this RFC. First, we need to add a reason for an attempt being cancelled by a peer. This is useful for the front-end application to properly communicate to the user what actually happened. This reason did not exist in v2 and has to be added in v3. More precisely, we can notice that the semantics of "attempts" is absent from the v2, even though the process of creating a secure channel between peers can still be cancelled and/or reset. Without dedicated semantics, it is hard to properly manage edge cases such as two devices for the same user trying to create a secure channel at the same time. For this reason, a new attempt semantics is defined here such that each attempt is properly identified and dealt with separately.

The second change is related to the concept of greeters. In Parsec v2, only the administrator that created an invitation is able to greet the invited user. This is something we want to change to open the way to easier invitations, where any administrator can greet a user that has been invited. This may be configurable of course, but the transport should allow for multiple greeters per invitation. Note that this is already the case for Shamir invitations, where the greeters are the Shamir recipients. For this reason, the new design can draw inspiration from what was done during the Shamir implementation. More precisely, each invitation will have its corresponding set of greeters which is different for each invitation type:
- for user invitation, this would be the administrators
- for device invitation, this would be the user alone
- for Shamir invitation, this would be the Shamir recipients

> [!NOTE]
> This set may change during the lifetime of a given invitation. This specifically affects Shamir invitations (where recipients can get revoked) and user invitations (where users can loose their administrator profile).


## Goals and Non-Goals

To sum it up, the goal is to design a new invite transport that:
- allows for network failures by removing the simultaneity constraints of Parsec v2 and using non-blocking HTTP requests
- adds new semantics to manage attempts distinctly and allow for cancelling them with a reason that's communicated to the other peer
- decouples the concept of greeter from the invitation author so that a user can be greeted by any administrator


## Terminology

Both terms `invite` and `invitation` are used in the code base. There are basically three usage of those words:

1. `invite` as verb, as in "an administrator invites a user to join an organization"
  * example: the text for `NewUserInvitationError::NotAllowed` is `Not allowed to invite a user`

2. `invite` as a noun, referring to the invite system in general
  * example: the commands related to the invite system are named `invite_*`

3. `invitation`, referring to a specific invitation that has to managed
  * example: most invite commands have return status called `invitation_not_found` or `invitation_deleted`

In order to avoid too many changes in the data model, we'll stick to this terminology and apply it in the implementation of this RFC.

## Semantics

This section offers a new semantics proposal, without going into the technical aspects of the exposed routes and their implementations.

Each concept is described with their identifier, immutable properties, internal state, corresponding getters and exposed methods.


### Invitation

An invitation is used to verify the identity of a user (called the claimer) on a specific device. The users allowed to verify the identity are called greeters. The invitation can be of 3 types:
- User invitation
- Device invitation
- Shared recovery (Shamir) invitation

An invitation is identified by:
- organization id
- invitation token

Immutable properties:
- type (user, device, Shamir)
- created-on timestamp
- created-by author
- claimer id:
  * email address (for user invitation)
  * or user id (for device or Shamir invitation)

Mutable state:
- state: `PENDING` | `CANCELLED` | `COMPLETED`

Getters:
- get greeters
  * for user invitation: all administrators
  * for device invitation: same as claimer id
  * for Shamir invitation: all recipients
- get greeting session for greeter

Methods:
- create invitation
- cancel invitation
- complete invitation


### Greeting session

A greeting session is dedicated to the verification of the claimer identity by a specific greeter.

A greeting session is identified by:
- organization id
- invitation token
- greeter id

Mutable properties:
- greeting attempts ids: list of greeting attempts
   * Invariant: only the most recent greeting attempt is active, all the rest is cancelled

Getters:
- get active greeting attempt

Methods:
- cancel active greeting attempt (and create a new one)


### Greeting attempt

A greeting attempt is the steps followed by the peers of a given greeting session when trying to establish a secure communication channel.

A new type of id called "greeting attempt id" is introduced to identify uniquely a specific greeting attempt in the context of a given organization.

Identified by:
- organization id
- greeting attempt id

Immutable properties:
- invitation token
- greeter id

Mutable properties:
- claimer joined: `NotSet` | `Timestamp`
- greeter joined: `NotSet` | `Timestamp`
- cancelled reason: `NotSet` | (`CLAIMER` | `GREETER`, `Reason`, `Timestamp`)

Inferred properties:
- state: `IDLE` | `HALF_JOINED` | `FULLY_JOINED` | `CANCELLED`
- is active: `True` if it is either `IDLE`, `HALF_JOINED` or `FULLY_JOINED`

Getters:
- get steps

Methods:
- claimer join greeting attempt
- greeter join greeting attempt
- claimer cancel greeting attempt
- greeter cancel greeting attempt


### Step

A step corresponds to the exchange of two messages between the peers in the context of a greeting attempt.

A step is identified by:
- organization id
- greeting attempt id
- step count

Immutable properties:
- invitation token
- greeter id

Mutable properties:
- claimer data: `NotSet` | `ClaimerStep`
- greeter data: `NotSet` | `GreeterStep`

Inferred properties:
- state: `PENDING` | `HALF_PENDING` | `COMPLETED`

Methods:
- set claimer data
- set greeter data


## Design

### New commands

7 commands are added in total:
- 3 new invited commands
    - [`invite_claimer_start_greeting_attempt`](#invite_claimer_start_greeting_attempt-command)
    - [`invite_claimer_cancel_greeting_attempt`](#invite_claimer_cancel_greeting_attempt-command)
    - [`invite_claimer_step`](#invite_claimer_step-command)
- 4 new authenticated commands
    - [`invite_greeter_start_greeting_attempt`](#invite_greeter_start_greeting_attempt-command)
    - [`invite_greeter_cancel_greeting_attempt`](#invite_greeter_cancel_greeting_attempt-command)
    - [`invite_greeter_step`](#invite_greeter_step-command)
    - [`invite_complete`](#invite_complete-command)

#### `invite_greeter_start_greeting_attempt` command

Two new commands are introduced to start a greeting attempt, which returns a greeting attempt ID used in the subsequent queries. This greeting attempt ID is the same for the greeter and the claimer, it is a UUID that uniquely identifies the greeting attempt in the context of the organization. For the greeter, an authenticated command is exposed.

In order for a greeter to start a greeting attempt, a couple of checks have to be performed:
- the invitation exists
- the invitation is pending (i.e. not completed or cancelled)
- the author is allowed (i.e. it is part of the greeters)

It's possible for the active greeting attempt to be in a state where the greeter has already joined. Usually, previous attempts are expected to be cancelled by one of the peers, but one can think of many cases where this cancellation didn't occur (e.g. the greeter device shutdown, or another device has an ongoing attempt running that hasn't been cancelled by the user).

In this case, the `invite_greeter_start_greeting_attempt` command cancels the active greeting attempts automatically (using a dedicated reason), and joins the new active greeting attempt. This means that the last device calling `start_greeting_attempt` always has the priority.

If no cancellation of the active greeting attempt is required, the greeter simply joins it and the command returns the corresponding greeting attempt ID.

Pseudo-code:

```python
def invite_greeter_start_greeting_attempt(
    token: InvitationToken,
) ->
  InvitationNotFound | InvitationCompleted | InvitationCancelled
  GreeterNotAllowed | OK[GreetingAttemptID]:
    # If the greeter has already joined the active greeting attempt:
    #    -> cancel the active greeting attempt and create a new one
    [...]
    # Get the active greeting attempt and call `greeter_join_greeting_attempt()`
    [...]
    # Return the greeting attempt ID
    [...]
```

Schema definition:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_greeter_start_greeting_attempt",
            "fields": [
                {
                    "name": "token",
                    "type": "InvitationToken"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "greeting_attempt",
                        "type": "GreetingAttemptID"
                    }
                ]
            },
            {
                // The invitation token doesn't correspond to any existing invitation
                "status": "invitation_not_found"
            },
            {
                // The invitation has already been completed
                "status": "invitation_completed"
            },
            {
                // The invitation has been cancelled
                "status": "invitation_cancelled"
            },
            {
                // The author is not part of the allowed greeters for this invitation
                "status": "author_not_allowed"
            }
        ]
    }
]
```


#### `invite_claimer_start_greeting_attempt` command

This is the new invited command introduced for the claimer to start a greeting attempt and get the corresponding greeting attempt ID.

In order for the claimer to start a greeting attempt, a couple of checks have to be performed:
- the greeter exists
- the greeter is allowed (i.e. it is part of the greeters)

> [!NOTE]
> The state of the invitation doesn't have to be checked since it is already checked by the `/invited` HTTP route.

> [!NOTE]
> The claimer chooses the greeter ID using the information it received from calling the existing `invite_info` command.

Otherwise this command has the same logic as `invite_greeter_start_greeting_attempt`.

Pseudo-code:

```python
def invite_claimer_start_greeting_attempt(
    token: InvitationToken,
    greeter: UserID,
) ->
  InvitationNotFound | InvitationCompleted | InvitationCancelled |
  GreeterNotFound | GreeterRevoked | GreeterNotAllowed
  OK[GreetingAttemptID]:
    # Perform checks
    [...]
    # If the claimer has already joined the active greeting attempt:
    #    -> cancel the active greeting attempt and create a new one
    [...]
    # Get the active greeting attempt and call `claimer_join_greeting_attempt()`
    [...]
    # Return the greeting attempt ID
    [...]
```

Schema definition:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_claimer_start_greeting_attempt",
            "fields": [
                {
                    "name": "greeter",
                    "type": "UserID"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "greeting_attempt",
                        "type": "GreetingAttemptID"
                    }
                ]
            },
            // The following statuses do exist for the greeter, but not for the claimer
            // Instead, in those cases, the claimer would get an HTTP 410 error, defined
            // as `InvitationAlreadyUsedOrDeleted`.
            // {
            //     // The invitation token doesn't correspond to any existing invitation
            //     "status": "invitation_not_found"
            // },
            // {
            //     // The invitation has already been completed
            //     "status": "invitation_completed"
            // },
            // {
            //     // The invitation has been cancelled
            //     "status": "invitation_cancelled"
            // },
            {
                // The provided greeter ID doesn't correspond to any existing greeter
                "status": "greeter_not_found"
            },
            {
                // The greeter has been revoked from the organization
                "status": "greeter_revoked"
            },
            {
                // The greeter is not part of the allowed greeters for this invitation
                // An example of valid case for this error happens for a user invitation,
                // if the profile of the chosen greeter changes from ADMIN to NORMAL after
                // `invite_info` was called by the claimer
                "status": "greeter_not_allowed"
            }
        ]
    }
]
```


#### `invite_greeter_cancel_greeting_attempt` command

Two new commands are introduced to cancel an attempt, which registers a cancellation reason for the given greeting attempt. For the greeter, an authenticated command is exposed. After using `invite_greeter_cancel_greeting_attempt`, the greeter can start a new greeting attempt by using the `invite_greeter_start_greeting_attempt` command.

In order for a greeter to cancel a greeting attempt, a couple of checks have to be performed:
- the invitation exists
- the invitation is pending (i.e. not completed or cancelled)
- the author is allowed (i.e. it is part of the greeters)
- the greeting attempt exists
- the greeting attempt is active (i.e. joined and not cancelled)

Then, the active greeting attempt is cancelled with the provided reason and a new active greeting attempt is created. The cancellation reason covers several cases:
- the user manually cancelled the greeting attempt
- the hashed nonce didn't match the provided nonce
- the SAS code communicated to the user was invalid
- the payload could not be deciphered
- the payload could not be deserialized
- the payload contained inconsistent information
- the greeting attempt has been automatically cancelled by a new `start_greeting_attempt` command

It is also registered that the greeter cancelled the greeting attempt, so that this information can be provided in subsequent `invitation_cancelled` replies, along with the corresponding reason and timestamp. This way, the front-end application has all the information to properly communicate to the user what happened during the greeting attempt.

Pseudo-code:

```python
def invite_greeter_cancel_greeting_attempt(
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) ->
  InvitationCompleted | InvitationCancelled
  GreetingAttemptNotFound | GreetingAttemptNotJoined | GreetingAttemptCancelled
  GreeterNotAllowed | OK:
    # Perform checks
    [...]
    # Call `greeter_cancel_greeting_attempt(reason)`
    [...]
```

Schema definition:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_greeter_cancel_greeting_attempt",
            "fields": [
                {
                    "name": "greeting_attempt",
                    "type": "GreetingAttemptID"
                },
                {
                    "name": "reason",
                    "type": "CancelledGreetingAttemptReason"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            {
                // The invitation has already been completed
                "status": "invitation_completed"
            },
            {
                // The invitation has been cancelled
                "status": "invitation_cancelled"
            },
            {
                // The author is no longer part of the allowed greeters for this invitation
                // An example of valid case for this error happens for a user invitation,
                // if the profile of the author changes from ADMIN to NORMAL after
                // `invite_greeter_start_greeting_attempt` was called by the claimer
                "status": "author_not_allowed"
            },
            {
                // The greeting attempt id doesn't correspond to any existing attempt
                "status": "greeting_attempt_not_found"
            },
            {
                // The author did not join the greeting attempt
                // This should not happen, since joining is required to get the greeting attempt ID
                "status": "greeting_attempt_not_joined"
            },
            {
                // The greeting attempt has been already cancelled
                "status": "greeting_attempt_already_cancelled",
                "fields": [
                    {
                        "name": "origin",
                        "type": "GreeterOrClaimer"
                    },
                    {
                        "name": "timestamp",
                        "type": "DateTime"
                    },
                    {
                        "name": "reason",
                        "type": "CancelledGreetingAttemptReason"
                    }
                ]
            }
        ]
    }
]
```


#### `invite_claimer_cancel_greeting_attempt` command

This is the new invited command introduced for the claimer to cancel a greeting attempt with a provided reason.

In order for the claimer to cancel a greeting attempt, a couple of checks have to be performed:
- the greeter exists
- the greeter is allowed (i.e. it is part of the greeters)
- the greeting attempt exists
- the greeting attempt is active (i.e. joined and not cancelled)

> [!NOTE]
> The state of the invitation doesn't have to be checked since it is already checked by the `/invited` HTTP route.

Otherwise this command has the same logic as `invite_greeter_cancel_greeting_attempt`.

Pseudo-code:

```python
def invite_claimer_cancel_greeting_attempt(
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) ->
  InvitationCompleted | InvitationCancelled
  GreetingAttemptNotFound | GreetingAttemptNotJoined | GreetingAttemptCancelled
  GreeterRevoked | GreeterNotAllowed | OK:
    # Perform checks
    [...]
    # Call `claimer_cancel_greeting_attempt(reason)`
    [...]
```

Schema definition:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_claimer_cancel_greeting_attempt",
            "fields": [
                {
                    "name": "greeting_attempt",
                    "type": "GreetingAttemptID"
                },
                {
                    "name": "reason",
                    "type": "CancelledGreetingAttemptReason"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            // The following statuses do exist for the greeter, but not for the claimer
            // Instead, in those cases, the claimer would get an HTTP 410 error, defined
            // as `InvitationAlreadyUsedOrDeleted`.
            // {
            //     // The invitation has already been completed
            //     "status": "invitation_completed"
            // },
            // {
            //     // The invitation has been cancelled
            //     "status": "invitation_cancelled"
            // },
            {
                // The greeter has been revoked from the organization
                "status": "greeter_revoked"
            },
            {
                // The greeter is no longer part of the allowed greeters for this invitation
                // An example of valid case for this error happens for a user invitation,
                // if the profile of the greeter changes from ADMIN to NORMAL after
                // `invite_claimer_start_greeting_attempt` was called by the claimer
                "status": "greeter_not_allowed"
            },
            {
                // The greeting attempt id doesn't correspond to any existing attempt
                "status": "greeting_attempt_not_found"
            },
            {
                // The author did not join the greeting attempt
                // This should not happen, since joining is required to get the greeting attempt ID
                "status": "greeting_attempt_not_joined"
            },
            {
                // The greeting attempt has already been cancelled
                "status": "greeting_attempt_already_cancelled",
                "fields": [
                    {
                        "name": "origin",
                        "type": "GreeterOrClaimer"
                    },
                    {
                        "name": "timestamp",
                        "type": "DateTime"
                    },
                    {
                        "name": "reason",
                        "type": "CancelledGreetingAttemptReason"
                    }
                ]
            }
        ]
    }
]
```


#### `invite_greeter_step` command

This authenticated command, along with `invite_claimer_step`, is the main route used to establish a secure channel between both peers.

It is used by the greeter to submit the data for a given step. The step is identified by:
- the provided greeting attempt id
- the type of the submitted step data

For the step data to be accepted, it has to pass the following checks:
- the invitation exists
- the invitation is pending (i.e. not completed nor cancelled)
- the author is allowed (i.e. it is part of the greeters)
- the greeting attempt exists
- the greeting attempt is active (i.e. joined and not cancelled)
- the previous steps must have been completed by both peers

Once the data is accepted, it either returns:
- the claimer step data if it is available
- a `not_ready` status otherwise

In the second case, the author can poll the command with the same parameters until it gets the claimer step data.

If the submitted step differs from the one that has already been registered, a dedicated `step_mismatch` status is returned.

Pseudo-code:

```python
def invite_greeter_step(
    greeting_attempt: GreetingAttemptID,
    step: GreeterStep,
) ->
  InvitationCompleted | InvitationCancelled
  GreeterNotAllowed
  GreetingAttemptNotFound | GreetingAttemptNotJoined | GreetingAttemptCancelled
  StepTooAdvanced | PayloadMismatch
  NotReady | OK[ClaimerStep]:
    # Perform checks
    [...]
    # Call `set_greeter_payload`
    [...]
    # Return `greeter_step` or `NotReady`
    [...]
```

Schema definition:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_greeter_step",
            "fields": [
                {
                    "name": "greeting_attempt",
                    "type": "GreetingAttemptID"
                },
                {
                    "name": "greeter_step",
                    "type": "GreeterStep"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "claimer_step",
                        "type": "ClaimerStep"
                    }
                ]
            },
            {
                // The claimer has not submitted its step yet
                "status": "not_ready"
            },
            {
                // The invitation has already been completed
                "status": "invitation_completed"
            },
            {
                // The invitation has been cancelled
                "status": "invitation_cancelled"
            },
            {
                // The author is no longer part of the allowed greeters for this invitation
                // An example of valid case for this error happens for a user invitation,
                // if the profile of the author changes from ADMIN to NORMAL after
                // `invite_greeter_start_greeting_attempt` was called by the claimer
                "status": "author_not_allowed"
            },
            {
                // The greeting attempt id doesn't correspond to any existing attempt
                "status": "greeting_attempt_not_found"
            },
            {
                // The author did not join the greeting attempt
                // This should not happen, since joining is required to get the greeting attempt ID
                "status": "greeting_attempt_not_joined"
            },
            {
                // The greeting attempt has been cancelled
                "status": "greeting_attempt_cancelled",
                "fields": [
                    {
                        "name": "origin",
                        "type": "GreeterOrClaimer"
                    },
                    {
                        "name": "timestamp",
                        "type": "DateTime"
                    },
                    {
                        "name": "reason",
                        "type": "CancelledGreetingAttemptReason"
                    }
                ]
            },
            {
                // The submitted step is too advanced
                // Every step before must have been completed by both peers
                "status": "step_too_advanced"
            },
            {
                // The submitted step somehow changed during polling
                "status": "step_mismatch"
            }
        ],
        "nested_types": [
            {
                // GreeterStep should be identical to the one in invite_claimer_step.json5
                "name": "GreeterStep",
                "discriminant_field": "step",
                "variants": [
                    [...]  // This is described in the protocol update section
                ]
            },
            {
                // ClaimerStep should be identical to the one in invite_claimer_step.json5
                "name": "ClaimerStep",
                "discriminant_field": "step",
                "variants": [
                  [...]  // This is described in the protocol update section
                ]
            }
        ]
    }
]
```

#### `invite_claimer_step` command

This invited command, along with `invite_greeter_step`, is the main route used to establish a secure channel between both peers.

It is used by the claimer to submit the data for a given step. The step is identified by:
- the provided greeting attempt id
- the type of the submitted step data

For the step data to be accepted, it has to pass the following the checks:
- the greeter exists
- the greeter is allowed (i.e. it is part of the greeters)
- the greeting attempt exists
- the greeting attempt is active (i.e. joined and not cancelled)
- the previous steps must have been completed by both peers

> [!NOTE]
> The state of the invitation doesn't have to be checked since it is already checked by the `/invited` HTTP route.

Once the data is accepted, it either returns:
- the greeter step data if it is available
- a `not_ready` status otherwise

In the second case, the author can poll the command with the same parameters until it gets the greeter step data.

If the submitted step differs from the one that has already been registered, a dedicated `step_mismatch` status is returned.

Pseudo-code:

```python
def invite_claimer_step(
    greeting_attempt: GreetingAttemptID,
    step: ClaimerStep
) ->
  InvitationCompleted | InvitationCancelled
  GreeterRevoked | GreeterNotAllowed
  GreetingAttemptNotFound | GreetingAttemptNotJoined | GreetingAttemptCancelled
  StepTooAdvanced | PayloadMismatch
  NotReady | OK[GreeterStep]:
    # Perform checks
    [...]
    # Call `set_claimer_payload`
    [...]
    # Return `greeter_step` or `NotReady`
    [...]
```

Schema definition:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_claimer_step",
            "fields": [
                {
                    "name": "greeting_attempt",
                    "type": "GreetingAttemptID"
                },
                {
                    "name": "claimer_step",
                    "type": "ClaimerStep"
                }
            ]
        },
        "reps": [
            {
                "status": "ok",
                "fields": [
                    {
                        "name": "greeter_step",
                        "type": "GreeterStep"
                    }
                ]
            },
            {
                // The claimer has not submitted its step yet
                "status": "not_ready"
            },
            // The following statuses do exist for the greeter, but not for the claimer
            // Instead, in those cases, the claimer would get an HTTP 410 error, defined
            // as `InvitationAlreadyUsedOrDeleted`.
            // {
            //     // The invitation has already been completed
            //     "status": "invitation_completed"
            // },
            // {
            //     // The invitation has been cancelled
            //     "status": "invitation_cancelled"
            // },
            {
                // The greeter has been revoked from the organization
                "status": "greeter_revoked"
            },
            {
                // The greeter is no longer part of the allowed greeters for this invitation
                // An example of valid case for this error happens for a user invitation,
                // if the profile of the greeter changes from ADMIN to NORMAL after
                // `invite_claimer_start_greeting_attempt` was called by the claimer
                "status": "greeter_not_allowed"
            },
            {
                // The greeting attempt id doesn't correspond to any existing attempt
                "status": "greeting_attempt_not_found"
            },
            {
                // The author did not join the greeting attempt
                // This should not happen, since joining is required to get the greeting attempt ID
                "status": "greeting_attempt_not_joined"
            },
            {
                // The greeting attempt has been cancelled
                "status": "greeting_attempt_cancelled",
                "fields": [
                    {
                        "name": "origin",
                        "type": "GreeterOrClaimer"
                    },
                    {
                        "name": "timestamp",
                        "type": "DateTime"
                    },
                    {
                        "name": "reason",
                        "type": "CancelledGreetingAttemptReason"
                    }
                ]
            },
            {
                // The submitted step is too advanced
                // Every step before must have been completed by both peers
                "status": "step_too_advanced"
            },
            {
                // The submitted step somehow changed during polling
                "status": "step_mismatch"
            }
        ],
        "nested_types": [
            {
                // GreeterStep should be identical to the one in invite_greeter_step.json5
                "name": "GreeterStep",
                "discriminant_field": "step",
                "variants": [
                    [...]  // This is described in the protocol update section
                ]
            },
            {
                // ClaimerStep should be identical to the one in invite_greeter_step.json5
                "name": "ClaimerStep",
                "discriminant_field": "step",
                "variants": [
                    [...]  // This is described in the protocol update section
                ]
            }
        ]
    }
]
```

#### `invite_complete` command

An extra authenticated command is added to complete an invitation.

In the previous implementation, this was done automatically when the last exchange was performed. However, this is confusing in some cases:
- Maybe the last exchange worked but the reply didn't reach one of the peers. In this case, the peer cannot retry the exchange since the invite is now completed.
- In a shared recovery (Shamir) invitation, the invitation must not complete after a successful greeting attempt since there might be more exchanges to complete with other recipients.

A better approach is then to explicitly complete the invitation with a dedicated command. The allowed authors for this command depends on the invitation type:
- User invitation: any administrator or the freshly registered user are allowed
- Device invitation: only the corresponding user is allowed
- Shamir invitation: any recipient of the freshly recovered user are allowed

In practice, invite_complete is called:
- by the greeter, for user and device invitation
- by the claimer, for shamir invitation

Pseudo-code:

```python
def invite_complete(token: InvitationToken) ->
  InvitationNotFound | InvitationCompleted | InvitationCancelled | Ok:
    # Perform checks
    [...]
    # Call `complete_invitation`
    [...]
    # Return `Ok`
    [...]
```

Schema definition:

```json5
[
    {
        "major_versions": [
            4
        ],
        "req": {
            "cmd": "invite_complete",
            "fields": [
                {
                    "name": "token",
                    "type": "InvitationToken"
                }
            ]
        },
        "reps": [
            {
                "status": "ok"
            },
            {
                "status": "invitation_not_found"
            },
            {
                "status": "author_not_allowed"
            },
            {
                "status": "invitation_already_completed"
            },
            {
                "status": "invitation_cancelled"
            }
        ]
    }
]
```


### Removed routes

All the `invite_<N>_claimer_<step name>` and `invite_<N>_greeter_<step name>` are removed and replaced by `invite_claimer_step` and `invite_greeter_step`.

This includes:
- `invite_1_greeter_wait_peer`
- `invite_2a_greeter_get_hashed_nonce`
- `invite_2b_greeter_send_nonce`
- `invite_3a_greeter_wait_peer_trust`
- `invite_3b_greeter_signify_trust`
- `invite_4_greeter_communicate`
- `invite_1_claimer_wait_peer`
- `invite_2a_claimer_send_hashed_nonce`
- `invite_2b_claimer_send_nonce`
- `invite_3a_claimer_signify_trust`
- `invite_3b_claimer_wait_peer_trust`
- `invite_4_claimer_communicate`


### Protocol update

Here is an overview of the different kinds of data that can be contained in a claimer or greeter step:
- public key: `PublicKey`
- nonce: `Bytes`
- hashed nonce: `HashDigest`
- acknowledgement: `None`
- payload: `Bytes`

The new steps are defined as such:

| Index | Greeter step       | Greeter data   | Claimer step        | Claimer data   |
|-------|--------------------|----------------|---------------------|----------------|
| 0     | `wait_peer`        | public key     | `wait_peer`         | public key     |
| 1     | `get_hashed_nonce` |                | `send_hashed_nonce` | hashed nonce   |
| 2     | `send_nonce`       | nonce          | `get_nonce`         |                |
| 3     | `get_nonce`        |                | `send_nonce`        | nonce          |
| 4     | `wait_peer_trust`  |                | `signify_trust`     | acknowledgment |
| 5     | `signify_trust`    | acknowledgment | `wait_peer_trust`   |                |
| 6     | `get_payload`      |                | `send_payload`      | payload        |
| 7     | `send_payload`     | payload        | `get_payload`       |                |
| 8     | `wait_peer_ack`    |                | `acknowledge`       | acknowledgment |

Those steps work very similarly to Parsec v2. There are a couple of differences though:
- Steps are not longer counted as `1`, `2a`, `2b`, `3a`, `3b`, and `4` like in version 2
- Three different exchanges were performed in steps `2a` and `2b`. Now those exchanges are expanded in steps `1`, `2` and `3`.
- Step `4` in version 2 could be used for several exchanges, until the greeter marked an exchange with `last=True`. Instead, those exchanges are expanded in steps `6` and `7`
- In version 2, the last step was used to complete the invite. In version 3 this is now done with a dedicated `invite_complete` command, so we add a last step (index `8`) for the claimer to acknowledge that it's been able to successfully process the greeter payload.

The `GreeterStep` nested data type is then define as such:
```json5
            {
                "name": "GreeterStep",
                "discriminant_field": "step",
                "variants": [
                    {
                        "name": "Number0WaitPeer",
                        "discriminant_value": "NUMBER_0_WAIT_PEER",
                        "fields": [
                            {
                                "name": "public_key",
                                "type": "PublicKey"
                            }
                        ]
                    },
                    {
                        "name": "Number1GetHashedNonce",
                        "discriminant_value": "NUMBER_1_GET_HASHED_NONCE"
                    },
                    {
                        "name": "Number2SendNonce",
                        "discriminant_value": "NUMBER_2_SEND_NONCE",
                        "fields": [
                            {
                                "name": "greeter_nonce",
                                "type": "Bytes"
                            }
                        ]
                    },
                    {
                        "name": "Number3GetNonce",
                        "discriminant_value": "NUMBER_3_GET_NONCE"
                    },
                    {
                        "name": "Number4WaitPeerTrust",
                        "discriminant_value": "NUMBER_4_WAIT_PEER_TRUST"
                    },
                    {
                        "name": "Number5SignifyTrust",
                        "discriminant_value": "5"
                    },
                    {
                        "name": "Number6GetPayload",
                        "discriminant_value": "NUMBER_6_GET_PAYLOAD"
                    },
                    {
                        "name": "Number7SendPayload",
                        "discriminant_value": "NUMBER_7_SEND_PAYLOAD",
                        "fields": [
                            {
                                "name": "greeter_payload",
                                "type": "Bytes"
                            }
                        ]
                    },
                    {
                        "name": "Number8WaitPeerAcknowledgment",
                        "discriminant_value": "NUMBER_8_WAIT_PEER_ACKNOWLEDGMENT"
                    }
                ]
            }
```

And the `ClaimerStep` nested data type has the following definition:

```json5
            {
                "name": "ClaimerStep",
                "discriminant_field": "step",
                "variants": [
                    {
                        "name": "Number0WaitPeer",
                        "discriminant_value": "NUMBER_0_WAIT_PEER",
                        "fields": [
                            {
                                "name": "public_key",
                                "type": "PublicKey"
                            }
                        ]
                    },
                    {
                        "name": "Number1SendHashedNonce",
                        "discriminant_value": "NUMBER_1_SEND_HASHED_NONCE",
                        "fields": [
                            {
                                "name": "hashed_nonce",
                                "type": "HashDigest"
                            }
                        ]
                    },
                    {
                        "name": "Number2GetNonce",
                        "discriminant_value": "NUMBER_2_GET_NONCE"
                    },
                    {
                        "name": "Number3SendNonce",
                        "discriminant_value": "NUMBER_3_SEND_NONCE",
                        "fields": [
                            {
                                "name": "claimer_nonce",
                                "type": "Bytes"
                            }
                        ]
                    },
                    {
                        "name": "Number4SignifyTrust",
                        "discriminant_value": "NUMBER_4_SIGNIFY_TRUST"
                    },
                    {
                        "name": "Number5WaitPeerTrust",
                        "discriminant_value": "NUMBER_5_WAIT_PEER_TRUST"
                    },
                    {
                        "name": "Number6SendPayload",
                        "discriminant_value": "NUMBER_6_SEND_PAYLOAD",
                        "fields": [
                            {
                                "name": "claimer_payload",
                                "type": "Bytes"
                            }
                        ]
                    },
                    {
                        "name": "Number7GetPayload",
                        "discriminant_value": "NUMBER_7_GET_PAYLOAD"
                    },
                    {
                        "name": "Number8Acknowledge",
                        "discriminant_value": "NUMBER_8_ACKNOWLEDGE"
                    }
                ]
            }
```

## Typical workflow

Typical workflow
----------------

Here's how a typical workflow for a user invitation can play out:

```python
# An admin creates a new invitation
invite_new_user(...) -> token

# Later the user receives the invitation and starts polling
invite_claimer_start_greeting_attempt(token, greeter) -> greeting attempt ID
invite_claimer_step(greeting_attempt, claimer_step_0) -> NotReady

# Later a greeter connects
invite_greeter_start_greeting_attempt(token) -> greeting attempt ID
invite_greeter_step(greeting_attempt, greeter_step_0) -> claimer_step_0

# ... while the claimer is still polling
invite_claimer_step(greeting_attempt, claimer_step_0) -> greeter_step_0

# Steps keep being submitted
invite_greeter_step(greeting_attempt, greeter_step_1) -> NotReady
invite_claimer_step(greeting_attempt, claimer_step_1) -> greeter_step_1
invite_greeter_step(greeting_attempt, greeter_step_1) -> claimer_step_1
[...]

# The step 8 is performed
invite_greeter_step(greeting_attempt, greeter_step_8) -> NotReady
invite_claimer_step(greeting_attempt, claimer_step_8) -> greeter_step_8
invite_greeter_step(greeting_attempt, greeter_step_8) -> claimer_step_8

# The greeter can now mark the invitation as completed
invite_greeter_complete(token)
```


## Alternatives Considered

The main alternative was to address the 3 goals of this RFC in separate and smaller patches over the v2 invite transport. There has been PRs and POCs going in this direction:
- [A PoC demonstrating the use of keepalive messages in the msgpack stream](https://github.com/Scille/parsec-cloud/pull/6982)
- [Handle gateway timeout for long-polling based invite commands](https://github.com/Scille/parsec-cloud/pull/6971)

But considering the expected simplifications over the current model, this RFC advocates for a proper re-design as a better long-term solution.

## Security/Privacy/Compliance

Those changes have few security implications. The main point to be careful with is to make sure that the greeting attempt system cannot be abused to brute-force the SAS exchange. However, since user interaction is part of the process, it's hard to see how cancelling many attempts would help an attacker in this regard.

## Risks

Overall, those changes should simplify the invite transport and make the overall process more robust, with clearer semantics. This makes it a low risk item, although the implementation should be tested thoroughly as the invitation process is a critical part of the product.
