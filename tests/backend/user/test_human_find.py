# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.data import UserProfile

from tests.common import freeze_time, customize_fixtures
from tests.backend.common import human_find


# When users have same label or for non-human, the sort could be in undeterminated order, we should use this class for the assertion.
class NonDeterministicOrderedResults(list):
    def __init__(self, *args, order_key=None):
        super().__init__(*args)
        self.order_key = order_key

    def __eq__(self, other):
        # First make sure every item is present
        if not isinstance(other, list):
            return False
        other_as_stack = list(other)
        self_as_stack = list(self)
        while other_as_stack and self_as_stack:
            needle = other_as_stack.pop()
            try:
                self_as_stack.pop(self_as_stack.index(needle))
            except ValueError:
                return False
        if other_as_stack or self_as_stack:
            return False
        # Now check the order is respected
        if self.order_key:
            if sorted(other, key=self.order_key) != other:
                return False
        return True

    def __neq__(self, other):
        return not self.__eq__(other)


@pytest.mark.parametrize(
    "is_equal,a,b",
    [
        (True, NonDeterministicOrderedResults(), NonDeterministicOrderedResults()),
        (False, NonDeterministicOrderedResults(), dict()),
        (False, NonDeterministicOrderedResults(), set()),
        (False, NonDeterministicOrderedResults(), tuple()),
        (True, NonDeterministicOrderedResults([1, 2]), [1, 2]),
        (True, NonDeterministicOrderedResults([2, 1]), [1, 2]),
        (True, NonDeterministicOrderedResults([2, 1, 2]), [1, 2, 2]),
        (False, NonDeterministicOrderedResults([1, 2]), [1, 2, 2]),
        (False, NonDeterministicOrderedResults([1, 2, 2]), [1, 2]),
        (False, NonDeterministicOrderedResults([1, 2, 3]), [1, 2]),
        (False, NonDeterministicOrderedResults([1, 2]), [1, 2, 4]),
        (True, NonDeterministicOrderedResults([{}, {}]), [{}, {}]),
        # Bad order
        (False, NonDeterministicOrderedResults([1, 2], order_key=lambda x: x), [2, 1]),
        # Different order, but still valid
        (
            True,
            NonDeterministicOrderedResults(
                [{"k": 1, "x": "a"}, {"k": 1, "x": "b"}], order_key=lambda x: x["k"]
            ),
            [{"k": 1, "x": "b"}, {"k": 1, "x": "a"}],
        ),
    ],
)
def test_non_deterministic_ordered_results(is_equal, a, b):
    if is_equal:
        assert a == b
        assert b == a
    else:
        assert b != a
        assert a != b


@pytest.fixture
async def access_testbed(
    backend_factory,
    backend_data_binder_factory,
    backend_sock_factory,
    organization_factory,
    local_device_factory,
):

    async with backend_factory(populated=False) as backend:
        binder = backend_data_binder_factory(backend)

        org = organization_factory("IFD")
        device = local_device_factory(
            org=org,
            base_device_id="godfrey@d1",
            base_human_handle="Godfrey Ho <godfrey.ho@ifd.com>",
        )
        with freeze_time("2000-01-01"):
            await binder.bind_organization(org, device)

        async with backend_sock_factory(backend, device) as sock:

            yield binder, org, device, sock


@pytest.mark.trio
async def test_isolation_from_other_organization(
    backend, alice, sock_from_other_organization_factory, alice_backend_sock
):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await human_find(sock, query=alice.human_handle.label)
        assert rep == {"status": "ok", "results": [], "per_page": 100, "page": 1, "total": 0}
        rep = await human_find(sock)
        rep_alice_sock = await human_find(alice_backend_sock)
        assert rep != rep_alice_sock


@pytest.mark.trio
@customize_fixtures(alice_profile=UserProfile.OUTSIDER)
async def test_not_allowed_for_outsider(alice_backend_sock):
    rep = await human_find(alice_backend_sock, query="whatever")
    assert rep == {"status": "not_allowed", "reason": "Not allowed for user with OUTSIDER profile."}


