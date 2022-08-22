.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_hosting_server:

==============
Hosting Server
==============


Requirements
============

- Python >= 3.6
- PostgreSQL >= 10

On top of that, an object storage service should also be provided to store the encrypted data blocks.
Both Amazon S3 or OpenStack Swift API are supported.


Hosting
=======

Communication between client and server is achieved using
`Websocket <https://tools.ietf.org/html/rfc6455>`_.
This allow bidirectional communication (needed by the server to be able to notify
the client of remote changes) while still relying on very well supported web
standard.

Parsec server respects the `twelve-factor app principles <https://12factor.net/>`_.
Hence each server instance is stateless and disposable, making it easy to host
it on PAAS infrastructures or in containers.

In short, from a hosting point of view, Parsec server is similar to a standard
web application.


Installation
============

.. code-block:: shell

    python -m venv venv
    . ./venv/bin/activate
    pip install parsec-cloud[backend]


Run
===

Use the ``parsec backend run`` command to start Parsec server, for instance:

.. code-block:: shell

    parsec backend run --port $PORT --host 0.0.0.0 --db postgresql://<...> --blockstore s3:<...> --administration-token <token>


Settings
========


.. note::

    Settings can be specified by using environment variable ``PARSEC_CMD_ARGS``.
    All available command line arguments can be used and environ variables
    within it will be expanded. For instance:

    .. code-block:: shell

        DB_URL=postgres:///parsec PARSEC_CMD_ARGS='--db=$DB_URL --host=0.0.0.0' parsec backend run

Host
----

* ``--host <host>, -H <host>``
* Environ: ``PARSEC_HOST``
* Default: ``127.0.0.1``

Host to listen on.

Port
----

* ``--port <port>, -P <port>``
* Environ: ``PARSEC_PORT``
* Default: ``6777``

Port to listen on.

Database URL
------------

* ``--db <url>``
* Environ: ``PARSEC_DB``

Database configuration.

Allowed values:

- ``MOCKED``: Mocked in memory
- ``postgresql://<...>``: Use PostgreSQL database

.. warning::

    ``MOCKED`` is only designed for development and testing, do not use it in production.

Database connections
--------------------

* ``--db-min-connections <int>``
* Environ: ``PARSEC_DB_MIN_CONNECTIONS``
* Default: ``5``

Minimal number of connections to the database if using PostgreSQL.

* ``--db-max-connections <int>``
* Environ: ``PARSEC_DB_MAX_CONNECTIONS``
* Default: ``7``

Maximum number of connections to the database if using PostgreSQL.

Blockstore URL
--------------

* ``--blockstore <url>, -b <url>``
* Environ: ``PARSEC_BLOCKSTORE``

Blockstore configuration.

Allowed values:

- ``MOCKED``: Mocked in memory
- ``POSTGRESQL``: Use the database specified in the ``--db`` param
- ``s3:[<endpoint_url>]:<region>:<bucket>:<key>:<secret>``: Use Amazon S3 storage
- ``swift:<auth_url>:<tenant>:<container>:<user>:<password>``: Use OpenStack SWIFT storage

