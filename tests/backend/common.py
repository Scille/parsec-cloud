import pytest
import os


DEFAULT_POSTGRESQL_TEST_URL = '/parsec_test'


@pytest.fixture
def postgresql_url():
    return os.environ.get('PARSEC_POSTGRESQL_TEST_URL', DEFAULT_POSTGRESQL_TEST_URL)
