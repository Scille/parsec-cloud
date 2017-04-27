import pytest
import os


DEFAULT_POSTGRESQL_TEST_URL = 'postgres://localhost:5740/parsec-test'


@pytest.fixture
def postgresql_url():
    return os.environ.get('PARSEC_POSTGRESQL_TEST_URL', DEFAULT_POSTGRESQL_TEST_URL)
