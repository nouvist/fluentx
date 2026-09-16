use std::future::Future;

use flutter_rust_bridge::JoinHandle;

use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;

mod frb_generated;

pub mod foundations {
    pub mod colors;
    pub mod event;
    pub mod window;
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let runtime = FLUTTER_RUST_BRIDGE_HANDLER.async_runtime();
    runtime.0.block_on(future)
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let runtime = FLUTTER_RUST_BRIDGE_HANDLER.async_runtime();
    runtime.0.spawn(future)
}
