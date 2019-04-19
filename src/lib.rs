/*!
A Rust RPC Client for Cloud Spanner based on Google's [Python RPC Client 
for Cloud Spanner](https://github.com/GoogleCloudPlatform/google-cloud-python/tree/master/spanner)

This crate provides a number of core abstractions to work with the RPC 
Cloud Spanner API.

# Authentication

To authenticate this application this client uses Google's [Application
Default Credentials](https://cloud.google.com/docs/authentication/production)
to find your application's credentials.

# Usage

This crate is [on crates.io](https://crates.io/crates/rustyspanner) and can be
used by adding `rustyspanner` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
rustyspanner = "0.1.0"
```

and this to your crate root:

```rust
extern crate rustyspanner;
```

The main abstraction is struct [`Client`] used manage connect to Instance
Admin API & Database Admin API.

```rust
use rustyspanner::client::Client;
let id = String::from("project_id");
let client = Client::new(id);
```

[`Client`]: client/struct.Client.html
!*/
extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate regex;
extern crate chrono;

mod google;
mod helpers;
pub mod client;
pub mod instance;
pub mod database;
pub mod operation;
pub mod status;
pub mod keyset;
pub mod session;
pub mod batch;
pub mod snapshot;
pub mod transaction;
pub mod streamed;