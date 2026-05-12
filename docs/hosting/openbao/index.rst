.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_openbao:

===========================
Single Sign-On with OpenBao
===========================

This section explains how to enable :abbr:`SSO (Single Sign-On)` in Parsec via `OpenBao`_.


Overview
========

Parsec supports :abbr:`SSO (Single Sign-On)` via an `OpenBao`_ server connected to an
:abbr:`OIDC (OpenID Connect)` identity provider.
This integration serves two purposes:

Device protection
   Parsec stores cryptographic device keys on the user's machine. These keys are protected by
   different mechanism depending on the selected
   :ref:`authentication method <doc_hosting_client_deploy_authentication_methods>`.

   With SSO, the keys are encrypted using the user's SSO identity. This way, an attacker who
   manages to steal the key file cannot decrypt it without also controlling the user's SSO account.

Asynchronous enrollment
   When a new user wants to join a Parsec organization, they normally go through a
   *synchronous enrollment*: both the new user and the organization administrator must be
   online at the same time and exchange codes to validate each other identity.

   With SSO, the identity is provided for both parties which means the enrollment can be done in an
   *asynchronous way*: the user sends a request to join, and the administrator approves (or reject)
   the join request later (no simultaneous presence is required).

.. note::

  `OpenBao`_ is an open-source secrets manager (a fork of HashiCorp Vault) managed by the
  Linux Foundation's :abbr:`OpenSSF (Open Source Security Foundation)` community.

  In Parsec, OpenBao can be seen as the cryptographic back-end for SSO support, allowing:

  - User authentication via OIDC
  - Encrypted device key file storage
  - Operations signature during enrollment

Architecture
============

.. mermaid::

  flowchart TD
  oip[OpenID Provider]
  bao[OpenBao Server]
  client[Parsec Client]
  server[Parsec Server]
  oip --(1) OIDC authentication (login / token) ---> client
  client --(2) Sign join request or (d)encrypt device keys---> bao
  client --(3) Submit signed join request---> server

1. The Parsec client authenticates the user against an **OpenID provider**
   (such as a corporate or government SSO). OpenBao is configured to trust that provider, so the
   resulting *token* is usable on OpenBao.

2. The Parsec client uses the *token* to call **OpenBao** to either:

   - Store or retrieve the encrypted device keys ("Device protection" use-case), or
   - Sign/verify an enrollment payload using the user's per-entity transit key
     ("Asynchronous enrollment" use-case).

3. For asynchronous enrollment, the signed enrollment request is then send to the **Parsec Server**,
   for later retrieval by the organization administrator
   (which can then verify its signature using OpenBao).

Requirements
============

- A running `OpenBao`_ instance reachable by both users and the Parsec Server.
- An OIDC-compatible identity provider
- The :command:`bao` CLI tool to configure OpenBao.

OpenBao configuration
=====================

The following steps are required to configure OpenBao to enable SSO in Parsec.

Before starting, make sure to define the ``OPENBAO_SERVER_URL`` variable with your
OpenBao instance URL:

.. code-block:: bash

  export OPENBAO_SERVER_URL=...

Enable the required secrets engines
-----------------------------------

.. code-block:: bash

   # Transit engine — used to sign/verify enrollment payloads
   bao secrets enable -address $OPENBAO_SERVER_URL -path=transit transit

   # KV v2 engine — used to store encrypted device key files
   bao secrets enable -address $OPENBAO_SERVER_URL -path=secret kv-v2

Enable and configure the OIDC auth method
-----------------------------------------

The path ``parsec_oidc`` is used below for the OIDC auth method. If you want you can
specify a different mount path.

Use your client credentials to replace ``<your-client-id>`` and load the secret from a
``client-secret.txt`` file so it is not displayed in the command line and to prevent it
to show up in the shell history.

.. code-block:: bash

   # Enable the OIDC auth method at path "parsec_oidc"
   bao auth enable -address $OPENBAO_SERVER_URL -path=parsec_oidc oidc

   # Point it at your identity provider and set the client credentials
   bao write -address $OPENBAO_SERVER_URL auth/parsec_oidc/config \
       oidc_discovery_url="https://parsec_oidc.example.com/login" \
       oidc_client_id="<your-client-id>" \
       oidc_client_secret=@client-secret.txt \
       default_role="default"

Configure the OIDC role
-----------------------

The role maps a field from the OIDC token to the OpenBao entity. Parsec uses the ``email`` claim to
identify users, so that the enrollment workflow can verify that the email in the enrollment payload
matches the authenticated user's email.

.. code-block:: bash

   bao write -address $OPENBAO_SERVER_URL auth/parsec_oidc/role/default \
       user_claim="email" \
       allowed_redirect_uris="https://<your-parsec-server>/client/oidc/callback" \
       token_policies="parsec-default"

.. note::

   ``allowed_redirect_uris`` must include all redirect URIs that the Parsec client will use.
   Consult your OpenBao deployment documentation for the exact values.

ACL policy
----------

To define the :abbr:`ACL (Access Control List)` policy, create a file
``parsec-default.hcl`` with the following content:

.. HCL syntax highlight below seems to work fine even though it generates a warning.
.. The :force: option disables the warning so there is no error in the CI

.. admonition:: parsec-default.hcl
   :collapsible: open

   .. literalinclude:: parsec-default.hcl
       :language: hcl
       :linenos:
       :force:

Apply the policy:

.. code-block:: bash

   bao policy write -address $OPENBAO_SERVER_URL parsec-default parsec-default.hcl

Verify the setup
----------------

.. code-block:: bash

   # Log in as a test user
   bao login -address $OPENBAO_SERVER_URL -method=oidc -path=parsec_oidc

   # Confirm the token has the expected capabilities:
   bao token capabilities -address $OPENBAO_SERVER_URL \
   transit/sign/entity-$(bao token lookup -format=json | jq -r '.data.entity_id')


.. _OpenBao: https://openbao.org
