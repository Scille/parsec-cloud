.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. only:: fr

   |english-only|

.. _doc_hosting_deployment_webapp:

==========================
Web application deployment
==========================

The following sections will configure the web application using the docker compose configuration detailed in :ref:`Container-Based deployment <doc_hosting_deployment_with_docker>`.
The steps can easily be adapted or ignored if you used the direct install method.


Obtaining the web application
=============================

.. _webapp: https://github.com/Scille/parsec-cloud/releases/download/v3.11.1-a.0+dev/parsec-web-3.11.1-a.0+dev.zip

For a given release, you can obtain the corresponding web application by looking for the asset named ``parsec-web-{version}.zip`` in the release's asset files.

#. For the current release, you can download the webapp by using this link: `webapp`_:

   .. code-block:: shell

      curl --fail -L -o parsec-web.zip \
        https://github.com/Scille/parsec-cloud/releases/download/v3.11.1-a.0+dev/parsec-web-3.11.1-a.0+dev.zip


#. Once the archive downloaded, extract it under ``parsec-web``:


   .. code-block:: shell

      unzip -d parsec-web parsec-web.zip

.. _doc_hosting_deployment_webapp_minimal:

Minimal configuration
=====================

This section describes the minimal configuration needed for the webapp.

With the webapp files downloaded, we need to update the server configuration to make it serve the webapp as such:

- Mount the webapp assets directory in the server container

  .. note::

     Ignore this step if you're using the direct install approach.

- Configure the server to inform where the webapp files are located, by creating the env file
  ``webapp/parsec-webapp.env``:

  .. admonition:: webapp/parsec-webapp.env
     :collapsible: open

     .. literalinclude:: ./parsec-webapp.env
        :language: bash

  .. note::

     Ignore this step if you're using the direct install approach: instead set the
     ``PARSEC_WITH_CLIENT_WEB_APP`` environment variable (or the ``--with-client-web-app``
     option) to the actual path of the ``parsec-web`` directory.

This can be summarized into the following patch that can be applied with:

.. code-block:: shell

   patch -t -i webapp/webapp-compose-server.patch

.. admonition:: webapp-compose-server.patch
   :collapsible: open

   .. literalinclude:: ./webapp-compose-server.patch
      :language: diff

Caching the web app files with the reverse proxy
=================================================

The web app static resources are designed for caching: they have a content hash in their name
(e.g. ``base-jFjh9D00.css``), and the Parsec Server serves them pre-compressed (brotli) with an
aggressive ``Cache-Control`` header. Hence no reverse proxy configuration is needed to get good
performances: the browsers don't even re-request the resources they already have.

However, the reverse proxy can cache the web app assets in order to reduce the load on the
Parsec Server (i.e. so it doesn't serve the resources to each new visitor). This can be done
with the following patch:

.. admonition:: parsec-nginx.conf.webapp.patch
   :collapsible: open

   .. literalinclude:: ./parsec-nginx.conf.webapp.patch
      :language: diff

This patch can be applied with:

.. code-block:: shell

   patch -t -i webapp/parsec-nginx.conf.webapp.patch

.. note::

   The web app files must still be served by the Parsec Server (see the
   :ref:`minimal configuration <doc_hosting_deployment_webapp_minimal>`): it is the one enforcing
   the host isolation and providing the Content-Security-Policy of the
   :ref:`Editics <doc_hosting_deployment_editics>` pages, so in particular those pages must
   never be served by the reverse proxy.

Further configuration
=====================

.. toctree::
  :maxdepth: 1
  :name: section-hosting-deployment-webapp

  idopte-pki-web