@pytest.mark.xfail(reason="not implemented yet")
@pytest.mark.trio
async def test_ascii_search_on_unicode_data(
    access_testbed, organization_factory, local_device_factory
):
    binder, org, godfrey1, sock = access_testbed

    cunyet = local_device_factory(base_human_handle="Cüneyt Arkin", org=org)
    await binder.bind_device(cunyet, certifier=godfrey1)

    rep = await human_find(sock, query="cuneyt")
    assert rep == {
        "status": "ok",
        "results": [
            {"user_id": cunyet.user_id, "human_handle": cunyet.human_handle, "revoked": False}
        ],
        "per_page": 100,
        "page": 1,
        "total": 1,
    }


@pytest.mark.trio
async def test_unicode_search(access_testbed, local_device_factory):
    binder, org, godfrey1, sock = access_testbed

    hwang = local_device_factory(base_human_handle="황정리", org=org)
    await binder.bind_device(hwang, certifier=godfrey1)

    rep = await human_find(sock, query="황")
    assert rep == {
        "status": "ok",
        "results": [
            {"user_id": hwang.user_id, "human_handle": hwang.human_handle, "revoked": False}
        ],
        "per_page": 100,
        "page": 1,
        "total": 1,
    }


@pytest.mark.trio
async def test_search_multiple_matches(access_testbed, local_device_factory):
    binder, org, godfrey1, sock = access_testbed

    le = local_device_factory(
        base_device_id="bruce_le@d1",
        base_human_handle="Bruce Le <bruce.le@bruceploitation.com>",
        org=org,
    )
    await binder.bind_device(le, certifier=godfrey1)

    li = local_device_factory(
        base_device_id="bruce_li@d1",
        base_human_handle="Bruce Li <bruce.li@bruceploitation.com>",
        org=org,
    )
    await binder.bind_device(li, certifier=godfrey1)

    lai = local_device_factory(
        base_device_id="bruce_lai@d1",
        base_human_handle="Bruce Lai <bruce.lai@bruceploitation.com>",
        org=org,
    )
    await binder.bind_device(lai, certifier=godfrey1)

    expected_rep = {
        "status": "ok",
        "results": [
            {"user_id": lai.user_id, "human_handle": lai.human_handle, "revoked": False},
            {"user_id": le.user_id, "human_handle": le.human_handle, "revoked": False},
            {"user_id": li.user_id, "human_handle": li.human_handle, "revoked": False},
        ],
        "per_page": 100,
        "page": 1,
        "total": 3,
    }

    # Simple search
    rep = await human_find(sock, query="Bruce")
    assert rep == expected_rep

    # Search by email address
    rep = await human_find(sock, query="bruce.l")
    assert rep == expected_rep


@pytest.mark.trio
async def test_search_multiple_user_same_human_handle(access_testbed, local_device_factory):
    binder, org, godfrey1, sock = access_testbed

    nick1 = local_device_factory(
        base_device_id="el_murcielago_enmascarado_ii@d1",
        base_human_handle="Rodolfo Guzman Huerta",
        org=org,
    )
    await binder.bind_device(nick1, certifier=godfrey1)
    await binder.bind_revocation(nick1.user_id, certifier=godfrey1)

    nick2 = local_device_factory(
        base_device_id="el_enmascarado_de_plata@d1", base_human_handle=nick1.human_handle, org=org
    )
    await binder.bind_device(nick2, certifier=godfrey1)
    await binder.bind_revocation(nick2.user_id, certifier=godfrey1)

    nick3 = local_device_factory(
        base_device_id="santo@d1", base_human_handle=nick1.human_handle, org=org
    )
    await binder.bind_device(nick3, certifier=godfrey1)

    rep = await human_find(sock, query="Guzman Huerta")
    # Users have same label, the sort will have an nondeterminated ordered result.
    assert rep == {
        "status": "ok",
        "results": NonDeterministicOrderedResults(
            [
                {"user_id": nick2.user_id, "human_handle": nick2.human_handle, "revoked": True},
                {"user_id": nick1.user_id, "human_handle": nick1.human_handle, "revoked": True},
                {"user_id": nick3.user_id, "human_handle": nick3.human_handle, "revoked": False},
            ],
            order_key=lambda x: x["human_handle"],
        ),
        "per_page": 100,
        "page": 1,
        "total": 3,
    }

    rep = await human_find(sock, query="Guzman Huerta", omit_revoked=True)
    assert rep == {
        "status": "ok",
        "results": [
            {"user_id": nick3.user_id, "human_handle": nick3.human_handle, "revoked": False}
        ],
        "per_page": 100,
        "page": 1,
        "total": 1,
    }


