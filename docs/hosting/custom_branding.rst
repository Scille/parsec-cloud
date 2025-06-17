.. _doc_hosting_custom_branding:

Custom branding
===============

Customizing the app
-------------------

Parsec can be partially customized to better fit your brand.

Customization is done by adding specific files in certain locations. If Parsec detects the presence of these files, it will use them instead of the regular ones.

- In web mode, the files have to be placed inside a ``custom`` folder in the static directory. A file should be reachable by doing ``/custom/<file>`` on the server.
- In desktop mode, the files have to be placed inside a ``custom`` folder in the configuration directory. The configuration directory will depend on the OS. On Windows, it will be located in ``%APPDATA/parsec3/app``; on Linux, in ``~/.config/parsec3/app`` and on MacOS, in ``~/Library/Application Support/parsec3/app``.

.. note::

  The file extension and type matter.

Here is the list of files that can be customized at the moment:

- ``logo.svg``: the logo that is visible on the home page, in the lower left corner.
- ``logo_icon.svg``: the icon version of the logo, currently unused.
- ``home_sidebar.png``: the drawing used on the homepage's sidebar.

Specifically for the desktop app, you may also add:

- ``app_icon.png``: the app icon, use in the taskbar or in the title of the window (not available on MacOS).
- ``tray_icon.png``: the app tray icon, used in the tray.
- ``splash.png``: the app splash screen. We recommend you match the size of the original splash screen, which is 624x424.

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

Requirements
------------

You may use this customization as you desire, but we do require that your ``custom`` folder contains at least:

- a ``parsec_logo.svg`` file

  .. image:: parsec_logo.svg
      :align: center
      :alt: parsec logo
      :class: logo-background

- a ``en_US.json`` with those keys:

  .. code-block:: json

    {
        "HomePage": {
            "sidebar": {
                "tagline": "Powered by"
            }
        }
    }

- a ``fr_FR.json`` with those keys:

  .. code-block:: json

    {
        "HomePage": {
            "sidebar": {
                "tagline": "Propulsé par"
            }
        }
    }

.. cspell:ignore Propulsé
This should display ``Powered by Parsec`` or ``Propulsé par Parsec`` in the lower left corner of the home page.
