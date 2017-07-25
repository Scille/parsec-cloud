import pytest
from base64 import encodebytes


def b64(raw):
    return encodebytes(raw).decode()


def can_side_effect_or_skip():
    if pytest.config.getoption('tx'):
        pytest.skip('Cannot run test with side effects with xdist concurrency')
