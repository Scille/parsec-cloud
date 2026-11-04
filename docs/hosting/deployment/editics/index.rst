.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. only:: fr

   |english-only|

.. _doc_hosting_deployment_editics:

==================
Editics Deployment
==================

Editics is the Parsec module allowing to edit documents directly in the application.

It's useful on :ref:`web <doc_hosting_deployment_webapp>` since users won't have access to the file system mountpoints provided by the desktop application.

It's also useful on desktop: the document never leaves the Parsec application, whereas there is no way to know what
an external application does with the file.

Configuration
=============

.. note::

  This configuration is only needed for the :ref:`web application <doc_hosting_deployment_webapp>`:
  the desktop application provides Editics without any server configuration.

Editics requires the web application to be served by the Parsec Server
(see :ref:`Minimal configuration <doc_hosting_deployment_webapp_minimal>`).

You also need a dedicated sub-domain pointing to the Parsec Server. For example, if your Parsec application
is hosted on ``mydomain.com``, you can use an ``editics.mydomain.com`` sub-domain.

Then pass its address with ``--editics-server-addr`` (or the ``PARSEC_EDITICS_SERVER_ADDR`` environment variable):

.. code-block:: bash

  PARSEC_SERVER_ADDR=parsec3://mydomain.com
  PARSEC_EDITICS_SERVER_ADDR=parsec3://editics.mydomain.com

The Parsec Server will then only serve the Parsec application on ``mydomain.com``,
and only serve Editics on ``editics.mydomain.com`` (The application prepend the hard-coded ``editics.`` prefix to the current host).

.. important::

  The Editics pages **must** be served from their own sub-domain for security reasons.

  Browsers isolate web pages by domain: pages served from the same domain can freely access each other's
  content and data. The Editics editor runs complex code on the content of the documents being edited,
  which may come from untrusted sources (e.g. a file shared by another user).

  If Parsec and Editics shared the same domain, a malicious document exploiting a flaw in the editor could:

  - take control of the Parsec application running in the same browser tab, read the decrypted documents
    of all the user's workspaces and act on behalf of the user (share, modify or delete files...);
  - access the data stored in the browser by the Parsec web application.

  With a dedicated sub-domain, the browser prevents Editics from accessing the Parsec application:
  both can only exchange messages through a restricted channel (``postMessage``).

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

      The ``Host`` header must be kept as ``editics.parsec.localhost``: the Parsec Server relies on it
      to only serve Editics on this domain.

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

Verify the deployment
=====================

Once configured, open ``https://editics.mydomain.com/client/home`` in a browser: you should get a **Forbidden** error.

If this URL serves the Parsec application instead, the isolation is not effective and the deployment is not secure.
