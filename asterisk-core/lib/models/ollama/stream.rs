use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Stream};
use pin_project::pin_project;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Error, Event, EventSource, RequestBuilderExt};

use crate::models::{ModelResult, ResponseStreamError};

use super::{OllamaModel, ResponseOk};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A stream of chunked responses from the OpenAI API using server-sent events.
#[pin_project]
pub struct ResponseStream {
    #[pin]
    stream: EventSource,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ResponseStream {
    /// Creates a new `ResponseStream` from the given request.
    pub fn new(request: RequestBuilder) -> Self {
        let stream = request.eventsource().unwrap();
        Self { stream }
    }

    fn map_error(err: Error) -> ModelResult<String> {
        let error = match err {
            Error::InvalidStatusCode(_, response) => ResponseStreamError::APIResponse(response),
            err => ResponseStreamError::EventSourceError(err),
        };

        Err(error.into())
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Stream for ResponseStream {
    type Item = ModelResult<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let event = ready!(this.stream.poll_next(cx));

        match event {
            Some(Ok(event)) => match event {
                Event::Message(message) => {
                    let data = message.data;
                    if data == "[DONE]" {
                        return Poll::Ready(None);
                    }

                    let body: ResponseOk = serde_json::from_str(&data)?;
                    let content = OllamaModel::extract_content_from_response_chunk(&body);

                    Poll::Ready(Some(Ok(content)))
                }
                Event::Open => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            },
            Some(Err(err)) => Poll::Ready(Some(Self::map_error(err))),
            None => Poll::Ready(None),
        }
    }
}
