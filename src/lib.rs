/// Builds a new tokio runtime and blocks on the given future `fun`.
///
/// # Configuration
/// Configuration is done via env variables.
/// The following variables are supported:
/// - `TOKIO_ENABLE_ALL` Whether to enable all types of thread pools. Defaults to `true`.
/// - `TOKIO_BLOCKING_THREADS` The amount of blocking threads to use.
/// - `TOKIO_WORKER_THREADS` The amount of worker threads to use.
/// - `TOKIO_THREAD_STACK_SIZE` The size of the stack for the created threads.
/// - `TOKIO_THREAD_NAME` The name for the created thread pool(s).
///
/// # Defaults
/// If the environment variable is not provided, it will fall back to the tokio defaults,
/// except for the `TOKIO_ENABLE_ALL` which defaults to `true`.
///
/// So an empty configuration unfolds like this:
/// ```
/// # fn main() {
/// tokio::runtime::Builder::new_multi_thread()
///     .enable_all()
///     .map(|runtime| runtime.block_on(fun));
/// # }
/// # async fn fun() {}
/// ```
///
/// # Usage
/// Usage of this library could look like this:
/// ```
/// fn main() {
///     println!("Initializing tokio runtime...");
///     let exit_code = tokio_env::start_with(run)
///         .expect("Failed to start tokio runtime!");
///     println!("Tokio runtime exited with code: {}", exit_code)
/// }
///
/// async fn run() -> i32 {
///     println!("Program started!");
///     // Your async logic here
///     0
/// }
/// ```
#[inline]
pub fn start_with<F: core::future::Future>(fun: F) -> tokio::io::Result<F::Output> {
    let mut builder = tokio::runtime::Builder::new_multi_thread();

    if std::env::var("TOKIO_ENABLE_ALL")
        .map(|str| str.eq("true"))
        .unwrap_or(true) {
        builder.enable_all();
    }

    if let Ok(blocking_count) = std::env::var("TOKIO_BLOCKING_THREADS")
        .map(|str| str.parse::<usize>().unwrap()) {
        builder.max_blocking_threads(blocking_count);
    }

    if let Ok(worker_threads) = std::env::var("TOKIO_WORKER_THREADS")
        .map(|str| str.parse::<usize>().unwrap()) {
        builder.worker_threads(worker_threads);
    }

    if let Ok(thread_stack_size) = std::env::var("TOKIO_THREAD_STACK_SIZE")
        .map(|str| str.parse::<usize>().unwrap()) {
        builder.thread_stack_size(thread_stack_size);
    }

    if let Ok(thread_name) = std::env::var("TOKIO_THREAD_NAME") {
        builder.thread_name(thread_name);
    }

    builder.build()
        .map(|runtime| runtime.block_on(fun))
}
