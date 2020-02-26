.. _doc_development:

===========
Development
===========

Get the sources
---------------

Source code is `available on github <https://github.com/Scille/parsec-cloud>`_.

1. You should fist clone it with ``git`` and move to the project dir::

    $ git clone git@github.com:Scille/parsec-cloud.git
    $ cd parsec-cloud

2. Create a Python virtual environment for the project::

    $ python -m venv venv
    $ . ./venv/script/activate

.. note::

    Parsec requires Python >= 3.6

3. Install Parsec within the virtualenv::

    $ pip install --editable .[all]

Note the ``--editable`` option, this tell the virtualenv to directly rely on the
source code within the parsec-cloud directory instead of copying it into it install
directory. This way you don't need to reinstall the project each time you modify
something in the project source code.


Run the tests
-------------

Run the tests with pytest::

    $ py.test tests

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
hide the Qt windows when running the GUI tests::

    $ apt install xvfb
    $ pip install pytest-xvfb


Run Parsec in local
-------------------

You can use the ``run_testenv`` scripts to easily create a development environment:

On linux::

    $ . ./tests/scripts/run_testenv.sh

On Windows::

    $ tests\\scripts\\run_testenv.bat

This script will:

- Start a development backend server with in-memory storage
- Configure environment variables to isolate the development environment from
  your global Parsec configuration
- Create a default organization
- Create multiple users and devices for this organization
