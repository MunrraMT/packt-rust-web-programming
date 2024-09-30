#[derive(Debug)]
pub struct JwtToken {
    pub message: String,
}

impl actix_web::FromRequest for JwtToken {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, actix_web::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let token = Self {
            message: match req.headers().get("token") {
                Some(data) => data.to_str().unwrap_or("nothing found").to_string(),
                None => "nothing found".to_string(),
            },
        };

        return futures::future::ok(token);
    }
}
