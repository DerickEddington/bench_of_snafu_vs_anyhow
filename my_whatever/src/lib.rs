//! As a separate crate.

use snafu::Snafu;


/// Like [`snafu::Whatever`] but optionally captures a backtrace conditionally based on the env
/// vars, similarly to how [`anyhow::Error`](
/// https://docs.rs/anyhow/latest/anyhow/struct.Error.html) does.
#[derive(Debug, Snafu)]
#[snafu(whatever)]
#[snafu(display("{message}"))]
#[snafu(provide(opt, ref, chain, dyn std::error::Error => source.as_deref()))]
pub struct MyWhatever
{
    #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
    #[snafu(provide(false))]
    source:    Option<Box<dyn std::error::Error>>,
    message:   String,
    /// This being `Option` is the difference and the purpose of using this type.
    backtrace: Option<snafu::Backtrace>,
}
