import os
import pytest

from tests.common import TEST_USERS
from tests.populate import populate_factory


pytest_plugins = "pytest_trio.plugin"


def pytest_addoption(parser):
    parser.addoption("--no-postgresql", action="store_true",
                     help="Don't run tests making use of PostgreSQL")


DEFAULT_POSTGRESQL_TEST_URL = '/parsec_test'


def postgresql_url():
    return os.environ.get('PARSEC_POSTGRESQL_TEST_URL', DEFAULT_POSTGRESQL_TEST_URL)


@pytest.fixture(params=['mocked', 'postgresql'])
def backend_store(request):
    if request.param == 'postgresql':
        if pytest.config.getoption('--no-postgresql'):
            pytest.skip('`--no-postgresql` option provided')
        pytest.importorskip('parsec.backend.postgresql')
        return postgresql_url()
    else:
        return 'mocked://'


@pytest.fixture(scope='session')
def alice():
    return TEST_USERS['alice@test']


@pytest.fixture(scope='session')
def bob():
    return TEST_USERS['bob@test']


@pytest.fixture(scope='session')
def mallory():
    return TEST_USERS['mallory@test']


@pytest.fixture
def alice_data(alice):
    return populate_factory(alice)


@pytest.fixture
def bob_data(bob):
    return populate_factory(bob)


@pytest.fixture
def mallory_data(mallory):
    return populate_factory(mallory)
