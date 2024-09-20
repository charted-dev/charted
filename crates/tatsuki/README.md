# 🐻‍❄️🗻 `tatsuki`
**Tatsuki** is a dead simple asynchronous-based job scheduling library for Rust applications which can support multiple kinds of jobs that can be processed in the background.

**Tatsuki** was built to have a small, robust library that can process background jobs without any persistence. I didn't want to use [`tokio-cron-scheduler`](https://docs.rs/tokio-cron-scheduler) due to it being very heavy and I didn't want to require configuring jobs from a database.

## Crate Features
| Name | Description | Enabled by default? |
| :--- | :---------- | :------------------ |
| `tracing` | Allows the library to emit **tracing** records to better understand what Tatsuki is doing | No. |
| `chrono`  | Allows the use of the [`chrono`] library to keep track of jobs instead of using the standard library. | Yes (by the `cron` feature) |
| `tokio`   | Enables the Tokio runtime for calculating if a job should be ran or cancelled alltogether. | Yes. |
| `cron`    | Enables the use of cron jobs via the [`cron`] crate. | Yes. |
| `log` | Same as the `tracing` feature but uses [`log`] instead. | No. |

## Example
```rust,no_run
// [dependencies]
// tatsuki = "0.1"
// tokio = { version = "*", features = ["full"] }

use tatsuki::Scheduler;
```

<!--
```rust,ignore
// [dependencies]
// tatsuki = { version = "*", features = ["tokio"] }
// tokio = { version = "*", features = ["full"] }

use tatsuki::EventLoop;

#[tokio::main]
async fn main() {
    // First, we build an `EventLoop` where all jobs will live in. It will depend
    // on the Tokio runtime.
    let mut scheduler = EventLoop::tokio();

    // Now, we can schedule a job
    scheduler.first_shot("a description of the job", myjob);

    // for the sake of this example, we'll keep track how many times
    // we processed a tick
    let mut times = 0;

    // `loop` isn't necessary! you can use `scheduler.in_background()` to schedule
    // a Tokio task which will run the tick each time, the method will also check if
    // the event loop was cancelled and will never process jobs again.
    loop {
        // Call the `tick` method as it'll run the first tick and
        // processes all jobs. You should see "i was executed :D"
        // printed 500ms later.
        scheduler.tick().await;
        times += 1;

        // if 5 ticks (2.5 seconds; 2500ms) were emitted, then we break
        // out of the loop
        if times > 5 {
            break;
        }
    }

    // Retain a snapshot of the scheduler, this will be a snapshot
    // of all jobs that were processed.
    let snapshot = scheduler.snapshot();
    dbg!(snapshot);

    // if the scheduler was dropped, it'll call `scheduler.cancel()`
    // and will cancel out all jobs.
}

// simple jobs can be simple async functions, it works! error type must implement `Into<{any impl. of std::error::Error}>`.
async fn myjob() -> Result<(), Box<dyn std::error::Error>> {
    println!("i was executed :D");

    Ok(())
}
```

## Runtimes

Since **tatsuki** is runtime agnostic, you will need to implement the [`tatsuki::rt::Runtime`](https://docs.rs/tatsuki) trait to process jobs successfully. Since `tatsuki` has default implementations for `async-std` and `tokio`, just enable the crate feature and Tatsuki will use it when you call `EventLoop::new`.

## Crate Features

### `async-std` [disabled by default]

> [!WARNING]
> Using [`async-std`](https://docs.rs/async-std) is highly experimental! Things might break.

This enables the use of [`async-std`](https://docs.rs/async-std) and uses the APIs from async-std as the runtime that Tatsuki will process all jobs in.

### `tokio` [enabled by default]

This enables Tokio usage as Tatsuki's runtime, since most people use Tokio, this is enabled by default.

### `cron` [enabled by default]

Allows processing cron jobs with the [`cron`](https://docs.rs/cron) library.

### `tracing` [disabled by default]

Enables the use of the [`tracing`](https://docs.rs/tracing) crate, which will emit logs and spans for each invocation of Tatsuki.

### `log` [disabled by default]

Enables the use of the [`log`](https://docs.rs/log) crate for logging.

### `serde` [disabled by default]

Enables the use of [`serde`](https://docs.rs/serde) to provide `Serialize` and `Deserialize` types for all jobs.

### `chrono` [disabled by default]

Uses the [`chrono`](https://docs.rs/chrono) library for analyzing job execution times instead of a Unix timestamp.

## License

**tatsuki** is released under the **MIT License** with love and care by [Noel Towa](https://floofy.dev)! :polar_bear::purple_heart:

[`charted-dev/charted`]: https://github.com/charted-dev/charted
[`auguwu/tatsuki`]: https://github.com/auguwu/tatsuki
[`tokio::select`]: https://tokio.rs/tokio/tutorial/select
[`Duration`]: https://doc.rust-lang.org/stable/core/time/struct.Duration.html
[`cron`]: https://docs.rs/cron
-->
