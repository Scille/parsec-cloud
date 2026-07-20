.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. cspell:words neworg

.. _doc_hosting_administration_create_organization:

======================
Create an Organization
======================

This section describes how to create an Organization on your
:ref:`self-hosted instance of Parsec Server <doc_hosting_deployment>`.


Before starting
===============

The process of creating an Organization in Parsec involves the following steps:

Create an organization
  Create an organization with a valid name (as described in this section) and
  obtain a *bootstrap link* to create the first user.

Set up :ref:`Sequester service<doc_hosting_sequester>` (optional)
  The sequester service allows to securely recover all data from an Organization.
  It can only be activated during the Organization bootstrap and *not afterwards*.

Bootstrap the organization
  Bootstrapping is the process of creating the first user for the Organization.
  This can be done by anyone having the *bootstrap link*.

  .. important::
    The first user is trusted by default (no code exchange is required) and is responsible for
    :ref:`inviting subsequent users <doc_userguide_manage_organization>` and therefore building
    trust via the steps to :ref:`join an Organization <doc_userguide_join_organization>`.

The following diagram summarizes the process:

.. mermaid::

    sequenceDiagram
        actor Admin as Server Admin
        participant Parsec
        actor Alice
        Admin->>Parsec: Create Organization<br/>with a valid name
        Parsec-->>Admin: bootstrap link
        opt
        Admin->>Parsec: Set up Sequester Service
        end
        alt Server Admin is part of the Organization
        Admin->>Parsec: Use bootstrap link<br/>to create first user
        else Server Admin is not part of the Organization
        Admin->>Alice: Send bootstrap link
        Alice->>Parsec: Use bootstrap link<br/>to create first user
        end

.. note::

  If you enabled *spontaneous bootstrap* on your Parsec Server **anyone can create an Organization**.
  This is done directly from the Parsec client application by specifying your server URL and
  following steps in Parsec to create the first user.

  The sequester service cannot be set up for Organizations created in this way.

  Refer to :ref:`Create an organization on my own Parsec server <doc_userguide_new_organization_custom>`.


.. _doc_hosting_administration_create_organization_create:

Create an Organization
======================

.. admonition:: Custom TLS certificates

  If you've used :ref:`custom TLS certificates <doc_hosting_deployment_tls_certificates>` to deploy Parsec Server
  (recommended only for testing purposes) you will need to provide the trusted CA (both to Parsec CLI and Parsec app).
  You can do this by exporting the ``SSL_CAFILE`` before proceeding:

  .. code-block:: bash

    export SSL_CAFILE=$PWD/custom-ca.crt

#. Define required information

   Define ``SERVER_ADDR`` to your server address and ``ORGANIZATION_NAME`` to the desired name for
   the organization (see :ref:`About Organization Names <doc_userguide_new_organization_naming>`).

   .. code-block:: bash

     PARSEC_SERVER_ADDR=parsec3://app.parsec.localhost
     ORGANIZATION_NAME=MyOrganization

   You can also specify this information manually in each command.

#. Define Administration token for the CLI

   If you followed the instructions in :ref:`Server Deployment <doc_hosting_deployment>` you can
   load the corresponding file before calling the CLI:

   .. code-block:: bash

     set -a && source secrets/parsec-admin-token.env && set +a

   Otherwise, create this file now with the ``PARSEC_ADMINISTRATION_TOKEN`` variable
   and load it so you can use it below.

#. Create the organization

   This can be done either with Parsec CLI or via the REST Administration API.

   .. admonition:: Using Parsec CLI

      To create an organization with Parsec CLI you will use the following command:

      .. code-block:: bash

        parsec-cli organization create --addr $PARSEC_SERVER_ADDR $ORGANIZATION_NAME

   .. admonition:: Using the REST Administration API

      To create an organization with the REST Administration API you need to make a ``POST`` request
      to the ``administration/organizations`` endpoint.

      Here is an example of how to run the query using :command:`curl` and :command:`jq`:

      .. code-block:: console

        $ DATA=$(jq -n --arg organization_id "$ORGANIZATION_NAME" '$ARGS.named')
        $ curl ${PARSEC_SERVER_ADDR}/administration/organizations \
          --oauth2-bearer "$PARSEC_ADMINISTRATION_TOKEN" \
          --request POST --data $DATA | jq

   Save the **Bootstrap link** or **bootstrap url** displayed in either case
   before proceeding to bootstrap.

#. Bootstrap the Organization

   #. Start Parsec client application (web or desktop)
   #. Select :menuselection:`Create or join --> Join``
   #. Paste the **bootstrap link** from previous step
   #. Follow the instructions to create the first user of the Organization.


Configure an Organization
=========================

Possible configuration options are:

``user_profile_outsider_allowed`` (default: ``true``)
  To allow or disallow users with **External** profile.
  See :ref:`User Profiles <doc_userguide_manage_organization_profiles>`.

``active_users_limit`` (default: none)
  The maximum number of active (i.e. non-revoked) users.
  By default, the number of active users is unlimited.
  The limit does not apply to users with External profiles (which are always unlimited).

``realm_minimum_archiving_period_before_deletion`` (default: ``2592000``, 30 days)
  When a user :ref:`deletes a workspace <userguide_delete_workspace>`, this is the minimum amount of
  time (in seconds) that must pass before the workspace is effectively deleted.

``tos`` (default: none)
  This option allows you to specify a custom set of :abbr:`ToS (Terms of Service)` that users will need to accept in order to connect to the organization.
  This is specified as a JSON object, with a language code as key, and a link to the term of services in that language as value. For example:

  .. code-block:: json

    {
      "fr": "link-to-tos-in-french.pdf",
      "en": "link-to-tos-in-english.pdf"
    }



They can be set using the REST Administration API, either during organization creation (step 3 above) or later with a ``PATCH`` request.


Here is an example using :command:`curl` and :command:`jq`:

.. code-block:: console

  $ DATA=$(jq -n \
    --arg organization_id $ORGANIZATION_NAME \
    --argjson user_profile_outsider_allowed false \
    --argjson active_users_limit 5 \
    --argjson tos "{\"fr\":\"$PARSEC_SERVER_ADDR/tos-FR\"}" \
    --argjson realm_minimum_archiving_period_before_deletion 864000 \
    '$ARGS.named' -c )

  $ curl ${PARSEC_SERVER_ADDR}/administration/organizations/$ORGANIZATION_NAME \
    --oauth2-bearer "$PARSEC_ADMINISTRATION_TOKEN" \
    --request PATCH --json $DATA | jq
