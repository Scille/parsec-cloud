.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. cspell:words pkiweb

.. _doc_hosting_deployment_idopte_pkiweb:

.. |idopte| replace:: `Idopte`_

.. _Idopte: https://www.idopte.com/

=====================================
Enable PKI Web with Idopte middleware
=====================================

Configure the :ref:`web application <doc_hosting_deployment_webapp>` to enable PKI integration using |idopte|'s middleware.

|idopte| provide a middleware installed on the user's device that provide the Smart Card Web Services (SCWS).

Why using Idopte service?
=========================

We use Idopte to provide the integration of smartcard features on the web application because at the moment of writing the `Web Smart Card API <https://wicg.github.io/web-smart-card/>`_ is in draft.

And the client that asked for the support of smartcard in web was already using that solution with other web applications.

.. note::
   :collapsible: open

   We're open to additional integration.

   But the proposed integration has to provide the following features:

   - List available certificate (at least their DER contents)
   - Provide a way to verify the trust in a certificate (either providing the CRL list or doing the check itself)
   - Allow to sign/verify data
   - Allow to encrypt/decrypt data

Prerequisite
============

Before starting to configure the PKI Web feature, ensure that you fill the following prerequisite:

.. _scws_mutual_auth: https://www.idopte.com/doc_scws.php

- You have an RSA Private key.

  .. tip::
     :collapsible: open

     You can generate a private key with ``openssl``:

     .. code-block:: bash

        openssl genrsa -out idopte-service.key 4096

- |idopte| provided you with a web certificate that include the public key of the private key above.

  .. tip::
     :collapsible: open

     ``openssl`` can help you generate the public key:

     .. code-block:: bash

        openssl rsa -in idopte-service.key -out idopte-service.pub

- On top of the web certificate, you have obtained from Idopte their public keys that are used during the `mutual authentication process <scws_mutual_auth_>`_


- You have a copy of the patched SCWS javascript file

  .. important::

     You cannot use a non-patched SCWS javascript file else it will fail to be loaded by the application.

  .. note::

     We cannot distribute that file patched because of license reason

Configure the webapp
====================

The web application need some configuration to have the feature enabled:

#. You need to copy the SCWS javascript file into the assets folder:

   .. tip::

      For better caching purpose, we recommend to rename the SCWS script file to have it's checksum in the name:

      .. code-block:: bash

         mv scwsapi.js scwsapi-$(sha256sum scwsapi.js | cut -c -6).js

#. And edit some `meta tags <https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta>`_ in the ``index.html`` file:

   - Put the web certificate provided by |idopte| in the content of the meta tag named ``scws-web_application_certificate``
   - Edit the content of the meta tag with the name ``scws-scwsapi_js-location`` to path from where the SCWS javascript file is served

Configure the server
====================

The server need additional configuration to be able to `perform the mutual authentication with the client middleware <scws_mutual_auth_>`_:

- Make |idopte|'s public keys accessible to the server.
  Put the keys into a file and either set the option ``--scws-idopte-public-keys-file`` or the env variable ``PARSEC_SCWS_IDOPTE_PUBLIC_KEYS_FILE`` to the location where that file reside.
- Provide the private key to use:

  .. important::

     The private key is expected to be provide in it's PEM format

  For that you can either provide the key in a file via the option ``--scws-web-application-private-key-file`` or the env variable ``PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_FILE``.

  Or via it's content in base64 with the env variable ``PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_CONTENT``

  .. code-block:: bash

     export PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_CONTENT=$(base64 idopte-service.key)

  .. warning::

     The server also provide the option ``--scws-web-application-private-key-content`` but it's not recommended to use it as the private key value will then be present in the shell history.
