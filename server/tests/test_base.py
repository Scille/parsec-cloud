# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx

from parsec._parsec import DateTime, DeviceID, FileManifest, OrganizationID, VlobID
from parsec.events import EVENT_VLOB_MAX_BLOB_SIZE, EventVlob

from .common import MinimalorgRpcClients


async def test_main_page(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/")
    assert response.status_code == 200
    assert "Parsec server is running!" in response.text


async def test_404(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/unknown")
    assert response.status_code == 404
    assert "The page you requested does not exist." in response.text


async def test_static(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/static/favicon.ico")
    assert response.status_code == 200
    assert response.content.startswith(b"\x89PNG")


async def test_static_404(client: httpx.AsyncClient) -> None:
    response = await client.get("http://parsec.invalid/static/unknown")
    assert response.status_code == 404
    assert response.json() == {"detail": "Not Found"}


# Currently only format version v0 is supported in production, however
# we used to have a vFF format for dev purpose that disabled zstd compression.
# Hence this test that ensures v0 format is still the main format used when
# doing serialization !
def test_serialization_uses_v0_format(minimalorg: MinimalorgRpcClients):
    now = DateTime.now()
    data = FileManifest(
        author=minimalorg.alice.device_id,
        timestamp=now,
        id=VlobID.new(),
        parent=VlobID.new(),
        version=1,
        created=now,
        updated=now,
        blocksize=512,
        size=0,
        blocks=(),
    )
    signed = data.dump_and_sign(minimalorg.alice.signing_key)
    # In this test we don't care about the signature, so remove it...
    serialized = minimalorg.alice.signing_key.verify_key.verify(signed)
    # ...what we care about is the serialization format version
    assert serialized[0] == 0x00
    # Format v0 uses Zstd compression, so we should find its magic header number
    # Magic number is 4 Bytes, little-endian format. Value : 0xFD2FB528
    # see https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#zstandard-frames
    assert serialized[1:5] == b"\x28\xb5\x2f\xfd"


def test_vlob_event_max_size_compatible_with_postgresql_notify(minimalorg: MinimalorgRpcClients):
    big_ts = DateTime.from_rfc3339("9999-12-31T12:59:59.999999Z")
    event = EventVlob(
        organization_id=OrganizationID("o" * 32),
        author=DeviceID.new(),
        realm_id=VlobID.new(),
        timestamp=big_ts,
        vlob_id=VlobID.new(),
        version=2**32 - 1,
        blob=b"x" * EVENT_VLOB_MAX_BLOB_SIZE,
        last_common_certificate_timestamp=big_ts,
        last_realm_certificate_timestamp=big_ts,
    )
    dumped = event.dump_as_apiv5_sse_payload()
    assert len(dumped) < 8000
