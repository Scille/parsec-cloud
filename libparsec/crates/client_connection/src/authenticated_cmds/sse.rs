// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Debug, marker::PhantomData, task::Poll, time::Duration};

use data_encoding::BASE64;
use eventsource_stream::{Event, EventStreamError};

use libparsec_platform_async::{BoxStream, stream::Stream};
use libparsec_protocol::API_LATEST_MAJOR_VERSION;
use libparsec_types::ProtocolRequest;

use crate::error::ConnectionError;

pub(crate) const EVENT_STREAM_CONTENT_TYPE: &str = "text/event-stream";

pub struct SSEStream<T>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
{
    /// The dynamic stream that yield SSE event.
    ///
    /// It is dynamic because of [`reqwest::Response::bytes_stream`] that provides an `impl
    /// Stream<Item = Result<Bytes>>`.
    pub(crate) event_source: BoxStream<'static, Result<Event, EventStreamError<reqwest::Error>>>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T> Debug for SSEStream<T>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SSEStream").finish_non_exhaustive()
    }
}

#[derive(Debug, PartialEq)]
pub struct SSEEvent<T>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    T::Response: Debug + PartialEq,
{
    pub id: Option<String>,
    pub retry: Option<Duration>,
    pub message: SSEResponseOrMissedEvents<T>,
}

#[derive(Debug, PartialEq)]
pub enum SSEResponseOrMissedEvents<T>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    T::Response: Debug + PartialEq,
{
    Response(T::Response),
    Empty,
    MissedEvents,
}

impl<T> Unpin for SSEStream<T> where T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static {}

impl<T> Stream for SSEStream<T>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    T::Response: Debug + PartialEq,
{
    type Item = Result<SSEEvent<T>, ConnectionError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let event_source = self.event_source.as_mut();

        match event_source.poll_next(cx) {
            Poll::Ready(Some(Ok(raw_event))) => handle_sse_event::<T>(raw_event).map(Some),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(handle_sse_error(err)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub(crate) fn handle_sse_event<T>(
    event: Event,
) -> std::task::Poll<Result<SSEEvent<T>, ConnectionError>>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    T::Response: Debug + PartialEq,
{
    let message = match event.event.as_ref() {
        "keepalive" => SSEResponseOrMissedEvents::Empty,
        "missed_events" => SSEResponseOrMissedEvents::MissedEvents,
        "message" if event.data.is_empty() => SSEResponseOrMissedEvents::Empty,
        "message" => {
            let raw = BASE64
                .decode(event.data.as_bytes())
                .map_err(|_| ConnectionError::BadContent)?;
            match T::api_load_response(raw.as_ref()) {
                Ok(cooked) => SSEResponseOrMissedEvents::Response(cooked),
                Err(e) => {
                    log::warn!("Invalid event data: {e}");
                    // We ignore bad deserialization to avoid breaking compatibility
                    // when new event types are added to the protocol
                    SSEResponseOrMissedEvents::Empty
                }
            }
        }

        // Unknown event should still be returned given it can modify `retry` param
        _ => {
            log::warn!("Unknown event type: {}", event.event);
            SSEResponseOrMissedEvents::Empty
        }
    };

    std::task::Poll::Ready(Ok(SSEEvent {
        id: if event.id.is_empty() {
            None
        } else {
            Some(event.id)
        },
        retry: event.retry,
        message,
    }))
}

pub(crate) fn handle_sse_error(error: EventStreamError<reqwest::Error>) -> ConnectionError {
    match error {
        EventStreamError::Utf8(_) | EventStreamError::Parser(_) => ConnectionError::BadContent,
        EventStreamError::Transport(e) => ConnectionError::NoResponse(Some(e)),
    }
}

pub struct RateLimiter {
    /// How much time we tried to backoff between a reset
    attempt: usize,
    desired_duration: Option<Duration>,
}

impl RateLimiter {
    pub const fn new() -> Self {
        Self {
            attempt: 0,
            desired_duration: None,
        }
    }

    pub fn reset(&mut self) {
        self.attempt = 0;
    }

    pub fn set_desired_duration(&mut self, duration: Duration) {
        self.desired_duration = Some(duration);
    }

    pub fn set_attempt(&mut self, attempt: usize) {
        self.attempt = attempt;
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub async fn wait(&mut self) {
        let duration_to_wait = self.get_duration_to_wait();
        self.attempt += 1;
        if duration_to_wait.is_zero() {
            return;
        }
        log::info!(
            "Retrying in SSE connection in {} seconds",
            duration_to_wait.as_secs()
        );
        libparsec_platform_async::sleep(duration_to_wait).await;
        log::info!("Retrying SSE connection");
    }

    fn get_duration_to_wait(&self) -> Duration {
        self.desired_duration.unwrap_or(match self.attempt {
            0 => Duration::from_secs(0),
            1 => Duration::from_secs(1),
            2 => Duration::from_secs(5),
            3 => Duration::from_secs(10),
            4 => Duration::from_secs(30),
            _ => Duration::from_secs(60),
        })
    }
}

#[cfg(test)]
#[path = "../../tests/unit/sse_rate_limiter.rs"]
mod tests;
