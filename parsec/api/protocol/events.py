# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec._parsec import EventsListenRep, EventsListenReq, EventsSubscribeRep, EventsSubscribeReq
from parsec.api.protocol.base import ApiCommandSerializer


__all__ = ("events_listen_serializer", "events_subscribe_serializer")


events_listen_serializer = ApiCommandSerializer(EventsListenReq, EventsListenRep)
events_subscribe_serializer = ApiCommandSerializer(EventsSubscribeReq, EventsSubscribeRep)
