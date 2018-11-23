import pytest
from pendulum import Pendulum

from parsec.backend.drivers.postgresql import init_db
from parsec.backend.exceptions import NotFoundError
from tests.common import freeze_time


@pytest.mark.trio
async def test_init_db(postgresql_url, backend_factory):
    with freeze_time("2000-01-01"):
        keys1 = await init_db(postgresql_url, "alice", "pc1", True)

    config = {"DB_URL": postgresql_url, "ROOT_VERIFY_KEY": keys1[0]}
    async with backend_factory(devices=[], config=config) as backend1:
        alice1 = await backend1.user.get("alice")
        assert alice1["created_on"] == Pendulum(2000, 1, 1)

    with freeze_time("2000-01-02"):
        await init_db(postgresql_url, "alice", "pc1", False)

    async with backend_factory(devices=[], config=config) as backend2:
        alice2 = await backend2.user.get("alice")
        assert alice2 == alice1

    with freeze_time("2000-01-03"):
        keys3 = await init_db(postgresql_url, "bob", "pc1", True)

    async with backend_factory(
        devices=[], config={**config, "ROOT_VERIFY_KEY": keys3[0]}
    ) as backend3:
        bob3 = await backend3.user.get("bob")
        assert bob3["created_on"] == Pendulum(2000, 1, 3)
        with pytest.raises(NotFoundError):
            await backend3.user.get("alice")

    assert keys3[0] != keys1[0]
    assert keys3[1] != keys1[1]
    assert keys3[2] != keys1[2]
