// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod backoff;

pub use eventsource_stream::Event;
use eventsource_stream::{EventStreamError, Eventsource};
use futures::{Stream, StreamExt};
use reqwest::{
    header::{self, HeaderValue},
    RequestBuilder,
};

type EventStreamResult<E> = Result<Event, EventStreamError<E>>;

type BoxStream<E> = libparsec_platform_async::BoxStream<'static, EventStreamResult<E>>;

pub enum EventMessage {
    /// A new connection as been opened.
    Open,
    /// A event received from the sse stream.
    Message(Event),
}

struct SSEStream {
    backoff: backoff::Limited,
    last_event_id: String,
    builder: RequestBuilder,
    current_stream: Option<BoxStream<reqwest::Error>>,
    is_closed: bool,
}

impl SSEStream {
    fn clone_request(&self) -> Result<RequestBuilder, Error> {
        self.builder.try_clone().ok_or(Error::CannotCloneRequest)
    }
}

pub fn sse_actor(builder: RequestBuilder) -> impl Stream<Item = Result<EventMessage, Error>> {
    let builder = builder.header(
        header::ACCEPT,
        HeaderValue::from_static("text/event-stream"),
    );
    let state = SSEStream {
        backoff: backoff::Limited::new(),
        last_event_id: String::new(),
        builder,
        current_stream: None,
        is_closed: false,
    };

    futures::stream::unfold(state, |mut state| async move {
        if state.is_closed {
            log::debug!("Closing sse stream");
            return None;
        }

        let stream = if let Some(stream) = &mut state.current_stream {
            stream
        } else {
            log::debug!("No current opened stream, opening a new one ...");
            state.backoff.wait().await;
            let cloned_builder = match state.clone_request() {
                Ok(builder) => builder,
                Err(e) => {
                    // Not to be able to clone the request is a critical error
                    // Since the loop is build around that capability.
                    state.is_closed = true;
                    return Some((Err(e), state));
                }
            };
            let response = match cloned_builder.send().await {
                Ok(response) => response,
                Err(err) => {
                    return Some((Err(Error::Transport(err)), state));
                }
            };

            if let Err(err) = check_response(&response) {
                return Some((Err(err), state));
            }
            state.backoff.reset();

            let mut sse_stream = response.bytes_stream().eventsource();
            sse_stream.set_last_event_id(state.last_event_id.clone());
            state.current_stream = Some(Box::pin(sse_stream));

            log::debug!("SSE stream successfully opened");
            return Some((Ok(EventMessage::Open), state));
        };

        match stream.next().await {
            Some(Ok(event)) => {
                log::trace!(
                    "[sse-id:{}] Received event `{}` with message `{:.10}...`",
                    event.id,
                    event.event,
                    event.data
                );
                if state.last_event_id != event.id {
                    state.last_event_id = event.id.clone();
                }
                if let Some(retry) = event.retry {
                    state.backoff.set_retry(retry)
                }
                Some((Ok(EventMessage::Message(event)), state))
            }
            Some(Err(err)) => Some((Err(Error::from(err)), state)),
            None => {
                log::trace!("The stream has ended");
                state.is_closed = true;
                Some((Err(Error::StreamEnded), state))
            }
        }
    })
}

fn check_response(response: &reqwest::Response) -> Result<(), Error> {
    use reqwest::StatusCode;

    match response.status() {
        StatusCode::OK => {}
        status => return Err(Error::InvalidStatusCode(status)),
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .ok_or_else(|| Error::InvalidContentType(HeaderValue::from_static("")))?;
    let mime_type = content_type
        .to_str()
        .map_err(|_| Error::InvalidContentType(content_type.clone()))?;
    if mime_type != "text/event-stream" {
        return Err(Error::InvalidContentType(content_type.clone()));
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot clone http request")]
    CannotCloneRequest,

    #[error("Invalid content type {0:?}")]
    InvalidContentType(HeaderValue),

    #[error("Invalid http response status code {0}")]
    InvalidStatusCode(reqwest::StatusCode),

    #[error(transparent)]
    Transport(reqwest::Error),

    #[error(transparent)]
    Internal(anyhow::Error),

    #[error("sse event as ended")]
    StreamEnded,
}

impl From<EventStreamError<reqwest::Error>> for Error {
    fn from(value: EventStreamError<reqwest::Error>) -> Self {
        match value {
            EventStreamError::Utf8(e) => {
                Self::Internal(anyhow::Error::from(e).context("Invalid Utf8"))
            }
            EventStreamError::Parser(e) => {
                Self::Internal(anyhow::Error::from(e).context("Parser error"))
            }
            EventStreamError::Transport(e) => Self::Transport(e),
        }
    }
}
