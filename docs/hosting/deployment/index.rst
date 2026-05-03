.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. cspell:words conninfo literalinclude linenos

.. _doc_hosting_deployment:

.. role:: bash(code)
  :language: bash

.. role:: yaml(code)
  :language: yaml

=================
Server Deployment
=================

This section describes how to deploy the **Parsec Server** on Linux.

Before you begin, take a look at the :ref:`Application architecture <doc_hosting_architecture>` section for an
overview of software systems and interactions involved with the Parsec Server.

The steps and requirements described in this section may vary based on your specific needs.
It is recommended to deploy and observe performance on a pilot project prior to using in production.

.. note::

  Some setup and administrative operations must be performed with **Parsec CLI** on Linux.
  Please refer to the :ref:`Install Parsec CLI on Linux <doc_hosting_install_cli>` section.

Deployment options
==================

Parsec offers the following deployment options:

- :ref:`Container-Based deployment <doc_hosting_deployment_with_docker>`
- :ref:`Direct installation on Linux server <doc_hosting_deployment_with_linux>`

.. _doc_hosting_deployment_prerequisites:

Prerequisites
=============

The Parsec Server depends on the following components:

- A `PostgreSQL`_ database to store Parsec metadata.
- An S3-like object storage (e.g. `OpenStack Swift`_ or `Amazon S3`_) to store encrypted data.
- An `SMTP server`_ to allow sending emails from the Parsec Server.
- A `TSL/SSL server certificate`_ for ``HTTPS`` communication with Parsec client applications.

Optionally, the following components can be used to support additional features:

  - A `Sentry Data Source Name (DSN)`_ to support Parsec Server telemetry reports.
  - A `CryptPad`_ server to support document editing from Parsec client applications.
  - An `OpenBAO`_ server to support authentication with SSO.

.. important::

  For security reasons, installation and configuration of these components are not covered in this guide.
  Please refer to their corresponding official documentation for instructions on how to do it.

  This guide provides instructions for quickly settings up mock-ups or basic installs of those components.
  Keep in mind that these instructions are provided for convenience and **should not be used in production**.

.. _PostgreSQL: https://www.postgresql.org/
.. _OpenStack Swift: https://docs.openstack.org/swift/latest/
.. _Amazon S3: https://aws.amazon.com/s3/
.. _SMTP server: https://en.wikipedia.org/wiki/Simple_Mail_Transfer_Protocol
.. _TSL/SSL server certificate: https://en.wikipedia.org/wiki/Public_key_certificate#TLS/SSL_server_certificate
.. _Sentry Data Source Name (DSN): https://docs.sentry.io/product/sentry-basics/concepts/dsn-explainer/
.. _CryptPad: https://github.com/cryptpad/cryptpad
.. _OpenBao: https://github.com/openbao/openbao

.. _doc_hosting_deployment_system_requirements:

System Requirements
===================

The following panel describes the minimum software and hardware requirements.

  .. admonition:: Minimum system requirements

    - **Hardware**: 1 vCPU/core with 1GB RAM
    - **Database**: PostgreSQL v16+, 20GB for metadata storage
    - **S3 Object Storage**: 2TB for encrypted data storage (around x100 metadata size)

  .. important::

    It is not recommended to deploy both **Parsec Server** and **PostgreSQL database** on a single system for
    production use, but it is a good option for testing purposes.


Preparation
===========

.. _doc_hosting_deployment_tls_certificates:

TLS certificates
----------------

This section describe how to generate the required TLS certificates with a custom Certificate Authority (CA)
created for this purpose.

.. warning::

  For a production environment, you should always use certificates issued from a trusted CA.

The ``setup-tls.sh`` script below will allow you to generate everything you need:

1. Generate the CA key & self-signed certificate (``custom-ca.{key,crt}``).
2. For ``parsec-s3`` and ``parsec-server`` services:

   a. Generate the service key & Certificate Signing Request (CSR) ``parsec-{service}.{key,csr}``.
   b. Generate the certificate using the CSR and the CA.

3. For ``parsec-server`` service:

   a. Change the key file group ID to ``1234`` (the GID used by the ``parsec-server`` container).
   b. Change the file mode to give read permission to the group ``1234``. This is required because
      Docker Compose does not allow to mount the file with the correct permissions in the container.

.. literalinclude:: setup-tls.sh
  :language: bash
  :linenos:

.. _doc_hosting_deployment_setup_env_files:

Set up the env files
--------------------

