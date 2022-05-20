#[cfg(not(target_arch = "wasm32"))]
mod default;
#[cfg(not(target_arch = "wasm32"))]
use default as platform;

pub use platform::{spawn, Task};
