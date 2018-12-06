import pytest
from nacl.signing import VerifyKey

from parsec.backend.config import config_factory


GOOD_ROOT_VERIFY_KEY = "DP3SBOVY6BAYNWUJKL5COQEFYTJSXDKKUKGO55ZU5SRI4XLMDHIQssss"


@pytest.mark.parametrize(
    "bad", [None, "not bytes", 42, b" " * 10, GOOD_ROOT_VERIFY_KEY[:-1]]  # Bad length
)
def test_bad_root_verify_key(bad):
    with pytest.raises(ValueError):
        config_factory(root_verify_key=bad)


def test_good_root_verify_key():
    config = config_factory(root_verify_key=GOOD_ROOT_VERIFY_KEY)
    assert isinstance(config.root_verify_key, VerifyKey)
