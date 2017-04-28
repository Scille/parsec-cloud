import os
import pytest
import asyncio


DEFAULT_POSTGRESQL_TEST_URL = '/parsec_test'


def postgresql_url():
    return os.environ.get('PARSEC_POSTGRESQL_TEST_URL', DEFAULT_POSTGRESQL_TEST_URL)


def init_or_skip_parsec_postgresql():
    module = pytest.importorskip('parsec.backend.postgresql')
    url = postgresql_url()
    loop = asyncio.get_event_loop()
    loop.run_until_complete(module._init_db(url, force=True))
    return module, url
