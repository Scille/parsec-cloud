.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. _doc_hosting_deployment_webapp:

==========================
Web application deployment
==========================

The following sections will configure the web application using the docker compose configuration detailed in :ref:`Container-Based deployment <doc_hosting_deployment_with_docker>`.
The steps can easily be adapted or ignored if you used the direct install method.


Obtaining the web application
=============================

.. _webapp: https://github.com/Scille/parsec-cloud/releases/download/v3.9.2-a.0+dev/parsec-web-3.9.2-a.0+dev.zip

For a given release, you can obtain the corresponding web application by looking for the asset named ``parsec-web-{version}.zip`` in the release's asset files.

#. For the current release, you can download the webapp by using this link: `webapp`_:

   .. code-block:: shell

      curl --fail -L -o parsec-web.zip \
        https://github.com/Scille/parsec-cloud/releases/download/v3.9.2-a.0+dev/parsec-web-3.9.2-a.0+dev.zip


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

- Configure the server to inform where the webapp files are located

This can be summarized into the following patch that can be applied with:

.. code-block:: shell

   patch -t -i webapp-compose-server.patch

.. admonition:: webapp-compose-server.patch
   :collapsible: open

   .. literalinclude:: ./webapp-compose-server.patch
      :language: diff

With a reverse proxy
====================

This section describes how to configure the reverse proxy configuration to serve the webapp files instead of having the server do it.

.. note::

   You still need to do the :ref:`minimal configuration <doc_hosting_deployment_webapp_minimal>`

#. Mount the webapp asset's directory into the reverse proxy container (nginx here):

   .. note::

      Ignore this step if you're using the direct install approach.

   .. admonition:: webapp-compose-nginx.patch
      :collapsible: open

      .. literalinclude:: ./webapp-compose-nginx.patch
         :language: diff

   This patch can be applied with:

   .. code-block:: shell

      patch -t -i webapp-compose-nginx.patch

#. Update the reverse proxy configuration to server the files:

   .. admonition:: parsec-nginx.conf.patch
      :collapsible: open

      .. literalinclude:: ./parsec-nginx.conf.webapp.patch
         :language: diff

   This patch can be applied with:

   .. code-block:: shell

      patch -t -i parsec-nginx.conf.patch

Further configuration
=====================

.. toctree::
  :maxdepth: 1
  :name: section-hosting-deployment-webapp

  idopte-pki-web
