.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_openbao:

=================================
Single Sign-On (SSO) with OpenBao
=================================

Overview
********

Parsec supports Single Sign-On (SSO) via an `OpenBao`_ server connected to an OpenID Connect
(OIDC) identity provider. This integration serves two purposes:

**Device protection**
   Parsec stores cryptographic device keys on the user's machine, typically protecting
   them by a password or using the OS secret manager.
   With SSO, those keys are instead encrypted using the user's SSO identity. This way
   an attacker who steals the key file cannot decrypt it without also controlling
   the user's SSO account.

**Asynchronous enrollment**
   When a new user wants to join a Parsec organization, they normally go through a
   synchronous enrollment (i.e. both new user and organization administrator are online
   at the same time and exchange codes to validate each other identity).
   On the contrary, SSO provides identity for both parties which means the enrollment
   can be done in an asynchronous way (i.e. no simultaneous presence is required,
   administrator can approve the enrollment request while the new user is offline).

`OpenBao`_ is an open-source secret manager (a community fork of HashiCorp Vault). It
acts as the cryptographic back-end for Parsec's SSO support: it authenticates users via
OIDC, stores their encrypted device keys, and provides signing operations used during
enrollment.

Architecture
************

.. code-block:: text

                         ┌──────────────────────┐
                         │   OpenID Provider    │
                         └──────────┬───────────┘
                                    │
                       (1) OIDC authentication
                           (login / token)
                                    │
   ┌────────────────────────────────▼───────────────────────────────┐
   │                        Parsec Client                           │
   └──────┬────────────────────────────────────────────────┬────────┘
          │                                                │
   (2) Sign enrollment request                   (3) Submit signed
       or (d)encrypt device keys                     enrollment request
          │                                                │
          ▼                                                ▼
   ┌──────────────┐                              ┌─────────────────┐
   │ OpenBao      │                              │  Parsec Server  │
   │ Server       │                              │                 │
   └──────────────┘                              └─────────────────┘

1. The Parsec client authenticates the user against the **OpenID provider** (e.g. a
   corporate SSO, Google Workspace, Okta). OpenBao is configured to trust that provider,
   so the resulting token is usable on OpenBao.

2. Using that token, the Parsec client calls **OpenBao** to either:

   - Store or retrieve the encrypted device keys (device protection use-case), or
   - Sign/verify an enrollment payload using the user's per-entity transit key
     (asynchronous enrollment use-case).

3. For asynchronous enrollment, the signed enrollment request is then send to
   the **Parsec server**, which stores it for later retrieval by the organization
   administrator (which can then verify its signature using OpenBao).

Requirements
************

- A running `OpenBao`_ instance reachable by both users and the Parsec server.
- An OIDC-compatible identity provider (Google Workspace, Microsoft Entra ID, Okta, …).
- The ``bao`` CLI tool to configure OpenBao.

OpenBao configuration
*********************

The following steps configure OpenBao for use with Parsec. Replace ``$OPENBAO_SERVER_URL``
with your OpenBao instance URL and ``my_oidc`` with the mount path you choose for the
OIDC auth method.

Enable the required secrets engines
====================================

.. code-block:: bash

   # Transit engine — used to sign/verify enrollment payloads
   bao secrets enable -address $OPENBAO_SERVER_URL -path=transit transit

   # KV v2 engine — used to store encrypted device key files
   bao secrets enable -address $OPENBAO_SERVER_URL -path=secret kv-v2

Enable and configure the OIDC auth method
==========================================

.. code-block:: bash

   # Enable the OIDC auth method at path "my_oidc"
   bao auth enable -address $OPENBAO_SERVER_URL -path=my_oidc oidc

   # Point it at your identity provider and set the client credentials
   bao write -address $OPENBAO_SERVER_URL auth/my_oidc/config \
       oidc_discovery_url="https://my_oidc.example.com/login" \
       oidc_client_id="<your-client-id>" \
       oidc_client_secret="<your-client-secret>" \
       default_role="default"

Configure the OIDC role
=======================

The role maps a field from the OIDC token to the OpenBao entity. Parsec uses the
``email`` claim to identify users, so that the enrollment workflow can verify that the
email in the enrollment payload matches the authenticated user's email.

.. code-block:: bash

   bao write -address $OPENBAO_SERVER_URL auth/my_oidc/role/default \
       user_claim="email" \
       allowed_redirect_uris="https://<your-parsec-server>/client/oidc/callback" \
       token_policies="parsec-default"

.. note::

   ``allowed_redirect_uris`` must include all redirect URIs that the Parsec client will
   use. Consult your OpenBao deployment documentation for the exact values.

Create the ACL policy
=====================

Create a file ``parsec-default.hcl`` with the following content, then apply it:

.. code-block:: hcl

   #
   # Token self-management
   #

   # Allow tokens to look up their own properties
   path "auth/token/lookup-self" {
       capabilities = ["read"]
   }

   # Allow tokens to renew themselves
   path "auth/token/renew-self" {
       capabilities = ["update"]
   }

   # Allow tokens to revoke themselves
   path "auth/token/revoke-self" {
       capabilities = ["update"]
   }

   # Allow a token to look up its own entity by id or name
   path "identity/entity/id/{{identity.entity.id}}" {
       capabilities = ["read"]
   }
   path "identity/entity/name/{{identity.entity.name}}" {
       capabilities = ["read"]
   }

   # Allow a token to look up its resultant ACL from all policies
   path "sys/internal/ui/resultant-acl" {
       capabilities = ["read"]
   }

   # Allow a token to make requests to the Authorization Endpoint for OIDC providers
   path "identity/oidc/provider/+/authorize" {
       capabilities = ["read", "update"]
   }

   #
   # Parsec per-entity device key store
   # (used to protect device files stored on the user's machine)
   #

   # Allow an entity to store its own device keys
   path "parsec-keys/data/{{identity.entity.id}}/*" {
       capabilities = ["create", "update", "patch", "read", "delete"]
   }
   path "parsec-keys/metadata/{{identity.entity.id}}/*" {
       capabilities = ["read", "list", "delete"]
   }

   #
   # Parsec entity-to-entity authenticated message passing
   # (used to sign and verify asynchronous enrollment payloads)
   #

   # User creates its own signing key in the transit engine
   path "transit/keys/entity-{{identity.entity.id}}" {
       capabilities = ["update"]
   }

   # User signs a message with its own key
   path "transit/sign/entity-{{identity.entity.id}}" {
       capabilities = ["update"]
   }

   # Any user can read another entity's metadata (to retrieve its email)
   path "identity/entity/id/*" {
       capabilities = ["read"]
   }

   # Any user can verify a signature made by another entity
   path "transit/verify/entity-*" {
       capabilities = ["update"]
   }

Apply the policy:

.. code-block:: bash

   bao policy write -address $OPENBAO_SERVER_URL parsec-default parsec-default.hcl

Verify the setup
================

Log in as a test user and confirm the token has the expected capabilities:

.. code-block:: bash

   bao login -address $OPENBAO_SERVER_URL -method=oidc -path=my_oidc
   bao token capabilities -address $OPENBAO_SERVER_URL transit/sign/entity-$(bao token lookup -format=json | jq -r '.data.entity_id')

.. _OpenBao: https://openbao.org
