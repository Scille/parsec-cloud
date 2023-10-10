// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Debug, marker::PhantomData, task::Poll, time::Duration};

use data_encoding::BASE64;
use eventsource_stream::{Event, EventStreamError};
use libparsec_platform_async::stream::Stream;
use reqwest::{header::HeaderValue, StatusCode};

use libparsec_platform_async::BoxStream;
use libparsec_protocol::API_LATEST_MAJOR_VERSION;
use libparsec_types::ProtocolRequest;

use crate::error::ConnectionError;

pub(crate) const EVENT_STREAM_CONTENT_TYPE: &str = "text/event-stream";

pub(crate) fn check_response(response: &reqwest::Response) -> Result<(), SSEConnectionError> {
    match response.status() {
        StatusCode::OK => {}
        status => return Err(SSEConnectionError::InvalidStatusCode(status)),
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .ok_or_else(|| SSEConnectionError::InvalidContentType(HeaderValue::from_static("")))?;
    let mime_type = content_type
        .to_str()
        .map_err(|_| SSEConnectionError::InvalidContentType(content_type.clone()))?;
    if mime_type != EVENT_STREAM_CONTENT_TYPE {
        return Err(SSEConnectionError::InvalidContentType(content_type.clone()));
    }
    Ok(())
}

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SSEStream").finish_non_exhaustive()
    }
}

#[derive(Debug, PartialEq)]
pub struct SSEEvent<T>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    T::Response: Debug + PartialEq,
{
    pub id: String,
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
        cx: &mut std::task::Context<'_>,
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
        "missed_events" => Ok(SSEResponseOrMissedEvents::MissedEvents),
        "message" => BASE64
            .decode(event.data.as_bytes())
            .map_err(|_| ConnectionError::BadContent)
            .and_then(|raw| {
                T::api_load_response(raw.as_ref()).map_err(|_| ConnectionError::BadContent)
            })
            .map(SSEResponseOrMissedEvents::Response),

        _ => return std::task::Poll::Pending,
    };

    std::task::Poll::Ready(message.map(|message| SSEEvent {
        id: event.id,
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

#[derive(Debug, thiserror::Error)]
pub enum SSEConnectionError {
    #[error(transparent)]
    Transport(reqwest::Error),
    #[error("Invalid content type {0:?}")]
    InvalidContentType(reqwest::header::HeaderValue),
    #[error("Invalid status code {0}")]
    InvalidStatusCode(reqwest::StatusCode),
}

impl From<SSEConnectionError> for ConnectionError {
    fn from(value: SSEConnectionError) -> Self {
        match value {
            SSEConnectionError::Transport(err) => ConnectionError::NoResponse(Some(err)),
            SSEConnectionError::InvalidContentType(_) => ConnectionError::BadContent,
            // All statuses except 200
            SSEConnectionError::InvalidStatusCode(code) => match code.as_u16() {
                415 => ConnectionError::BadContent,
                // TODO: cannot  access the response headers here...
                // 422 => crate::error::unsupported_api_version_from_headers(
                //     resp.headers(),
                // )),
                460 => ConnectionError::ExpiredOrganization,
                461 => ConnectionError::RevokedUser,
                // We typically use HTTP 503 in the tests to simulate server offline,
                // so it should behave just like if we were not able to connect
                503 => ConnectionError::NoResponse(None),
                _ => ConnectionError::InvalidResponseStatus(code),
            },
        }
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
}

impl RateLimiter {
    pub async fn wait(&mut self) {
        let duration_to_wait = self.get_duration_to_wait();
        self.attempt += 1;
        libparsec_platform_async::sleep(duration_to_wait).await;
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
#[path = "../../tests/unit/rate_limiter.rs"]
mod rate_limiter;
