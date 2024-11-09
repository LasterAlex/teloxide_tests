use std::{
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
};

use futures_util::Stream;
use teloxide::{
    stop::StopToken,
    types::Update,
    update_listeners::{AsUpdateStream, UpdateListener},
    Bot, RequestError,
};

// It isn't really a listener, it just takes the updates and feeds them one by one to the
// dispather, until there is no more.
pub(crate) struct InsertingListener {
    pub updates: Vec<Update>,
}

pub(crate) struct InsertingListenerStream {
    updates: Mutex<Vec<Update>>,
}

impl Stream for InsertingListenerStream {
    type Item = Result<Update, RequestError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.updates.lock().unwrap().len() == 0 {
            // Returning Poll::Ready(None) means that there is nothing more to poll, and the
            // dispatcher closes. If we wanted it to continue, Poll::Pending is the way.
            return Poll::Ready(None);
        }
        // Returns updates one by one
        let update = self.updates.lock().unwrap().remove(0);
        Poll::Ready(Some(Ok(update)))
    }
}

impl UpdateListener for InsertingListener {
    type Err = RequestError;

    fn stop_token(&mut self) -> StopToken {
        // This is a workaround, StopToken fields are private
        let token = "1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO";
        let bot = Bot::new(token);
        teloxide::update_listeners::Polling::builder(bot)
            .build()
            .stop_token()
    }
}

impl<'a> AsUpdateStream<'a> for InsertingListener {
    type StreamErr = RequestError;
    type Stream = InsertingListenerStream;

    fn as_stream(&'a mut self) -> Self::Stream {
        InsertingListenerStream {
            updates: self.updates.clone().into(),
        }
    }
}
