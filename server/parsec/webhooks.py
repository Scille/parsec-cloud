# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import httpx
from pydantic import BaseModel, PlainSerializer
from typing_extensions import Annotated

from parsec._parsec import (
    DeviceID,
    DeviceLabel,
    OrganizationID,
)
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
