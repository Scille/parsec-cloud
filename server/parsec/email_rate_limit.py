# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections import deque
from dataclasses import dataclass
from typing import Literal

from parsec._parsec import DateTime, EmailAddress


@dataclass(slots=True)
class SentEmail:
    """
    Keep stats about the sent email is needed for rate-limiting.
    """

    timestamp: DateTime
    recipient: EmailAddress
    client_ip_address: str | Literal[""]
    """
    IP address of the client that started the operation that leads to
    sending this email.

    ⚠️ IP address can be an empty string if the IP is not known
    """


class EmailRateLimit:
    def __init__(
        self,
        cooldown_delay,
        max_per_hour,
    ):
        self.cooldown_delay = cooldown_delay
        self.max_per_hour = max_per_hour
        # Sorted by timestamp from older to more recent
        self._last_sent_mails: deque[SentEmail] = deque()

    def register_send_intent(
        self,
        now: DateTime,
        client_ip_address: str | Literal[""],
        recipient: EmailAddress,
    ) -> None | DateTime:
        # 1) Start with some cleanup: since we use a 1 hours sliding window, we
        #    discard anything older than that.

        oldest_considered_timestamp = now.add(hours=-1)
        while True:
            try:
                oldest_mail = self._last_sent_mails[0]
            except IndexError:
                break
            if oldest_mail.timestamp <= oldest_considered_timestamp:
                self._last_sent_mails.popleft()
            else:
                break

        # 2) Filter to only keep the mails relevant to our recipient/client_ip_address

        if client_ip_address == "":
            relevant_emails = [m for m in self._last_sent_mails if m.recipient == recipient]
        else:
            relevant_emails = [
                m
                for m in self._last_sent_mails
                if m.recipient == recipient or m.client_ip_address == client_ip_address
            ]

        # 3) Actually check the limits

        if not relevant_emails:
            pass
        elif (now - relevant_emails[-1].timestamp) < self.cooldown_delay:
            # Last mail is too recent
            wait_until = relevant_emails[-1].timestamp.add(seconds=self.cooldown_delay)
            return wait_until
        elif self.max_per_hour > 0 and len(relevant_emails) >= self.max_per_hour:
            # Too many mails in the last hour
            wait_until = relevant_emails[0].timestamp.add(hours=1)
            return wait_until

        # 4) The email send is allowed, register it

        self._last_sent_mails.append(
            SentEmail(timestamp=now, recipient=recipient, client_ip_address=client_ip_address)
        )

        return None
