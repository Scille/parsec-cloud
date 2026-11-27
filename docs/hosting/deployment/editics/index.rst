.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. only:: fr

   |english-only|

.. _doc_hosting_deployment_editics:

==================
Editics Deployment
==================

Editics is the Parsec module allowing collaborative document (e.g. .docx/.xlsx/.pptx) edition directly in the application.

It's useful on :ref:`web <doc_hosting_deployment_webapp>` since users won't have access to the file system mountpoints provided by the desktop application.

It's also useful on desktop: the document never leaves the Parsec application, whereas there is no way to know what
an external application does with the file.

Configuration
=============

You also need a dedicated sub-domain pointing to the Parsec Server. For example, if your Parsec application
is hosted on ``mydomain.com``, you can use an ``editics.mydomain.com`` sub-domain.

Then pass its address with ``--editics-server-addrs`` (or the ``PARSEC_EDITICS_SERVER_ADDRS`` environment variable):

.. code-block:: bash

  PARSEC_SERVER_ADDR=parsec3://mydomain.com
  PARSEC_EDITICS_SERVER_ADDRS=parsec3://editics.mydomain.com

The Parsec Server will then only serve the Parsec application on ``mydomain.com``,
and only serve Editics on ``editics.mydomain.com`` (the application retrieves the editics domain
from the server configuration).

.. important::

  When using the web app, the Editics pages **must** be served from their own sub-domain for security reasons.

  Browsers isolate web pages by domain: pages served from the same domain can freely access each other's
  content and data. The Editics editor runs complex code on the content of the documents being edited,
  which may come from untrusted sources (e.g. a file shared by another user).

  If Parsec and Editics shared the same domain, a malicious document exploiting a flaw in the editor could:

  - take control of the Parsec application running in the same browser tab, read the decrypted documents
    of all the user's workspaces and act on behalf of the user (share, modify or delete files...);
  - access the data stored in the browser by the Parsec web application.

  With a dedicated sub-domain, the browser prevents Editics from accessing the Parsec application:
  both can only exchange messages through a restricted channel (``postMessage``).

Horizontal scaling
------------------

The document edition sessions are hosted in memory by the Parsec Server instance that started
them. Hence Editics can be scaled horizontally by running multiple Parsec Server instances, each
one reachable from its own dedicated editics domain (e.g. ``editics0.mydomain.com``,
``editics1.mydomain.com``, ...).

Each instance must then be configured with the addresses of **all** the editics domains
(comma-separated), since the document edition sessions are sharded across the instances:
for a given document, the client and the server both determine the instance hosting its
edition session by running the same sharding algorithm (based on the document's organization,
realm and document IDs).

.. code-block:: bash

  # This configuration must be identical on all the Parsec Server instances,
  # and any change must be applied by updating & restarting all of them.
  PARSEC_SERVER_ADDR=parsec3://mydomain.com
  PARSEC_EDITICS_SERVER_ADDRS=parsec3://editics0.mydomain.com,parsec3://editics1.mydomain.com

The editics server instance a request is addressed to is determined by its ``Host`` header
(the Parsec Server relies on it, see the note about the nginx configuration below): the reverse
proxy fronting the instances must hence route each editics domain's requests to its own
instance, while preserving the ``Host`` header. In practice this means one ``server`` block
per editics domain in the nginx configuration.

Deploying using the docker-compose stack
========================================

To enable Editics on :ref:`the docker-compose stack <doc_hosting_deployment_with_docker>`, where the Parsec Server
is reachable at ``https://app.parsec.localhost``, Editics will be served on ``https://editics.app.parsec.localhost``.

#. Configure the web application to be served by the Parsec Server
   (see :ref:`Minimal configuration <doc_hosting_deployment_webapp_minimal>`).

#. Create the env file ``editics/parsec-editics.env`` to configure the Editics domain of the Parsec Server
   (``PARSEC_SERVER_ADDR`` is already defined in ``parsec.env``):

   .. admonition:: editics/parsec-editics.env
      :collapsible: open

      .. literalinclude:: ./parsec-editics.env
         :language: bash

   .. note::

      Add that file in ``services.parsec.env_file[]`` of the docker compose file.

#. Retrieve the patch file for the nginx configuration, which forwards the requests made to ``editics.app.parsec.localhost``
   to the Parsec Server:

   .. admonition:: editics/parsec-nginx.conf.editics.patch
      :collapsible: open

      .. literalinclude:: ./parsec-nginx.conf.editics.patch
         :language: diff

   .. note::

      The ``Host`` header is forwarded as-is to the Parsec Server: it relies on it to identify
      which domain (i.e. main vs editics) the request is addressed to, and hence to only serve
      Editics on this domain.

      Any request whose ``Host`` header doesn't match one of the domains declared as
      ``server_name`` in the nginx configuration is rejected by the catch-all server block.
      Hence a forged ``Host`` header never reaches the Parsec Server, but any legit domain
      must be explicitly declared: each editics domain requires its own ``server`` block,
      and must be covered by the TLS certificate.

   This patch can be applied with:

   .. code-block:: shell

      patch -t -i editics/parsec-nginx.conf.editics.patch

.. note::

  The path of the different files above contain the ``editics/`` prefix, because that's where those files are stored on our public repository.
  You are free to download them somewhere else, but note that you will need to update the path to ``parsec-editics.env``
  in the docker file (``services.parsec.env_file[]``).

Next you need to start the updated ``docker-compose`` stack:

.. code-block:: shell

   docker compose --file ./parsec-server.docker.yaml up --detach

If you have an already running stack, you need to stop & start the services
(``restart`` is not enough to reload the service configuration):

.. code-block:: shell

   docker compose --file ./parsec-server.docker.yaml stop proxy parsec
   docker compose --file ./parsec-server.docker.yaml up --detach
