use actix_web::{http::Error, FromRequest};
use futures::future::{ok, Ready};

pub struct JwToken {
    pub message: String,
}

impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let token = Self {
                    message: data.to_str().unwrap().to_string(),
                };

                ok(token)
            }

            None => {
                let token = Self {
                    message: String::from("nothing found"),
                };

                ok(token)
            }
        }
    }
}
