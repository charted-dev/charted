// 🐻‍❄️🪆 tatsuki: Dead simple job scheduling library
// Copyright (c) 2024 Noel Towa <cutie@floofy.dev>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // First, we build an `EventLoop` where all jobs will live in. It will depend
    // on the Tokio runtime.
    let mut scheduler = tatsuki::tokio();

    // Now, we can schedule a job
    // scheduler.first_shot("a description of the job", myjob);

    // for the sake of this example, we'll keep track how many times
    // we processed a tick
    let mut times = 0;

    // a loop block isn't always necessary! you can use `scheduler.schedule()` to schedule
    // event loop ticks per 500ms, for this example, we'll pretend we called `scheduler.schedule()`
    loop {
        // Call `tick` so jobs can be scheduled, if `tick` was called when an event
        // loop was cancelled to execute, then it'll not do anymore progress.
        scheduler.tick().await;

        // break out of the loop if the event loop was cancelled
        if scheduler.is_cancelled() {
            break;
        }

        times += 1;

        // if we reached the 5th iteration (~2.5 seconds), then we will cancel
        // the event loop
        if times >= 5 {
            scheduler.cancel();
        }
    }

    // Retain a snapshot of the scheduler, this will be a snapshot
    // of all jobs that were processed.
    let snapshot = scheduler.snapshot();
    dbg!(snapshot);

    // if the scheduler was dropped, it'll call `scheduler.cancel()`
    // and will cancel out all jobs.
}
