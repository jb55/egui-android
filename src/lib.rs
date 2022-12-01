#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "android")]
pub use crate::android::run_android;

pub mod app;
pub use crate::app::SimpleApp;
