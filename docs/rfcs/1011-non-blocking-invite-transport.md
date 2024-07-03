<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Non-blocking invite transport

## Overview

This RFC describes the new invite transport model, adapted to work with non-blocking HTTP routes and providing a better semantic for handling attempts at building a secure channel between peers.

## Background & Motivation

The invite system is based on the creation of a secure channel between two peers, a claimer and a greeter. This is done using the Short Authentication String algorithm and requires a couple of messages to exchange by the peers, along with some acknowledgments.

In the following paragraphs, we'll use `invite protocol` to describe the rules that define the **content** of those messages and what the clients are expected to do with them. We'll use `invite transport` to describe the semantics behind the **handling** of those messages between the clients and the server. This RFC is mostly about the `invite transport` while `invite protocol` is almost left untouched.

In Parsec v2, the communication between the clients and the server is based on websockets which allows for building blocking APIs. The `invite transport` is based on blocking commands for each step of the message passing protocol. Each step consists of a handshake between the peers where two messages are exchanged. The commands are blocking while waiting for the other peers. If one of the peers is disconnected during the process, the attempt is reset and the peers have to start over from the beginning.

In Parsec v3, the communication between the clients and the server is now based on regular HTTP routes (along with Server-Sent Events). Blocking HTTP requests would be very cumbersome considering that gateways are basically free to close the connection however they please after a while. Worse, a connection loss in the context of the building of an invitation channel would cause the process to start over. Since the invite process already weighs heavily on the user experience, keeping this unreliable process is highly unadvisable. A more thorough analysis is available in the [issue #7018 - Update the internal invitation logic for better network failure resilience](https://github.com/Scille/parsec-cloud/issues/7018).

In addition, we can notice that the constraint of having both peers connected to the server at the same time in order to make the exchange doesn't provide any substantial gain. On the contrary, it's fine to have both peers simply depositing and retrieving their payload as they please, as long as it is done in a consistent way. The retrieval is the blocking part, which can be either implemented with a blocking request or non-blocking one, returning a "not ready" status when the payload is not available. This is the approach used here in order to avoid dealing with blocking requests altogether (although it would be valid in this context since retrying after a network error is mostly equivalent to retrying after a "not ready" status).

Since the invite transport is being redesigned, we're including two more changes to this RFC. First, we need to add a reason for an attempt being cancelled by a peer. This is useful for the front-end application to properly communicate to the user what actually happened. This reason did not exist in v2 and has to be added in v3. More precisely, we can notice that the semantics of "attempts" is absent from the v2, even though the process of creating a secure channel between peers can still be cancelled and/or reset. Without dedicated semantics, it is hard to properly manage edge cases such as two devices for the same user trying to create a secure channel at the same time. For this reason, a new attempt semantics is defined here such that each attempt is properly identified and dealt with separately.

The second change is related to the concept of greeters. In Parsec v2, only the administrator that created an invitation is able to greet the invited user. This is something we want to change to open the way to easier invitations, where any administrator can greet a user that has been invited. This may be configurable of course, but the transport should allow for multiple greeters per invitation. Note that this is already the case for shamir invitations, where the greeters are the shamir recipients. For this reason, the new design can draw inspiration from what was done during the shamir implementation. More precisely, each invitation will have its corresponding set of greeters which is different for each invitation type:
- for user invitation, this would be the administrators
- for device invitation, this would be the user alone
- for shamir invitation, this would be the shamir recipients
Note that this set may evolve overtime as users can be revoked and user profiles can change.


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
- Shared recovery (shamir) invitation

An invitation is identified by:
- organization id
- invitation token

Immutable properties:
- type (user, device, shamir)
- created-on timestamp
- created-by author
- claimer id:
  * email address (for user invitation)
  * or user id (for device or shamir invitation)

Mutable state:
- state: PENDING | CANCELLED | COMPLETED

Getters:
- get greeters
  * for user invitation: all administrators
  * for device invitation: same as claimer id
  * for shamir invitation: all recipients
- get channel for greeter

Methods:
- create invitation
- cancel invitation
- complete invitation


### Channels

A channel is dedicated to the verification of the claimer identity by a specific greeter.

A channel is identified by:
- organization id
- invitation token
- greeter id

Mutable properties:
- attempts ids: list of attempts
   * Invariant: most recent attempt is active, all the rest is cancelled

Getters:
- get active attempt

Methods:
- cancel active attempt (and create a new one)


### Attempt

An attempt is the steps followed by the peers of a given channel when trying to establish a secure communication channel.

An attempt is identified by:
- organization id
- invitation token
- greeter id
- attempt id

Mutable properties:
- claimer joined: NotSet | Timestamp
- greeter joined: NotSet | Timestamp
- cancelled reason: NotSet | (CLAIMER | GREETER, Reason, Timestamp)

Inferred properties:
- state: IDLE | HALF JOINED | FULLY JOINED | CANCELLED
- is active: is either IDLE, HALF JOINED or FULLY JOINED

Getters:
- get steps

Methods:
- claimer join attempt
- greeter join attempt
- claimer cancel attempt
- greeter cancel attempt


### Step

A step corresponds to the exchange of two messages between the peers in the context of an attempt.

A step is identified by:
- organization id
- invitation token
- greeter id
- attempt id
- step count

Mutable properties:
- claimer payload: NotSet | Bytes
- greeter payload NotSet | Bytes

Inferred properties:
- state: PENDING | HALF PENDING | COMPLETED

Methods:
- set claimer payload
- set greeter payload


## Design

TODO

## Alternatives Considered

The main alternative was to address the 3 goals of this RFC in separate and smaller patches over the v2 invite transport. There has been PRs and POCs going in this direction:
- [A PoC demonstrating the use of keepalive messages in the msgpack stream](https://github.com/Scille/parsec-cloud/pull/6982)
- [Handle gateway timeout for long-polling based invite commands](https://github.com/Scille/parsec-cloud/pull/6971)

But considering the expected simplifications over the current model, this RFC advocates for a proper re-design as a better long-term solution.

## Security/Privacy/Compliance

Those changes have few security implications. The main point to be careful with is to make sure that the attempt system cannot be abused to brute-force the SAS exchange. However, since user interaction is part of the process, it's hard to see how cancelling many attempts would help an attacker in this regard.

## Risks

Overall, those changes should simplify the invite transport and make the overall process more robust, with clearer semantics. This makes it a low risk item, although the implementation should be tested thoroughly as the invitation process being a critical part of the product.
