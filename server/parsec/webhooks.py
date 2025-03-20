# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import json
from base64 import b64encode
from typing import Annotated

import httpx
from pydantic import BaseModel, PlainSerializer

from parsec._parsec import (
    DateTime,
    DeviceID,
    DeviceLabel,
    HashAlgorithm,
    OrganizationID,
    SecretKeyAlgorithm,
    SequesterServiceID,
    VlobID,
)
from parsec.components.sequester import RejectedBySequesterService, SequesterServiceUnavailable
from parsec.config import BackendConfig
from parsec.logging import get_logger

logger = get_logger()


OrganizationIDField = Annotated[OrganizationID, PlainSerializer(lambda x: x.str, return_type=str)]
DeviceIDField = Annotated[DeviceID, PlainSerializer(lambda x: x.str, return_type=str)]
DeviceLabelField = Annotated[DeviceLabel, PlainSerializer(lambda x: x.str, return_type=str)]


class OrganizationBootstrapWebhook(BaseModel):
    model_config = {"arbitrary_types_allowed": True}

    organization_id: OrganizationIDField
    device_id: DeviceIDField
    device_label: DeviceLabelField
    human_email: str
    human_label: str


class WebhooksComponent:
    def __init__(self, config: BackendConfig, http_client: httpx.AsyncClient) -> None:
        self._config = config
        self._http_client = http_client

    async def on_organization_bootstrap(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        device_label: DeviceLabel,
        human_email: str,
        human_label: str,
    ) -> None:
        if not self._config.organization_bootstrap_webhook_url:
            return

        data = OrganizationBootstrapWebhook(
            organization_id=organization_id,
            device_id=device_id,
            device_label=device_label,
            human_email=human_email,
            human_label=human_label,
        ).model_dump_json()

        try:
            ret = await self._http_client.post(
                self._config.organization_bootstrap_webhook_url,
                content=data,
                headers={"Content-Type": "application/json; charset=utf-8"},
            )
            if not ret.is_success:
                logger.warning(
                    "webhook bad return status",
                    url=self._config.organization_bootstrap_webhook_url,
                    data=data,
                    return_status=ret.status_code,
                )
        except OSError as exc:
            logger.warning(
                "webhook failure",
                url=self._config.organization_bootstrap_webhook_url,
                data=data,
                exc_info=exc,
            )

    async def sequester_service_on_vlob_create_or_update(
        self,
        webhook_url: str,
        service_id: SequesterServiceID,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlob_id: VlobID,
        key_index: int,
        version: int,
        timestamp: DateTime,
        blob: bytes,
    ) -> None | SequesterServiceUnavailable | RejectedBySequesterService:
        # Proceed webhook service before storage (guarantee data are not stored if they are rejected)
        try:
            ret = await self._http_client.post(
                webhook_url,
                params={
                    "type": "vlob_create_or_update",
                    "service_id": service_id.hex,
                    "organization_id": organization_id.str,
                    "author": author.hex,
                    "realm_id": realm_id.hex,
                    "vlob_id": vlob_id.hex,
                    "key_index": key_index,
                    "version": version,
                    "timestamp": timestamp.to_rfc3339(),
                },
                content=blob,
            )
            if ret.status_code == 400:
                raw_body = await ret.aread()
                try:
                    match json.loads(raw_body):
                        case {"reason": str() as reason}:
                            pass
                        case {} | {"reason": None}:
                            reason = None
                        case _:
                            raise ValueError

                except (json.JSONDecodeError, ValueError):
                    logger.warning(
                        "Invalid rejection reason body returned by webhook",
                        organization_id=organization_id.str,
                        service_id=service_id.hex,
                        body=raw_body,
                    )
                    reason = None
                return RejectedBySequesterService(service_id=service_id, reason=reason)

            elif not ret.is_success:
                logger.warning(
                    "Invalid HTTP status returned by webhook",
                    organization_id=organization_id.str,
                    service_id=service_id.hex,
                    status=ret.status_code,
                )
                return SequesterServiceUnavailable(
                    service_id=service_id,
                )

        except OSError as exc:
            logger.warning(
                "Cannot reach webhook server",
                organization_id=organization_id.str,
                service_id=service_id.hex,
                exc_info=exc,
            )
            return SequesterServiceUnavailable(
                service_id=service_id,
            )

    async def sequester_service_on_realm_rotate_key(
        self,
        webhook_url: str,
        service_id: SequesterServiceID,
        organization_id: OrganizationID,
        keys_bundle: bytes,
        keys_bundle_access: bytes,
        author: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
        key_index: int,
        encryption_algorithm: SecretKeyAlgorithm,
        hash_algorithm: HashAlgorithm,
        key_canary: bytes,
    ) -> None | SequesterServiceUnavailable | RejectedBySequesterService:
        # Proceed webhook service before storage (guarantee data are not stored if they are rejected)
        try:
            ret = await self._http_client.post(
                webhook_url,
                params={
                    "type": "realm_rotate_key",
                    "service_id": service_id.hex,
                    "organization_id": organization_id.str,
                    "author": author.hex,
                    "timestamp": timestamp.to_rfc3339(),
                    "realm_id": realm_id.hex,
                    "key_index": key_index,
                    "encryption_algorithm": encryption_algorithm.str,
                    "hash_algorithm": hash_algorithm.str,
                    "key_canary": b64encode(key_canary),
                    "keys_bundle": b64encode(keys_bundle),
                    "keys_bundle_access": b64encode(keys_bundle_access),
                },
            )
            if ret.status_code == 400:
                raw_body = await ret.aread()
                try:
                    match json.loads(raw_body):
                        case {"reason": str() as reason}:
                            pass
                        case {} | {"reason": None}:
                            reason = None
                        case _:
                            raise ValueError

                except (json.JSONDecodeError, ValueError):
                    logger.warning(
                        "Invalid rejection reason body returned by webhook",
                        organization_id=organization_id.str,
                        service_id=service_id.hex,
                        body=raw_body,
                    )
                    reason = None
                return RejectedBySequesterService(service_id=service_id, reason=reason)

            elif not ret.is_success:
                logger.warning(
                    "Invalid HTTP status returned by webhook",
                    organization_id=organization_id.str,
                    service_id=service_id.hex,
                    status=ret.status_code,
                )
                return SequesterServiceUnavailable(
                    service_id=service_id,
                )

        except OSError as exc:
            logger.warning(
                "Cannot reach webhook server",
                organization_id=organization_id.str,
                service_id=service_id.hex,
                exc_info=exc,
            )
            return SequesterServiceUnavailable(
                service_id=service_id,
            )
