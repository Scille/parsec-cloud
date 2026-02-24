.. _doc_hosting_custom_branding:

Parsec customization
====================

Parsec application can be partially customized to better fit your brand.

Customization is done by adding specific files in certain locations. If Parsec detects the presence of these files, it will use them instead of the regular ones.

- In web mode, the files have to be placed inside a ``custom`` folder in the server's static files directory. A file should be reachable by doing ``/custom/<file>`` on the server.
- In desktop mode, the files have to be placed inside a ``custom`` folder in the configuration directory. The configuration directory will depend on the OS.

    - Windows: ``%APPDATA%/parsec3/app``
    - Linux ``~/.config/parsec3/app``
    - MacOS ``~/Library/Application Support/parsec3/app``


Images
------

The following images can be customized at the moment:

- ``logo.svg``: the logo displayed in the bottom left corner of the home page.
- ``logo_icon.svg``: the icon version of the logo, it will be used for your organizations.
- ``home_sidebar.png``: the image displayed in the home page sidebar.

Specifically for the desktop app on ``Linux`` and ``Windows``, you may also add:

- ``app_icon.png``: the icon displayed in the taskbar and window title.
- ``tray_icon.png``: the icon displayed in the system tray or notification area.
- ``splash.png``: the splash screen image. We recommend you match the size of the original splash screen, which is 624x424.

.. note::

  The image type and file extension matter. If you use a different type, Parsec may not be able to load it.

We recommend that you check how your images are displayed in Parsec using different display resolutions.
Some images may look great on your screen, but completely off on another.
If you use the recommended sizes when mentioned, you shouldn't have any problem.


Texts
-----

You can also customize some specific texts in Parsec. To do so, simply:

1. Copy the following ``.json`` code excerpts to ``custom_en-US.json`` and ``custom_fr-FR.json`` files

2. Edit the original text values (but not the keys)

3. Put both files in the same ``custom`` folder as before


.. tabs::

  .. group-tab:: custom_en-US.json

    .. literalinclude:: custom_en-US.json
      :language: json

  .. group-tab:: custom_fr-FR.json

    .. literalinclude:: custom_fr-FR.json
      :language: json

Server emails & HTML pages
--------------------------

Parsec server emails and HTML pages (index, 404) are based on the `Jinja template syntax`_.

.. _Jinja template syntax: https://jinja.palletsprojects.com/en/stable/templates/

You can customize them by providing a custom template directory when running the server with ``--template-dir`` or by setting the environment variable ``PARSEC_TEMPLATE_DIR``.

The directory should contain the following files:

- ``index.html``: default landing page when you access the server.
- ``404.html``: resource not found page.
- ``email/account_create.[html|txt].j2``: HTML and TEXT templates for the email send to confirm Parsec account creation.
- ``email/account_delete.[html|txt].j2``: HTML and TEXT email templates to confirm Parsec account deletion.
- ``email/account_recover.[html|txt].j2``: HTML and TEXT email templates to confirm Parsec account recovery.
- ``email/invitation.[html|txt].j2``: HTML and TEXT email templates to send invitation to join an organization.

.. note::

   You can base your customization on the default server's templates `found here <parsec-server-template-src_>`_.

.. _parsec-server-template-src: https://github.com/Scille/parsec-cloud/tree/v3.8.0-rc.0/server/parsec/templates
