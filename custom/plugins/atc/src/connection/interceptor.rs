use std::str::FromStr;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use tonic::{metadata::AsciiMetadataValue, service::Interceptor, Status};

pub struct ATCClientInterceptor {
    auth_token: String,
    user: String,
    password: String,
}

impl ATCClientInterceptor {
    pub fn new(auth_token: &String, user: &String, password: &String) -> Self {
        Self {
            auth_token: auth_token.clone(),
            user: user.clone(),
            password: password.clone(),
        }
    }

    fn create_auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.user, self.password);
        let encoded = BASE64.encode(credentials);
        format!("Basic {}", encoded)
    }
}

impl Interceptor for ATCClientInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        if let Ok(value) = AsciiMetadataValue::from_str(&self.auth_token) {
            request.metadata_mut().insert("auth-token", value);
        }

        match self.create_auth_header().parse() {
            Ok(auth_header) => {
                request.metadata_mut().insert("authorization", auth_header);
                Ok(request)
            }
            Err(e) => Err(Status::from_error(Box::new(e))),
        }
    }
}
