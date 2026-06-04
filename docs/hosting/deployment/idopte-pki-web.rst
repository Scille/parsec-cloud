.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

.. cspell:words pkiweb

.. _doc_hosting_deployment_idopte_pkiweb:

.. |idopte| replace:: `Idopte`_

.. _Idopte: https://www.idopte.com/

=====================================
Enable PKI and SmartCard support
=====================================

This section describes how to enable PKI and SmartCard support for the :ref:`web application <doc_hosting_deployment_webapp>` using |idopte|'s *Smart Card Middleware*.

|idopte| *Smart Card Middleware*, installed on the user's computer, provides the *Smart Card Web Services* (SCWS)
that are necessary to communicate with a smart card from the web browser.

Why Idopte?
===========

We use Idopte to provide the integration of smartcard features on the web application because at the moment of writing the `Web Smart Card API <https://wicg.github.io/web-smart-card/>`_ is in draft.

And the client that asked for the support of smartcard in web was already using that solution with other web applications.

.. note::
   :collapsible: open

   We are open to consider other options for smart card web support.

   Please feel free to contact us if you know of any other alternatives 
   supporting following features:

   - List available certificate (at least their DER contents)
   - Check certificate trust (either providing :abbr:`CRL (Certificate revocation list)` or doing the check itself)
   - Allow to sign/verify data
   - Allow to encrypt/decrypt data

Prerequisites
=============

Before you begin, make sure you meet the following prerequisites:

.. _scws_mutual_auth: https://www.idopte.com/doc_scws.php

- You have an RSA Private key.

  .. tip::
     :collapsible: open

     You can generate a private key with ``openssl``:

     .. code-block:: bash

        openssl genrsa -out idopte-service.key 4096

- |idopte| provided you a web certificate including the public key of the RSA private key above.

  .. tip::
     :collapsible: open

     ``openssl`` can help you generate the public key:

     .. code-block:: bash

        openssl rsa -in idopte-service.key -out idopte-service.pub

- You have Idopte's public keys, used during the `mutual authentication process <scws_mutual_auth_>`_


- You have a copy of the patched ``scwsapi.js`` file

  .. important::

     You cannot use a non-patched ``scwsapi.js`` file else the web application will not be able to load it.

  .. note::

     For legal reasons, we are currently unable to distribute the patched ``scwsapi.js`` file.

Configure the webapp
====================

Follow these steps to configure the web application:

#. Copy the ``scwsapi.js`` file into the assets folder:

   .. tip::

      To improve caching, we recommend renaming the ``scwsapi.js`` file to have it's checksum in the name:

      .. code-block:: bash

         mv scwsapi.js scwsapi-$(sha256sum scwsapi.js | cut -c -6).js

#. Edit the following `meta tags <https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta>`_ in the ``index.html`` file:

   - Put the web certificate provided by |idopte| in the content of the meta tag named ``scws-web_application_certificate``
   - Edit the content of the meta tag with the name ``scws-scwsapi_js-location`` to path from where the SCWS javascript file is served

Configure Parsec Server
=======================

The server needs additional configuration to be able to `perform the mutual authentication with the client middleware <scws_mutual_auth_>`_:

- Make |idopte|'s public keys accessible to the server.
  Put the keys into a file and either set the option ``--scws-idopte-public-keys-file`` or the env variable ``PARSEC_SCWS_IDOPTE_PUBLIC_KEYS_FILE`` 
  to the file location.
- Provide the private key to use:

  .. important::

     The private key is expected to be provide in its PEM format

  For that you can either specify the key file via the ``--scws-web-application-private-key-file`` option 
  or the env variable ``PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_FILE``.

  Or via it's content in base64 with the env variable ``PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_CONTENT``

  .. code-block:: bash

     export PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_CONTENT=$(base64 idopte-service.key)

  .. warning::

     The server also provides the ``--scws-web-application-private-key-content`` option but it's not recommended 
     because the private key will then be present in the shell history.
