use std::error::Error;

use tracing::{debug, error, event, info, instrument, span, trace, warn, Level};


macro_rules! span_logs {
    ($fn:ident, $level:ident, $tag:expr) => {
        async fn $fn() {
            // const s: String = "{tag}".to_string();
            let span = span!(Level::$level, $tag); // create the span.
            let _enter = span.enter(); // enter the span.
            event!(Level::TRACE, "event macro");
            trace!("trace event");
            debug!("debug event");
            info!("info event");
            warn!("warn event");
            error!("error event");
        }
    };
}

span_logs!(trace,TRACE,"trace"); // interestingly, this one doesn't print spans in the logs. Maybe a formatting default.
span_logs!(info,INFO, "info");
span_logs!(debug,DEBUG,"debug");
span_logs!(warn,WARN,"warn");
span_logs!(error,ERROR,"error");

async fn no_span_event() {
    event!(Level::WARN, "a thing inside no span");
    debug!("another thing");
}
// instruments create spans with the functions name. default level is info. Choose another level:
// #[instrument(level = "trace")] // https://tracing.rs/tracing/attr.instrument.html
#[instrument]
async fn like_and_subscribe(argy: usize) {
    error!("like n subscribe to argy: {argy}", argy = argy);
    info!("like n subscribe to argy: {argy}", argy = argy);
    debug!("like n subscribe to argy: {argy}", argy = argy);
    trace!("like n subscribe to argy: {argy}", argy = argy);
}
// events and spans are aggregated by Subscribers.
// we have two spans: my_doot and like_and_subscribe.
fn set_up_logs() {
    // tracing_subscriber::fmt::init(); // default log level is info
    let _subscriber = tracing_subscriber::fmt()
        // .with_max_level(Level::TRACE) // all the logs
        .with_max_level(Level::INFO) // Some logs
        // .with_max_level(Level::ERROR) // only errors
        .init();
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    set_up_logs();
    trace().await;
    debug().await;
    info().await;
    warn().await;
    error().await;
    no_span_event().await;
    like_and_subscribe(42).await;
    println!("Hello, world!");
    Ok(())
}