The easiest way to configure the Parsec Sever is by using environment variables.
These variables can be stored in a file and sourced before running the server.

In this guide, the environment variables are stored into multiple files in order to
better describe how to configure each component.

The administration token
^^^^^^^^^^^^^^^^^^^^^^^^

To be able to perform admin tasks (like creating an organization) on the server, an administration token is required.
Below you will find a simple script to generate a token:

.. literalinclude:: gen-admin-token.sh
  :language: shell
  :linenos:

The script will generate a random token (:bash:`openssl rand -hex 32`) and create the env file ``parsec-admin-token.env``

.. note::

  The step :bash:`TOKEN=$(openssl rand -hex 32)` could also be replaced by a value generated by a password-generator for example.

  The token doesn't have to be a valid hexadecimal value: any string with enough entropy can be used.

  On top of the administration token, ``gen-admin-token.sh`` also generates `FAKE_ACCOUNT_PASSWORD_ALGORITHM_SEED` which
  is a secret used to make unpredictable the password algorithm configuration returned for non-existing accounts.

Database env file
^^^^^^^^^^^^^^^^^

Create the file ``parsec-db.env`` and specify the  the following content to configure the access to the PostgreSQL database:

.. literalinclude:: parsec-db.env
  :language: ini
  :linenos:

SMTP env file
^^^^^^^^^^^^^

Create the file ``parsec-smtp.env`` to configure the access to the SMTP server (``mailhog`` in this case).

We need to set the connection information, the sender information, the default language the emails are sent in:

.. literalinclude:: parsec-smtp.env
  :language: ini
  :linenos:

S3 env file
^^^^^^^^^^^

Create the file ``parsec-s3.env`` with the following content to set the URL for the S3-like service:

.. literalinclude:: parsec-s3.env
  :language: ini
  :linenos:

.. note::

   We need to escape the ``:`` with a ``\`` when specifying the port of the service.

Parsec env file
^^^^^^^^^^^^^^^

Create the file ``parsec.env`` with the following content to configure the ``parsec-server`` service:

.. literalinclude:: parsec.env
  :language: ini
  :linenos:

.. note::

  To see the full list of environment variables that you can use to configure Parsec, you can run:

  .. code-block:: bash

    python -m parsec run --help

  Look for the sections ``[env var: VARIABLE]`` next to each configuration option. For example:

  .. code-block:: bash

    --administration-token TOKEN    Secret token to access the Administration API
                                    [env var: PARSEC_ADMINISTRATION_TOKEN; required]

.. _doc_hosting_deployment_with_docker:

Deploy with Docker
==================

This section describes how to install Parsec Server directly on Linux. This method is an
alternative to the :ref:`Direct installation on Linux server <doc_hosting_deployment_with_linux>`.

Additional Requirements
-----------------------

In addition to the :ref:`base requirements <doc_hosting_deployment_prerequisites>`, you will need:

- `Docker Engine`_
- `Docker Compose`_ (plugin)

.. _Docker Engine: https://docs.docker.com/engine/
.. _Docker Compose: https://docs.docker.com/compose/

.. _doc_hosting_deployment_the_docker_compose_file:

The Docker Compose file
-----------------------

You can use the following Docker Compose file (``parsec-server.docker.yaml``) to deploy Parsec Server for testing:

.. literalinclude:: parsec-server.docker.yaml
  :language: yaml
  :linenos:

It will setup 4 services:

+---------------------+-----------------------------------------------------------------------------------+
| Service name        | Description                                                                       |
+=====================+===================================================================================+
| ``parsec-postgres`` | The PostgreSQL database                                                           |
+---------------------+-----------------------------------------------------------------------------------+
| ``parsec-s3``       | The Object Storage service                                                        |
+---------------------+-----------------------------------------------------------------------------------+
| ``parsec-smtp``     | A mock SMTP server                                                                |
+---------------------+-----------------------------------------------------------------------------------+
| ``parsec-server``   | The Parsec Server                                                                 |
+---------------------+-----------------------------------------------------------------------------------+
| ``parsec-proxy``    | A Nginx proxy server, used as an example to configure a reverse proxy.            |
|                     |                                                                                   |
|                     | Learn more about                                                                  |
|                     | :ref:`using Parsec behind a reverse proxy<doc_hosting_deployment_behind_a_proxy>` |
+---------------------+-----------------------------------------------------------------------------------+

Starting the services
---------------------

The docker containers can be started as follows:

