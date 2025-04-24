[![Workflow Status](https://github.com/CIAO-systems/ciao-rs/actions/workflows/build-crate.yml/badge.svg)](https://github.com/CIAO-systems/ciao-rs/actions/workflows/build-crate.yml)

# General
This is library crate that has pre-compiled protobuf messages and gRPC service stubs for the [ciao-proto project](https://github.com/CIAO-systems/ciao-proto) in [Rust](https://www.rust-lang.org/)

# Dependencies
For the protobuf/gRPC stubs, this crate uses the [tonic](https://github.com/hyperium/tonic) and the [prost](https://github.com/tokio-rs/prost) implementation for [protocol buffers](https://protobuf.dev/)

## Git dependencies
For the git dependencies (`grpc-utils-rs`), the git cli must be used, so that SSH authentiction works:
```bash
cargo --config net.git-fetch-with-cli=true fetch
```
### Configuration in `config.toml`
To make this setting available in all Cargo projects, add the following to the `~/.cargo/config.toml`:
```toml
[net]
git-fetch-with-cli = true
```

# Client implementation
To make it easier for Rust clients to access services in the CIAO gRPC backend, there is a client struct, that contains all the services and some caches.
If the `Client.services` is `None` the client is not connected.

```rust 
type TimeTrackingClient = 
     TimeTrackingServiceClient<
        InterceptedService<Channel, ClientInterceptor>>;
type DevicesClient = 
     DeviceServiceClient<
        InterceptedService<Channel, ClientInterceptor>>;

#[derive(Debug)]
struct Services {
    time_tracking: TimeTrackingClient,
    devices: DevicesClient,
}

pub struct Client {
    services: Option<Services>,
}
```
## Client interceptor for providing the API key
For general information about implementing interceptors in Rust, go the [interceptor example on GitHub](https://github.com/hyperium/tonic/tree/master/examples/src/interceptor).
The interceptor here is implemented using the `Interceptor` trait. The interceptor adds the API key from the environment/configuration (environment variable `ROCKET_CIAO_API_KEY`) as meta data value `x-api-key`

## TLS 
To connect to a gRPC backend that is using TLS, [this information](https://github.com/hyperium/tonic/issues/1811#issuecomment-2254614351) was very helpful. The code to connect looks something like this:
```rust
let url = "https://secure.example.com";
match Channel::from_shared(String::from(url)) {
    Ok(endpoint) => {
        match endpoint
              .tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
        {
            Ok(channel) => {
                match channel.connect().await {
                    Ok(channel) => {
                        // Everything fine here. 
                        // connect the gRPC with the channel
                    }
                }
            }
        }
    }
}
```

# Example
```rust
// [...]
use ciao_rs::{ciao::{interceptor::APIKeyClientInterceptor, ClientManager}, interceptors};

async fn login()  -> Result<(), Box<dyn std::error::Error>> {
    // [...]
    let url = "http://localhost:50051";
    let api_key = "top-secret-api-key";

    let mut manager = ClientManager::new(
        url,
        interceptors!(APIKeyClientInterceptor::new(
            api_key.to_string()
        )),
    )
    .await?;

    let login_result = manager
        .authentication_client
        .login_email("demo.user@ciao-systems.com", "secret")
        .await?;
    println!("{:?}", login_result.account);

    // continue ...
}
```