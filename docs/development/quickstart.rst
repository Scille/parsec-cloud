.. Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

.. _doc_development_quickstart:


==================
Hacking Quickstart
==================

Get the sources
---------------

Source code is `available on github <https://github.com/Scille/parsec-cloud>`_.

1. You need to have the following tools available:
    - `git <https://git-scm.com/>`_
    - `python3.9 <https://www.python.org/>`_
    - `poetry <https://python-poetry.org/>`_

2. You must first clone it with ``git`` and move to the project dir

.. code-block:: shell

    git clone git@github.com:Scille/parsec-cloud.git
    cd parsec-cloud

3. Initialize a poetry env

.. code-block:: shell

    poetry install -E core -E backend

.. note::

    Parsec requires Python 3.9

4. Start a shell with the initialized virtual env

.. code-block:: shell

    poetry shell

5. To run parsec do

.. code-block:: shell

    poetry run parsec

Run the tests
-------------

Run the tests with pytest

.. code-block:: shell

    poetry run py.test tests

On top of that, multiple options are available:

===================   ========================
``--runmountpoint``   Include mountpoint tests
``--rungui``          Include GUI tests
``--runslow``         Include slow tests
``--postgresql``      Use PostgreSQL in the backend instead of a mock in memory
``-n 4``              Run tests in parallel
===================   ========================

Note you can mix&match the flags, e.g. ``py.test tests --runmountpoint --postgresql --runslow -n auto``.

If you want to run GUI test, it is a good idea to install ``pytest-xvfb`` in order to
hide the Qt windows when running the GUI tests

.. code-block:: shell

    apt install xvfb
    pip install pytest-xvfb


Run Parsec in local
-------------------

You can use the ``run_testenv`` scripts to easily create a development environment:

On linux:

.. code-block:: shell

    . ./tests/scripts/run_testenv.sh

On Windows:

.. code-block:: cmd

    .\tests\scripts\run_testenv.bat

This script will:

- Start a development backend server with in-memory storage
- Configure environment variables to isolate the development environment from
  your global Parsec configuration
- Create a default organization
- Create multiple users and devices for this organization
