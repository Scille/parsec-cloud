# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import json
from contextlib import contextmanager

import pytest


@pytest.fixture
def webhook_spy(monkeypatch):
    events = []

    class MockedRep:
        @property
        def status(self):
            return 200

    @contextmanager
    def _mock_urlopen(req, **kwargs):
        # Webhook are alway POST with utf-8 JSON body
        assert req.method == "POST"
        assert req.headers == {"Content-type": "application/json; charset=utf-8"}
        cooked_data = json.loads(req.data.decode("utf-8"))
        events.append((req.full_url, cooked_data))
        yield MockedRep()

    monkeypatch.setattr("parsec.webhooks.urlopen", _mock_urlopen)
    return events
