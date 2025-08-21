use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct UuidParam(pub Uuid);

impl<'de> Deserialize<'de> for UuidParam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::parse_str(&s)
            .map(UuidParam)
            .map_err(serde::de::Error::custom)
    }
}

impl FromRequest for UuidParam {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.match_info().get("id") {
            Some(id_str) => match Uuid::parse_str(id_str) {
                Ok(uuid) => ready(Ok(UuidParam(uuid))),
                Err(_) => ready(Err(actix_web::error::ErrorBadRequest("Invalid UUID"))),
            },
            None => ready(Err(actix_web::error::ErrorBadRequest(
                "Missing UUID parameter",
            ))),
        }
    }
}