Note endpoint_url/auth_url are considered as https by default (e.g.
`s3:foo.com:[...]` -> https://foo.com).
Escaping must be used to provide a custom scheme (e.g. `s3:http\\://foo.com:[...]`).

On top of that, multiple blockstore configurations can be provided to form a
RAID0/1/5 cluster.

Each configuration must be provided with the form
``<raid_type>:<node>:<config>`` with ``<raid_type>`` RAID0/RAID1/RAID5, ``<node>`` a
integer and ``<config>`` the MOCKED/POSTGRESQL/S3/SWIFT config.

For instance, to configure a RAID0 with 2 nodes::

    $ parsec backend run -b RAID0:0:MOCKED -b RAID0:1:POSTGRESQL [...]

.. warning::

    ``MOCKED`` and ``POSTGRESQL`` are only designed for development and testing,
    do not use them in production.

Administration token
--------------------

* ``--administration-token <token>``
* Environ: ``PARSEC_ADMINISTRATION_TOKEN``

Secret token to access the administration api.

SSL
---

* ``--ssl-keyfile <file>``
* Environ: ``PARSEC_SSL_KEYFILE``

SSL key file. This setting enables serving Parsec over SSL.

* ``--ssl-certfile <file>``
* Environ: ``PARSEC_SSL_CERTFILE``

SSL certificate file. This setting enables serving Parsec over SSL.

* ``--forward-proto-enforce-https``
* Environ: ``PARSEC_FORWARD_PROTO_ENFORCE_HTTPS``

Enforce HTTPS by redirecting incoming request that do not comply with the provided header.
This is useful when running Parsec behind a forward proxy handing the SSL layer.
You should *only* use this setting if you control your proxy or have some other
guarantee that it sets/strips this header appropriately.
Typical value for this setting should be `X-Forwarded-Proto:https`.

Logs
----

* ``--log-level <level>, -l <level>``
* Environ: ``PARSEC_LOG_LEVEL``
* Default: ``WARNING``

The granularity of Error log outputs.

Must be one of ``DEBUG``, ``INFO``, ``WARNING``, ``ERROR``, ``CRITICAL``.

* ``--log-format <format>, -f <format>``
* Environ: ``PARSEC_LOG_FORMAT``
* Default: ``CONSOLE``

Log formatting to use.
Must be one of ``CONSOLE``, ``JSON``.

* ``--log-file <file>, -o <file>``
* Environ: ``PARSEC_LOG_FILE``
* Default: log to stderr

The log file to write to.

Email
-----

* ``--backend-addr``
* Environ: ``PARSEC_BACKEND_ADDR``

URL to reach this server (typically used in invitation emails).

* ``--email-host``
* Environ: ``PARSEC_EMAIL_HOST``

The host to use for sending email.

* ``--email-port``
* Environ: ``PARSEC_EMAIL_PORT``
* Default: ``25``

Port to use for the SMTP server defined in EMAIL_HOST.

* ``--email-host-user``
* Environ: ``PARSEC_EMAIL_HOST_USER``

Username to use for the SMTP server defined in EMAIL_HOST.

* ``--email-host-password``
* Environ: ``PARSEC_EMAIL_HOST_PASSWORD``

Password to use for the SMTP server defined in EMAIL_HOST.
This setting is used in conjunction with EMAIL_HOST_USER when authenticating to the SMTP server.

* ``--email-use-ssl``
* Environ: ``PARSEC_EMAIL_USE_SSL``

Whether to use a TLS (secure) connection when talking to the SMTP server.
This is used for explicit TLS connections, generally on port 587.

* ``--email-use-tls``
* Environ: ``PARSEC_EMAIL_USE_TLS``

Whether to use an implicit TLS (secure) connection when talking to the SMTP server.
In most email documentation this type of TLS connection is referred to as SSL.
It is generally used on port 465.
Note that ``--email-use-tls``/``--email-use-ssl`` are mutually exclusive, so only set one of those settings to True.

* ``--email-language``
* Environ: ``PARSEC_EMAIL_LANGUAGE``
* Default: ``en``

Language used in email (Allowed values: ``en`` or ``fr``).

Webhooks
--------

* ``--spontaneous-organization-bootstrap``
* Environ: ``PARSEC_SPONTANEOUS_ORGANIZATION_BOOTSTRAP``

Allow organization bootstrap without prior creation.

Without this flag, an organization must be created by administration
(see ``parsec core create_organization`` command) before bootstrap can occur.

With this flag, the server allows anybody to bootstrap an organanization
by providing an empty bootstrap token given 1) the organization is not boostrapped yet
and 2) the organization hasn't been created by administration (which would act as a
reservation and change the bootstrap token)

* ``--organization-bootstrap-webhook``
* Environ: ``PARSEC_ORGANIZATION_BOOTSTRAP_WEBHOOK``

URL to notify 3rd party service that a new organization has been bootstrapped.

Each time an organization is bootstrapped, an HTTP POST will be send to the URL
with an ``application/json`` body with the following fields:
``organization_id``, ``device_id``, ``device_label`` (can be null), ``human_email`` (can be null), ``human_label`` (can be null).

Example:

.. code-block:: json

    {
        "organization_id": "MyOrg",
        "device_id": "123@abc",
        "device_label": "laptop",
        "human_email": "j.doe@example.com",
        "human_label": "John Doe"
    }

Sentry
------

* ``--sentry-dsn <url>``
* Environ: ``PARSEC_SENTRY_DSN``

`Sentry <https://sentry.io/>`_ URL for telemetry report.

* ``--sentry-environment <name>``
* Environ: ``PARSEC_SENTRY_ENVIRONMENT``
* Default: ``production``

Customize environment name for Sentry's telemetry reports.

Debug
-----

* ``--debug``
* Environ: ``PARSEC_DEBUG``

Enable debug informations.

* ``--dev``

Equivalent to ``--debug --db=MOCKED --blockstore=MOCKED --administration-token=s3cr3t
--email-sender=no-reply@parsec.com --email-host=MOCKED
--backend-addr=parsec://localhost:<port>(?no_ssl=False if ssl is not set)``.
