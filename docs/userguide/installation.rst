.. _doc_userguide_install_client:


Install Parsec client
=====================

Windows
-------

Windows installers `are available in both 32 and 64 bits <https://github.com/Scille/parsec-build/releases/latest>`_, if not sure you should
use the 64bits version (called ``parsec-vX.Y.Z-win64-setup.exe``).


Linux
-----

Parsec is available on Snap:

.. raw:: html

    <iframe src="https://snapcraft.io/parsec/embedded?button=black" frameborder="0" width="100%" height="350px" style="border: 1px solid #CCC; border-radius: 2px;"></iframe>

If you are familiar with Snap, you may notice that Parsec snap is provided in classic
mode (i.e. without sandbox). This is needed because Parsec needs
`Fuse <https://en.wikipedia.org/wiki/Filesystem_in_Userspace>`_ to mount your
data as a virtual directory, which is not allowed by the Snap sandbox.


.. note::

    You can install the snap from the command line by doing:

    .. code-block:: shell

        sudo snap install parsec --classic


MacOS
-----

Parsec is not yet available on MacOS.


Via pip
-------

Given that Parsec is written in Python, an alternative is to install it through
`pip (the Python package repository) <https://pypi.org/project/parsec-cloud/>`_.

.. code-block:: shell

    pip install parsec-cloud

.. note::

    Parsec requires Python >= 3.6 to work.
