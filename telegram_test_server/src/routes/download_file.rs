use std::fmt::Error;

use actix_web::{
    error::ErrorBadRequest,
    web::{self, Bytes},
    HttpResponse,
};
use futures_util::{future::ok, stream::once};

use crate::FILES;

pub async fn download_file(path: web::Path<(String, String)>) -> HttpResponse {
    if FILES
        .lock()
        .unwrap()
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
