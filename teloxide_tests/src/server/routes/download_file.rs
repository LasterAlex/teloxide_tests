use std::{fmt::Error, sync::Mutex};

use actix_web::{
    error::ErrorBadRequest,
    web::{self, Bytes},
    HttpResponse,
};
use futures_util::{future::ok, stream::once};

use crate::state::State;

pub async fn download_file(
    path: web::Path<(String, String)>,
    state: web::Data<Mutex<State>>,
) -> HttpResponse {
    if state
        .lock()
        .unwrap()
        .files
        .clone()
        .into_iter()
        .find(|f| f.path == path.1)
        .is_none()
    {
        return ErrorBadRequest("No such file found").into();
    }

    let stream = once(ok::<_, Error>(Bytes::copy_from_slice(
        "Hello, world!".as_bytes(),
    )));

    HttpResponse::Ok().streaming(stream)
}