@pytest.mark.trio
async def test_pagination(access_testbed, local_device_factory):
    binder, org, godfrey1, sock = access_testbed

    richard = local_device_factory(base_human_handle=f"Richard <richard@cobra.com>", org=org)
    await binder.bind_device(richard, certifier=godfrey1)

    roger = local_device_factory(base_human_handle=f"Roger <roger@cobra.com>", org=org)
    await binder.bind_device(roger, certifier=godfrey1)

    blacky3 = local_device_factory(base_human_handle=f"Blacky3 <blacky3@cobra.com>", org=org)
    await binder.bind_device(blacky3, certifier=godfrey1)

    mike = local_device_factory(base_human_handle=f"Mike <mike@cobra.com>", org=org)
    await binder.bind_device(mike, certifier=godfrey1)

    blacky = local_device_factory(base_human_handle=f"Blacky <blacky@cobra.com>", org=org)
    await binder.bind_device(blacky, certifier=godfrey1)

    blacky4 = local_device_factory(base_human_handle=f"Blacky4 <blacky4@cobra.com>", org=org)
    await binder.bind_device(blacky4, certifier=godfrey1)

    blacky2 = local_device_factory(base_human_handle=f"Blacky2 <blacky2@cobra.com>", org=org)
    await binder.bind_device(blacky2, certifier=godfrey1)

    # Find all, they should be in the same order as the insert
    rep = await human_find(sock)
    assert rep == {
        "status": "ok",
        "results": NonDeterministicOrderedResults(
            [
                {"user_id": blacky.user_id, "human_handle": blacky.human_handle, "revoked": False},
                {
                    "user_id": blacky2.user_id,
                    "human_handle": blacky2.human_handle,
                    "revoked": False,
                },
                {
                    "user_id": blacky3.user_id,
                    "human_handle": blacky3.human_handle,
                    "revoked": False,
                },
                {
                    "user_id": blacky4.user_id,
                    "human_handle": blacky4.human_handle,
                    "revoked": False,
                },
                {
                    "user_id": godfrey1.user_id,
                    "human_handle": godfrey1.human_handle,
                    "revoked": False,
                },
                {"user_id": mike.user_id, "human_handle": mike.human_handle, "revoked": False},
                {
                    "user_id": richard.user_id,
                    "human_handle": richard.human_handle,
                    "revoked": False,
                },
                {"user_id": roger.user_id, "human_handle": roger.human_handle, "revoked": False},
            ]
        ),
        "page": 1,
        "per_page": 100,
        "total": 8,
    }

    # Find with pagination
    rep = await human_find(sock, per_page=4)
    assert rep == {
        "status": "ok",
        "results": [
            {"user_id": blacky.user_id, "human_handle": blacky.human_handle, "revoked": False},
            {"user_id": blacky2.user_id, "human_handle": blacky2.human_handle, "revoked": False},
            {"user_id": blacky3.user_id, "human_handle": blacky3.human_handle, "revoked": False},
            {"user_id": blacky4.user_id, "human_handle": blacky4.human_handle, "revoked": False},
        ],
        "per_page": 4,
        "page": 1,
        "total": 8,
    }

    # Continue pagination
    rep = await human_find(sock, page=2, per_page=4)
    assert rep == {
        "status": "ok",
        "results": [
            {"user_id": godfrey1.user_id, "human_handle": godfrey1.human_handle, "revoked": False},
            {"user_id": mike.user_id, "human_handle": mike.human_handle, "revoked": False},
            {"user_id": richard.user_id, "human_handle": richard.human_handle, "revoked": False},
            {"user_id": roger.user_id, "human_handle": roger.human_handle, "revoked": False},
        ],
        "per_page": 4,
        "page": 2,
        "total": 8,
    }

    # Test out of pagination
    rep = await human_find(sock, page=3, per_page=4)
    assert rep == {"status": "ok", "results": [], "per_page": 4, "page": 3, "total": 0}

    # Test sort is before pagination when pagination and test non-sensitive sort
    rep = await human_find(sock, page=1, per_page=1, query="BlaCkY")
    assert rep == {
        "status": "ok",
        "results": [
            {"user_id": blacky.user_id, "human_handle": blacky.human_handle, "revoked": False}
        ],
        "per_page": 1,
        "page": 1,
        "total": 4,
    }


