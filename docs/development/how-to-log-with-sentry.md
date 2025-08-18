<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# How to log with Sentry

This documents aims to describe the absolute minimum you should know about Sentry.

The full details are available in [Sentry Developer Docs].

## Sentry 101

### Key terms

- An *event* is one instance of sending data to Sentry (generally an error or exception).
- An *issue* is a grouping of similar events.
- All events have a *fingerprint*. Events with the same fingerprint are *grouped together* into an issue
  - Sentry's grouping behaviour can be customized by either tuning the fingerprint at code-level via [SDK Fingerprinting] or adapting [Fingerprint Rules] at project-level.

### Events

Sentry events support additional context to make debugging simpler.

- Attachments: such as config files, that can be uploaded to a single event or to all events.
- Breadcrumbs: these are events that do not generate an issue. Instead, they are recorded by Sentry until an event/error occurs, then they are added to the issue as contextual information (useful for debugging!).
  - Sentry uses the 'type' and 'category' of a breadcrumb to display with different colors or icons.

See [Enriching Events] for more details.

### Tracing

TODO

### Profiling

TODO

[Sentry Developer Docs]: https://develop.sentry.dev/getting-started/
[SDK Fingerprinting]: https://docs.sentry.io/platforms/python/usage/sdk-fingerprinting/
[Fingerprint Rules]: https://docs.sentry.io/concepts/data-management/event-grouping/fingerprint-rules/
[Enriching Events]: https://docs.sentry.io/platforms/python/enriching-events/attachments/
