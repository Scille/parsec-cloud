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
    'attrs==17.3.0',
    'click==6.7',
    'marshmallow==2.14.0',
    'pendulum==1.3.1',
    'PyNaCl==1.2.0',
    'simplejson==3.10.0',
    # TODO: wait for trio 0.2.0 release...
    'trio==0.1.0-dev'
]
dependency_links=[
    # need to use --process-dependency-links option for this
    'git+https://github.com/python-trio/trio.git@389f1e1e01b410756e2833cffb992fd1ff856ae5#egg=trio-0.1.0-dev',
]

test_requirements = [
    'pytest',
    'pytest-xdist',
    'pytest-cov',
    'freezegun',
    'tox',
    'wheel',
    'Sphinx',
    'flake8',
    'bumpversion',
]

extra_requirements = {
    'drive': ["pydrive==1.3.1"],
    'dropbox': ["dropbox==7.2.1"],
    'fuse': ['fusepy==2.0.4'],
    'postgresql': ["psycopg2==2.7.1", "aiopg==0.13.0"],
    's3': ['boto3==1.4.4', 'botocore==1.5.46'],
    'dev': test_requirements
}
extra_requirements['all'] = sum(extra_requirements.values(), [])
extra_requirements['oeuf-jambon-fromage'] = extra_requirements['all']

setup(
    name='parsec-cloud',
    version='0.5.0',
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
    dependency_links=dependency_links,
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
