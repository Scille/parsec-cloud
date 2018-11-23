import pytest
from nacl.signing import VerifyKey

from parsec.backend.config import config_factory


GOOD_ROOT_VERIFY_KEY = "jFXHe520IMZ_bm39Dvjz5jc4fW5nM6VGj2vFq1ioZxg="


def test_missing_root_verify_key():
    with pytest.raises(ValueError):
        config_factory(environ={})


@pytest.mark.parametrize(
    "bad", [None, "not bytes", 42, b" " * 10, GOOD_ROOT_VERIFY_KEY[:-1]]  # Bad length
)
def test_bad_root_verify_key(bad):
    with pytest.raises(ValueError):
        config_factory(environ={"ROOT_VERIFY_KEY": bad})


def test_good_root_verify_key():
    config = config_factory(environ={"ROOT_VERIFY_KEY": GOOD_ROOT_VERIFY_KEY})
    assert isinstance(config.root_verify_key, VerifyKey)
