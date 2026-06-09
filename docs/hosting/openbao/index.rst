.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. cSpell:words raPGTZdARXdY0KvHcWSpp5wWZIHNT

.. _doc_hosting_openbao:

===========================
Single Sign-On with OpenBao
===========================

This section explains how to enable :abbr:`SSO (Single Sign-On)` in Parsec via `OpenBao`_,
an open-source secrets manager from the Linux Foundation's `OpenSSF`_ community.

.. _OpenBao: https://openbao.org
.. _OpenSSF: https://openssf.org

Overview
========

Parsec supports :abbr:`SSO (Single Sign-On)` via an `OpenBao`_ server connected to an
:abbr:`OIDC (OpenID Connect)` identity provider.
This integration serves the following purposes:

User Authentication & Device protection
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

.. note:: Only `ProConnect`_ OIDC is fully supported for the moment.

.. _ProConnect: https://www.proconnect.gouv.fr/

Security
========

The security level of storing opaque keys in OpenBao is lower than traditional
approaches, such as storing the keys on the OS Keyring or deriving them from a
strong password.

On the other hand, if your users are already familiar with SSO and use it for
other services, they probably already have an account which will greatly
simplify both Parsec login and enrolment.

This is therefore a typical trade-off between security and user-friendliness
and the decision to whether use this method or not must be made by the server
administrator and should be consistent with your own threat model.


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

OIDC Identity provider configuration
====================================

OIDC authentication works by passing a token to the client
`through an HTTP redirection initiated by the identity provider <https://openid.net/developers/how-connect-works/>`_.

For this, the OIDC identity provider must be configured two redirection URL to use in web and native:

- For web, the URL is ``https://<your-parsec-server>/client/oidc/callback``.
- For the native client, the URL is always ``https://callback.parsec.cloud.invalid/oidc/callback``.

.. note::

   The native client doesn't actually request the redirection URL (and this is not
   even possible given `it is a .invalid TLD <https://en.wikipedia.org/wiki/.invalid>`_).
   Instead this special domain name is detected by the client to prevent the redirection
   and directly extract the authentication token from the URL.

OpenBao configuration
=====================

The following steps are required to configure OpenBao to enable SSO in Parsec.

Before starting, make sure to define the following environment variables:

- ``BAO_ADDR``: The URL of your OpenBao instance
- ``BAO_TOKEN``: The root token to authenticate to the OpenBao instance

For instance:

.. code-block:: bash

  export BAO_ADDR=https://openbao.example.com
  export BAO_TOKEN=s.raPGTZdARXdY0KvHcWSpp5wWZIHNT

Enable the required secrets engines
-----------------------------------

.. code-block:: bash

   # Transit engine — used to sign/verify enrollment payloads
   bao secrets enable -path=transit transit

   # KV v2 engine — used to store encrypted device key files
   bao secrets enable -path=secret kv-v2

Enable and configure the OIDC auth method
-----------------------------------------

The path ``parsec_oidc`` is used below for the OIDC auth method. If you want you can
specify a different mount path.

Use your client credentials to replace ``<your-client-id>`` and load the secret from a
``client-secret.txt`` file so it is not displayed in the command line and to prevent it
to show up in the shell history.

.. code-block:: bash

   # Enable the OIDC auth method at path "parsec_oidc"
   bao auth enable -path=parsec_oidc oidc

   # Point it at your identity provider and set the client credentials
   bao write auth/parsec_oidc/config \
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

   bao write auth/parsec_oidc/role/default \
       user_claim="email" \
       allowed_redirect_uris="https://<your-parsec-server>/client/oidc/callback" \
       allowed_redirect_uris="https://callback.parsec.cloud.invalid/oidc/callback" \
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

   bao policy write parsec-default parsec-default.hcl

Verify the setup
----------------

.. code-block:: bash

   # Log in as a test user
   bao login -method=oidc -path=parsec_oidc

   # Confirm the token has the expected capabilities:
   bao token capabilities \
   transit/sign/entity-$(bao token lookup -format=json | jq -r '.data.entity_id')


Parsec Server configuration
===========================

Once OpenBao server is set up, you will need to configure your Parsec Server.

Following the same approach used during deployment, create a file to store the
required environment variables (enter the ``OPENBAO_SERVER_URL`` from before):

.. admonition:: parsec-openbao.env
   :collapsible: open

   .. literalinclude:: parsec-openbao.env
      :language: bash


Parsec Server with Docker
-------------------------

If you deployed Parsec Server with Docker, edit the Docker Compose file to add
``parsec-openbao.env``:

.. admonition:: parsec-server.docker.yaml
   :collapsible: open

   .. code-block:: yaml
      :emphasize-lines: 7

          env_file:
            - parsec.env
            - parsec-s3.env
            - parsec-db.env
            - parsec-smtp.env
            - parsec-admin-token.env
            - parsec-openbao.env

And then restart the server.

Parsec Server on Linux
----------------------

If you deployed Parsec Server directly on Linux, edit your run script to add
``parsec-openbao.env``:

.. admonition:: run-parsec-server
   :collapsible: open

   .. code-block:: bash
      :emphasize-lines: 6

      # Load the virtual env
      source venv/bin/activate

      # Load the env files into the environment table
      set -a
      source parsec-openbao.env
      source parsec-admin-token.env
      source parsec-db.env
      source parsec-smtp.env
      source parsec-s3.env
      source parsec.env
      set +a

      # Start Parsec Server
      python -m parsec run

And then restart the server.
