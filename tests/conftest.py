import os
import pytest


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
