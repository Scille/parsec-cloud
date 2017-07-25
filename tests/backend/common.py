import os
import pytest


DEFAULT_POSTGRESQL_TEST_URL = '/parsec_test'


def postgresql_url():
    return os.environ.get('PARSEC_POSTGRESQL_TEST_URL', DEFAULT_POSTGRESQL_TEST_URL)


async def init_or_skiptest_parsec_postgresql(loop):
    if pytest.config.getoption('--no-postgresql'):
        pytest.skip('`--no-postgresql` option provided')
    from parsec.backend import postgresql
    module = pytest.importorskip('parsec.backend.postgresql')
    url = postgresql_url()
    await module._init_db(url, force=True, loop=loop)
    return module, url
