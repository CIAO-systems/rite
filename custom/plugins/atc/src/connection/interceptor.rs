use std::str::FromStr;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::Engine as _;
use tonic::{metadata::AsciiMetadataValue, service::Interceptor};

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
            Err(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use tonic::{
        metadata::{errors::InvalidMetadataValue, Ascii, AsciiMetadataValue, MetadataValue},
        service::Interceptor,
    };

    use crate::connection::interceptor::ATCClientInterceptor;

    #[test]
    fn test_new() {
        let interceptor = ATCClientInterceptor::new(
            &String::from("auth_token"),
            &String::from("user"),
            &String::from("password"),
        );

        assert_eq!(interceptor.auth_token, "auth_token");
        assert_eq!(interceptor.user, "user");
        assert_eq!(interceptor.password, "password");
    }

    #[test]
    fn test_create_auth_header() {
        let interceptor = ATCClientInterceptor::new(
            &String::from("auth_token"),
            &String::from("user"),
            &String::from("password"),
        );

        let header = interceptor.create_auth_header();
        assert_eq!(header, "Basic dXNlcjpwYXNzd29yZA==");
    }

    #[test]
    fn test_create_auth_header_parse() {
        let interceptor = ATCClientInterceptor::new(
            &String::from("auth_token"),
            &String::from("user"),
            &String::from("password"),
        );

        let header: Result<MetadataValue<Ascii>, InvalidMetadataValue> =
            interceptor.create_auth_header().parse();
        assert!(header.is_ok_and(|v| v == "Basic dXNlcjpwYXNzd29yZA=="));
    }

    #[test]
    fn test_call() {
        let mut interceptor = ATCClientInterceptor::new(
            &String::from("auth_token"),
            &String::from("user"),
            &String::from("password"),
        );

        let request = tonic::Request::new(());

        let result = interceptor.call(request);
        println!("{:?}", result);

        assert!(result.is_ok());
        let result = result.unwrap();
        let metadata = result.metadata();
        // Assert our interceptor inserted the correct headers
        assert_eq!(
            metadata.get("auth-token").unwrap(),
            &AsciiMetadataValue::from_str("auth_token").unwrap()
        );
        assert_eq!(
            metadata.get("authorization").unwrap(),
            &AsciiMetadataValue::from_str("Basic dXNlcjpwYXNzd29yZA==").unwrap()
        );
    }
}
