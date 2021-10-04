# tokio-env

A configuration library for convenient setup of the [tokio](https://github.com/tokio-rs/tokio) runtime.

## Configuration

All configuration is made vie environmental variables. The following variables are supported:

 - `TOKIO_ENABLE_ALL` Whether to enable all types of thread pools. Defaults to `true`.
 - `TOKIO_BLOCKING_THREADS` The amount of blocking threads to use.
 - `TOKIO_WORKER_THREADS` The amount of worker threads to use.
 - `TOKIO_THREAD_STACK_SIZE` The size of the stack for the created threads.
 - `TOKIO_THREAD_NAME` The name for the created thread pool(s).
 
If the environment variable is not provided, it will fall back to the tokio defaults,
except for the `TOKIO_ENABLE_ALL` which defaults to `true`.

So an empty configuration unfolds like this:
```
tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .map(|runtime| runtime.block_on(fun));
```

## Usage
Usage of this library could look like this:
```rust
fn main() {
    println!("Initializing tokio runtime...");
    let exit_code = tokio_env::start_with(run)
        .expect("Failed to start tokio runtime!");
    println!("Tokio runtime exited with code: {}", exit_code)
}

async fn run() -> i32 {
    println!("Program started!");
    // Your async logic here
    0
}
```

## But... why?

I'm tired of writing the same boilerplate code over and over again, so I made it a one-liner!