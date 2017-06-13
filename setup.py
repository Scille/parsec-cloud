#!/usr/bin/env python
# -*- coding: utf-8 -*-


try:
    from setuptools import setup, find_packages
except ImportError:
    from distutils.core import setup, find_packages


with open('README.rst') as readme_file:
    readme = readme_file.read()

with open('HISTORY.rst') as history_file:
    history = history_file.read()

requirements = [
    "aiocache==0.5.2",
    "blinker==1.4",
    "click==6.7",
    "cryptography==1.7.1",
    "gnupg==2.2.0",
    "Logbook==1.0.0",
    "marshmallow==2.13.5",
    "pyaml==16.12.2",
    "python-dateutil==2.6.0",
    "simplejson==3.10.0",
    "ujson==1.35",
    "websockets==3.3",
]

extra_requirements = {
    'drive': ["pydrive==1.3.1"],
    'dropbox': ["dropbox==7.2.1"],
    'fuse': ['fusepy==2.0.4'],
    'postgresql': ["psycopg2==2.7.1", "aiopg==0.13.0"],
    's3': ['boto3==1.4.4', 'botocore==1.5.46'],
}
extra_requirements['all'] = sum(extra_requirements.values(), [])
extra_requirements['oeuf-jambon-fromage'] = extra_requirements['all']

test_requirements = [
    # TODO: put package test requirements here
]

setup(
    name='parsec-cloud',
    version='0.1.0',
    description="Secure cloud framework",
    long_description=readme + '\n\n' + history,
    author="Scille SAS",
    author_email='contact@scille.fr',
    url='https://github.com/Scille/parsec-cloud',
    packages=find_packages(),
    package_data={'parsec': ['resources']},
    package_dir={'parsec': 'parsec'},
    include_package_data=True,
    install_requires=requirements,
    extras_require=extra_requirements,
    entry_points={
        'console_scripts': [
            'parsec = parsec.cli:cli',
        ],
    },
    license="GPLv3",
    zip_safe=False,
    keywords='parsec',
    classifiers=[
        'Development Status :: 2 - Pre-Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)',
        'Natural Language :: English',
        'Programming Language :: Python :: 3.5',
    ],
    test_suite='tests',
    tests_require=test_requirements
)