@pytest.mark.trio
async def test_bad_args(access_testbed, local_device_factory):
    binder, org, godfrey1, sock = access_testbed
    # Test bad params
    for bad in [{"page": 0}, {"per_page": 0}, {"per_page": 101}]:
        rep = await human_find(sock, **bad)
        assert rep["status"] == "bad_message"


@pytest.mark.trio
@customize_fixtures(
    alice_has_human_handle=False, bob_has_human_handle=False, adam_has_human_handle=False
)
async def test_find_with_query_ignore_non_human(alice_backend_sock, alice, bob, adam):
    # Find all first
    rep = await human_find(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "results": NonDeterministicOrderedResults(
            [
                {"user_id": alice.user_id, "revoked": False, "human_handle": None},
                {"user_id": adam.user_id, "revoked": False, "human_handle": None},
                {"user_id": bob.user_id, "revoked": False, "human_handle": None},
            ]
        ),
        "per_page": 100,
        "page": 1,
        "total": 3,
    }

    rep = await human_find(alice_backend_sock, query=alice.user_id)
    assert rep == {"status": "ok", "results": [], "per_page": 100, "page": 1, "total": 0}
    rep = await human_find(alice_backend_sock, query="alice")
    assert rep == {"status": "ok", "results": [], "per_page": 100, "page": 1, "total": 0}


@pytest.mark.trio
async def test_no_query_users_with_and_without_human_label(access_testbed, local_device_factory):
    binder, org, godfrey1, sock = access_testbed

    ninja = local_device_factory(
        base_human_handle=f"Ninja Warrior <Ninja_Warrior@ninja.com>", org=org
    )
    await binder.bind_device(ninja, certifier=godfrey1)

    roger = local_device_factory(base_human_handle=f"Roger <roger@cobra.com>", org=org)
    await binder.bind_device(roger, certifier=godfrey1)

    blacky3 = local_device_factory(base_human_handle=f"Blacky3 <blacky3@cobra.com>", org=org)
    await binder.bind_device(blacky3, certifier=godfrey1)

    mike = local_device_factory(has_human_handle=False, org=org)
    await binder.bind_device(mike, certifier=godfrey1)

    blacky = local_device_factory(base_human_handle=f"Blacky <blacky@cobra.com>", org=org)
    await binder.bind_device(blacky, certifier=godfrey1)

    easy = local_device_factory(has_human_handle=False, org=org)
    await binder.bind_device(easy, certifier=godfrey1)

    ice = local_device_factory(base_human_handle=f"ice <ice@freeze.com>", org=org)
    await binder.bind_device(ice, certifier=godfrey1)

    richard = local_device_factory(base_human_handle=f"richard <richard@example.com>", org=org)
    await binder.bind_device(richard, certifier=godfrey1)

    zoe = local_device_factory(base_human_handle=f"zoe <zoe@example.com>", org=org)
    await binder.bind_device(zoe, certifier=godfrey1)

    titeuf = local_device_factory(has_human_handle=False, org=org)
    await binder.bind_device(titeuf, certifier=godfrey1)

    # Users with human label should be sorted
    non_human = NonDeterministicOrderedResults(
        [
            {"user_id": easy.user_id, "revoked": False, "human_handle": None},
            {"user_id": mike.user_id, "revoked": False, "human_handle": None},
            {"user_id": titeuf.user_id, "revoked": False, "human_handle": None},
        ]
    )
    human = [
        {"user_id": blacky.user_id, "revoked": False, "human_handle": blacky.human_handle},
        {"user_id": blacky3.user_id, "revoked": False, "human_handle": blacky3.human_handle},
        {"user_id": godfrey1.user_id, "revoked": False, "human_handle": godfrey1.human_handle},
        {"user_id": ice.user_id, "revoked": False, "human_handle": ice.human_handle},
        {"user_id": ninja.user_id, "revoked": False, "human_handle": ninja.human_handle},
        {"user_id": richard.user_id, "revoked": False, "human_handle": richard.human_handle},
        {"user_id": roger.user_id, "revoked": False, "human_handle": roger.human_handle},
        {"user_id": zoe.user_id, "revoked": False, "human_handle": zoe.human_handle},
    ]
    rep = await human_find(sock, per_page=8, page=1)
    assert rep == {"status": "ok", "results": human, "per_page": 8, "page": 1, "total": 11}
    rep = await human_find(sock, per_page=8, page=2)
    assert rep == {"status": "ok", "results": non_human, "per_page": 8, "page": 2, "total": 11}
