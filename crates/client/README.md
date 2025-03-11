# 🐻‍❄️📦 `charted-client`

The **charted-client** Rust crate is an official crate maintained by the **charted** team at [Noelware, LLC.](https://noelware.org) to provide a rich and easy HTTP client to interact with charted's [HTTP REST specification](https://charts.noelware.org/docs/server/latest/api).

## Example

```rust,no_run
// Cargo.toml:
//
// [dependencies]
// charted-client = "^0"
// tokio = { version = "^1", features = ["rt", "macros"] }

use charted_client::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let noel = client.users().get("noel").await?;

    // => Ok(User { .. })

    let repository = client
        .repositories()
        .get(noel.id, "ume")
        .await?;

    // => Ok(Repository { .. })

    Ok(())
}
```
