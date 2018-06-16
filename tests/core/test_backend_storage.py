# import pytest

# from parsec.utils import to_jsonb64
# from parsec.core.backend_storage import BackendStorage, BackendError, BackendConcurrencyError
# from parsec.core.backend_connections_multiplexer import (
#     BackendConnectionsMultiplexer,
#     BackendNotAvailable,
# )

# from tests.common import AsyncMock


# @pytest.fixture
# def mock_bcm():
#     return AsyncMock(spec=BackendConnectionsMultiplexer)


# @pytest.mark.parametrize(
#     "funcname,params",
#     [
#         ("fetch_user_manifest", []),
#         ("sync_user_manifest", [2, b"<blob>"]),
#         ("fetch_manifest", ["<id>", "<rts>"]),
#         ("sync_manifest", ["<id>", "<wts>", 2, b"<blob>"]),
#         ("sync_new_manifest", [b"<blob>"]),
#         ("sync_new_block", [b"<block>"]),
#         ("fetch_block", ["<id>"]),
#     ],
# )
# @pytest.mark.trio
# async def test_backend_not_available(funcname, params, mock_bcm):
#     storage = BackendStorage(mock_bcm)
#     mock_bcm.send.side_effect = [BackendNotAvailable]
#     func = getattr(storage, funcname)
#     with pytest.raises(BackendNotAvailable):
#         await func(*params)


# @pytest.mark.trio
# async def test_fetch_user_manifest(mock_bcm):
#     storage = BackendStorage(mock_bcm)
#     mock_bcm.send.side_effect = [{"status": "ok", "version": 42, "blob": to_jsonb64(b"<blob>")}]

#     obtained = await storage.fetch_user_manifest()
#     assert obtained == b"<blob>"


# @pytest.mark.trio
# async def test_fetch_user_manifest_version(mock_bcm):
#     storage = BackendStorage(mock_bcm)
#     mock_bcm.send.side_effect = [{"status": "ok", "version": 1, "blob": to_jsonb64(b"<blob>")}]

#     obtained = await storage.fetch_user_manifest(version=1)
#     assert obtained == b"<blob>"


# @pytest.mark.trio
# async def test_fetch_user_manifest_bad_version(mock_bcm):
#     storage = BackendStorage(mock_bcm)
#     mock_bcm.send.side_effect = [{"status": "user_vlob_error", "reason": "Wrong blob version."}]

#     with pytest.raises(BackendError):
#         await storage.fetch_user_manifest(version=42)


# @pytest.mark.trio
# async def test_sync_user_manifest(mock_bcm):
#     storage = BackendStorage(mock_bcm)
#     mock_bcm.send.side_effect = [{"status": "ok"}]
#     await storage.sync_user_manifest(version=1, blob=b"<blob>")
#     mock_bcm.send.assert_called_with(
#         {"cmd": "user_vlob_update", "version": 1, "blob": to_jsonb64(b"<blob>")}
#     )


# @pytest.mark.trio
# async def test_concurrency_error_sync_user_manifest(mock_bcm):
#     storage = BackendStorage(mock_bcm)
#     mock_bcm.send.side_effect = [{"status": "user_vlob_error", "reason": "Wrong blob version."}]
#     with pytest.raises(BackendConcurrencyError):
#         await storage.sync_user_manifest(version=2, blob=b"<blob>")
