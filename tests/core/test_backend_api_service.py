import pytest

from parsec.backend.vlob_service import Vlob
from parsec.core import MockedBackendAPIService


@pytest.fixture(params=[MockedBackendAPIService, ])
def backend_api_svc(request):
    return request.param()


class TestBackendAPIService:

    @pytest.mark.asyncio
    async def test_cmd(self, backend_api_svc):
        vlob = await backend_api_svc.vlob_create('foo')
        assert isinstance(vlob, Vlob)

    @pytest.mark.asyncio
    async def test_event(self, backend_api_svc):
        vlob = await backend_api_svc.vlob_create('First version')

        is_callback_called = False

        def _on_vlob_updated(sender):
            nonlocal is_callback_called
            is_callback_called = sender

        backend_api_svc.on_vlob_updated.connect(_on_vlob_updated)
        await backend_api_svc.vlob_update(vlob.id, 2, 'Next version')
        assert is_callback_called == vlob.id
