# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from itertools import count

from parsec._parsec import DateTime, EmailAddress
from parsec.email_rate_limit import EmailRateLimit


def test_disabled():
    now = DateTime.now()
    rl = EmailRateLimit(
        cooldown_delay=0,
        max_per_hour=0,
    )
    for _ in range(5):
        assert (
            rl.register_send_intent(
                now=now,
                client_ip_address="192.168.1.1",
                recipient=EmailAddress("zack@example.com"),
            )
            is None
        )


def test_no_ip_address():
    now = DateTime.now()
    rl = EmailRateLimit(
        cooldown_delay=1,
        max_per_hour=1,
    )
    for i in range(5):
        assert (
            rl.register_send_intent(
                now=now,
                client_ip_address="",
                recipient=EmailAddress(f"agent{i}@example.com"),
            )
            is None
        )


def test_rate_limit():
    generate_unrelated = (
        (f"192.162.1.{i}", EmailAddress(f"agent{i}@example.com")) for i in count()
    )

    rl = EmailRateLimit(
        cooldown_delay=60,
        max_per_hour=3,
    )

    t1 = DateTime(2000, 1, 1)

    # Multiple unrelated don't block each others

    for _ in range(10):
        client_ip_address, recipient = next(generate_unrelated)
        assert (
            rl.register_send_intent(
                now=t1,
                client_ip_address=client_ip_address,
                recipient=recipient,
            )
            is None
        )

    # Test cooldown delay

    t2 = DateTime(2000, 2, 1)

    previous_client_ip_address, previous_recipient = next(generate_unrelated)
    new_client_ip_address, new_recipient = next(generate_unrelated)
    for i, (client_ip_address, recipient) in enumerate(
        [
            (previous_client_ip_address, new_recipient),
            (new_client_ip_address, previous_recipient),
        ]
    ):
        t2x = t2.add(hours=24 * i)

        # First legit send
        assert (
            rl.register_send_intent(
                now=t2x,
                client_ip_address=previous_client_ip_address,
                recipient=previous_recipient,
            )
            is None
        )

        # Cannot send under 60s
        assert rl.register_send_intent(
            now=t2x.add(seconds=59),
            client_ip_address=client_ip_address,
            recipient=recipient,
        ) == t2x.add(seconds=60)

        # Can send right after 60s
        assert (
            rl.register_send_intent(
                now=t2x.add(seconds=60),
                client_ip_address=client_ip_address,
                recipient=recipient,
            )
            is None
        )

    # Test max per hour

    t3 = DateTime(2000, 3, 1)

    previous_client_ip_address, previous_recipient = next(generate_unrelated)
    new_client_ip_address, new_recipient = next(generate_unrelated)
    for i, (client_ip_address, recipient) in enumerate(
        [
            (previous_client_ip_address, new_recipient),
            (new_client_ip_address, previous_recipient),
        ]
    ):
        t3x = t3.add(hours=24 * i)

        # First legit sends
        for j in range(3):
            assert (
                rl.register_send_intent(
                    now=t3x.add(seconds=60 * j),
                    client_ip_address=previous_client_ip_address,
                    recipient=previous_recipient,
                )
                is None
            )

        # Cannot send a new email under an hour of the first email
        assert rl.register_send_intent(
            now=t3x.add(seconds=3599),
            client_ip_address=client_ip_address,
            recipient=recipient,
        ) == t3x.add(hours=1)

        # But can right after 1 hours
        assert (
            rl.register_send_intent(
                now=t3x.add(seconds=3600),
                client_ip_address=client_ip_address,
                recipient=recipient,
            )
            is None
        )
