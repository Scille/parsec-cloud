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

.. code::

    $ python -m venv venv
    $ . ./venv/bin/activate
    $ pip install parsec-cloud[backend]


Run
===

Use the ``parsec backend run`` command to start Parsec server, for instance::

    $ parsec backend run --port $PORT --host 0.0.0.0 --db postgresql://<...> --blockstore s3:<...> --administration-token <token>


Settings
========


.. note::

    Settings can be specified by using environment variable ``PARSEC_CMD_ARGS``.
    All available command line arguments can be used and environ variables
    within it will be expanded. For instance::

        $ DB_URL=postgres:///parsec PARSEC_CMD_ARGS='--db=$DB_URL --host=0.0.0.0' parsec backend run

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
- ``s3:<endpoint_url>:<region>:<bucket>:<key>:<secret>``: Use Amazon S3 storage
- ``swift:<authurl>:<tenant>:<container>:<user>:<password>``: Use OpenStack SWIFT storage

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

Custom SSL key file.

* ``--ssl-certfile <file>``
* Environ: ``PARSEC_SSL_CERTFILE``

Custom SSL certificate file.

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

Sentry
------

* ``--sentry-url <url>``
* Environ: ``PARSEC_SENTRY_URL``

`Sentry <https://sentry.io/>`_ URL for telemetry report.

Debug
-----

* ``--debug``
* Environ: ``PARSEC_DEBUG``

Enable debug informations.

* ``--dev``

Equivalent to ``--debug --db=MOCKED --blockstore=MOCKED --administration-token=s3cr3t``.
