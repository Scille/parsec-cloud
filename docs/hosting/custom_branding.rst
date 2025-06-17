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

The following files can be customized at the moment:

- ``logo.svg``: the logo displayed in the bottom left corner of the home page.
- ``logo_icon.svg``: the icon version of the logo, it will be used for your organizations.
- ``home_sidebar.png``: the image displayed in the home page sidebar.

Specifically for the desktop app, you may also add:

- ``app_icon.png``: the icon displayed in the taskbar and window title (doesn't work on MacOS).
- ``tray_icon.png``: the icon displayed in the system tray or notification area.
- ``splash.png``: the splash screen image. We recommend you match the size of the original splash screen, which is 624x424.

.. note::

  The file extension and type matter.

.. note::

  We recommend that you check the app using different display resolutions, as some images may look great on your screen, but completely off on another. If you use the recommended sizes when mentioned, you shouldn't have any problem.

You can also customize the texts, by adding:

- ``en_US.json``: the English translations, based on `en_US.json <https://github.com/Scille/parsec-cloud/blob/e7c5cdbc4234f606ccf3ab2be7e9edc22db16feb/client/src/locales/en-US.json>`_

- ``fr_FR.json``: the French translations, based on `fr_FR.json <https://github.com/Scille/parsec-cloud/blob/e7c5cdbc4234f606ccf3ab2be7e9edc22db16feb/client/src/locales/fr-FR.json>`_

You do not need to replace every element in those files, only the values you want to change. For example, to replace the ``Welcome to Parsec`` message of the homepage with ``Welcome to MyApp``, your ``en_US.json`` file should look like this:

  .. code-block:: json

    {
      "HomePage": {
        "topbar": {
            "welcome": "Welcome to MyApp"
        }
      }
    }


It will only replace the value with the key ``HomePage.topbar.welcome`` and leave the default values for every other key.

.. note::

  The files containing the texts are not particularly well ordered and are in need of some cleaning. You may struggle to find the right texts to replace.