.. code-block:: bash

  docker compose -f parsec-server.docker.yaml up

Initial configuration
---------------------

On the first start, a one-time configuration is required for the database and s3 services.

Applying the database migration
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

(optional) Check that the database is accessible with:

.. code-block:: bash

  set -a
  source parsec-db.env
  docker exec -t parsec-postgres psql 'postgresql://DB_USER:DB_PASS@0.0.0.0:5432/parsec' -c "\conninfo"

.. note::

  You should have something like display on your console:

  .. code-block::

    You are connected to database "parsec" as user "parsec" on host "0.0.0.0" at port "5432".

To bootstrap the database we just need to apply the migrations with:

.. code-block:: bash

  docker compose -f parsec-server.docker.yaml run parsec-server migrate

Create the S3 Bucket
^^^^^^^^^^^^^^^^^^^^

Access the console at https://127.0.0.1:9090. You will need to use the credential specified in the Docker Compose file
(``parsec-server.docker.yaml``) at :yaml:`services.parsec-s3.environment.MINIO_ROOT_{USER,PASSWORD}`.

Go to https://127.0.0.1:9090/buckets/add-bucket to create a new bucket named ``parsec`` with the features ``object locking`` toggled on.

After that you will need to restart the ``parsec-server`` (that likely exited because it wasn't able to access the S3 bucket):

.. code-block:: bash

  docker compose -f parsec-server.docker.yaml restart parsec-server

Test the SMTP configuration & server
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

You can test ``mailhog`` with:

.. literalinclude:: ping-mailhog.sh
  :language: bash
  :linenos:

You can then check if the email is present in the web interface at http://127.0.0.1:8025

.. _doc_hosting_deployment_with_linux:

Deploy with Linux
=================

This section describes how to install Parsec Server directly on Linux. This method is an
alternative to the :ref:`Container-Based deployment <doc_hosting_deployment_with_docker>`.

Additional Requirements
-----------------------

In addition to the :ref:`base requirements <doc_hosting_deployment_prerequisites>`, you will need:

- Python v3.12 with ``pip`` and ``venv`` modules

Configure the environment
-------------------------

1. Configure the env files, follow :ref:`Set up the env files <doc_hosting_deployment_setup_env_files>`.

Installation
------------

1. Set up a virtual env:

  .. code-block:: bash

    python -m venv venv

2. Configure your shell to use the virtual env:

  .. code-block:: bash

    source venv/bin/activate

3. Install ``parsec-server``

  The ``parsec-server`` is available as a python package hosted on https://pypi.org/project/parsec-cloud/.

  You need to install it with the extra ``backend`` enabled.

  .. code-block:: bash

    python -m pip install 'parsec-cloud==3.8.2-a.0.dev.20576+c3d3ecf'

4. Prepare the database by applying the migrations:

  .. code-block:: bash

    source venv/bin/activate
    set -a
    source parsec-db.env
    python -m parsec migrate

Start the server
----------------

1. Create a wrapper script ``run-parsec-server``

  .. code-block:: bash

    # Load the virtualenv.
    source venv/bin/activate

    # Load the env file into the environment table.
    set -a
    source parsec-admin-token.env
    source parsec-db.env
    source parsec-smtp.env
    source parsec-s3.env
    source parsec.env
    set +a

    # Start Parsec Server.
    python -m parsec run

2. Execute the wrapper script ``run-parsec-server``

  .. note::

    To run the wrapper with only ``run-parsec-server`` you need to have set the executable mode on the script file (``chmod +x run-parsec-server``).
    Otherwise, you need to execute it with the ``bash`` shell (``bash run-parsec-server``).


Start using Parsec Server
=========================

Create the first organization
-----------------------------

.. code-block:: bash

  set -a
  source parsec-admin-token.env
  export SSL_CAFILE=$PWD/custom-ca.crt
  parsec-cli organization create --addr parsec3://127.0.0.1:6777 <orgname>

.. note::

   Change ``<orgname>`` to the organization's name that suit you.

Save the link after ``Bootstrap organization url:`` you will need it to create the first user (owner) of the organization.

Add the first user to the organization
--------------------------------------

First, start ``parsec`` with the custom CA:

.. code-block:: bash

  export SSL_CAFILE=$PWD/custom-ca.crt
  parsec

After that go to ``Menu``/``Join an organization`` (or ``CTRL+O``) and paste the link from before (should already be filled in the text field). Follow the instructions to create the first user of the organization.

.. _doc_hosting_deployment_behind_a_proxy:

Running behind a reverse proxy
==============================

To run Parsec behind a reverse proxy you will need to add the option ``--proxy-trusted-address`` or set the environment variable ``PARSEC_PROXY_TRUSTED_ADDRESS`` to the address of the reverse proxy (e.g.: ``localhost``).

If this option is not set, the gunicorn/uvicorn ``FORWARDED_ALLOW_IPS`` environment variable is used, defaulting to trusting only localhost if absent.

.. tip::

  You can provide multiple addresses by separating them with a comma.

  Example: Use the option ``--proxy-trusted-address '::1,10.0.0.42'`` will trust the address ``::1`` and ``10.0.0.42``

An example of a reverse proxy configuration for ``nginx`` can be found in :ref:`the Docker Compose file <doc_hosting_deployment_the_docker_compose_file>`:

.. literalinclude:: parsec-server.docker.yaml
  :language: yaml
  :linenos:
  :start-at: parsec-proxy
  :end-before: parsec-postgres

The provided configuration for ``nginx`` is:

.. literalinclude:: parsec-nginx.conf
  :language: nginx
  :emphasize-lines: 10,19-22,25,28
  :linenos:

It configures Nginx to serve the domain ``example.com`` by listening on port 80 and 443, and proxy the requests to the Parsec Server.

The important takeaways are:

- Set the headers ``X-Forwarded-For``, ``X-Forwarded-Proto``, ``X-Forwarded-Host`` and ``X-Forwarded-Port``.

  .. note::
    Currently, Parsec only uses the ``X-Forwarded-For`` and ``X-Forwarded-Proto`` headers.
    But it better to overwrite all of them to avoid any issue.

- Remove the header ``Forwarded``.

  .. note::
    The ``Forwarded`` header (`RFC-7239 <https://datatracker.ietf.org/doc/html/rfc7239>`_) is not used by Parsec, but it may be in the future.

- Set the header ``host`` to the accessible address. Here we force the value to be ``example.com``, but you can set it to ``$host`` like for ``X-Forwarded-Host``.

TLS Recommendation
==================

We recommend that connections to the service are made using a TLS layer.
If you are using a :ref:`reverse proxy<doc_hosting_deployment_behind_a_proxy>` refer to it's documentation on how to configure TLS:

- `Nginx SSL module configuration <https://nginx.org/en/docs/http/ngx_http_ssl_module.html>`_
- `Apache httpd SSL Howto <https://httpd.apache.org/docs/current/en/ssl/ssl_howto.html>`__
- `Traefik TLS configuration <https://doc.traefik.io/traefik/https/tls>`__

Or if you do not use a reverse proxy, see how to configure :ref:`TLS on the server <doc_hosting_deployment_tls_certificates>`

.. _doc_hosting_deployment_server-tls-config:

TLS Server configuration
========================

We recommend that when user directly connects to the server (i.e. without using a :ref:`reverse proxy<doc_hosting_deployment_behind_a_proxy>`)
to configure the TLS settings on the server.

We provide 3 options to configure the TLS connection:

.. cspell: ignore Recommandations, sécurité

- ``--ssl-keyfile`` (``PARSEC_SSL_KEYFILE``): The TLS key file
- ``--ssl-certfile`` (``PARSEC_SSL_CERTFILE``): The TLS certificate file
- ``--ssl-ciphers`` (``PARSEC_SSL_CIPHERS``): A list of ciphers that can be used when the client & server negotiate which algorithm to use when doing the TLS handcheck

  .. note::

    You are not required to provide the ciphers list as we use a default list that was recommended by the
    `French Cybersecurity Agency (ANSSI) <https://cyber.gouv.fr/en>`__ in
    `Recommandations de sécurité relatives à TLS <doc_hosting_deployment_anssi_reco_tls_>`_

.. _doc_hosting_deployment_anssi_reco_tls: https://cyber.gouv.fr/sites/default/files/2017/07/anssi-guide-recommandations_de_securite_relatives_a_tls-v1.2.pdf

.. hint::

   If you followed the installation described in `Deploy with Docker`_,
   you should only have to replace the file ``parsec-server.crt`` and ``parsec-server.key`` that where generated on section `TLS certificates`_.
   The env variable ``PARSEC_SSL_KEYFILE`` and ``PARSEC_SSL_CERTFILE`` are already configured in ``parsec.env`` that was defined in `Parsec env file`_.
