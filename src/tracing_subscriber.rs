//! `tracing-subscriber` support for `tui-logger`

use super::TUI_LOGGER;
use log::{self, Log, Record};
use std::collections::HashMap;
use std::fmt;
use tracing_subscriber::Layer;

struct LogFormatter<'a>(&'a tracing::Event<'a>);
struct ToStringVisitor<'a, 'b>(&'b mut fmt::Formatter<'a>);

impl fmt::Display for LogFormatter<'_> {
    fn fmt<'a, 'b>(&self, f: &'b mut fmt::Formatter<'a>) -> fmt::Result {
        let mut visitor = ToStringVisitor::<'a, 'b>(f);
        self.0.record(&mut visitor);
        fmt::Result::Ok(())
    }
}

impl<'a, 'b> tracing::field::Visit for ToStringVisitor<'a, 'b> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        write!(self.0, " {}: {}", field.name(), value);
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        write!(self.0, " {}: {}", field.name(), value);
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        write!(self.0, " {}: {}", field.name(), value);
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        write!(self.0, " {}: {}", field.name(), value);
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        write!(self.0, " {}: {}", field.name(), value);
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        write!(self.0, " {}: {}", field.name(), value);
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        write!(self.0, " {}: {:?}", field.name(), value);
    }
}

#[allow(clippy::needless_doctest_main)]
///  tracing-subscriber-compatible layer that feeds messages to `tui-logger`.
///
///  ## Basic usage:
///  ```
///  //use tui_logger;
///
///  fn main() {
///     tracing_subscriber::registry()
///          .with(tui_logger::tracing_subscriber_layer())
///          .init();
///     info!(log, "Logging via tracing works!");
///  }
pub struct TuiTracingSubscriberLayer;

impl<S> Layer<S> for TuiTracingSubscriberLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let formatter = LogFormatter(event);

        let level = match *event.metadata().level() {
            tracing::Level::ERROR => log::Level::Error,
            tracing::Level::WARN => log::Level::Warn,
            tracing::Level::INFO => log::Level::Info,
            tracing::Level::DEBUG => log::Level::Debug,
            tracing::Level::TRACE => log::Level::Trace,
        };

        TUI_LOGGER.log(
            &Record::builder()
                .args(format_args!("{}", formatter))
                .level(level)
                .target(event.metadata().target())
                .file(event.metadata().file())
                .line(event.metadata().line())
                .module_path(event.metadata().module_path())
                .build(),
        );
    }
}
